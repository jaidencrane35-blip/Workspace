/**
 * Sole Conversation-side Capability Runtime entry (P12 Finalization).
 * Presentation Purity: no provider-specific IPC from React/TS Operator façade.
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
    const message =
      error instanceof IpcCommandError
        ? error.message
        : "That action failed.";
    return {
      ok: false,
      message,
      domain: intent.domain,
      operation: intent.operation,
      target: intent.query ?? null,
    };
  }
}
