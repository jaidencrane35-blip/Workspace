/**
 * Operator Intelligence — sole authority from Conversation to Capability Runtime.
 * Not AGI. Not autonomous. Governed decide → plan → execute → compose.
 */

import { resolveIntent, type IntentAction } from "../intentBridge";
import { composeStepReply, sanitizeUserText } from "./compose";
import { isCapabilityIntent, planFromIntent } from "./planner";
import { invokeProviderStep } from "./runtimeBridge";
import type { OperatorOutcome, OperatorPhase, ProviderStepResult } from "./types";

let phase: OperatorPhase = "idle";

export function getOperatorPhase(): OperatorPhase {
  return phase;
}

function setPhase(next: OperatorPhase): void {
  phase = next;
}

/**
 * Conversation → Operator protocol entry.
 * Shell presentation directives are returned; capability work is executed here.
 */
export async function handleOperatorUtterance(
  utterance: string,
): Promise<OperatorOutcome> {
  setPhase("interpreting");
  const intent = resolveIntent(utterance);

  if (intent.kind === "unknown") {
    setPhase("responding");
    const outcome: OperatorOutcome = {
      kind: "reply",
      text: sanitizeUserText(intent.reply),
      suggestion: intent.suggestion,
    };
    setPhase("idle");
    return outcome;
  }

  // Clarification intents arrive as unknown with helpful copy from Intent Layer,
  // or as capability kinds that plan empty — Intent Layer already clarifies
  // “Move this window.” as unknown. Capability path continues below.

  if (!isCapabilityIntent(intent)) {
    setPhase("responding");
    const outcome: OperatorOutcome = { kind: "shell", action: intent };
    setPhase("idle");
    return outcome;
  }

  setPhase("planning");
  const plan = planFromIntent(intent);
  if (!plan || plan.steps.length === 0) {
    setPhase("clarifying");
    const outcome: OperatorOutcome = {
      kind: "reply",
      text: "I need a clearer request before I can act.",
    };
    setPhase("idle");
    return outcome;
  }

  setPhase("executing");
  const results: ProviderStepResult[] = [];

  if (plan.compositionId === "app.open_or_focus" && intent.kind === "appOpen") {
    const found = await invokeProviderStep(plan.steps[0]!);
    results.push(found);
    if (found.ok && (found.items?.length ?? 0) > 0) {
      const focused = await invokeProviderStep({
        domain: "application",
        operation: "focus",
        args: { query: intent.query },
      });
      results.push(focused);
    } else {
      const launched = await invokeProviderStep({
        domain: "application",
        operation: "launch",
        args: { query: intent.query },
      });
      results.push(launched);
    }
  } else {
    for (const step of plan.steps) {
      const result = await invokeProviderStep(step);
      results.push(result);
      if (!result.ok) {
        break;
      }
    }
  }

  setPhase("responding");
  const text = composeStepReply(intent, plan, results);
  setPhase("idle");
  return { kind: "reply", text };
}

/** Test helper — plan without executing. */
export function inspectOperatorPlan(action: IntentAction) {
  return planFromIntent(action);
}
