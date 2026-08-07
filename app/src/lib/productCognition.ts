/**
 * Product Cognition assessment (P16.35) — evidence classification only.
 *
 * Classifies Intent outcomes for Product Proof batteries.
 * Does not expand grammar or aliases. Does not call providers.
 */

import type { IntentAction } from "./intentBridge";
import type { ExecutionPlan } from "./executionPlanner";

export type CognitionOutcome =
  | "supported"
  | "partially_supported"
  | "unsupported"
  | "misunderstood";

export interface CognitionAssessment {
  utterance: string;
  outcome: CognitionOutcome;
  kind: string;
  reason: string;
}

/** Expected family for hostile batteries — what “good” looks like. */
export type CognitionExpectation =
  | { family: "locate"; kinds: string[] }
  | { family: "beside"; kinds: string[] }
  | { family: "discovery"; kinds: string[] }
  | { family: "resume"; kinds: string[] }
  | { family: "screenshots_folder"; kinds: string[] }
  | { family: "end_session"; kinds: string[] }
  | { family: "open_site"; kinds: string[] }
  | { family: "open_app"; kinds: string[] }
  | { family: "window_ops"; kinds: string[] }
  | { family: "enumerate"; kinds: string[] }
  | { family: "honest_unknown"; kinds: string[] };

export function assessCognition(
  utterance: string,
  action: IntentAction,
  expectation: CognitionExpectation,
  plan?: ExecutionPlan,
): CognitionAssessment {
  const kind = action.kind;
  const reply = action.reply ?? "";

  if (/Provider|Registry|Kernel|WinRT/i.test(reply)) {
    return {
      utterance,
      outcome: "misunderstood",
      kind,
      reason: "exposed internals",
    };
  }

  if (
    kind === "proposal" &&
    expectation.family === "discovery"
  ) {
    return {
      utterance,
      outcome: "misunderstood",
      kind,
      reason: "discovery routed to product-evolution proposal",
    };
  }

  if (
    "query" in action &&
    typeof action.query === "string" &&
    /\.exe$/i.test(action.query) &&
    !/^shell:/i.test(action.query) &&
    !/^ms-/i.test(action.query)
  ) {
    return {
      utterance,
      outcome: "misunderstood",
      kind,
      reason: "invented executable query",
    };
  }

  if (expectation.kinds.includes(kind)) {
    const partial =
      (expectation.family === "screenshots_folder" &&
        /can’t filter|cannot filter|yesterday/i.test(reply)) ||
      (expectation.family === "resume" &&
        /don’t invent|cannot reconstruct|approve/i.test(reply)) ||
      (expectation.family === "end_session" && kind === "unknown");

    if (expectation.family === "beside" && plan && !plan.multiStep) {
      return {
        utterance,
        outcome: "partially_supported",
        kind,
        reason: "beside without multi-step plan",
      };
    }

    return {
      utterance,
      outcome: partial ? "partially_supported" : "supported",
      kind,
      reason: partial ? "truthful partial (honest limit)" : "matched expectation",
    };
  }

  if (kind === "unknown") {
    return {
      utterance,
      outcome: "unsupported",
      kind,
      reason: "unknown / recovery",
    };
  }

  return {
    utterance,
    outcome: "misunderstood",
    kind,
    reason: `expected one of [${expectation.kinds.join(", ")}]`,
  };
}
