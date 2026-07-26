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
  authority_effect: string;
}

export interface DecisionQueueSummary {
  workspace_id: string;
  generated_at: string;
  pending_count: number;
  high_priority_count: number;
  items: DecisionItem[];
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
  score: DecisionScore;
  explanation: DecisionExplanation;
  related_goal_ids: string[];
  pending_approval_ids: string[];
  outcome: DecisionOutcome;
  created_at: string;
  handoff_command: string;
  authority_effect: string;
}

export interface DecisionEngineState {
  workspace_id: string;
  generated_at: string;
  context: DecisionContext;
  candidates: DecisionCandidate[];
  top_candidates: DecisionCandidate[];
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

export interface RecommendationItem {
  id: string;
  kind: RecommendationKind;
  title: string;
  reason: string;
  evidence: RecommendationEvidence[];
  impact: string;
  confidence: RecommendationConfidence;
  related_attention_id: string | null;
  related_task_id: string | null;
  related_purpose_label: string | null;
  related_decision_id: string | null;
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

export interface WorkspaceIntelligenceComparison {
  left_workspace_id: string;
  right_workspace_id: string;
  differences: string[];
}
