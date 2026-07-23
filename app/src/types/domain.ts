export interface ResourceRef {
  kind: "workspace" | "zone" | "application" | "widget";
  id: string;
}

export interface Workspace {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface Zone {
  id: string;
  workspace_id: string;
  name: string;
  position_metadata: string | null;
}

export type SuggestionType =
  | "workspace_activity"
  | "resource_growth"
  | "layout_activity";

export type SuggestionStatus =
  | "pending"
  | "accepted"
  | "rejected"
  | "expired";

export interface SuggestionConfidence {
  supporting_observation_count: number;
  basis: string;
}

export interface Suggestion {
  id: string;
  suggestion_type: SuggestionType;
  title: string;
  description: string;
  source_context: string;
  related_resource_ref: ResourceRef | null;
  confidence: SuggestionConfidence;
  status: SuggestionStatus;
}

export type SuggestionLifecycleState =
  | "created"
  | "presented"
  | "accepted"
  | "rejected"
  | "expired";

export interface SuggestionLifecycleRecord {
  suggestion_id: string;
  state: SuggestionLifecycleState;
  occurred_at: string;
  actor_type: string;
  actor_id: string | null;
  related_resource_ref: ResourceRef | null;
  metadata: string | null;
}

export interface SuggestionIntentRequest {
  suggestion_id: string;
  intent_id: string;
  resource_ref: ResourceRef | null;
  created_at: string;
  actor_type: string;
  metadata: string | null;
}

export type IntentExecutionStatus =
  | "requested"
  | "authorized"
  | "executed"
  | "failed"
  | "cancelled";

export interface IntentExecutionRequest {
  id: string;
  suggestion_id: string;
  action_intent_id: string;
  actor_type: string;
  actor_id: string | null;
  status: IntentExecutionStatus;
  created_at: string;
}

export type ExecutionOutcomeStatus = "completed" | "failed" | "cancelled";

export interface ExecutionOutcome {
  execution_request_id: string;
  status: ExecutionOutcomeStatus;
  command_name: string;
  completed_at: string;
  success: boolean;
  failure_reason: string | null;
  suggestion_id: string | null;
  intent_id: string | null;
}

export type ExecutionState =
  | "unknown"
  | "completed"
  | "failed"
  | "cancelled";

export interface ExecutionReconciliation {
  execution_request_id: string;
  current_state: ExecutionState;
  dispatch_allowed: boolean;
  cancellation_allowed: boolean;
}

export interface CancellationRequest {
  id: string;
  execution_request_id: string;
  requested_at: string;
  requested_by: string;
  reason: string;
  status: "requested" | "approved" | "rejected";
}

export interface ExecutionContextSummary {
  recent_completed_count: number;
  recent_failed_count: number;
  recent_cancelled_count: number;
  last_execution_time: string | null;
  recent_commands: string[];
}

export interface WorkspaceMetrics {
  observation_count: number;
  resource_change_count: number;
  [key: string]: unknown;
}

export interface WorkspaceContext {
  generated_at: string;
  workspace: ResourceRef;
  snapshot: {
    workspace_name: string;
    zones: unknown[];
    applications: unknown[];
    widgets: unknown[];
  };
  observations: unknown[];
  metrics: WorkspaceMetrics;
  capabilities: {
    capabilities: unknown[];
    available_intents: unknown[];
  };
  execution_context: ExecutionContextSummary;
  execution_states: ExecutionReconciliation[];
}
