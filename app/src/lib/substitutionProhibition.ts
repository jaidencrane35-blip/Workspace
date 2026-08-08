/**
 * P23.S2 — Substitution Prohibition Enforcement. Intent Layer.
 *
 * Workspace must never substitute a desktop effect for an answer it does not
 * have. When comprehension (P23.S1) determines that the Owner's request is
 * fulfilled by knowledge rather than by machine change, an effect resolved from
 * action-shaped vocabulary is a substitution and is refused.
 *
 * This is enforcement, not planning. It reads the Goal Contract as semantic
 * evidence and may only *remove* an effect, never choose one. It selects no
 * capability, no provider, and no execution step, and it introduces no new
 * desktop or browser behaviour.
 *
 * Constitutional position: Spec v2 forbids Meaning → Effect. The Kernel
 * Operator remains the sole composition authority; nothing here reaches it.
 */

import type { GoalContract } from "./goalContract";
import type { IntentAction } from "./intentBridge";

/** Outcomes fulfilled by knowledge, not by machine change. */
const ANSWER_OUTCOMES = new Set<GoalContract["outcome"]>([
  "KNOW",
  "COMPUTE",
  "SOCIAL",
]);

/**
 * Modes in which the Owner requested no desktop work at all. Hybrid goals are
 * excluded on purpose: "open Screenshots and count the pictures" wants both an
 * answer and desktop work, and the prohibition is about substitution, not about
 * banning desktop work whenever a request has a result.
 */
const ANSWER_ONLY_MODES = new Set<GoalContract["mode"]>([
  "information",
  "computation",
  "conversation",
]);

/** Actions that only speak. Everything else changes the machine or the surface. */
const SPEAKING_KINDS = new Set<IntentAction["kind"]>([
  "unknown",
  "capabilityExplain",
  "browserExplain",
  "voiceExplain",
  "voiceStatus",
]);

/** True when fulfilling this request means telling the Owner something. */
export function isAnswerOnlyGoal(goal: GoalContract): boolean {
  return ANSWER_OUTCOMES.has(goal.outcome) && ANSWER_ONLY_MODES.has(goal.mode);
}

/**
 * Comprehension classifies any unrecognised question as KNOW by default. A
 * default is not evidence, so it must never override a specific desktop match —
 * otherwise an approximation in the Intent Layer would silently disable real
 * capabilities. Refusing an effect requires a positive signal.
 */
export function hasPositiveOutcomeEvidence(goal: GoalContract): boolean {
  return !goal.evidence.includes("bare_question");
}

function isSpeaking(action: IntentAction): boolean {
  return SPEAKING_KINDS.has(action.kind);
}

function isSoftMiss(action: IntentAction): boolean {
  return action.kind === "unknown" && action.softMiss === true;
}

/**
 * An effect is permitted for an answer-seeking goal only when the existing
 * downstream routing authority independently selected an external information
 * mechanism. A KNOW outcome never implies it by itself.
 */
function isIndependentInformationRoute(action: IntentAction): boolean {
  return action.kind === "browserOpen" && action.informationHandoff === true;
}

/** Recover the Owner's own capitalisation for a phrase taken from normalised text. */
function ownerCasing(utterance: string, phrase: string | null): string | null {
  if (!phrase) return null;
  const at = utterance.toLowerCase().indexOf(phrase.toLowerCase());
  return at >= 0 ? utterance.slice(at, at + phrase.length) : phrase;
}

/** Describe the request in the Owner's terms — never in desktop terms. */
function describeRequest(goal: GoalContract): string {
  const subject = ownerCasing(goal.utterance, goal.subject);
  if (!subject && !goal.requestedResult) {
    // Nothing was pinned down, so quote the Owner rather than paraphrase them.
    return `“${goal.utterance.replace(/[.!?]+$/g, "")}”`;
  }
  switch (goal.requestedResult) {
    case "current time":
      return subject ? `for the current time in ${subject}` : "for the current time";
    case "current date":
      return subject ? `for the date in ${subject}` : "for today’s date";
    case "location":
      return subject ? `where ${subject} is` : "for that location";
    case "count":
      return subject ? `how many there are in ${subject}` : "for a count";
    case "list":
      return "for a list";
    case "description":
      return "for a description of that";
    default:
      return `about ${subject}`;
  }
}

function limitationReply(goal: GoalContract): IntentAction {
  if (goal.outcome === "COMPUTE") {
    return {
      kind: "unknown",
      reply:
        "You’re asking me to work that out. I can’t calculate it reliably yet, so I won’t guess at an answer.",
    };
  }
  return {
    kind: "unknown",
    reply: `I understand you’re asking ${describeRequest(goal)}. I don’t have a reliable way to get that yet, so I won’t guess — and I won’t do something on the desktop instead.`,
  };
}

function socialReply(goal: GoalContract): IntentAction {
  const text = goal.normalized;
  if (/\bhow are you\b|\bhow'?s it going\b|\bhow are things\b/.test(text)) {
    return {
      kind: "unknown",
      reply: "I’m good — here and ready when you need the desktop.",
    };
  }
  if (/\bhow'?s (?:my|your) day\b/.test(text)) {
    return {
      kind: "unknown",
      reply: "Going well, thanks for asking. I’m here whenever you need me.",
    };
  }
  return { kind: "unknown", reply: "Hi — I’m here with you on the desktop." };
}

/**
 * Refuse a desktop effect that was substituted for an answer.
 *
 * Returns the action unchanged for every goal that legitimately wants desktop
 * work, including hybrid goals and observations of the machine.
 */
export function enforceSubstitutionProhibition(
  goal: GoalContract,
  action: IntentAction,
): IntentAction {
  if (!isAnswerOnlyGoal(goal)) {
    return action;
  }

  if (goal.outcome === "SOCIAL") {
    // A greeting is never a reason to touch the desktop or to propose a command.
    if (!isSpeaking(action) || isSoftMiss(action)) {
      return socialReply(goal);
    }
    if (action.kind === "unknown" && action.suggestion) {
      return { kind: "unknown", reply: action.reply };
    }
    return action;
  }

  if (isSpeaking(action)) {
    // A truthful answer or status report stands; a bare refusal is reframed
    // around the Owner's goal. Neither can disable a capability.
    return isSoftMiss(action) ? limitationReply(goal) : action;
  }

  if (!hasPositiveOutcomeEvidence(goal) || isIndependentInformationRoute(action)) {
    return action;
  }

  return limitationReply(goal);
}
