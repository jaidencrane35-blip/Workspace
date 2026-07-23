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

export interface DesktopWindowSnapshot {
  hwnd: string;
  title: string;
  process_id: number;
  visible: boolean;
}

export interface ApplicationReference {
  id: string;
  workspace_id: string;
  name: string;
  identifier: string | null;
  executable_path: string | null;
}

export interface ApplicationLaunchResult {
  application_id: string;
  name: string;
  executable_path: string;
  process_id: number | null;
  simulated: boolean;
}

export type PermissionApprovalStatus = "pending" | "approved" | "denied";

export interface PermissionApprovalRequest {
  id: string;
  created_at: string;
  status: PermissionApprovalStatus;
  requesting_actor_type: string;
  requesting_actor_id: string;
  command_name: string;
  capability: string;
  subject: string;
  intent_type: string;
  reason: string;
  decided_at: string | null;
  decided_by_actor_id: string | null;
}

export interface CapabilityGrant {
  id: string;
  approval_request_id: string;
  grantee_actor_id: string;
  capability: string;
  command_name: string | null;
  grant_kind: "allow_once" | "lasting";
  status: "active" | "consumed" | "revoked";
  created_at: string;
  consumed_at: string | null;
}

export interface ApprovalDecisionResult {
  request: PermissionApprovalRequest;
  grant: CapabilityGrant | null;
}

export interface AiGoal {
  id: string;
  statement: string;
  requesting_actor_id: string;
  created_at: string;
}

export interface AiActionProposal {
  id: string;
  goal_id: string;
  action_intent_id: string;
  target_resource: ResourceRef | null;
  command_name: string;
  explanation: string | null;
  created_at: string;
}

export interface AiPlan {
  goal: AiGoal;
  proposals: AiActionProposal[];
}

export type AiProposalAuthorityOutcome =
  | { kind: "allowed" }
  | { kind: "denied"; reason: string }
  | {
      kind: "approval_required";
      reason: string;
      approval_request_id: string;
    };

export interface AiProposalSubmission {
  proposal: AiActionProposal;
  request: {
    requesting_actor_id: string;
    command_name: string;
    goal_id: string | null;
    proposal_id: string | null;
    reason: string | null;
  };
  outcome: AiProposalAuthorityOutcome;
}

export interface AiPlanSubmissionResult {
  plan: AiPlan;
  submissions: AiProposalSubmission[];
}

export type AiProposalValidity =
  | { kind: "valid" }
  | { kind: "invalid"; reason: string };

export type AiProposalRelevance =
  | { kind: "relevant" }
  | { kind: "irrelevant"; reason: string }
  | { kind: "unknown" };

export type AiProposalQualityIssue =
  | { kind: "invalid"; reason: string }
  | { kind: "irrelevant"; reason: string }
  | { kind: "duplicate"; of_proposal_id: string }
  | { kind: "unnecessary"; reason: string };

export type AiProposalOutcomeClass =
  | "created"
  | "not_submitted"
  | "approval_required"
  | "denied"
  | "succeeded"
  | "failed";

export interface AiProposalEvaluation {
  proposal_id: string;
  goal_id: string;
  command_name: string;
  target: string | null;
  validity: AiProposalValidity;
  relevance: AiProposalRelevance;
  quality_issues: AiProposalQualityIssue[];
  outcome: AiProposalOutcomeClass;
  outcome_detail: string | null;
  evaluated_at: string;
}

export interface AiEvaluationSummary {
  proposal_count: number;
  valid_count: number;
  invalid_count: number;
  relevant_count: number;
  irrelevant_count: number;
  duplicate_count: number;
  unnecessary_count: number;
  rejected_count: number;
  approval_required_count: number;
  successful_count: number;
  failed_count: number;
}

export interface AiPlanEvaluationReport {
  goal_id: string;
  goal_statement: string;
  evaluations: AiProposalEvaluation[];
  summary: AiEvaluationSummary;
  generated_at: string;
  authority_note: string;
}

export type AiOrchestratedPlanState =
  | "proposed"
  | "awaiting_approval"
  | "partially_approved"
  | "executing"
  | "completed"
  | "failed"
  | "cancelled";

export type AiPlanStepState =
  | "pending"
  | "awaiting_approval"
  | "running"
  | "completed"
  | "denied"
  | "failed"
  | "cancelled";

export interface AiPlanStep {
  id: string;
  ordinal: number;
  proposal: AiActionProposal;
  depends_on: string[];
  state: AiPlanStepState;
  approval_request_id: string | null;
  last_outcome_detail: string | null;
}

export interface AiOrchestratedPlan {
  id: string;
  goal: AiGoal;
  requesting_actor_id: string;
  steps: AiPlanStep[];
  state: AiOrchestratedPlanState;
  current_step_index: number | null;
  created_at: string;
  updated_at: string;
}

export interface ActionCatalogEntry {
  intent_id: string;
  name: string;
  description: string;
  category: string;
  command_name: string;
  capability_required: { id: string; name?: string; scope?: string };
  target_requirement: "none" | "optional" | "required";
  allowed_target_kinds: string[];
}

export interface ActionCatalog {
  entries: ActionCatalogEntry[];
  generated_at: string;
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
