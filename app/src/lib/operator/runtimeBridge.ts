/**
 * Sole Conversation-side Capability Runtime entry (P12 Finalization).
 * Presentation Purity: no provider-specific IPC from React/TS Operator façade.
 * P17.S1: transport/IPC catch never forwards raw Error.message to Conversation.
 */

import { IpcCommandError, invokeIpc } from "../ipc";
import type { CapabilityIntent, OperatorTurnResult } from "./types";

export const CAPABILITY_INTENT_COMMAND = "execute_capability_intent" as const;

const BANNED_PROVIDER_COMMANDS = [
  "read_clipboard",
  "write_clipboard",
  "execute_application_operation",
  "execute_window_operation",
] as const;

export function isBannedProviderCommand(command: string): boolean {
  return (BANNED_PROVIDER_COMMANDS as readonly string[]).includes(command);
}

/** Defense-in-depth Owner language when IPC fails before Kernel compose returns. */
export function composeTransportFailureMessage(intent: CapabilityIntent): string {
  switch (intent.domain) {
    case "notifications":
      return "I couldn’t show that notification.";
    case "screenshots":
      return "I couldn’t capture that.";
    case "browser":
      return "I couldn’t open that website.";
    case "clipboard":
      return "I couldn’t use the clipboard just now.";
    case "window":
      return "I couldn’t change that window just now.";
    case "application":
      return "I couldn’t open or focus that app just now.";
    default:
      return "That didn’t work. Try again in a moment.";
  }
}

export async function executeCapabilityIntent(
  intent: CapabilityIntent,
): Promise<OperatorTurnResult> {
  try {
    return await invokeIpc<OperatorTurnResult>(CAPABILITY_INTENT_COMMAND, {
      intent: {
        domain: intent.domain,
        operation: intent.operation,
        text: intent.text ?? null,
        query: intent.query ?? null,
        path: intent.path ?? null,
        hwnd: intent.hwnd ?? null,
        pid: intent.pid ?? null,
        x: intent.x ?? null,
        y: intent.y ?? null,
        width: intent.width ?? null,
        height: intent.height ?? null,
        monitorIndex: intent.monitorIndex ?? null,
        snap: intent.snap ?? null,
        title: intent.title ?? null,
        category: intent.category ?? null,
        priority: intent.priority ?? null,
        duration: intent.duration ?? null,
      },
    });
  } catch (error) {
    // Preserve engineering detail in console for local diagnose; never show to Owner.
    if (error instanceof IpcCommandError) {
      console.warn(
        "[capability] IPC failure composed for Conversation",
        intent.domain,
        intent.operation,
        error.code,
        error.message,
      );
    } else {
      console.warn(
        "[capability] unexpected failure composed for Conversation",
        intent.domain,
        intent.operation,
        error,
      );
    }
    return {
      ok: false,
      message: composeTransportFailureMessage(intent),
      domain: intent.domain,
      operation: intent.operation,
      target: intent.query ?? null,
    };
  }
}
