import type { IntentAction } from "../intentBridge";

/** Capability domains registered in Capability Runtime today. */
export type OperatorDomain = "clipboard" | "application" | "window";

export interface OperatorPlanStep {
  domain: OperatorDomain;
  operation: string;
  args?: Record<string, unknown>;
}

export interface OperatorPlan {
  steps: OperatorPlanStep[];
  /** Catalogue id when multi-step / multi-domain. */
  compositionId?: string;
}

export type OperatorOutcome =
  | { kind: "reply"; text: string; suggestion?: string }
  | { kind: "shell"; action: IntentAction };

export type OperatorPhase =
  | "idle"
  | "interpreting"
  | "clarifying"
  | "planning"
  | "executing"
  | "responding";

export interface ProviderStepResult {
  ok: boolean;
  message?: string;
  text?: string;
  preview?: string;
  format?: string;
  bytes?: number;
  status?: string;
  target?: string;
  items?: Array<{ title: string; processId?: number; minimized?: boolean }>;
  monitors?: Array<{ index: number; name: string; isPrimary: boolean }>;
}
