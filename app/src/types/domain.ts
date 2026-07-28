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
  | "in_progress"
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

export interface ModelInvocationSummary {
  provider_id: string;
  model_id: string;
  status: string;
  candidate_count: number;
  request_id: string;
}

export interface AiPlan {
  goal: AiGoal;
  proposals: AiActionProposal[];
  model_invocation?: ModelInvocationSummary | null;
}

export type ModelProviderAvailability =
  | "available"
  | "unavailable"
  | "degraded";

export type ModelProviderCapability =
  | "text_generation"
  | "structured_proposals"
  | "planning_assist"
  | "context_interpretation";

export interface ModelProviderDescriptor {
  provider_id: string;
  model_id: string;
  display_name: string;
  version: string;
  capabilities: ModelProviderCapability[];
  availability: ModelProviderAvailability;
  runtime_metadata: string | null;
}

export type ModelResponseStatus = "success" | "failed";

export interface ModelProposalCandidate {
  command_name: string;
  target_application_id: string | null;
  explanation: string | null;
}

export interface ModelResponse {
  request_id: string;
  provider_id: string;
  model_id: string;
  status: ModelResponseStatus;
  text_output: string | null;
  proposal_candidates: ModelProposalCandidate[];
  metadata: string | null;
  error_message: string | null;
  created_at: string;
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

export type AiAssistantWorkflowState =
  | "receiving_goal"
  | "understanding"
  | "generating_plan"
  | "evaluating"
  | "awaiting_confirmation"
  | "submitting_actions"
  | "waiting_for_permission"
  | "completed"
  | "failed"
  | "cancelled";

/** Product UX labels mapped from workflow state. */
export type AiAssistantProductState =
  | "idle"
  | "understanding"
  | "planning"
  | "evaluating"
  | "awaiting_confirmation"
  | "awaiting_permission"
  | "executing"
  | "completed"
  | "failed"
  | "cancelled";

export interface AiAssistantActionExplanation {
  why_suggested: string;
  why_permission: string;
  what_if_approve: string;
  influence_tags: string[];
}

export interface AiAssistantActionPreview {
  step_id: string;
  ordinal: number;
  command_name: string;
  target: string | null;
  explanation: string | null;
  step_state: string;
  capability_hint: string | null;
  structured_explanation?: AiAssistantActionExplanation | null;
}

export interface AiAssistantPlanPreview {
  plan_id: string;
  goal_statement: string;
  actions: AiAssistantActionPreview[];
  permission_note: string;
  influence_summary?: string | null;
}

export interface AiAssistantPlanRevision {
  revision: number;
  plan_id: string;
  goal_statement: string;
  preview: AiAssistantPlanPreview;
  created_at: string;
}

export interface AiAssistantPlanComparison {
  workflow_id: string;
  left: AiAssistantPlanRevision;
  right: AiAssistantPlanRevision;
  differences: string[];
}

export interface AiAssistantWorkflow {
  id: string;
  user_goal: string;
  requesting_actor_id: string;
  state: AiAssistantWorkflowState;
  orchestrated_plan_id: string | null;
  plan_preview: AiAssistantPlanPreview | null;
  status_message: string;
  created_at: string;
  updated_at: string;
  plan_revisions?: AiAssistantPlanRevision[];
  application_ids?: string[];
  workspace_id?: string | null;
  revision_counter?: number;
}

export function assistantProductState(
  state: AiAssistantWorkflowState | null | undefined,
): AiAssistantProductState {
  if (!state) return "idle";
  switch (state) {
    case "receiving_goal":
      return "idle";
    case "understanding":
      return "understanding";
    case "generating_plan":
      return "planning";
    case "evaluating":
      return "evaluating";
    case "awaiting_confirmation":
      return "awaiting_confirmation";
    case "submitting_actions":
      return "executing";
    case "waiting_for_permission":
      return "awaiting_permission";
    case "completed":
      return "completed";
    case "failed":
      return "failed";
    case "cancelled":
      return "cancelled";
  }
}

export type MemoryType =
  | "session"
  | "workspace"
  | "user_preference"
  | "system_knowledge";

export type MemoryLifecycleState =
  | "created"
  | "updated"
  | "viewed"
  | "deleted";

export interface MemoryMetadata {
  confidence_level: number;
  occurrence_count: number;
  user_visible: boolean;
  attributes: string | null;
}

export interface MemoryEntry {
  id: string;
  memory_type: MemoryType;
  key: string;
  summary: string;
  source: string;
  workspace_id: string | null;
  lifecycle: MemoryLifecycleState;
  metadata: MemoryMetadata;
  created_at: string;
  updated_at: string;
}

export interface AiMemoryAwareness {
  entries: MemoryEntry[];
  assembled_at: string;
}

export type PreferenceCategory =
  | "workflow"
  | "application"
  | "layout"
  | "communication"
  | "planning";

export type PreferenceSource =
  | "user_defined"
  | "user_confirmed"
  | "imported";

export interface UserPreference {
  id: string;
  category: PreferenceCategory;
  key: string;
  value: string;
  label: string | null;
  source: PreferenceSource;
  confidence: number;
  scope_workspace_id: string | null;
  editable: boolean;
  attributes: string | null;
  created_at: string;
  updated_at: string;
  deleted: boolean;
}

export interface UserPreferenceProfile {
  preferences: UserPreference[];
  personalization_enabled: boolean;
  assembled_at: string;
}

export interface PersonalizedPlanComparison {
  personalized: AiPlan;
  neutral: AiPlan;
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

export type ProjectStatus = "active" | "paused" | "completed" | "archived";
export type TaskStatus =
  | "todo"
  | "in_progress"
  | "blocked"
  | "done"
  | "cancelled";
export type TaskPriority = "low" | "medium" | "high";

export interface Project {
  id: string;
  workspace_id: string;
  name: string;
  description: string | null;
  status: ProjectStatus;
  metadata: string | null;
  created_at: string;
  updated_at: string;
  deleted: boolean;
}

export interface Task {
  id: string;
  project_id: string;
  workspace_id: string;
  title: string;
  status: TaskStatus;
  priority: TaskPriority;
  created_at: string;
  updated_at: string;
  deleted: boolean;
}

export interface WorkGoal {
  id: string;
  workspace_id: string;
  project_id: string | null;
  task_id: string | null;
  description: string;
  status: "active" | "achieved" | "abandoned";
  created_at: string;
  updated_at: string;
  deleted: boolean;
}

export interface WorkflowContext {
  workspace_id: string;
  active_project_id: string | null;
  active_task_id: string | null;
  related_plan_ids: string[];
  pending_decision_notes: string[];
  blocker_notes: string[];
  updated_at: string;
}

export interface WorkspaceRecommendation {
  id: string;
  title: string;
  explanation: string;
  kind: string;
  /** Carried through from the Attention item; empty for bootstrap entries. */
  reasons: AttentionReason[];
}

export interface IntelligenceHighlight {
  id: string;
  label: string;
  summary: string;
  source: string;
}

export interface PendingDecisionSummary {
  id: string;
  summary: string;
  explanation: string;
}

export interface BlockedActionSummary {
  id: string;
  summary: string;
  explanation: string;
}

export interface RecentActivityItem {
  event_type: string;
  summary: string;
  timestamp: string;
}

export interface IntelligenceApplicationSummary {
  id: string;
  name: string;
  appears_active: boolean;
}

export type AutomationTriggerKind =
  | "manual"
  | "scheduled"
  | "event"
  | "pattern";

export type AutomationContractStatus =
  | "draft"
  | "pending_approval"
  | "approved"
  | "paused"
  | "revoked"
  | "completed";

export type AutomationContractApprovalState =
  | "not_approved"
  | "pending"
  | "approved"
  | "revoked";

export interface AutomationTriggerDefinition {
  kind: AutomationTriggerKind;
  definition: string;
}

export interface AutomationIntentDefinition {
  statement: string;
}

export interface AutomationContract {
  id: string;
  workspace_id: string;
  project_id: string;
  task_id: string | null;
  name: string;
  description: string | null;
  status: AutomationContractStatus;
  trigger_definition: AutomationTriggerDefinition;
  intent_definition: AutomationIntentDefinition;
  scope: "project" | "task";
  required_capabilities: string[];
  approval_state: AutomationContractApprovalState;
  created_by_actor: string;
  approved_by_actor: string | null;
  approved_at: string | null;
  approved_definition_fingerprint: string | null;
  created_at: string;
  updated_at: string;
  deleted: boolean;
}

export interface AutomationContractSummary {
  id: string;
  name: string;
  project_id: string;
  task_id: string | null;
  status: string;
  approval_state: string;
  intent_statement: string;
  required_capabilities: string[];
  created_by_actor: string;
  approved_by_actor: string | null;
  definition_fingerprint: string;
  approval_matches_definition: boolean;
}

export interface AutomationContractIntentRequest {
  contract_id: string;
  workspace_id: string;
  project_id: string;
  task_id: string | null;
  intent_statement: string;
  required_capabilities: string[];
  requesting_actor_id: string;
  definition_fingerprint: string;
  approved_by_actor: string | null;
  approved_at: string | null;
  governance_note: string;
  audit_metadata: string;
}

export type TriggerEventType =
  | "application_opened"
  | "workspace_changed"
  | "project_context_changed"
  | "task_state_changed"
  | "user_requested_evaluation"
  | "manual_evaluation_requested";

export interface TriggerEvent {
  id: string;
  workspace_id: string;
  event_type: TriggerEventType;
  source: string;
  context: string;
  project_id: string | null;
  task_id: string | null;
  actor_id: string;
  actor_type: string;
  created_at: string;
  authority_effect: string;
}

export type AutomationIntentProposalStatus =
  | "pending_review"
  | "accepted"
  | "rejected"
  | "expired";

export interface AutomationIntentProposal {
  id: string;
  contract_id: string;
  workspace_id: string;
  project_id: string;
  task_id: string | null;
  trigger_event_id: string;
  intent_definition: AutomationIntentDefinition;
  required_capabilities: string[];
  status: AutomationIntentProposalStatus;
  explanation: string;
  definition_fingerprint: string;
  created_at: string;
  updated_at: string;
}

export interface TriggerRejection {
  contract_id: string;
  contract_name: string;
  reason: string;
}

export interface TriggerEvaluationResult {
  trigger_event: TriggerEvent;
  proposals: AutomationIntentProposal[];
  rejections: TriggerRejection[];
  authority_effect: string;
}

export interface AutomationIntentProposalSummary {
  id: string;
  contract_id: string;
  status: string;
  explanation: string;
  intent_statement: string;
  trigger_event_id: string;
}

export interface TriggerRejectionSummary {
  contract_id: string;
  reason: string;
}

export type DecisionSourceType =
  | "intent_proposal"
  | "pending_approval"
  | "blocked_action"
  | "planning_continuation";

export type DecisionCategory =
  | "permission"
  | "automation"
  | "blocked"
  | "planning"
  | "contract_definition";

export type DecisionState =
  | "pending"
  | "viewed"
  | "deferred"
  | "dismissed"
  | "accepted"
  | "rejected"
  | "expired";

export type DecisionPriority = "critical" | "high" | "normal" | "low";

export interface DecisionItem {
  id: string;
  workspace_id: string;
  source_type: DecisionSourceType;
  source_id: string;
  category: DecisionCategory;
  title: string;
  summary: string;
  explanation: string;
  recommended_action: string;
  decision_state: DecisionState;
  priority: DecisionPriority;
  created_at: string;
  expires_at: string | null;
  actor_id: string;
  actor_type: string;
  project_id: string | null;
  required_capabilities: string[];
  handoff_command: string | null;
  authority_effect: string;
}

export interface DecisionHandoff {
  decision_item_id: string;
  source_type: DecisionSourceType;
  source_id: string;
  next_command: string;
  note: string;
  authority_effect: string;
}

export interface DecisionActionResult {
  item: DecisionItem | null;
  handoff: DecisionHandoff | null;
  delegated: boolean;
  authority_effect: string;
}

export interface DecisionQueue {
  workspace_id: string;
  generated_at: string;
  items: DecisionItem[];
  pending_count: number;
  high_priority_count: number;
  /** Terminal / orphan overlay evidence — never actionable. */
  history?: DecisionOverlayHistoryEntry[];
  history_count?: number;
  authority_effect: string;
}

export interface DecisionOverlayHistoryEntry {
  decision_item_id: string;
  source_type: DecisionSourceType;
  source_id: string;
  /** Terminal overlay state (`dismissed` or `expired`). */
  decision_state: DecisionState;
  /** Live aggregation source was absent when projected. */
  orphaned: boolean;
  updated_at: string;
  actor_id: string;
  /** Always false — history never joins actionable queues. */
  actionable: boolean;
  authority_effect: string;
}

export interface DecisionQueueSummary {
  workspace_id: string;
  generated_at: string;
  pending_count: number;
  high_priority_count: number;
  /** Actionable overlay states only. */
  items: DecisionItem[];
  /** Truncated terminal/orphan overlay evidence (never actionable). */
  history?: DecisionOverlayHistoryEntry[];
  history_count?: number;
  authority_effect: string;
}

export type ActivityType =
  | "project"
  | "task"
  | "work_goal"
  | "automation_contract"
  | "trigger_event"
  | "intent_proposal"
  | "decision_item"
  | "permission_approval"
  | "planning_continuation"
  | "blocked_action"
  | "execution_outcome"
  | "audit_signal";

export type ActivitySourceType =
  | "work_context"
  | "automation_contract"
  | "trigger_evaluation"
  | "decision_queue"
  | "permission_approval"
  | "planning"
  | "execution"
  | "audit";

export interface WorkspaceActivity {
  id: string;
  workspace_id: string;
  project_id: string | null;
  task_id: string | null;
  activity_type: ActivityType;
  source_type: ActivitySourceType;
  source_id: string;
  parent_activity_id: string | null;
  related_activity_ids: string[];
  summary: string;
  explanation: string;
  timestamp: string;
  actor_id: string;
  actor_type: string;
  unresolved: boolean;
  authority_effect: string;
}

export interface WorkspaceActivityGraph {
  workspace_id: string;
  generated_at: string;
  activities: WorkspaceActivity[];
  timeline: WorkspaceActivity[];
  relationship_count: number;
  unresolved_count: number;
  authority_effect: string;
}

export interface WorkspaceActivityGraphSummary {
  workspace_id: string;
  generated_at: string;
  activity_count: number;
  relationship_count: number;
  unresolved_count: number;
  recent_timeline: WorkspaceActivity[];
  authority_effect: string;
}

export type ContinuityFacetKind =
  | "current_focus"
  | "interrupted_work"
  | "resumable_work"
  | "outstanding_decision"
  | "dormant_project"
  | "active_commitment"
  | "recent_progress"
  | "recent_outcome"
  | "blocker"
  | "suggested_next_step";

export interface ContinuityFacet {
  id: string;
  workspace_id: string;
  kind: ContinuityFacetKind;
  title: string;
  summary: string;
  why: string;
  evidence_refs: string[];
  what_changed: string;
  source_type: string;
  source_id: string;
  project_id: string | null;
  task_id: string | null;
  authority_effect: string;
}

export interface WorkspaceContinuityState {
  workspace_id: string;
  generated_at: string;
  session_anchor: string;
  current_focus: ContinuityFacet | null;
  interrupted_work: ContinuityFacet[];
  resumable_work: ContinuityFacet[];
  outstanding_decisions: ContinuityFacet[];
  dormant_projects: ContinuityFacet[];
  active_commitments: ContinuityFacet[];
  recent_progress: ContinuityFacet[];
  recent_outcomes: ContinuityFacet[];
  blockers: ContinuityFacet[];
  suggested_next_step: ContinuityFacet | null;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceContinuitySummary {
  workspace_id: string;
  generated_at: string;
  session_anchor: string;
  current_focus: ContinuityFacet | null;
  interrupted_count: number;
  resumable_count: number;
  outstanding_decision_count: number;
  blocker_count: number;
  dormant_project_count: number;
  active_commitment_count: number;
  suggested_next_step: ContinuityFacet | null;
  recent_progress: ContinuityFacet[];
  summary: string;
  authority_effect: string;
}

export type AttentionSourceType =
  | "decision_queue"
  | "continuity"
  | "activity_graph"
  | "workflow_context"
  | "automation_contract"
  | "task_graph"
  | "environment"
  | "composition"
  | "purpose"
  | "evolution"
  | "recommendation_engine"
  | "pattern";

export type AttentionCategory =
  | "requires_decision"
  | "blocker"
  | "interrupted"
  | "resumable"
  | "informative"
  | "commitment"
  | "can_wait";

export type AttentionPriority = "critical" | "high" | "normal" | "low";
export type AttentionUrgency = "immediate" | "soon" | "whenever";
export type AttentionConfidence = "high" | "medium" | "low";
export type AttentionState =
  | "new"
  | "visible"
  | "acknowledged"
  | "deferred"
  | "resolved"
  | "expired";

export type AttentionSignal =
  | "outstanding_decision"
  | "blocked_action"
  | "blocked_task"
  | "waiting_task"
  | "in_progress_task"
  | "interrupted_work"
  | "resumable_work"
  | "current_focus"
  | "dormant_work"
  | "commitment_pending"
  | "environment_disconnect"
  | "missing_application"
  | "composition_gap"
  | "high_priority_intent"
  | "purpose_obstacle"
  | "purpose_outcome"
  | "evolution_insight"
  | "activity_progress"
  | "recommendation_candidate"
  | "pattern_observation";

/** Structured why-attention reason (independent of score math). */
export interface AttentionReason {
  source: AttentionSourceType;
  signal: AttentionSignal;
  weight: number;
  explanation_key: string;
}

export interface AttentionItem {
  id: string;
  workspace_id: string;
  source_type: AttentionSourceType;
  source_id: string;
  category: AttentionCategory;
  priority: AttentionPriority;
  urgency: AttentionUrgency;
  confidence: AttentionConfidence;
  score: number;
  score_factors: string[];
  reasons: AttentionReason[];
  title: string;
  explanation: string;
  created_at: string;
  expires_at: string | null;
  attention_state: AttentionState;
  authority_effect: string;
}

export interface WorkspaceAttentionState {
  workspace_id: string;
  generated_at: string;
  items: AttentionItem[];
  top_items: AttentionItem[];
  requires_decision_count: number;
  blocker_count: number;
  informative_count: number;
  can_wait_count: number;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceAttentionSummary {
  workspace_id: string;
  generated_at: string;
  item_count: number;
  requires_decision_count: number;
  blocker_count: number;
  informative_count: number;
  can_wait_count: number;
  top_items: AttentionItem[];
  summary: string;
  authority_effect: string;
}

export type DecisionOutcome =
  | "open"
  | "selected"
  | "dismissed"
  | "postponed"
  | "expired";

export interface DecisionReason {
  kind: string;
  summary: string;
  evidence_ref: string | null;
  /** Set when the rationale came from Attention; render `explanation_key`, not `summary`. */
  attention_reason: AttentionReason | null;
}

export interface DecisionScore {
  total: number;
  attention_contribution: number;
  memory_contribution: number;
  personalization_contribution: number;
  goal_contribution: number;
  factors: string[];
}

export interface DecisionExplanation {
  headline: string;
  reasons: DecisionReason[];
  confidence: string;
}

export interface DecisionContext {
  workspace_id: string;
  active_project_id: string | null;
  active_task_id: string | null;
  attention_item_count: number;
  memory_highlight_count: number;
  preference_highlight_count: number;
  pending_approval_count: number;
  pending_plan_count: number;
  task_graph_open_count: number;
  task_graph_blocked_count: number;
}

export interface DecisionCandidate {
  id: string;
  workspace_id: string;
  title: string;
  goal_statement: string;
  originating_goal: string | null;
  attention_item_id: string | null;
  recommendation_id: string | null;
  /** Provenance when created from DE intake; otherwise null/undefined. */
  intake_candidate_id?: string | null;
  creation_request_id?: string | null;
  package_seal_digest?: string | null;
  /** native | recommendation_intake */
  origin?: string;
  score: DecisionScore;
  explanation: DecisionExplanation;
  related_goal_ids: string[];
  pending_approval_ids: string[];
  outcome: DecisionOutcome;
  created_at: string;
  handoff_command: string;
  authority_effect: string;
}

/** DE-owned evaluation resolution — scoring-path admission only, never scores. */
export interface DecisionCandidateEvaluationResolution {
  resolution_id: string;
  workspace_id: string;
  decision_candidate_id: string;
  /** native | recommendation_intake */
  origin: string;
  /** awaiting_resolution | accepted_for_scoring | rejected_for_scoring | blocked */
  resolution_state: string;
  evaluation_complete: boolean;
  lifecycle_valid: boolean;
  provenance_valid: boolean;
  candidate_active: boolean;
  intake_candidate_id: string | null;
  creation_request_id: string | null;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
  resolved_at: string | null;
  resolution_reason: string | null;
  scoring_applied: boolean;
  ranking_applied: boolean;
  creates_decision_score: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  mutates_recommendation_engine: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned DecisionScore result — scoring only; never ranks/selects/plans. */
export interface DecisionCandidateScore {
  score_id: string;
  workspace_id: string;
  decision_candidate_id: string;
  /** native | recommendation_intake */
  origin: string;
  resolution_id: string;
  score: DecisionScore;
  scoring_factors: string[];
  scored_at: string;
  intake_candidate_id: string | null;
  creation_request_id: string | null;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
  ranking_applied: boolean;
  selects_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  mutates_recommendation_engine: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** One ordered position in DecisionCandidateRanking — not a selection. */
export interface DecisionCandidateRankingEntry {
  /** 1-based comparative position (not a winner). */
  rank: number;
  decision_candidate_id: string;
  score_id: string;
  /** native | recommendation_intake */
  origin: string;
  score_total: number;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
}

/** DE-owned comparative ranking of scored candidates — never selects/plans. */
export interface DecisionCandidateRanking {
  ranking_id: string;
  workspace_id: string;
  ranked_at: string;
  entries: DecisionCandidateRankingEntry[];
  ranking_factors: string[];
  selects_candidate: boolean;
  selected_candidate_id: string | null;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  mutates_recommendation_engine: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned selection decision after ranking — progression only; never executes/plans. */
export interface DecisionCandidateSelection {
  selection_id: string;
  workspace_id: string;
  decision_candidate_id: string;
  /** native | recommendation_intake */
  origin: string;
  /** awaiting_selection | selected | rejected | withdrawn */
  selection_state: string;
  ranking_id: string | null;
  ranking_position: number | null;
  score_id: string | null;
  has_ranking_entry: boolean;
  has_score: boolean;
  provenance_valid: boolean;
  lifecycle_valid: boolean;
  candidate_active: boolean;
  intake_candidate_id: string | null;
  creation_request_id: string | null;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
  selected_at: string | null;
  selection_reason: string | null;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  mutates_recommendation_engine: boolean;
  mutates_candidate_outcome: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned progression request after selection — never planner/execution. */
export interface DecisionCandidateProgressionRequest {
  request_id: string;
  workspace_id: string;
  decision_candidate_id: string;
  /** native | recommendation_intake */
  origin: string;
  /** pending | requested | cancelled | blocked */
  request_state: string;
  selection_id: string;
  selection_state: string;
  ranking_id: string | null;
  ranking_position: number | null;
  score_id: string | null;
  selection_valid: boolean;
  provenance_valid: boolean;
  lifecycle_valid: boolean;
  candidate_active: boolean;
  intake_candidate_id: string | null;
  creation_request_id: string | null;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
  requested_at: string | null;
  request_reason: string | null;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  mutates_recommendation_engine: boolean;
  mutates_candidate_outcome: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned progression acknowledgement — receipt only; never planner/execution. */
export interface DecisionCandidateProgressionAcknowledgement {
  acknowledgement_id: string;
  workspace_id: string;
  decision_candidate_id: string;
  /** native | recommendation_intake */
  origin: string;
  /** awaiting_acknowledgement | acknowledged | rejected | expired */
  acknowledgement_state: string;
  request_id: string;
  request_state: string;
  selection_id: string;
  ranking_id: string | null;
  score_id: string | null;
  request_valid: boolean;
  provenance_valid: boolean;
  lifecycle_valid: boolean;
  candidate_active: boolean;
  intake_candidate_id: string | null;
  creation_request_id: string | null;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
  acknowledged_at: string | null;
  acknowledgement_reason: string | null;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  mutates_recommendation_engine: boolean;
  mutates_candidate_outcome: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned evaluation origin contract — origin rules only, never scoring. */
export interface DecisionCandidateEvaluationOriginContract {
  evaluation_id: string;
  workspace_id: string;
  decision_candidate_id: string;
  /** native | recommendation_intake */
  origin: string;
  /** unevaluated | eligible_for_evaluation | blocked | evaluated */
  evaluation_state: string;
  lifecycle_valid: boolean;
  provenance_valid: boolean;
  origin_supported: boolean;
  recommendation_visible: boolean;
  package_identity_traceable: boolean;
  intake_candidate_id: string | null;
  creation_request_id: string | null;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
  evaluated_at: string | null;
  scoring_applied: boolean;
  ranking_applied: boolean;
  creates_decision_score: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  mutates_recommendation_engine: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned lifecycle integration — origin-aware, provenance-preserving. */
export interface DecisionCandidateLifecycleIntegration {
  integration_id: string;
  workspace_id: string;
  decision_candidate_id: string;
  /** native | recommendation_intake */
  origin: string;
  outcome: string;
  intake_candidate_id: string | null;
  creation_request_id: string | null;
  package_seal_digest: string | null;
  recommendation_reference: string | null;
  /** integrated | blocked | provenance_invalid */
  integration_state: string;
  provenance_immutable: boolean;
  provenance_complete: boolean;
  scoring_applied: boolean;
  ranking_applied: boolean;
  creates_decision_score: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned candidate creation boundary — may create DecisionCandidate without scoring. */
export interface DecisionEngineCandidateCreation {
  creation_id: string;
  workspace_id: string;
  intake_candidate_id: string;
  creation_request_id: string;
  recommendation_reference: string;
  package_seal_digest: string;
  /** blocked | eligible_for_creation | created */
  creation_state: string;
  decision_candidate_id: string | null;
  title: string | null;
  goal_statement: string | null;
  created_at: string | null;
  evidence: string[];
  creates_decision_candidate: boolean;
  creates_decision_score: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned candidate creation request — request only, not DecisionCandidate creation. */
export interface DecisionEngineCandidateCreationRequest {
  request_id: string;
  workspace_id: string;
  intake_candidate_id: string;
  recommendation_reference: string;
  /** not_requested | requested | rejected | created */
  request_state: string;
  promotion_boundary_state: string;
  disposition_retained: boolean;
  acceptance_active: boolean;
  seal_aligned: boolean;
  evidence: string[];
  creates_decision_candidate: boolean;
  creates_decision_score: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned promotion boundary — readiness for future DecisionCandidate promotion only. */
export interface DecisionEngineIntakePromotionBoundary {
  workspace_id: string;
  intake_candidate_id: string;
  recommendation_reference: string;
  /** not_ready | promotion_allowed | promotion_blocked | promoted */
  boundary_state: string;
  evaluation_complete: boolean;
  disposition_retained: boolean;
  intake_active: boolean;
  acceptance_active: boolean;
  seal_aligned: boolean;
  evidence: string[];
  creates_decision_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned intake disposition — lifecycle decision, not planning authority. */
export interface DecisionEngineIntakeDisposition {
  disposition_id: string;
  workspace_id: string;
  intake_candidate_id: string;
  evaluation_id: string;
  disposed_at: string;
  /** retained | dismissed | deferred */
  disposition_state: string;
  disposition_reason: string;
  creates_decision_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned intake evaluation — examination record, not planning authority. */
export interface DecisionEngineIntakeEvaluation {
  evaluation_id: string;
  workspace_id: string;
  intake_candidate_id: string;
  evaluated_at: string;
  /** evaluated | rejected | deferred */
  evaluation_state: string;
  evaluation_reason: string;
  creates_decision_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned intake candidate lifecycle — not DecisionCandidate / planner / execution lifecycle. */
export interface DecisionEngineIntakeCandidateLifecycle {
  /** active | withdrawn | invalidated */
  lifecycle_state: string;
  reason: string | null;
  updated_at: string;
  authority_effect: string;
}

/** DE-owned intake lifecycle acknowledgement — not a DecisionCandidate. */
export interface DecisionEngineIntakeCandidate {
  intake_candidate_id: string;
  workspace_id: string;
  intake_receipt_reference: string;
  recommendation_reference: string;
  package_seal_digest: string;
  acceptance_reference: string;
  compatibility_version: string;
  created_at: string;
  /** observed | ready_for_future_evaluation | blocked | withdrawn */
  state: string;
  lifecycle: DecisionEngineIntakeCandidateLifecycle;
  /** Always false — IntakeCandidate ≠ DecisionCandidate. */
  is_decision_candidate: boolean;
  creates_decision_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned observational eligibility for future candidate consideration — not a candidate. */
export interface DecisionEngineIntakeEligibility {
  workspace_id: string;
  recommendation_id: string;
  sealed_intake_package_digest: string;
  /** not_eligible | blocked | duplicate | superseded | stale | eligible */
  eligibility_state: string;
  /** Informational only — never means create candidate. */
  is_eligible: boolean;
  receipt_observed: boolean;
  seal_aligned: boolean;
  acceptance_active: boolean;
  assessment_valid: boolean;
  is_duplicate: boolean;
  is_superseded: boolean;
  is_stale: boolean;
  evidence: string[];
  creates_decision_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  decision_engine_object_id: string | null;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned observational assessment of an intake receipt — not a candidate. */
export interface DecisionEngineIntakeAssessment {
  workspace_id: string;
  recommendation_id: string;
  sealed_intake_package_digest: string;
  /** blocked | superseded | duplicate | stale | valid | eligible_for_future_candidate */
  assessment_state: string;
  receipt_observed: boolean;
  receipt_current: boolean;
  seal_valid: boolean;
  acceptance_active: boolean;
  is_duplicate: boolean;
  is_superseded: boolean;
  is_stale: boolean;
  /** Informational only — never means create candidate. */
  eligible_for_future_candidate: boolean;
  evidence: string[];
  creates_decision_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  ownership_transferred: boolean;
  decision_engine_object_id: string | null;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

/** DE-owned observational receipt of an accepted RE sealed package — not a candidate. */
export interface DecisionEngineIntakeReceipt {
  workspace_id: string;
  recommendation_id: string;
  /** observed | seal_mismatch */
  receipt_state: string;
  acceptance_state: string;
  ownership_state: string;
  ownership_transferred: boolean;
  current_owner: string;
  sealed_intake_package_digest: string;
  seal_aligned: boolean;
  contract_version: string;
  contract_family: string;
  decision_engine_object_id: string | null;
  creates_decision_candidate: boolean;
  creates_goal: boolean;
  creates_intent: boolean;
  adapter_invoked: boolean;
  planner_invoked: boolean;
  handoff_command: string | null;
  note: string;
  authority_effect: string;
}

export interface DecisionEngineState {
  workspace_id: string;
  generated_at: string;
  context: DecisionContext;
  candidates: DecisionCandidate[];
  top_candidates: DecisionCandidate[];
  /** Observational RE intake receipts — never DecisionCandidates. */
  intake_receipts?: DecisionEngineIntakeReceipt[];
  /** Observational intake assessments — never DecisionCandidates. */
  intake_assessments?: DecisionEngineIntakeAssessment[];
  /** Observational intake eligibility — never DecisionCandidates. */
  intake_eligibilities?: DecisionEngineIntakeEligibility[];
  /** DE-owned intake lifecycle acknowledgements — never DecisionCandidates. */
  intake_candidates?: DecisionEngineIntakeCandidate[];
  /** DE-owned intake evaluations — never DecisionCandidates or planning authority. */
  intake_evaluations?: DecisionEngineIntakeEvaluation[];
  /** DE-owned intake dispositions — never DecisionCandidates or planning authority. */
  intake_dispositions?: DecisionEngineIntakeDisposition[];
  /** DE-owned promotion boundaries — never DecisionCandidate creation. */
  intake_promotion_boundaries?: DecisionEngineIntakePromotionBoundary[];
  /** DE-owned candidate creation requests — never DecisionCandidate creation. */
  candidate_creation_requests?: DecisionEngineCandidateCreationRequest[];
  /** DE-owned candidate creation boundary — may create DecisionCandidate without scoring. */
  candidate_creations?: DecisionEngineCandidateCreation[];
  /** DE-owned lifecycle integrations — origin-aware, no scoring/planner. */
  lifecycle_integrations?: DecisionCandidateLifecycleIntegration[];
  /** DE-owned evaluation origin contracts — origin rules only, never scoring. */
  evaluation_origin_contracts?: DecisionCandidateEvaluationOriginContract[];
  /** DE-owned evaluation resolutions — scoring-path admission only, never scores. */
  evaluation_resolutions?: DecisionCandidateEvaluationResolution[];
  /** DE-owned DecisionScore results — scoring only; never ranking/selection. */
  candidate_scores?: DecisionCandidateScore[];
  /** DE-owned comparative ranking — never selection/planner. */
  candidate_ranking?: DecisionCandidateRanking | null;
  /** DE-owned selection decisions — progression only; never execution/planner. */
  candidate_selections?: DecisionCandidateSelection[];
  /** DE-owned progression requests — never planner/execution. */
  progression_requests?: DecisionCandidateProgressionRequest[];
  /** DE-owned progression acknowledgements — receipt only; never planner/execution. */
  progression_acknowledgements?: DecisionCandidateProgressionAcknowledgement[];
  summary: string;
  authority_effect: string;
}

export interface DecisionEngineSummary {
  workspace_id: string;
  generated_at: string;
  candidate_count: number;
  open_count: number;
  top_candidates: DecisionCandidate[];
  summary: string;
  authority_effect: string;
}

export interface DecisionEngineHandoff {
  candidate_id: string;
  next_command: string;
  goal_statement: string;
  workspace_id: string;
  note: string;
  authority_effect: string;
}

export interface DecisionEngineActionResult {
  candidate: DecisionCandidate | null;
  handoff: DecisionEngineHandoff | null;
  authority_effect: string;
}

export interface WorkspaceIntelligenceState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  current_project: Project | null;
  current_task: Task | null;
  workflow_context: WorkflowContext;
  recent_goals: WorkGoal[];
  recent_activity: RecentActivityItem[];
  pending_plans: string[];
  pending_approvals: PendingDecisionSummary[];
  blocked_actions: BlockedActionSummary[];
  recommended_actions: WorkspaceRecommendation[];
  memory_highlights: IntelligenceHighlight[];
  preference_highlights: IntelligenceHighlight[];
  current_applications: IntelligenceApplicationSummary[];
  automation_contracts: AutomationContractSummary[];
  pending_automation_proposals: AutomationIntentProposalSummary[];
  recent_trigger_rejections: TriggerRejectionSummary[];
  decision_queue: DecisionQueueSummary;
  activity_graph: WorkspaceActivityGraphSummary;
  continuity: WorkspaceContinuitySummary;
  attention: WorkspaceAttentionSummary;
  decision_engine: DecisionEngineSummary;
  task_graph: TaskGraphSummary;
  environment: WorkspaceEnvironmentSummary;
  composition: WorkspaceCompositionSummary;
  purpose: WorkspacePurposeSummary;
  evolution: WorkspaceEvolutionSummary;
  recommendation_engine: WorkspaceRecommendationEngineSummary;
  operating_state: WorkspaceOperatingStateSummary;
  pattern: WorkspacePatternSummary;
  adaptation: WorkspaceAdaptationSummary;
  /** Preparedness for current work — distinct from workspace_health (kernel lifecycle). */
  readiness: WorkspaceReadinessSummary;
  /** Semantic kind-of-work — after Experience; never executes. */
  work_context: WorkspaceWorkContextSummary;
  /** Interaction paths over understanding — after Work Context; never executes. */
  navigation: WorkspaceNavigationSummary;
  /** Progress toward meaningful outcomes — after Navigation; never plans/executes. */
  milestones: WorkspaceMilestoneSummary;
  /** How work usually happens — after Milestones; never profiles/executes. */
  working_style: WorkspaceWorkingStyleSummary;
  /** Movement between work states — after Working Style; never restores/executes. */
  transition: WorkspaceTransitionSummary;
  /** What the user can interact with — after Transition; never executes. */
  interaction: WorkspaceInteractionSummary;
  /** User-owned environment setups — after Interaction; never executes. */
  profiles: WorkspaceProfileSummary;
  workspace_health: string;
  summary: string;
  authority_effect: string;
}

export type WorkspaceTaskStatus =
  | "proposed"
  | "planned"
  | "waiting"
  | "in_progress"
  | "blocked"
  | "completed"
  | "cancelled";

export type WorkspaceTaskPriority = "low" | "medium" | "high" | "critical";

export type TaskRelationshipKind =
  | "depends_on"
  | "blocks"
  | "related_to"
  | "child_of"
  | "parent_of";

export interface TaskMetadata {
  labels: string[];
  notes: string | null;
  attributes_json: string | null;
}

export interface WorkspaceTask {
  id: string;
  workspace_id: string;
  project_id: string | null;
  title: string;
  status: WorkspaceTaskStatus;
  priority: WorkspaceTaskPriority;
  metadata: TaskMetadata;
  source_intent_task_id: string | null;
  work_goal_id: string | null;
  progress_percent: number;
  explanation: string;
  created_at: string;
  updated_at: string;
  authority_effect: string;
}

export interface TaskNode {
  task: WorkspaceTask;
  blocker_ids: string[];
  dependency_ids: string[];
  child_ids: string[];
  parent_ids: string[];
  waiting_reason: string | null;
}

export interface TaskRelationship {
  id: string;
  workspace_id: string;
  from_task_id: string;
  to_task_id: string;
  kind: TaskRelationshipKind;
  created_at: string;
  authority_effect: string;
}

export interface TaskGraph {
  workspace_id: string;
  generated_at: string;
  nodes: TaskNode[];
  relationships: TaskRelationship[];
  active_count: number;
  blocked_count: number;
  waiting_count: number;
  completed_count: number;
  progress_percent: number;
  summary: string;
  integrity_ok: boolean;
  integrity_notes: string[];
  authority_effect: string;
}

export interface TaskGraphSummary {
  workspace_id: string;
  generated_at: string;
  node_count: number;
  relationship_count: number;
  active_count: number;
  blocked_count: number;
  waiting_count: number;
  completed_count: number;
  progress_percent: number;
  top_nodes: TaskNode[];
  summary: string;
  integrity_ok: boolean;
  authority_effect: string;
}

export type WindowIdentityConfidence = "high" | "medium" | "low" | "ephemeral";

export interface WorkspaceObservationPass {
  id: string;
  captured_at: string;
  schema_version: number;
  source: string;
  foreground_hwnd: string | null;
  window_count: number;
  monitor_count: number;
  duration_ms: number | null;
  metadata_json: string;
  authority_effect: string;
}

export interface ObservedMonitor {
  id: string;
  pass_id: string;
  monitor_index: number;
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  work_x: number;
  work_y: number;
  work_w: number;
  work_h: number;
  is_primary: boolean;
  dpi_scale: number | null;
  authority_effect: string;
}

export interface ObservedWindow {
  id: string;
  pass_id: string;
  hwnd: string;
  stable_window_id: string | null;
  title: string;
  process_id: number;
  process_name: string | null;
  x: number;
  y: number;
  width: number;
  height: number;
  monitor_id: string | null;
  visible: boolean;
  minimized: boolean;
  focused: boolean;
  z_order: number | null;
  authority_effect: string;
}

export interface ObservationWindowIdentity {
  id: string;
  process_id: number;
  title_fingerprint: string;
  first_seen_at: string;
  last_seen_at: string;
  last_hwnd: string;
  confidence: WindowIdentityConfidence;
  authority_effect: string;
}

export interface WorkspaceObservationSnapshot {
  pass: WorkspaceObservationPass;
  windows: ObservedWindow[];
  monitors: ObservedMonitor[];
  identities: ObservationWindowIdentity[];
  authority_effect: string;
}

export interface WorkspaceObservationCaptureResult {
  snapshot_id: string;
  captured_at: string;
  window_count: number;
  monitor_count: number;
  identity_count: number;
  snapshot: WorkspaceObservationSnapshot;
}

export type ObservationFreshness =
  | "unavailable"
  | "fresh"
  | "recent"
  | "stale";

export type ObservationCaptureErrorClass =
  | "windows_integration"
  | "validation"
  | "persistence"
  | "internal";

export interface ObservationCaptureFailure {
  failed_at: string;
  error_class: ObservationCaptureErrorClass;
  message: string;
  source: string | null;
  authority_effect: string;
}

export interface WorkspaceObservationStatus {
  has_observation: boolean;
  freshness: ObservationFreshness;
  pass_id: string | null;
  captured_at: string | null;
  age_seconds: number | null;
  window_count: number | null;
  monitor_count: number | null;
  identity_count: number | null;
  source: string | null;
  last_failure: ObservationCaptureFailure | null;
  authority_effect: string;
}

/** Explicit Manual ensure via TriggerAuthority — never silent automation. */
export interface ObservationFreshnessEnsureResult {
  consumer_id: string;
  trigger_outcome: string;
  refresh_decision: string;
  freshness_before: string;
  freshness_after: string;
  age_seconds_after: number | null;
  captured: boolean;
  explanation: string;
  authority_effect: string;
}

/** Lightweight window identity for observation deltas. */
export interface ObservationWindowRef {
  stable_window_id: string | null;
  hwnd: string;
  title: string;
  process_id: number;
}

export interface ObservationFocusedWindowChange {
  previous: ObservationWindowRef | null;
  current: ObservationWindowRef | null;
}

export interface ObservationWindowMove {
  stable_window_id: string | null;
  hwnd: string;
  title: string;
  from_x: number;
  from_y: number;
  to_x: number;
  to_y: number;
}

export interface ObservationWindowResize {
  stable_window_id: string | null;
  hwnd: string;
  title: string;
  from_width: number;
  from_height: number;
  to_width: number;
  to_height: number;
}

export interface ObservationMinimizedChange {
  stable_window_id: string | null;
  hwnd: string;
  title: string;
  was_minimized: boolean;
  is_minimized: boolean;
}

export interface ObservationMonitorAssignmentChange {
  stable_window_id: string | null;
  hwnd: string;
  title: string;
  from_monitor_index: number | null;
  to_monitor_index: number | null;
  from_monitor_name: string | null;
  to_monitor_name: string | null;
}

/** Read-only facts describing change between two observation snapshots. */
export interface WorkspaceObservationDelta {
  previous_pass_id: string | null;
  current_pass_id: string | null;
  previous_captured_at: string | null;
  current_captured_at: string | null;
  opened_windows: ObservationWindowRef[];
  closed_windows: ObservationWindowRef[];
  focused_window_changed: ObservationFocusedWindowChange | null;
  moved_windows: ObservationWindowMove[];
  resized_windows: ObservationWindowResize[];
  minimized_changes: ObservationMinimizedChange[];
  monitor_changes: ObservationMonitorAssignmentChange[];
  has_changes: boolean;
  authority_effect: string;
}

/** Canonical runtime state projection (observation + delta). Distinct from kernel lifecycle. */
export interface WorkspaceStateMetadata {
  state_id: string;
  created_at: string;
  observation_pass_id: string | null;
  latest_delta_reference: string | null;
  window_count: number;
  monitor_count: number;
  has_changes: boolean;
  authority_effect: string;
}

export interface WorkspaceActiveApplication {
  process_id: number;
  process_name: string | null;
  window_count: number;
}

/** Window row on WorkspaceState for Environment and other consumers. */
export interface WorkspaceStateWindow {
  stable_window_id: string | null;
  hwnd: string;
  title: string;
  process_id: number;
  process_name: string | null;
  visible: boolean;
  focused: boolean;
  minimized: boolean;
  x: number;
  y: number;
  width: number;
  height: number;
  monitor_index: number | null;
  monitor_name: string | null;
}

export interface WorkspaceState {
  metadata: WorkspaceStateMetadata;
  focused_window: ObservationWindowRef | null;
  active_applications: WorkspaceActiveApplication[];
  windows: WorkspaceStateWindow[];
  authority_effect: string;
}

/** Capture request origin. Event enters via ObservationEventGateway only (admission still rejects). */
export type CaptureRequestSource =
  | "manual"
  | "system"
  | "scheduled"
  | "event"
  | "plugin";

/** Contract-only observation event kinds (Sprint 117). */
export type ObservationEventKind =
  | "window_changed"
  | "focus_changed"
  | "monitor_changed"
  | "display_configuration_changed"
  | "desktop_state_changed"
  | "unknown";

/** Canonical internal event before ObservationTriggerAuthority (not an OS hook). */
export interface ObservationEvent {
  source: string;
  event_kind: ObservationEventKind;
  timestamp: string;
  context: string | null;
  correlation_id: string | null;
  metadata: string;
  authority_effect: string;
}

/** Provenance attachable to observation capture lifecycle. */
export interface CaptureProvenance {
  source: CaptureRequestSource;
  reason: string | null;
  context: string | null;
}

export interface CaptureRequest {
  source: CaptureRequestSource;
  reason: string | null;
  context: string | null;
}

/** Consumer freshness contract — does not trigger capture. */
export type ObservationFreshnessRequirement =
  | { kind: "any_available" }
  | { kind: "fresh" }
  | { kind: "not_stale" }
  | { kind: "max_age_seconds"; max_age_seconds: number };

export type ObservationRefreshBlockedReason = "capture_in_progress";

export type ObservationRefreshDecision =
  | { decision: "fresh_enough" }
  | { decision: "refresh_required" }
  | { decision: "observation_unavailable" }
  | {
      decision: "refresh_blocked";
      reason: ObservationRefreshBlockedReason;
    };

export interface ObservationConsumerFreshnessNeed {
  consumer_id: string;
  requirement: ObservationFreshnessRequirement;
  context: string | null;
}

export type ObservationTriggerSource = CaptureRequestSource;

export interface ObservationTriggerRequest {
  source: ObservationTriggerSource;
  reason: string | null;
  context: string | null;
  freshness_requirement: ObservationFreshnessRequirement;
}

/** Minimal schedule config — not persisted / no UI in this sprint. */
export interface ObservationScheduleConfig {
  enabled: boolean;
  interval_seconds: number;
}

/** Read-only observation scheduler runtime health (no snapshots). */
export interface ObservationSchedulerStatus {
  running: boolean;
  enabled: boolean;
  interval_seconds: number;
  started_at: string | null;
  last_tick_at: string | null;
  last_tick_duration_ms: number | null;
  ticks_emitted: number;
  captures_requested: number;
  captures_skipped: number;
  rate_limited_count: number;
  consecutive_failures: number;
  authority_effect: string;
}

export type ObservationTriggerOutcome =
  | "accepted_capture"
  | "ignored_fresh"
  | "blocked_capture_in_progress"
  | "unavailable"
  | "rate_limited"
  | "rejected_source";

export type ObservationTriggerAdmissionDecision =
  | { decision: "admitted" }
  | { decision: "rate_limited"; explanation: string }
  | { decision: "rejected_source"; explanation: string };

export type EnvironmentWindowState = "open" | "minimized" | "focused" | "unknown";

export interface EnvironmentWindow {
  id: string;
  hwnd: string;
  title: string;
  process_id: number;
  state: EnvironmentWindowState;
  matched_application_id: string | null;
  matched_application_name: string | null;
  project_id: string | null;
  task_id: string | null;
  layout_id: string | null;
  display_label: string;
  explanation: string;
  authority_effect: string;
}

export interface EnvironmentApplication {
  application_id: string;
  name: string;
  identifier: string | null;
  appears_running: boolean;
  window_count: number;
  focused: boolean;
  project_id: string | null;
  task_id: string | null;
  explanation: string;
}

export interface EnvironmentWindowGroup {
  id: string;
  label: string;
  application_id: string | null;
  process_id: number | null;
  window_ids: string[];
  project_id: string | null;
  explanation: string;
}

export interface EnvironmentLayoutAssociation {
  layout_id: string;
  layout_name: string;
  explanation: string;
}

export interface EnvironmentGap {
  kind: string;
  title: string;
  explanation: string;
  application_id: string | null;
  project_id: string | null;
  task_id: string | null;
}

export interface WorkspaceEnvironmentState {
  workspace_id: string;
  generated_at: string;
  active_project_id: string | null;
  active_task_id: string | null;
  windows: EnvironmentWindow[];
  applications: EnvironmentApplication[];
  window_groups: EnvironmentWindowGroup[];
  layout_associations: EnvironmentLayoutAssociation[];
  gaps: EnvironmentGap[];
  focused_window_id: string | null;
  running_application_count: number;
  missing_application_count: number;
  disconnected_work: boolean;
  observation_freshness: string;
  observation_refresh_decision: string;
  observation_age_seconds: number | null;
  observation_has_observation: boolean;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceEnvironmentSummary {
  workspace_id: string;
  generated_at: string;
  window_count: number;
  running_application_count: number;
  missing_application_count: number;
  group_count: number;
  disconnected_work: boolean;
  focused_window_title: string | null;
  top_applications: EnvironmentApplication[];
  top_gaps: EnvironmentGap[];
  observation_freshness: string;
  observation_refresh_decision: string;
  observation_age_seconds: number | null;
  observation_has_observation: boolean;
  summary: string;
  authority_effect: string;
}

export type CompositionMemberKind =
  | "application"
  | "window"
  | "layout"
  | "task_node"
  | "project"
  | "active_work"
  | "environment"
  | "activity"
  | "continuity";

export interface CompositionMember {
  id: string;
  kind: CompositionMemberKind;
  ref_id: string;
  label: string;
  present: boolean;
  explanation: string;
  evidence: string[];
  authority_effect: string;
}

export interface CompositionRelationship {
  id: string;
  from_member_id: string;
  to_member_id: string;
  kind: string;
  explanation: string;
  evidence: string[];
}

export interface CompositionGap {
  kind: string;
  title: string;
  explanation: string;
  evidence: string[];
  member_id: string | null;
}

export interface WorkspaceCompositionState {
  workspace_id: string;
  generated_at: string;
  label: string;
  active_project_id: string | null;
  active_project_name: string | null;
  active_task_id: string | null;
  focus_label: string | null;
  members: CompositionMember[];
  relationships: CompositionRelationship[];
  gaps: CompositionGap[];
  present_application_count: number;
  missing_application_count: number;
  task_node_count: number;
  window_count: number;
  outstanding_decision_count: number;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceCompositionSummary {
  workspace_id: string;
  generated_at: string;
  label: string;
  active_project_name: string | null;
  focus_label: string | null;
  present_application_count: number;
  missing_application_count: number;
  task_node_count: number;
  window_count: number;
  outstanding_decision_count: number;
  member_count: number;
  relationship_count: number;
  gap_count: number;
  top_members: CompositionMember[];
  top_gaps: CompositionGap[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export type PurposeEvidenceKind =
  | "work_goal"
  | "project"
  | "task_graph"
  | "composition"
  | "continuity"
  | "activity"
  | "decision_queue"
  | "attention";

export interface PurposeEvidence {
  id: string;
  kind: PurposeEvidenceKind;
  ref_id: string;
  label: string;
  explanation: string;
  evidence: string[];
}

export interface PurposeRelationship {
  id: string;
  from_id: string;
  to_id: string;
  kind: string;
  explanation: string;
  evidence: string[];
  progress_note: string | null;
  incomplete_note: string | null;
}

export interface PurposeObstacle {
  kind: string;
  title: string;
  explanation: string;
  evidence: string[];
}

export interface WorkspacePurposeState {
  workspace_id: string;
  generated_at: string;
  label: string;
  primary_work_goal_id: string | null;
  primary_work_goal_description: string | null;
  active_project_id: string | null;
  active_project_name: string | null;
  active_task_id: string | null;
  composition_label: string | null;
  focus_label: string | null;
  progress_percent: number;
  open_task_count: number;
  completed_task_count: number;
  blocked_task_count: number;
  outstanding_decision_count: number;
  interrupted_count: number;
  evidence_items: PurposeEvidence[];
  relationships: PurposeRelationship[];
  obstacles: PurposeObstacle[];
  recent_progress: string[];
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspacePurposeSummary {
  workspace_id: string;
  generated_at: string;
  label: string;
  primary_work_goal_description: string | null;
  active_project_name: string | null;
  composition_label: string | null;
  focus_label: string | null;
  progress_percent: number;
  open_task_count: number;
  completed_task_count: number;
  blocked_task_count: number;
  outstanding_decision_count: number;
  interrupted_count: number;
  obstacle_count: number;
  relationship_count: number;
  top_evidence: PurposeEvidence[];
  top_obstacles: PurposeObstacle[];
  recent_progress: string[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export type EvolutionInsightKind =
  | "task_progression"
  | "purpose_progression"
  | "composition_shift"
  | "interrupted_work"
  | "decision_outcome"
  | "focus_change";

export type EvolutionSourceModel =
  | "activity"
  | "task_graph"
  | "purpose"
  | "composition"
  | "continuity"
  | "decision_queue";

export interface EvolutionEvent {
  id: string;
  kind: string;
  ref_id: string;
  source_model: EvolutionSourceModel;
  title: string;
  change: string;
  evidence: string[];
  impact: string;
  timestamp: string;
  authority_effect: string;
}

export interface EvolutionRelationship {
  id: string;
  from_id: string;
  to_id: string;
  kind: string;
  explanation: string;
  evidence: string[];
}

export interface EvolutionInsight {
  id: string;
  kind: EvolutionInsightKind;
  title: string;
  explanation: string;
  evidence: string[];
  related_event_ids: string[];
}

export interface WorkspaceEvolutionState {
  workspace_id: string;
  generated_at: string;
  label: string;
  events: EvolutionEvent[];
  insights: EvolutionInsight[];
  relationships: EvolutionRelationship[];
  event_count: number;
  insight_count: number;
  relationship_count: number;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceEvolutionSummary {
  workspace_id: string;
  generated_at: string;
  label: string;
  event_count: number;
  insight_count: number;
  relationship_count: number;
  top_events: EvolutionEvent[];
  top_insights: EvolutionInsight[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export type RecommendationKind =
  | "continue_work"
  | "resolve_blocker"
  | "review_decision"
  | "complete_task"
  | "reorganize_workspace"
  | "restore_context"
  | "explore_opportunity";

export type RecommendationConfidence = "high" | "medium" | "low";

export interface RecommendationEvidence {
  id: string;
  source_model: string;
  source_ref: string;
  summary: string;
}

export interface RecommendationRelationship {
  id: string;
  from_id: string;
  to_id: string;
  kind: string;
  explanation: string;
  evidence: string[];
}

/** Structured, non-authoritative "why shown" view — never grants authority. */
export interface RecommendationExplanationView {
  why_suggested: string;
  impact: string;
  confidence: string;
  lifecycle_state: string;
  lifecycle_note: string;
  source_domains: string[];
  evidence_summaries: string[];
  evidence_refs: string[];
  explanation_keys: string[];
  experience_trace_match_keys: string[];
  related_attention_id: string | null;
  related_task_id: string | null;
  related_decision_id: string | null;
  continuity_fingerprint: string;
  authority_effect: string;
}

export interface RecommendationItem {
  id: string;
  kind: RecommendationKind;
  title: string;
  reason: string;
  evidence: RecommendationEvidence[];
  impact: string;
  confidence: RecommendationConfidence;
  related_attention_id: string | null;
  /** Carried from the Attention item in `related_attention_id`; empty for other sources. */
  attention_reasons: AttentionReason[];
  related_task_id: string | null;
  related_purpose_label: string | null;
  related_decision_id: string | null;
  /** Lifecycle overlay projection — null until durable overlay applied. */
  lifecycle_state?: string | null;
  lifecycle_presented_at?: string | null;
  lifecycle_resolved_at?: string | null;
  lifecycle_resolution_type?: string | null;
  /** Structured surface explanation — null until projected. */
  explanation?: RecommendationExplanationView | null;
  /** Structured outcome when resolved — immutable feedback only. */
  outcome?: RecommendationOutcomeView | null;
  /** Structured future-DE intake context — observational only. */
  decision_context?: RecommendationDecisionContext | null;
  /** Read-only Decision Engine handoff readiness — never creates commands. */
  decision_readiness?: RecommendationDecisionReadiness | null;
  /** Explicit RE↔DE ownership / intent boundary — never executes. */
  decision_boundary?: RecommendationDecisionBoundary | null;
  /** Explicit confirmation beyond accept — never creates DE/intent/execution. */
  decision_confirmation?: RecommendationDecisionConfirmation | null;
  /** Typed future-DE intake package after confirmation — never creates DE objects. */
  decision_intake?: RecommendationDecisionIntakeRequest | null;
  /** Integrity inspection of intake — inspect ≠ handoff / DE ownership. */
  decision_intake_inspection?: RecommendationDecisionIntakeInspection | null;
  /** Versioned intake package identity — compatible ≠ transfer / handoff. */
  decision_intake_compatibility?: RecommendationDecisionIntakeCompatibility | null;
  /** Compatible ≠ proceed / consume / adapter permission. */
  decision_intake_proceed_denial?: RecommendationDecisionIntakeProceedDenial | null;
  /** Frozen intake package digest — seal ≠ handoff / adapter. */
  decision_intake_package_seal?: RecommendationDecisionIntakePackageSeal | null;
  /** Prepared adapter path — prepare ≠ invoke / DE ownership. */
  decision_intake_adapter_preparation?: RecommendationDecisionIntakeAdapterPreparation | null;
  /** Non-executing handoff request — request ≠ performed handoff / DE object. */
  decision_handoff_request?: RecommendationDecisionHandoffRequest | null;
  /** Non-executing DE acceptance — accept ≠ ownership transfer / DE object. */
  decision_engine_acceptance?: RecommendationDecisionEngineAcceptance | null;
  authority_effect: string;
}

/** Non-executing DE acceptance of handoff request — never transfers ownership or creates DE objects. */
export interface RecommendationDecisionEngineAcceptance {
  recommendation_id: string;
  workspace_id: string;
  /** awaiting_acceptance | accepted | declined | revoked */
  acceptance_state: string;
  /** retained_by_recommendation | accepted_for_future_decision_engine | declined_by_decision_engine */
  ownership_state: string;
  /** Always false — acceptance ≠ performed ownership transfer. */
  ownership_transferred: boolean;
  current_owner: string;
  declared_future_owner: string | null;
  handoff_request_state: string;
  handoff_requested: boolean;
  sealed_intake_package_digest: string;
  contract_version: string;
  contract_family: string;
  confirmation_intent: string;
  accepted_at: string | null;
  declined_at: string | null;
  revoked_at: string | null;
  decision_engine_object_id: string | null;
  adapter_invoked: boolean;
  handoff_performed: boolean;
  permission_effect: string;
  note: string;
  authority_effect: string;
}

/** Non-executing RE→future-DE handoff request — never performs handoff or creates DE objects. */
export interface RecommendationDecisionHandoffRequest {
  recommendation_id: string;
  workspace_id: string;
  /** requested | revoked */
  request_state: string;
  handoff_requested: boolean;
  /** Always false — request ≠ performed handoff. */
  handoff_performed: boolean;
  requested_at: string | null;
  revoked_at: string | null;
  confirmation_intent: string;
  confirmed_at: string;
  sealed_intake_package_digest: string;
  contract_version: string;
  contract_family: string;
  continuity_fingerprint: string;
  preparation_state_at_request: string;
  preparation_prepared_at: string | null;
  preparation_active: boolean;
  seal_aligned: boolean;
  current_owner: string;
  decision_engine_object_id: string | null;
  adapter_invoked: boolean;
  permission_effect: string;
  note: string;
  authority_effect: string;
}

/** Prepared RE→future-DE adapter path — never invokes adapter or creates DE objects. */
export interface RecommendationDecisionIntakeAdapterPreparation {
  recommendation_id: string;
  workspace_id: string;
  /** prepared | revoked */
  preparation_state: string;
  prepared_at: string | null;
  revoked_at: string | null;
  confirmation_intent: string;
  sealed_intake_package_digest: string;
  contract_version: string;
  continuity_fingerprint_at_prep: string;
  seal_aligned: boolean;
  current_owner: string;
  declared_consumer_role: string;
  adapter_invoked: boolean;
  mapping_performed: boolean;
  decision_engine_object_id: string | null;
  suggested_mapping_notes: string[];
  proceed_authorized: boolean;
  handoff_performed: boolean;
  permission_effect: string;
  note: string;
  authority_effect: string;
}

/** Frozen RE intake snapshot digest — never proceed/adapter/handoff. */
export interface RecommendationDecisionIntakePackageSeal {
  recommendation_id: string;
  intake_package_digest: string;
  continuity_fingerprint_at_seal: string;
  contract_version: string;
  confirmation_intent: string;
  confirmed_at: string;
  eligibility_state: string;
  sealed: boolean;
  /** sealed | seal_mismatch */
  seal_state: string;
  package_matches_seal: boolean;
  sealed_at: string;
  current_owner: string;
  proceed_authorized: boolean;
  consume_authorized: boolean;
  adapter_invokable: boolean;
  handoff_performed: boolean;
  decision_engine_object_id: string | null;
  permission_effect: string;
  note: string;
  authority_effect: string;
}

/** Explicit denial that compatibility is not proceed permission. */
export interface RecommendationDecisionIntakeProceedDenial {
  recommendation_id: string;
  compatibility_compatible: boolean;
  contract_version: string;
  current_owner: string;
  declared_consumer_role: string;
  /** identity_pin_only | incompatible_blocked */
  eligibility_state: string;
  proceed_authorized: boolean;
  consume_authorized: boolean;
  adapter_invokable: boolean;
  permission_effect: string;
  denial_reasons: string[];
  handoff_performed: boolean;
  decision_engine_object_id: string | null;
  note: string;
  authority_effect: string;
}

/** Versioned intake package pin for a future consumer — never transfer/handoff. */
export interface RecommendationDecisionIntakeCompatibility {
  recommendation_id: string;
  contract_family: string;
  contract_version: string;
  schema_version: number;
  producer: string;
  declared_consumer: string;
  required_field_floor: string[];
  inspection_valid: boolean;
  field_floor_satisfied: boolean;
  version_current: boolean;
  compatible: boolean;
  may_migrate: boolean;
  transfer_authorized: boolean;
  handoff_performed: boolean;
  decision_engine_object_id: string | null;
  findings: string[];
  note: string;
  authority_effect: string;
}

/** Read-only integrity check of intake for a future consumer — never a handoff. */
export interface RecommendationDecisionIntakeInspection {
  recommendation_id: string;
  /** valid | stale_context | binding_failed | fingerprint_mismatch | provenance_mismatch | non_authoritative_violation */
  inspection_state: string;
  safe_to_inspect: boolean;
  fingerprint_matches: boolean;
  confirmation_bound: boolean;
  context_compatible: boolean;
  provenance_intact: boolean;
  ownership_intact: boolean;
  findings: string[];
  handoff_performed: boolean;
  note: string;
  authority_effect: string;
}

/** Typed RE→future-DE intake package — not a Decision Engine object. */
export interface RecommendationDecisionIntakeRequest {
  recommendation_id: string;
  workspace_id: string;
  confirmation_intent: string;
  confirmed_at: string;
  kind: string;
  title: string;
  suggested_goal_statement: string;
  continuity_fingerprint: string;
  explanation_ref: string | null;
  evidence_refs: string[];
  explanation_keys: string[];
  outcome_id: string | null;
  related_task_id: string | null;
  related_attention_id: string | null;
  related_decision_id: string | null;
  /** Always null — intake does not create DE objects. */
  decision_engine_object_id: string | null;
  /** requested — package only. */
  intake_state: string;
  handoff_performed: boolean;
  note: string;
  authority_effect: string;
}

/** Explicit confirmation: accept ≠ want future Decision Engine creation. */
export interface RecommendationDecisionConfirmation {
  recommendation_id: string;
  /** not_required | required | confirmed | declined */
  confirmation_state: string;
  /** agreement_only | create_future_decision | request_action_review */
  confirmation_intent: string;
  recommendation_owner: string;
  confirmation_owner: string;
  decision_owner: string;
  execution_owner: string;
  creates_decision_engine_object: boolean;
  creates_intent: boolean;
  grants_execution_authority: boolean;
  handoff_performed: boolean;
  confirmed_at: string | null;
  note: string;
  authority_effect: string;
}

/** Explicit separation: recommendation acceptance ≠ DE intake / execution. */
export interface RecommendationDecisionBoundary {
  recommendation_id: string;
  /** recommendation_only | context_ready | awaiting_decision_engine_intake */
  transition_state: string;
  /** Always handoff_not_performed today. */
  handoff_state: string;
  /** none | recommendation_agreement | recommendation_rejection | recommendation_terminal */
  user_intent_kind: string;
  recommendation_owner: string;
  decision_owner: string;
  execution_owner: string;
  governance_owner: string;
  experience_owner: string;
  accepted_as_recommendation_decision: boolean;
  creates_intent: boolean;
  creates_decision_engine_object: boolean;
  grants_execution_authority: boolean;
  handoff_performed: boolean;
  note: string;
  authority_effect: string;
}

/**
 * Structured intake snapshot for a future Decision Engine.
 * Not a DE object, intent, or handoff — observational only.
 */
export interface RecommendationDecisionContext {
  workspace_id: string;
  recommendation_id: string;
  kind: string;
  title: string;
  family: string;
  explanation_ref: string | null;
  explanation_keys: string[];
  evidence_refs: string[];
  experience_trace_match_keys: string[];
  continuity_fingerprint: string;
  lifecycle_state: string;
  lifecycle_resolution: string | null;
  outcome_id: string | null;
  outcome_history_refs: string[];
  user_decision: string | null;
  result_kind: string | null;
  related_task_id: string | null;
  related_attention_id: string | null;
  related_decision_id: string | null;
  related_purpose_label: string | null;
  missing: string[];
  complete: boolean;
  /** Always null until future DE intake wiring. */
  decision_engine_object_id: string | null;
  /** Always false — context never performs handoff. */
  handoff_performed: boolean;
  note: string;
  authority_effect: string;
}

/** Prerequisite for a future Recommendation → Decision Engine handoff. */
export interface RecommendationDecisionPrerequisite {
  id: string;
  label: string;
  satisfied: boolean;
  detail: string;
}

/**
 * Read-only assessment of future DE handoff eligibility.
 * Informational only — never creates intents/commands or calls Gateway.
 */
export interface RecommendationDecisionReadiness {
  recommendation_id: string;
  outcome_id: string | null;
  lifecycle_state: string;
  /** incomplete | blocked | handoff_deferred */
  readiness_state: string;
  prerequisites: RecommendationDecisionPrerequisite[];
  missing: string[];
  ready_for_future_handoff: boolean;
  note: string;
  authority_effect: string;
}

/** Compact IPC outcome on review actions. */
export type RecommendationFamily =
  | "recommendation_engine"
  | "decision_engine"
  | "intelligence"
  | "adaptation"
  | "decision_queue";

export interface RecommendationIdentity {
  native_id: string;
  family: RecommendationFamily;
  source_domain: string;
  originating_reasoning_ref: string | null;
  decision_ref: string | null;
  action_proposal_ref: string | null;
}

export interface RecommendationProvenance {
  recommendation_id: string;
  family: RecommendationFamily;
  source_evidence: RecommendationEvidence[];
  reasoning_origins: AttentionReason[];
  explanation_keys: string[];
  experience_trace_match_keys: string[];
  confidence: string | null;
  priority_or_impact: string | null;
  related_attention_id: string | null;
  future_capability_target: string | null;
}

export type RecommendationResolutionType =
  | "accepted"
  | "rejected"
  | "expired"
  | "superseded";

export type RecommendationUserDecision =
  | "accepted"
  | "rejected"
  | "deferred"
  | "expired"
  | "superseded";

export type RecommendationResultKind =
  | "accepted_follow_through"
  | "rejected_by_user"
  | "expired_without_action"
  | "superseded"
  | "downstream_execution_linked";

export interface RecommendationOutcomeQuality {
  confidence_at_outcome: string | null;
  useful_to_user: boolean | null;
  notes: string | null;
}

/** Full serialized recommendation outcome owned by the domain crate. */
export interface RecommendationOutcome {
  id: string;
  identity: RecommendationIdentity;
  provenance: RecommendationProvenance;
  lifecycle_resolution: RecommendationResolutionType | null;
  user_decision: RecommendationUserDecision;
  result_kind: RecommendationResultKind;
  recorded_at: string;
  quality: RecommendationOutcomeQuality;
  experience_trace_match_keys: string[];
  authority_effect: string;
}

/** Structured outcome projection for Operator / Work history. */
export interface RecommendationOutcomeView {
  outcome_id: string;
  recommendation_id: string;
  user_decision: string;
  result_kind: string;
  lifecycle_resolution: string | null;
  recorded_at: string;
  explanation_keys: string[];
  evidence_refs: string[];
  experience_trace_match_keys: string[];
  is_system_failure: boolean;
  authority_effect: string;
}

export interface RecommendationHistoryEntry {
  native_id: string;
  lifecycle_state: string;
  outcome: RecommendationOutcomeView;
  resolved_at: string | null;
  authority_effect: string;
}

export interface RecommendationReviewActionResult {
  workspace_id: string;
  recommendation_id: string;
  lifecycle_state: string;
  outcome: RecommendationOutcome | null;
  decision_context?: RecommendationDecisionContext | null;
  decision_readiness?: RecommendationDecisionReadiness | null;
  decision_boundary?: RecommendationDecisionBoundary | null;
  decision_confirmation?: RecommendationDecisionConfirmation | null;
  decision_intake?: RecommendationDecisionIntakeRequest | null;
  decision_intake_inspection?: RecommendationDecisionIntakeInspection | null;
  decision_intake_compatibility?: RecommendationDecisionIntakeCompatibility | null;
  decision_intake_proceed_denial?: RecommendationDecisionIntakeProceedDenial | null;
  decision_intake_package_seal?: RecommendationDecisionIntakePackageSeal | null;
  decision_intake_adapter_preparation?: RecommendationDecisionIntakeAdapterPreparation | null;
  decision_handoff_request?: RecommendationDecisionHandoffRequest | null;
  decision_engine_acceptance?: RecommendationDecisionEngineAcceptance | null;
  explanation: string;
  authority_effect: string;
}

export interface WorkspaceRecommendationEngineState {
  workspace_id: string;
  generated_at: string;
  label: string;
  candidates: RecommendationItem[];
  relationships: RecommendationRelationship[];
  candidate_count: number;
  relationship_count: number;
  history?: RecommendationHistoryEntry[];
  history_count?: number;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceRecommendationEngineSummary {
  workspace_id: string;
  generated_at: string;
  label: string;
  candidate_count: number;
  relationship_count: number;
  top_candidates: RecommendationItem[];
  /** Truncated terminal/orphan evidence — mirrors Rust summary_projection. */
  history?: RecommendationHistoryEntry[];
  history_count?: number;
  explanation: string;
  summary: string;
  authority_effect: string;
}

export type OperatingSignalKind =
  | "purpose"
  | "project"
  | "active_work"
  | "environment"
  | "composition"
  | "progress"
  | "blocker"
  | "pending_decision"
  | "recommendation"
  | "attention"
  | "continuity"
  | "evolution";

export interface OperatingSignal {
  id: string;
  kind: OperatingSignalKind;
  current_value: string;
  source_model: string;
  source_ref: string;
  evidence: string[];
  authority_effect: string;
}

export interface OperatingRelationship {
  id: string;
  from_id: string;
  to_id: string;
  kind: string;
  explanation: string;
  evidence: string[];
}

export interface OperatingContext {
  purpose_label: string;
  active_project_label: string | null;
  active_task_label: string | null;
  environment_summary: string;
  composition_label: string;
  recent_progress: string[];
  current_blockers: string[];
  pending_decisions: string[];
  top_recommendations: string[];
  attention_priorities: string[];
  continuity_focus: string | null;
}

export interface OperatingSummary {
  headline: string;
  purpose_line: string;
  environment_line: string;
  progress_line: string;
  pending_line: string;
  suggested_line: string;
  narrative: string;
}

export interface WorkspaceOperatingState {
  workspace_id: string;
  generated_at: string;
  context: OperatingContext;
  signals: OperatingSignal[];
  relationships: OperatingRelationship[];
  operating_summary: OperatingSummary;
  signal_count: number;
  relationship_count: number;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceOperatingStateSummary {
  workspace_id: string;
  generated_at: string;
  context: OperatingContext;
  operating_summary: OperatingSummary;
  signal_count: number;
  relationship_count: number;
  top_signals: OperatingSignal[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export type PatternKind =
  | "application_pattern"
  | "workflow_pattern"
  | "task_pattern"
  | "decision_pattern"
  | "environment_pattern";

export type PatternConfidence = "high" | "medium" | "low";

export interface PatternEvidence {
  id: string;
  source_model: string;
  source_ref: string;
  summary: string;
}

export interface PatternRelationship {
  id: string;
  from_id: string;
  to_id: string;
  kind: string;
  explanation: string;
  evidence: string[];
}

export interface WorkspacePattern {
  id: string;
  kind: PatternKind;
  title: string;
  observation: string;
  evidence: PatternEvidence[];
  confidence: PatternConfidence;
  impact: string;
  authority_effect: string;
}

export interface PatternSummary {
  headline: string;
  recurring_line: string;
  workflow_line: string;
  environment_line: string;
  narrative: string;
}

export interface WorkspacePatternState {
  workspace_id: string;
  generated_at: string;
  label: string;
  patterns: WorkspacePattern[];
  relationships: PatternRelationship[];
  pattern_summary: PatternSummary;
  pattern_count: number;
  relationship_count: number;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspacePatternSummary {
  workspace_id: string;
  generated_at: string;
  label: string;
  pattern_count: number;
  relationship_count: number;
  top_patterns: WorkspacePattern[];
  pattern_summary: PatternSummary;
  explanation: string;
  summary: string;
  authority_effect: string;
}

export type AdaptationKind =
  | "layout_improvement"
  | "application_grouping"
  | "workspace_organization"
  | "workflow_shortcut"
  | "context_restoration"
  | "task_organization";

export type AdaptationTargetKind =
  | "layout"
  | "application_group"
  | "composition"
  | "continuity"
  | "task_graph"
  | "purpose"
  | "environment";

export type AdaptationStatus =
  | "proposed"
  | "reviewed"
  | "accepted"
  | "rejected";

export interface AdaptationEvidence {
  id: string;
  source_model: string;
  source_ref: string;
  summary: string;
}

export interface AdaptationTarget {
  kind: AdaptationTargetKind;
  ref_id: string;
  label: string;
}

export interface AdaptationImpact {
  benefit: string;
  risk: string;
}

export interface AdaptationProposal {
  id: string;
  kind: AdaptationKind;
  title: string;
  reason: string;
  evidence: AdaptationEvidence[];
  impact: AdaptationImpact;
  target: AdaptationTarget;
  status: AdaptationStatus;
  related_pattern_id: string | null;
  related_recommendation_id: string | null;
  authority_effect: string;
}

export interface AdaptationSummary {
  headline: string;
  top_proposal_line: string;
  review_line: string;
  narrative: string;
}

export interface WorkspaceAdaptationState {
  workspace_id: string;
  generated_at: string;
  label: string;
  proposals: AdaptationProposal[];
  proposal_count: number;
  open_count: number;
  adaptation_summary: AdaptationSummary;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceAdaptationSummary {
  workspace_id: string;
  generated_at: string;
  label: string;
  proposal_count: number;
  open_count: number;
  top_proposals: AdaptationProposal[];
  adaptation_summary: AdaptationSummary;
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface AdaptationHandoff {
  proposal_id: string;
  next_command: string;
  goal_statement: string;
  workspace_id: string;
  note: string;
  authority_effect: string;
}

export interface AdaptationActionResult {
  proposal: AdaptationProposal | null;
  handoff: AdaptationHandoff | null;
  authority_effect: string;
}

export type ReadinessKind =
  | "environment_readiness"
  | "context_readiness"
  | "task_readiness"
  | "decision_readiness"
  | "purpose_readiness";

export type ReadinessStatus = "ready" | "partially_ready" | "blocked";

export interface ReadinessSignal {
  id: string;
  source_model: string;
  source_ref: string;
  summary: string;
}

export interface ReadinessGap {
  id: string;
  kind: string;
  title: string;
  explanation: string;
  impact: string;
  source_model: string;
  source_ref: string;
}

export interface ReadinessAssessment {
  id: string;
  kind: ReadinessKind;
  title: string;
  status: ReadinessStatus;
  reason: string;
  signals: ReadinessSignal[];
  gaps: ReadinessGap[];
  impact: string;
  authority_effect: string;
}

export interface ReadinessSummary {
  headline: string;
  status_line: string;
  gap_line: string;
  narrative: string;
}

export interface WorkspaceReadinessState {
  workspace_id: string;
  generated_at: string;
  label: string;
  overall_status: ReadinessStatus;
  assessments: ReadinessAssessment[];
  assessment_count: number;
  gap_count: number;
  ready_count: number;
  partially_ready_count: number;
  blocked_count: number;
  readiness_summary: ReadinessSummary;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceReadinessSummary {
  workspace_id: string;
  generated_at: string;
  label: string;
  overall_status: ReadinessStatus;
  assessment_count: number;
  gap_count: number;
  ready_count: number;
  partially_ready_count: number;
  blocked_count: number;
  top_assessments: ReadinessAssessment[];
  top_gaps: ReadinessGap[];
  readiness_summary: ReadinessSummary;
  explanation: string;
  summary: string;
  authority_effect: string;
}

export type WorkspaceHealthLevel =
  | "unknown"
  | "healthy"
  | "degraded"
  | "stale"
  | "blocked";

export interface SubsystemHealthEntry {
  subsystem: string;
  level: WorkspaceHealthLevel;
  detail: string;
}

export interface WorkspaceRuntimeHealth {
  id: string;
  workspace_id: string;
  overall: WorkspaceHealthLevel;
  subsystems: SubsystemHealthEntry[];
  degraded_services: string[];
  stale_observations: boolean;
  observation_freshness: WorkspaceHealthLevel;
  governance_readiness: WorkspaceHealthLevel;
  cognition_readiness: WorkspaceHealthLevel;
  experience_readiness: WorkspaceHealthLevel;
  publication_blocked: boolean;
  authority_effect: string;
}

export interface OperatorContextProjection {
  id: string;
  workspace_id: string;
  runtime_health_label: string;
  cognition_health_label: string;
  governance_health_label: string;
  observation_freshness_label: string;
  publication_readiness_label: string;
  publication_blocked: boolean;
  review_status_label: string;
  generated_at: string;
  authority_effect: string;
}

export interface OperatorRuntimeOverview {
  id: string;
  workspace_id: string;
  generated_at: string;
  health_overall: WorkspaceHealthLevel;
  runtime_context_id: string;
  operator_context_id: string;
  diagnostic_snapshot_id: string;
  consistency_has_errors: boolean;
  dependency_summary: string;
  capability_summary: string;
  governance_summary: string;
  coherence_ok: boolean;
  publication_blocked: boolean;
  authority_effect: string;
}

/** Live runtime projection wiring (Sprints 182–185) — observational only. */
export interface WorkspaceRuntimeOperatorView {
  workspace_id: string;
  generated_at: string;
  runtime_context_id: string;
  health: WorkspaceRuntimeHealth;
  operator_context: OperatorContextProjection;
  overview: OperatorRuntimeOverview;
  coherence_ok: boolean;
  architecture_review_passed: boolean;
  consistency_has_errors: boolean;
  diagnostic_snapshot_id: string;
  observation_status: WorkspaceObservationStatus | null;
  publication_blocked: boolean;
  authority_effect: string;
}

export type SessionMemberKind =
  | "project"
  | "task"
  | "purpose"
  | "application"
  | "decision"
  | "recommendation"
  | "adaptation"
  | "continuity_facet"
  | "pattern"
  | "composition";

export interface SessionMember {
  id: string;
  kind: SessionMemberKind;
  label: string;
  why: string;
  source_projection: string;
  source_ref: string;
  authority_effect: string;
}

export interface SessionFocus {
  project_label: string | null;
  task_label: string | null;
  purpose_label: string;
  continuity_focus: string | null;
  composition_label: string | null;
  why: string;
  source_projection: string;
  authority_effect: string;
}

export interface SessionTimelineItem {
  id: string;
  summary: string;
  timestamp: string;
  why: string;
  source_projection: string;
  source_ref: string;
}

export interface SessionDecisionRef {
  id: string;
  title: string;
  summary: string;
  priority_line: string;
  why: string;
  source_projection: string;
  source_ref: string;
}

export interface SessionRecommendationRef {
  id: string;
  title: string;
  reason: string;
  why: string;
  source_projection: string;
  source_ref: string;
}

export interface SessionReadinessView {
  overall_status: ReadinessStatus;
  status_line: string;
  gap_line: string;
  gap_count: number;
  why: string;
  source_projection: string;
}

export interface SessionRisk {
  id: string;
  title: string;
  explanation: string;
  why: string;
  source_projection: string;
  source_ref: string;
}

export interface SessionHealth {
  kernel_health: string;
  readiness_status: ReadinessStatus;
  note: string;
  why: string;
  source_projection: string;
}

export interface SessionInterruption {
  id: string;
  title: string;
  summary: string;
  why: string;
  source_projection: string;
  source_ref: string;
}

export interface SessionMomentum {
  progress_line: string;
  evolution_line: string;
  activity_count: number;
  open_task_count: number;
  why: string;
  source_projection: string;
}

export interface SessionSummary {
  headline: string;
  doing_line: string;
  matters_line: string;
  blocked_line: string;
  ready_line: string;
  changed_line: string;
  narrative: string;
}

export interface WorkspaceSessionState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  session_summary: SessionSummary;
  members: SessionMember[];
  focus: SessionFocus;
  timeline: SessionTimelineItem[];
  decisions: SessionDecisionRef[];
  recommendations: SessionRecommendationRef[];
  readiness: SessionReadinessView;
  risks: SessionRisk[];
  health: SessionHealth;
  interruptions: SessionInterruption[];
  momentum: SessionMomentum;
  member_count: number;
  decision_count: number;
  recommendation_count: number;
  risk_count: number;
  interruption_count: number;
  intelligence_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceSessionSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  session_summary: SessionSummary;
  focus: SessionFocus;
  readiness: SessionReadinessView;
  health: SessionHealth;
  momentum: SessionMomentum;
  member_count: number;
  decision_count: number;
  recommendation_count: number;
  risk_count: number;
  interruption_count: number;
  top_decisions: SessionDecisionRef[];
  top_recommendations: SessionRecommendationRef[];
  top_risks: SessionRisk[];
  intelligence_generated_at: string;
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceSessionComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export type ExperienceSectionKind =
  | "primary_focus"
  | "todays_work"
  | "suggested_attention"
  | "waiting_on"
  | "blocked_work"
  | "recent_progress"
  | "recommended_next_step"
  | "helpful_improvements"
  | "session_health";

export type ExperienceVisibility =
  | "immediate"
  | "highlighted"
  | "collapsed"
  | "deferred";

export type DisplayImportance = "high" | "medium" | "low";

/** Experience translation of one AttentionReason — keep source reasons alongside. */
export interface DisplayReason {
  title: string;
  description: string;
  importance: DisplayImportance;
  explanation_key: string;
  signal: string;
  source: string;
  weight: number;
  known: boolean;
}

export interface ExperienceItem {
  id: string;
  title: string;
  summary: string;
  visibility: ExperienceVisibility;
  why: string;
  source_session_field: string;
  source_ref: string;
  authority_effect: string;
}

export interface ExperienceSection {
  kind: ExperienceSectionKind;
  title: string;
  visibility: ExperienceVisibility;
  items: ExperienceItem[];
  item_count: number;
  collapsed_hint: string | null;
  why: string;
  source_session_field: string;
}

export interface ExperienceSummary {
  headline: string;
  focus_line: string;
  matters_line: string;
  blocked_line: string;
  ready_line: string;
  next_line: string;
  narrative: string;
}

export interface WorkspaceExperienceState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  experience_summary: ExperienceSummary;
  sections: ExperienceSection[];
  section_count: number;
  immediate_count: number;
  highlighted_count: number;
  collapsed_count: number;
  deferred_count: number;
  session_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceExperienceSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  experience_summary: ExperienceSummary;
  section_count: number;
  immediate_count: number;
  highlighted_count: number;
  collapsed_count: number;
  deferred_count: number;
  top_sections: ExperienceSection[];
  session_generated_at: string;
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceExperienceComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export type WorkContextType =
  | "development"
  | "research"
  | "administration"
  | "communication"
  | "creative"
  | "learning"
  | "operations"
  | "planning"
  | "custom";

export type WorkContextStatus =
  | "active"
  | "interrupted"
  | "blocked"
  | "dormant"
  | "candidate";

export type WorkContextConfidence = "high" | "medium" | "low";

export type WorkContextRelationKind =
  | "primary"
  | "supporting"
  | "nested"
  | "recently_active"
  | "interrupted"
  | "dormant"
  | "candidate";

export interface WorkContextEvidence {
  label: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface WorkContextAssociation {
  id: string;
  label: string;
  kind: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface WorkContext {
  id: string;
  name: string;
  context_type: WorkContextType;
  evidence: WorkContextEvidence[];
  confidence: WorkContextConfidence;
  associated_projects: WorkContextAssociation[];
  associated_tasks: WorkContextAssociation[];
  associated_applications: WorkContextAssociation[];
  associated_purpose: WorkContextAssociation | null;
  associated_decisions: WorkContextAssociation[];
  associated_activity: WorkContextAssociation[];
  current_status: WorkContextStatus;
  suggested_focus: string;
  blocked_reasons: string[];
  recent_progress: string[];
  why: string;
  authority_effect: string;
}

export interface WorkContextRelationship {
  from_context_id: string;
  to_context_id: string;
  kind: WorkContextRelationKind;
  why: string;
  authority_effect: string;
}

export interface WorkspaceWorkContextState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  contexts: WorkContext[];
  primary_context_id: string | null;
  relationships: WorkContextRelationship[];
  context_count: number;
  active_count: number;
  blocked_count: number;
  dormant_count: number;
  candidate_count: number;
  session_generated_at: string;
  experience_generated_at: string;
  intelligence_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceWorkContextSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  context_count: number;
  active_count: number;
  blocked_count: number;
  dormant_count: number;
  candidate_count: number;
  primary_context_name: string | null;
  primary_context_type: string | null;
  top_contexts: WorkContext[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceWorkContextComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export interface WorkspaceWorkContextValidation {
  valid: boolean;
  messages: string[];
  authority_effect: string;
}

export type NavigationPathKind =
  | "current_focus"
  | "suggested_destination"
  | "related_work"
  | "blocking_item"
  | "connected_task"
  | "connected_project"
  | "connected_context"
  | "relevant_decision"
  | "relevant_recommendation"
  | "recent_change"
  | "possible_next_inspection"
  | "dependency_chain"
  | "breadcrumb";

export type NavigationRelationKind =
  | "current"
  | "related"
  | "depends_on"
  | "blocked_by"
  | "supports"
  | "leads_to"
  | "recently_visited"
  | "suggested_next"
  | "dormant"
  | "disconnected";

export interface NavigationNode {
  id: string;
  label: string;
  kind: NavigationPathKind;
  summary: string;
  why: string;
  source_projection: string;
  source_ref: string;
  authority_effect: string;
}

export interface NavigationEdge {
  id: string;
  from_node_id: string;
  to_node_id: string;
  kind: NavigationRelationKind;
  why: string;
  authority_effect: string;
}

export interface NavigationPath {
  kind: NavigationPathKind;
  title: string;
  nodes: NavigationNode[];
  node_count: number;
  why: string;
}

export interface NavigationSummary {
  headline: string;
  current_path_line: string;
  related_line: string;
  blocked_line: string;
  next_inspection_line: string;
  breadcrumb_line: string;
  narrative: string;
}

export interface WorkspaceNavigationState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  navigation_summary: NavigationSummary;
  paths: NavigationPath[];
  nodes: NavigationNode[];
  edges: NavigationEdge[];
  breadcrumbs: NavigationNode[];
  path_count: number;
  node_count: number;
  edge_count: number;
  blocked_count: number;
  suggested_count: number;
  session_generated_at: string;
  experience_generated_at: string;
  work_context_generated_at: string;
  intelligence_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceNavigationSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  navigation_summary: NavigationSummary;
  path_count: number;
  node_count: number;
  edge_count: number;
  blocked_count: number;
  suggested_count: number;
  top_paths: NavigationPath[];
  breadcrumbs: NavigationNode[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceNavigationComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export interface WorkspaceNavigationValidation {
  valid: boolean;
  messages: string[];
  authority_effect: string;
}

export type MilestoneStatus =
  | "current"
  | "upcoming"
  | "blocked"
  | "completed";

export type MilestoneRelationKind =
  | "current"
  | "upcoming"
  | "blocked"
  | "completed"
  | "depends_on"
  | "contributes_to"
  | "supersedes"
  | "supports";

export type MilestoneReadinessBand =
  | "ready"
  | "partially_ready"
  | "blocked"
  | "unknown";

export interface MilestoneEvidence {
  label: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface MilestoneAssociation {
  id: string;
  label: string;
  kind: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface WorkspaceMilestone {
  id: string;
  title: string;
  summary: string;
  status: MilestoneStatus;
  readiness: MilestoneReadinessBand;
  progress_percent: number;
  evidence: MilestoneEvidence[];
  related_tasks: MilestoneAssociation[];
  related_projects: MilestoneAssociation[];
  supporting_contexts: MilestoneAssociation[];
  outstanding_decisions: MilestoneAssociation[];
  dependencies: MilestoneAssociation[];
  why: string;
  authority_effect: string;
}

export interface MilestoneRelationship {
  id: string;
  from_milestone_id: string;
  to_milestone_id: string;
  kind: MilestoneRelationKind;
  why: string;
  authority_effect: string;
}

export interface MilestoneSummary {
  headline: string;
  current_line: string;
  closest_line: string;
  blocked_line: string;
  completed_line: string;
  next_attention_line: string;
  narrative: string;
}

export interface WorkspaceMilestoneState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  milestone_summary: MilestoneSummary;
  milestones: WorkspaceMilestone[];
  relationships: MilestoneRelationship[];
  current_milestone_id: string | null;
  milestone_count: number;
  current_count: number;
  upcoming_count: number;
  blocked_count: number;
  completed_count: number;
  session_generated_at: string;
  experience_generated_at: string;
  work_context_generated_at: string;
  navigation_generated_at: string;
  intelligence_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceMilestoneSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  milestone_summary: MilestoneSummary;
  milestone_count: number;
  current_count: number;
  upcoming_count: number;
  blocked_count: number;
  completed_count: number;
  current_milestone_title: string | null;
  top_milestones: WorkspaceMilestone[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceMilestoneComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export interface WorkspaceMilestoneValidation {
  valid: boolean;
  messages: string[];
  authority_effect: string;
}

export type WorkingStyleKind =
  | "rhythm"
  | "organization"
  | "workflow"
  | "interaction_preference"
  | "observed_usage"
  | "context_switching";

export type WorkingStyleOrigin =
  | "observed_behaviour"
  | "explicit_preference";

export type WorkingStyleConfidence = "high" | "medium" | "low";

export interface WorkingStyleEvidence {
  label: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface WorkingStyleObservation {
  id: string;
  kind: WorkingStyleKind;
  origin: WorkingStyleOrigin;
  title: string;
  summary: string;
  confidence: WorkingStyleConfidence;
  evidence: WorkingStyleEvidence[];
  affected_context: string;
  explanation: string;
  why: string;
  authority_effect: string;
}

export interface WorkingStyleSummary {
  headline: string;
  rhythm_line: string;
  organization_line: string;
  workflow_line: string;
  context_switching_line: string;
  preference_line: string;
  observed_vs_preferred_line: string;
  narrative: string;
}

export interface WorkspaceWorkingStyleState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  style_summary: WorkingStyleSummary;
  observations: WorkingStyleObservation[];
  observation_count: number;
  observed_count: number;
  preference_count: number;
  rhythm_count: number;
  organization_count: number;
  workflow_count: number;
  context_switching_count: number;
  session_generated_at: string;
  experience_generated_at: string;
  work_context_generated_at: string;
  navigation_generated_at: string;
  milestones_generated_at: string;
  intelligence_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceWorkingStyleSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  style_summary: WorkingStyleSummary;
  observation_count: number;
  observed_count: number;
  preference_count: number;
  top_observations: WorkingStyleObservation[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceWorkingStyleComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export interface WorkspaceWorkingStyleValidation {
  valid: boolean;
  messages: string[];
  authority_effect: string;
}

export type TransitionKind =
  | "entering_context"
  | "leaving_context"
  | "returning_to_work"
  | "switching_focus"
  | "continuing_interrupted_work"
  | "completing_work_state"
  | "starting_new_work_state";

export type TransitionRelationKind =
  | "previous"
  | "current"
  | "returned_from"
  | "interrupted_by"
  | "continued_into"
  | "related_to"
  | "blocked_by";

export type TransitionConfidence = "high" | "medium" | "low";

export interface TransitionEvidence {
  label: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface TransitionAssociation {
  id: string;
  label: string;
  kind: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface WorkspaceTransition {
  id: string;
  kind: TransitionKind;
  title: string;
  previous_state: string;
  current_state: string;
  changed_elements: string[];
  evidence: TransitionEvidence[];
  related_context: TransitionAssociation[];
  related_milestones: TransitionAssociation[];
  open_decisions: TransitionAssociation[];
  interrupted_work: TransitionAssociation[];
  confidence: TransitionConfidence;
  explanation: string;
  why: string;
  authority_effect: string;
}

export interface TransitionRelationship {
  id: string;
  from_transition_id: string;
  to_ref: string;
  kind: TransitionRelationKind;
  why: string;
  authority_effect: string;
}

export interface TransitionSummary {
  headline: string;
  left_off_line: string;
  current_transition_line: string;
  changed_line: string;
  returned_line: string;
  context_switch_line: string;
  interrupted_line: string;
  narrative: string;
}

export interface WorkspaceTransitionState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  transition_summary: TransitionSummary;
  transitions: WorkspaceTransition[];
  relationships: TransitionRelationship[];
  current_transition_id: string | null;
  transition_count: number;
  returning_count: number;
  switching_count: number;
  interrupted_count: number;
  session_generated_at: string;
  experience_generated_at: string;
  work_context_generated_at: string;
  navigation_generated_at: string;
  milestones_generated_at: string;
  working_style_generated_at: string;
  intelligence_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceTransitionSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  transition_summary: TransitionSummary;
  transition_count: number;
  returning_count: number;
  switching_count: number;
  interrupted_count: number;
  current_transition_title: string | null;
  top_transitions: WorkspaceTransition[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceTransitionComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export interface WorkspaceTransitionValidation {
  valid: boolean;
  messages: string[];
  authority_effect: string;
}

export type InteractionKind =
  | "continue_work"
  | "review_decision"
  | "inspect_recommendation"
  | "review_adaptation"
  | "resolve_blocker"
  | "open_context"
  | "review_progress"
  | "understand_change";

export type InteractionItemState = "available" | "selected" | "handed_off";

export type InteractionPriority = "high" | "medium" | "low";

export interface InteractionEvidence {
  label: string;
  source_projection: string;
  source_ref: string;
  why: string;
}

export interface InteractionItem {
  id: string;
  kind: InteractionKind;
  source_projection: string;
  title: string;
  description: string;
  explanation: string;
  available_action: string;
  required_intent: string;
  state: InteractionItemState;
  priority: InteractionPriority;
  evidence: InteractionEvidence[];
  why: string;
  authority_effect: string;
}

export interface InteractionSummary {
  headline: string;
  continue_line: string;
  decision_line: string;
  recommendation_line: string;
  adaptation_line: string;
  blocker_line: string;
  progress_line: string;
  narrative: string;
}

export interface WorkspaceInteractionState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  interaction_summary: InteractionSummary;
  items: InteractionItem[];
  item_count: number;
  continue_count: number;
  decision_count: number;
  recommendation_count: number;
  adaptation_count: number;
  blocker_count: number;
  session_generated_at: string;
  experience_generated_at: string;
  transition_generated_at: string;
  intelligence_generated_at: string;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceInteractionSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  interaction_summary: InteractionSummary;
  item_count: number;
  continue_count: number;
  decision_count: number;
  recommendation_count: number;
  adaptation_count: number;
  blocker_count: number;
  top_items: InteractionItem[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface InteractionHandoff {
  interaction_id: string;
  kind: InteractionKind;
  next_command: string;
  intent_statement: string;
  workspace_id: string;
  note: string;
  authority_effect: string;
}

export interface InteractionSelectResult {
  item: InteractionItem | null;
  handoff: InteractionHandoff | null;
  authority_effect: string;
}

export interface WorkspaceInteractionComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export interface WorkspaceInteractionValidation {
  valid: boolean;
  messages: string[];
  authority_effect: string;
}

export type WorkspaceProfileStatus = "active" | "archived";

export type WorkspaceProfileMemberType =
  | "application"
  | "layout"
  | "project"
  | "task"
  | "work_context"
  | "preference";

export type WorkspaceProfileRelationship =
  | "expected"
  | "preferred"
  | "related";

export type WorkspaceProfileAlignment =
  | "aligned"
  | "partial"
  | "divergent"
  | "empty";

export interface WorkspaceProfileMember {
  id: string;
  profile_id: string;
  member_type: WorkspaceProfileMemberType;
  reference_id: string;
  relationship: WorkspaceProfileRelationship;
  evidence: string;
  label: string;
  authority_effect: string;
}

export interface WorkspaceProfileMemberInput {
  member_type: WorkspaceProfileMemberType;
  reference_id: string;
  relationship: WorkspaceProfileRelationship;
  evidence: string;
  label: string;
}

export interface WorkspaceProfile {
  id: string;
  workspace_id: string;
  name: string;
  description: string;
  status: WorkspaceProfileStatus;
  members: WorkspaceProfileMember[];
  created_at: string;
  updated_at: string;
  authority_effect: string;
}

export interface WorkspaceProfileEvidence {
  label: string;
  member_type: string;
  reference_id: string;
  why: string;
}

export interface WorkspaceProfileDifference {
  kind: string;
  member_type: string;
  reference_id: string;
  expected: string;
  observed: string;
  why: string;
}

export interface WorkspaceProfileComparison {
  profile_id: string;
  profile_name: string;
  workspace_id: string;
  alignment: WorkspaceProfileAlignment;
  matching_evidence: WorkspaceProfileEvidence[];
  differences: WorkspaceProfileDifference[];
  missing_members: WorkspaceProfileMember[];
  matched_count: number;
  missing_count: number;
  explanation: string;
  authority_effect: string;
}

export interface ProfileSummaryLines {
  headline: string;
  alignment_line: string;
  missing_line: string;
  narrative: string;
}

export interface WorkspaceProfileState {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  profile_summary: ProfileSummaryLines;
  profiles: WorkspaceProfile[];
  comparisons: WorkspaceProfileComparison[];
  profile_count: number;
  active_count: number;
  best_alignment: WorkspaceProfileAlignment | null;
  best_profile_name: string | null;
  explanation: string;
  evidence: string[];
  summary: string;
  authority_effect: string;
}

export interface WorkspaceProfileSummary {
  workspace_id: string;
  workspace_name: string;
  generated_at: string;
  label: string;
  profile_summary: ProfileSummaryLines;
  profile_count: number;
  active_count: number;
  best_alignment: string | null;
  best_profile_name: string | null;
  top_profiles: WorkspaceProfile[];
  top_comparisons: WorkspaceProfileComparison[];
  explanation: string;
  summary: string;
  authority_effect: string;
}

export interface WorkspaceProfileStateComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
  authority_effect: string;
}

export interface WorkspaceProfileValidation {
  valid: boolean;
  messages: string[];
  authority_effect: string;
}

export interface WorkspaceIntelligenceComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
}
