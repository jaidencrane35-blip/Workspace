import type { IntentAction } from "../intentBridge";

/** Mirrors kernel `CapabilityIntent` (camelCase IPC). */
export interface CapabilityIntent {
  domain: string;
  operation: string;
  text?: string | null;
  query?: string | null;
  path?: string | null;
  hwnd?: string | null;
  pid?: number | null;
  x?: number | null;
  y?: number | null;
  width?: number | null;
  height?: number | null;
  monitorIndex?: number | null;
  snap?: string | null;
  title?: string | null;
  category?: string | null;
  priority?: string | null;
  duration?: string | null;
}

/** Mirrors kernel `OperatorTurnResult`. */
export interface OperatorTurnResult {
  ok: boolean;
  message: string;
  status?: string | null;
  domain: string;
  operation: string;
  target?: string | null;
  preview?: string | null;
  text?: string | null;
  items?: Array<{ title: string }> | null;
  monitors?: Array<{
    index: number;
    name: string;
    isPrimary: boolean;
  }> | null;
  compositionId?: string | null;
}

export type OperatorOutcome =
  | { kind: "reply"; text: string; suggestion?: string }
  | { kind: "shell"; action: IntentAction };
