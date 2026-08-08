/**
 * Goal Resolution Engine (P16.36) — Intent Layer.
 *
 * Sits between Situation Goals / IntentAction and Execution Planning.
 * Deterministic only — no ML, no hidden AI, no phrase-alias catalogues.
 *
 * Produces: intended outcome, missing information, clarification need,
 * candidate plans, ranked selection, recovery strategy, completion criteria,
 * and an evidence trail.
 */

import { generateRecoveryGuidance } from "./capabilityRegistry";
import {
  buildExecutionPlan,
  type ExecutionPlan,
} from "./executionPlanner";
import type { IntentAction } from "./intentBridge";

export interface GoalCandidate {
  id: string;
  rank: number;
  rationale: string;
  plan: ExecutionPlan;
}

export interface GoalResolution {
  utterance: string;
  intendedOutcome: string;
  missingInformation: string[];
  needsClarification: boolean;
  candidates: GoalCandidate[];
  selectedPlan: ExecutionPlan;
  recoveryStrategy: string;
  completionCriteria: string[];
  evidence: string[];
  /** Action after goal resolution (may refine underspecified unknowns). */
  action: IntentAction;
  /** True when Goal Resolution changed the pre-resolution action. */
  refined: boolean;
}

function norm(raw: string): string {
  return raw
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/,/g, " ")
    .replace(/\s+/g, " ")
    .replace(/^(please|can you|could you|would you|i think)\s+/i, "")
    .replace(/\s+(please|for me|now|thanks|thank you)$/i, "")
    .trim();
}

/** Underspecified resume/workspace goals — route to Continue, never invent layouts. */
function isUnderspecifiedResume(text: string): boolean {
  return (
    /^(where was i|where am i)$/i.test(text) ||
    /\btake me where i was\b/.test(text) ||
    /\bi need (everything|it all|my (workspace|desk|desktop)) back\b/.test(text) ||
    /\bi need (my )?workspace\b/.test(text) ||
    /\bi was coding\b/.test(text) ||
    /\bi was (working|programming|developing)\b/.test(text) ||
    /\bneed everything back\b/.test(text) ||
    /\bget (everything|my workspace) back\b/.test(text)
  );
}

/** Underspecified locate — missing the target entity. */
function isUnderspecifiedLocate(text: string): boolean {
  return (
    /^(i'?ve lost it|i lost it|lost it)$/i.test(text) ||
    /^(i'?m looking for something|looking for something)$/i.test(text) ||
    /^(find it|where did it go)$/i.test(text) ||
    /^(i'?ve lost (something|that)|i lost (something|that))$/i.test(text)
  );
}

/** Pronoun / filler locate targets that must never execute as window titles. */
function isUnboundPronounQuery(query: string): boolean {
  return /^(it|that|this|something|one)(\s+now)?$/i.test(query.trim());
}

function resumeAction(): IntentAction {
  return {
    kind: "navigate",
    view: "resume",
    reply:
      "Let’s get your workspace back through Continue — pick a saved Moment to restore.",
  };
}

function clarifyLocateAction(seed: string): IntentAction {
  const recovery = generateRecoveryGuidance(seed);
  return {
    kind: "unknown",
    reply:
      "You’ve lost something on the desktop — name the window, app, or site and I’ll look.",
    suggestion: recovery.suggestion,
  };
}

function outcomeFor(action: IntentAction): string {
  switch (action.kind) {
    case "browserOpenBeside":
      return "Place a site beside another window on the desktop";
    case "winFocus":
      return "Locate and bring a window forward";
    case "navigate":
      return "Open Continue so you can approve a restore plan";
    case "capabilityExplain":
      return "Explain what Conversation can control on this desktop";
    case "appLaunch":
    case "appOpen":
      return "Open an application or known folder";
    case "browserOpen":
    case "browserOpenFocus":
      return "Open a website in the browser";
    case "winEnumerate":
      return "List windows visible on the desktop";
    case "winEnumerateControls":
      return "List UI Automation controls inside a window";
    case "winFindControl":
      return "Find a named control inside a window";
    case "winClickControl":
      return "Click a named control inside a window";
    case "winTypeControl":
      return "Type text into a named control inside a window";
    case "winWaitCondition":
      return "Wait for an observable desktop condition before continuing";
    case "unknown":
      return "Clarify or recover without inventing desktop actions";
    default:
      return action.reply || "Complete a desktop action truthfully";
  }
}

function missingFor(action: IntentAction, text: string): string[] {
  const missing: string[] = [];
  if (action.kind === "unknown") {
    if (isUnderspecifiedLocate(text)) {
      missing.push("target entity (window, app, site, or folder)");
    } else if (/done|finished/i.test(text)) {
      missing.push("which window or surface to put away");
    } else {
      missing.push("a concrete desktop target or action");
    }
  }
  if (action.kind === "navigate" && "view" in action && action.view === "resume") {
    missing.push("Owner approval of a saved Moment before restore");
  }
  if (action.kind === "winFocus" && "query" in action && /chrome|browser/i.test(action.query)) {
    // Heuristic browser focus — honest gap
    if (/browser i had|lost/i.test(text)) {
      missing.push("exact prior browser window identity (no history API)");
    }
  }
  return missing;
}

function completionFor(action: IntentAction): string[] {
  switch (action.kind) {
    case "browserOpenBeside":
      return [
        "Site resolved from known entities",
        "Beside target window located or attempted",
        "Kernel composition returns truthful result",
      ];
    case "navigate":
      return [
        "Continue surface opens",
        "No restore mutates desktop until Owner approves a plan",
      ];
    case "winFocus":
      return ["Matching window sought by title", "Truthful success or miss reported"];
    case "capabilityExplain":
      return ["Reply generated only from Capability Registry"];
    case "unknown":
      return ["No invented program or window", "Owner given a next step"];
    default:
      return ["Kernel/Conversation reports truthful outcome"];
  }
}

function recoveryFor(action: IntentAction): string {
  if (action.kind === "unknown") {
    return action.suggestion
      ? `Clarify, then try: ${action.suggestion}`
      : "Ask what I can do, or name a concrete desktop target.";
  }
  if (action.kind === "navigate") {
    return "If no Moment exists, Save one first — then we can restore from there.";
  }
  return generateRecoveryGuidance(action.kind).reply;
}

/**
 * Build candidate plans and rank them. Primary = selected action's plan.
 * Alternatives are Registry-truthful only (never invent ops).
 */
function rankCandidates(
  utterance: string,
  action: IntentAction,
): GoalCandidate[] {
  const primary = buildExecutionPlan(utterance, action);
  const candidates: GoalCandidate[] = [
    {
      id: "primary",
      rank: 1,
      rationale: "Best match for resolved goal",
      plan: primary,
    },
  ];

  // Alternative: for beside, open-without-beside is a lower-ranked fallback plan shape.
  if (action.kind === "browserOpenBeside" && "url" in action) {
    const openOnly: IntentAction = {
      kind: "browserOpen",
      url: action.url,
      reply: `Opening without beside layout.`,
    };
    candidates.push({
      id: "open-only",
      rank: 2,
      rationale: "Fallback if beside composition cannot complete",
      plan: buildExecutionPlan(utterance, openOnly),
    });
  }

  // Alternative: for resume navigation, discovery is a lower-ranked orientation path.
  if (action.kind === "navigate") {
    const discover: IntentAction = {
      kind: "capabilityExplain",
      reply: "Describe capabilities from the Registry.",
    };
    candidates.push({
      id: "discover",
      rank: 3,
      rationale: "Orient if Continue is not what you wanted",
      plan: buildExecutionPlan(utterance, discover),
    });
  }

  return candidates.sort((a, b) => a.rank - b.rank);
}

/**
 * Resolve goal structure from an IntentAction. May refine underspecified unknowns.
 */
export function resolveGoal(
  utterance: string,
  incoming: IntentAction,
): GoalResolution {
  const text = norm(utterance);
  let action = incoming;
  let refined = false;
  const evidence: string[] = [`incoming_kind=${incoming.kind}`];

  if (isUnderspecifiedResume(text) && action.kind !== "navigate") {
    action = resumeAction();
    refined = true;
    evidence.push("underspecified_resume→continue");
  } else if (
    (action.kind === "winFocus" &&
      "query" in action &&
      isUnboundPronounQuery(action.query)) ||
    (isUnderspecifiedLocate(text) &&
      !(
        action.kind === "winFocus" &&
        "query" in action &&
        typeof action.query === "string" &&
        action.query.length > 0 &&
        !isUnboundPronounQuery(action.query)
      ))
  ) {
    // Pronoun / empty locate targets must clarify — never focus “it”.
    // If Workspace Context already bound a concrete query, keep it.
    if (action.kind !== "unknown" || isUnderspecifiedLocate(text)) {
      const alreadyClarify =
        action.kind === "unknown" &&
        /what are you looking for/i.test(action.reply);
      if (!alreadyClarify) {
        action = clarifyLocateAction(text);
        refined = true;
        evidence.push("underspecified_locate→clarify");
      }
    }
  }

  const missingInformation = missingFor(action, text);
  const needsClarification =
    action.kind === "unknown" || missingInformation.some((m) => /target entity|put away/i.test(m));

  const candidates = rankCandidates(utterance, action);
  const selectedPlan = candidates[0]!.plan;

  evidence.push(
    `candidates=${candidates.length}`,
    `selected=${candidates[0]!.id}`,
    `clarify=${needsClarification}`,
    `refined=${refined}`,
  );

  return {
    utterance: utterance.trim(),
    intendedOutcome: outcomeFor(action),
    missingInformation,
    needsClarification,
    candidates,
    selectedPlan,
    recoveryStrategy: recoveryFor(action),
    completionCriteria: completionFor(action),
    evidence,
    action,
    refined,
  };
}

/** Apply Goal Resolution to an IntentAction (runtime Intent Layer finalize). */
export function applyGoalResolution(
  utterance: string,
  action: IntentAction,
): IntentAction {
  return resolveGoal(utterance, action).action;
}
