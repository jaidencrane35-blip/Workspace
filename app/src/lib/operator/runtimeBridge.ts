/**
 * Sole Conversation-side bridge to Capability Runtime IPC.
 * Operator Authority: only this module may invoke provider commands.
 */

import { IpcCommandError, invokeIpc } from "../ipc";
import type { OperatorDomain, OperatorPlanStep, ProviderStepResult } from "./types";

const PROVIDER_COMMANDS = [
  "read_clipboard",
  "write_clipboard",
  "execute_application_operation",
  "execute_window_operation",
] as const;

export type ProviderCommand = (typeof PROVIDER_COMMANDS)[number];

export function isProviderCommand(command: string): boolean {
  return (PROVIDER_COMMANDS as readonly string[]).includes(command);
}

function asResult(raw: Record<string, unknown>): ProviderStepResult {
  return {
    ok: Boolean(raw.ok ?? true),
    message: typeof raw.message === "string" ? raw.message : undefined,
    text: typeof raw.text === "string" ? raw.text : undefined,
    preview: typeof raw.preview === "string" ? raw.preview : undefined,
    format: typeof raw.format === "string" ? raw.format : undefined,
    bytes: typeof raw.bytes === "number" ? raw.bytes : undefined,
    status: typeof raw.status === "string" ? raw.status : undefined,
    target: typeof raw.target === "string" ? raw.target : undefined,
    items: Array.isArray(raw.items)
      ? (raw.items as ProviderStepResult["items"])
      : undefined,
    monitors: Array.isArray(raw.monitors)
      ? (raw.monitors as ProviderStepResult["monitors"])
      : undefined,
  };
}

export async function invokeProviderStep(
  step: OperatorPlanStep,
): Promise<ProviderStepResult> {
  try {
    switch (step.domain) {
      case "clipboard":
        return invokeClipboard(step.operation, step.args ?? {});
      case "application":
        return invokeApplication(step.operation, step.args ?? {});
      case "window":
        return invokeWindow(step.operation, step.args ?? {});
      default: {
        const _exhaustive: never = step.domain;
        return { ok: false, message: `Unsupported domain: ${_exhaustive}` };
      }
    }
  } catch (error) {
    const message =
      error instanceof IpcCommandError
        ? error.message
        : "That action failed.";
    return { ok: false, message };
  }
}

async function invokeClipboard(
  operation: string,
  args: Record<string, unknown>,
): Promise<ProviderStepResult> {
  if (operation === "read") {
    const raw = await invokeIpc<Record<string, unknown>>("read_clipboard");
    return asResult({ ...raw, ok: true });
  }
  if (operation === "write") {
    const raw = await invokeIpc<Record<string, unknown>>("write_clipboard", {
      text: String(args.text ?? ""),
    });
    return asResult({ ...raw, ok: true });
  }
  return { ok: false, message: `Unsupported clipboard operation: ${operation}` };
}

async function invokeApplication(
  operation: string,
  args: Record<string, unknown>,
): Promise<ProviderStepResult> {
  const raw = await invokeIpc<Record<string, unknown>>(
    "execute_application_operation",
    {
      operation,
      query: (args.query as string | null | undefined) ?? null,
      path: (args.path as string | null | undefined) ?? null,
      hwnd: (args.hwnd as string | null | undefined) ?? null,
    },
  );
  return asResult(raw);
}

async function invokeWindow(
  operation: string,
  args: Record<string, unknown>,
): Promise<ProviderStepResult> {
  const raw = await invokeIpc<Record<string, unknown>>(
    "execute_window_operation",
    {
      operation,
      query: (args.query as string | null | undefined) ?? null,
      path: (args.path as string | null | undefined) ?? null,
      hwnd: (args.hwnd as string | null | undefined) ?? null,
      pid: (args.pid as number | null | undefined) ?? null,
      x: (args.x as number | null | undefined) ?? null,
      y: (args.y as number | null | undefined) ?? null,
      width: (args.width as number | null | undefined) ?? null,
      height: (args.height as number | null | undefined) ?? null,
      monitor_index:
        (args.monitor_index as number | null | undefined) ??
        (args.monitorIndex as number | null | undefined) ??
        null,
      snap: (args.snap as string | null | undefined) ?? null,
    },
  );
  return asResult(raw);
}

export function assertDomain(domain: string): asserts domain is OperatorDomain {
  if (domain !== "clipboard" && domain !== "application" && domain !== "window") {
    throw new Error(`Unknown operator domain: ${domain}`);
  }
}
