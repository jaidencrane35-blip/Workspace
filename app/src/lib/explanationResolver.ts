/**
 * Experience explanation resolver (Sprint 130).
 *
 * Translates stable AttentionReason.explanation_key values into user-facing
 * DisplayReason wording. Mirrors packages/kernel/src/services/explanation_resolver.rs.
 * Experience owns human wording; Domain keeps signal / source / weight / key.
 */

import type { AttentionReason, DecisionReason } from "../types/domain";

export type DisplayImportance = "high" | "medium" | "low";

export interface DisplayReason {
  title: string;
  description: string;
  importance: DisplayImportance;
  explanation_key: string;
  signal: string;
  source: string;
  weight: number;
  known: boolean;
}

export function displayImportanceFromWeight(weight: number): DisplayImportance {
  const magnitude = Math.abs(weight);
  if (magnitude >= 50) return "high";
  if (magnitude >= 25) return "medium";
  return "low";
}

function fallbackTitle(signal: string): string {
  switch (signal) {
    case "outstanding_decision":
      return "Outstanding decision needs attention";
    case "blocked_action":
      return "Blocked action needs attention";
    case "blocked_task":
      return "Blocked task needs attention";
    case "waiting_task":
      return "Waiting task needs attention";
    case "in_progress_task":
      return "In-progress task needs attention";
    case "interrupted_work":
      return "Interrupted work needs attention";
    case "resumable_work":
      return "Resumable work needs attention";
    case "current_focus":
      return "Current focus needs attention";
    case "dormant_work":
      return "Dormant work needs attention";
    case "commitment_pending":
      return "Pending commitment needs attention";
    case "environment_disconnect":
      return "Environment disconnect needs attention";
    case "missing_application":
      return "Missing application needs attention";
    case "composition_gap":
      return "Composition gap needs attention";
    case "high_priority_intent":
      return "High-priority intent needs attention";
    case "purpose_obstacle":
      return "Purpose obstacle needs attention";
    case "purpose_outcome":
      return "Purpose outcome needs attention";
    case "evolution_insight":
      return "Evolution insight needs attention";
    case "activity_progress":
      return "Recent activity needs attention";
    case "recommendation_candidate":
      return "Recommendation needs attention";
    case "pattern_observation":
      return "Pattern observation needs attention";
    default:
      return `Attention signal '${signal}' needs attention`;
  }
}

function exactCatalog(key: string): [string, string] | null {
  switch (key) {
    case "decision.base.blocker":
      return [
        "Blocked decision needs attention",
        "because a blocked Decision Queue item is stopping progress",
      ];
    case "decision.base.outstanding":
      return [
        "Outstanding decision needs attention",
        "because a pending Decision Queue item still needs a human choice",
      ];
    case "decision.priority.critical":
      return [
        "Critical decision raises focus",
        "because this Decision Queue item is marked critical",
      ];
    case "decision.priority.high":
      return [
        "High-priority decision raises focus",
        "because this Decision Queue item is marked high priority",
      ];
    case "decision.priority.normal":
      return [
        "Decision contributes to focus",
        "because this Decision Queue item carries normal priority",
      ];
    case "decision.deferred":
      return [
        "Deferred decision still matters",
        "because a deferred Decision Queue item remains unresolved",
      ];
    case "continuity.interrupted":
      return [
        "Interrupted work needs attention",
        "because Continuity shows work that was left unfinished",
      ];
    case "continuity.resumable":
      return [
        "Resumable work is ready",
        "because Continuity can pick up where you left off",
      ];
    case "continuity.commitment":
      return [
        "Pending commitment needs attention",
        "because an automation commitment is still awaiting resolution",
      ];
    case "continuity.dormant":
      return [
        "Dormant work needs attention",
        "because Continuity shows work that has gone quiet",
      ];
    case "continuity.focus":
      return [
        "Current focus needs attention",
        "because Continuity identifies this as the active focus",
      ];
    case "activity.progress":
      return [
        "Recent progress is worth noticing",
        "because Activity Graph recorded related work recently",
      ];
    case "purpose.outcome":
      return [
        "Purpose progress is visible",
        "because Purpose reports meaningful progress toward the goal",
      ];
    case "evolution.insight":
      return [
        "Workspace evolution needs attention",
        "because Evolution surfaced how work recently changed",
      ];
    case "recommendation.candidate":
      return [
        "A next-step suggestion is available",
        "because the Recommendation Engine proposed a candidate action",
      ];
    case "pattern.observation":
      return [
        "A work pattern was observed",
        "because Pattern Model noticed a recurring workspace signal",
      ];
    default:
      return null;
  }
}

function lookupKey(key: string): [string, string] | null {
  const exact = exactCatalog(key);
  if (exact) return exact;

  if (key.startsWith("task.base.")) {
    const rest = key.slice("task.base.".length);
    switch (rest) {
      case "blocked":
        return [
          "Blocked task needs attention",
          "because work is currently waiting on completion",
        ];
      case "waiting":
        return [
          "Waiting task needs attention",
          "because this task is waiting on a dependency",
        ];
      case "in_progress":
        return [
          "In-progress task needs attention",
          "because active work is underway and still open",
        ];
      case "ready":
        return [
          "Ready task needs attention",
          "because open work is ready to continue",
        ];
      default:
        return [
          "Open task needs attention",
          "because Task Graph still lists this work as open",
        ];
    }
  }

  if (key.startsWith("task.priority.")) {
    const rest = key.slice("task.priority.".length);
    switch (rest) {
      case "critical":
        return [
          "Critical priority raises focus",
          "because this task is marked critical",
        ];
      case "high":
        return [
          "High priority raises focus",
          "because this task is marked high priority",
        ];
      case "normal":
        return [
          "Normal priority contributes to focus",
          "because this task carries a normal priority band",
        ];
      case "low":
        return [
          "Low priority still contributes",
          "because open low-priority work remains on the graph",
        ];
      default:
        return [
          "Task priority contributes to focus",
          "because Task Graph priority influenced this ranking",
        ];
    }
  }

  if (key.startsWith("purpose.obstacle.")) {
    const rest = key.slice("purpose.obstacle.".length);
    if (rest === "blocked_task") {
      return [
        "Purpose blocked by a task",
        "because a blocked Task Graph node sits on the path to Purpose",
      ];
    }
    if (rest === "blocked_work") {
      return [
        "Purpose blocked by open work",
        "because blocked Continuity work is stalling Purpose progress",
      ];
    }
    if (rest === "interrupted_work") {
      return [
        "Purpose interrupted",
        "because interrupted work is pulling focus away from Purpose",
      ];
    }
    if (rest === "outstanding_decisions") {
      return [
        "Purpose waiting on decisions",
        "because outstanding decisions gate Purpose progress",
      ];
    }
    if (rest.startsWith("composition:")) {
      return [
        "Purpose blocked by composition",
        "because a Composition gap is obstructing Purpose progress",
      ];
    }
    return [
      "Purpose obstacle needs attention",
      "because Purpose reports an obstacle on the current path",
    ];
  }

  if (key.startsWith("composition.gap.")) {
    const rest = key.slice("composition.gap.".length);
    if (rest === "disconnected_work") {
      return [
        "Composition is disconnected from work",
        "because the working environment does not match active work",
      ];
    }
    if (rest === "missing_application") {
      return [
        "Composition is missing an application",
        "because a required application is not present in the composition",
      ];
    }
    return [
      "Composition gap needs attention",
      "because Composition reports a membership gap",
    ];
  }

  if (key.startsWith("environment.gap.")) {
    const rest = key.slice("environment.gap.".length);
    if (rest === "disconnected_work") {
      return [
        "Desktop is disconnected from work",
        "because open windows do not align with active work",
      ];
    }
    if (rest === "missing_application") {
      return [
        "Required application is missing",
        "because Environment expects an application that is not present",
      ];
    }
    return [
      "Environment gap needs attention",
      "because Environment reports a desktop–work gap",
    ];
  }

  return null;
}

/** Resolve one Attention reason into display wording. Deterministic; no ranking. */
export function resolveAttentionReason(reason: AttentionReason): DisplayReason {
  const importance = displayImportanceFromWeight(reason.weight);
  const looked = lookupKey(reason.explanation_key);
  if (looked) {
    const [title, description] = looked;
    return {
      title,
      description,
      importance,
      explanation_key: reason.explanation_key,
      signal: reason.signal,
      source: reason.source,
      weight: reason.weight,
      known: true,
    };
  }
  return {
    title: fallbackTitle(reason.signal),
    description: `No Experience translation for '${reason.explanation_key}' yet (signal ${reason.signal} from ${reason.source}, weight ${reason.weight}).`,
    importance,
    explanation_key: reason.explanation_key,
    signal: reason.signal,
    source: reason.source,
    weight: reason.weight,
    known: false,
  };
}

/** Resolve many reasons in input order — no reordering, no filtering. */
export function resolveAttentionReasons(
  reasons: AttentionReason[],
): DisplayReason[] {
  return reasons.map(resolveAttentionReason);
}

/**
 * Decision Engine reasons: prefer Attention translation when present;
 * otherwise keep Decision's own factual summary as the display title.
 */
export function resolveDecisionReason(reason: DecisionReason): DisplayReason {
  if (reason.attention_reason) {
    return resolveAttentionReason(reason.attention_reason);
  }
  return {
    title: reason.summary,
    description: reason.kind,
    importance: "medium",
    explanation_key: `decision.${reason.kind}`,
    signal: reason.kind,
    source: "decision_engine",
    weight: 0,
    known: true,
  };
}
