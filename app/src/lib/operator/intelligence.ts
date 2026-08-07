/**
 * Conversation façade — Intent Layer + single Kernel Operator IPC.
 * No provider composition, execution order, or permission policy in TypeScript.
 */

import { resolveIntent } from "../intentBridge";
import { isCapabilityIntentAction, toCapabilityIntent } from "./intentMap";
import { executeCapabilityIntent } from "./runtimeBridge";
import type { OperatorOutcome } from "./types";

/**
 * Conversation → Intent → (shell | Kernel Operator via single IPC).
 */
export async function handleOperatorUtterance(
  utterance: string,
): Promise<OperatorOutcome> {
  const intent = resolveIntent(utterance);

  if (intent.kind === "unknown" || intent.kind === "browserExplain") {
    return {
      kind: "reply",
      text: intent.reply,
      suggestion: "suggestion" in intent ? intent.suggestion : undefined,
    };
  }

  if (!isCapabilityIntentAction(intent)) {
    return { kind: "shell", action: intent };
  }

  const capabilityIntent = toCapabilityIntent(intent);
  if (!capabilityIntent) {
    return {
      kind: "reply",
      text: "I need a clearer request before I can act.",
    };
  }

  const result = await executeCapabilityIntent(capabilityIntent);
  return { kind: "reply", text: result.message };
}
