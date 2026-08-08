/**
 * Conversation façade — Intent Layer + single Kernel Operator IPC.
 * No provider composition, execution order, or permission policy in TypeScript.
 */

import { resolveIntent } from "../intentBridge";
import { invokeIpc } from "../ipc";
import { getVoiceStatus } from "../voice";
import { isCapabilityIntentAction, toCapabilityIntent } from "./intentMap";
import { executeCapabilityIntent } from "./runtimeBridge";
import type { OperatorOutcome } from "./types";

/** Optional Owner-facing working copy before long IPC (P17.S4). */
export type OperatorWorkingNotify = (message: string) => void;

export interface HandleOperatorUtteranceOptions {
  onWorking?: OperatorWorkingNotify;
}

/**
 * Conversation → Intent → (shell | Kernel Operator via single IPC).
 * Voice recognition is a Conversation input device — not Kernel Operator work.
 */
export async function handleOperatorUtterance(
  utterance: string,
  options?: HandleOperatorUtteranceOptions,
): Promise<OperatorOutcome> {
  const intent = resolveIntent(utterance);

  if (
    intent.kind === "unknown" ||
    intent.kind === "browserExplain" ||
    intent.kind === "voiceExplain" ||
    intent.kind === "capabilityExplain"
  ) {
    return {
      kind: "reply",
      text: intent.reply,
      suggestion: "suggestion" in intent ? intent.suggestion : undefined,
    };
  }

  if (intent.kind === "voiceStatus") {
    const status = await getVoiceStatus();
    // Status copy already tells the Owner what to do next — do not invent “Voice is ready”.
    return {
      kind: "reply",
      text: status.message,
    };
  }

  if (intent.kind === "supportBundle") {
    // P17.S4: surface prepared “Creating…” before IPC — not only after completion.
    options?.onWorking?.(intent.reply);
    try {
      const result = await invokeIpc<{ path: string; message: string }>(
        "export_support_bundle",
      );
      return { kind: "reply", text: result.message };
    } catch {
      return {
        kind: "reply",
        text: "I couldn’t create a support package. Try again in a moment.",
      };
    }
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
