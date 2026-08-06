/** AUTO-GENERATED — do not edit.
 * Source: packages/domain (Product Proof Experience wire types)
 * Regenerate: pnpm sync:contracts
 * Verify: pnpm verify:contracts
 * Constitution: ARCHITECTURAL_CONSTITUTION_V2.md §4.4 / §4.7
 */

export type WorkspaceId = string;

export type SavedContextId = string;

export type ProjectedDisposition = "will_attempt" | "will_skip_unsupported" | "will_skip_unresolvable";

export type ItemDisposition = "completed" | "failed" | "skipped_unsupported" | "skipped_unresolvable" | "refused_changed" | "not_attempted" | "outcome_unknown";

export type OperationOutcome = "completed" | "partially_completed" | "failed" | "cancelled" | "indeterminate";

export type ProposedEffect = { "kind": "place", x: number, y: number, width: number, height: number, monitor_index: number | null, minimized: boolean, } | { "kind": "focus" } | { "kind": "unsupported", intent: string, };

export type PilotInterviewPhase = "baseline" | "week_four";

export type SavedContextScopeItem = { key: string, summary: string, };

export type SavedContextCaptureScope = { id: string, purpose: string, captured: Array<SavedContextScopeItem>, excluded: Array<SavedContextScopeItem>, };

export type SavedContextRestoreIdentity = { identity_schema_version: string, desktop_session_id: string, captured_hwnd: string, captured_process_id: number, title_fingerprint: string, captured_at: string, };

export type SavedContextWindow = { id: string, title: string, 
/**
 * Normalised the same way the observation layer normalises it.
 */
process_id: number, x: number, y: number, width: number, height: number, monitor_index: number | null, minimized: boolean, focused: boolean, z_order: number | null, 
/**
 * Present only when complete restore identity was captured under scope v2+.
 */
restore_identity: SavedContextRestoreIdentity | null, 
/**
 * Present when restore identity is unavailable (legacy or incomplete capture).
 */
restore_identity_unavailable_reason: string | null, };

export type SavedContextMonitor = { id: string, monitor_index: number, name: string, x: number, y: number, width: number, height: number, is_primary: boolean, };

export type SavedContext = { id: SavedContextId, workspace_id: WorkspaceId, name: string, created_at: string, 
/**
 * The capture scope the user confirmed before anything was captured.
 */
approved_scope: string, 
/**
 * User-authored intended next action. Never AI-generated or inferred.
 */
handoff_note: string, 
/**
 * Provenance of the capture this context was built from.
 */
observation_pass_id: string, captured_at: string, windows: Array<SavedContextWindow>, monitors: Array<SavedContextMonitor>, };

export type Workspace = { id: WorkspaceId, name: string, created_at: string, updated_at: string, };

export type ActionTargetDescriptor = { item_id: string, action_type: string, target_summary: string, restore_identity: SavedContextRestoreIdentity | null, identity_unavailable_reason: string | null, proposed_effect: ProposedEffect, 
/**
 * Callers must not supply a threshold. Presence is contract-invalid.
 */
confidence_threshold: string | null, 
/**
 * Forbidden. Presence is contract-invalid (ADM-AC-27).
 */
saved_context_id: string | null, };

export type ActionPlanItem = { item_id: string, action_type: string, target_summary: string, proposed_effect: ProposedEffect, permission_scope: string, projected_disposition: ProjectedDisposition, reason: string | null, error_code: string | null, 
/**
 * Exact target descriptor used for digest binding and re-resolution.
 */
target: ActionTargetDescriptor, };

export type ActionPlan = { plan_id: string, expires_at: string, plan_digest: string, purpose: string, items: Array<ActionPlanItem>, };

export type RestoreCompatibilitySummary = { total_items: number, will_attempt: number, will_skip_unsupported: number, will_skip_unresolvable: number, 
/**
 * Items skipped because the exact-session window was not found (closed apps).
 */
missing_window_count: number, 
/**
 * `high` | `steady` | `limited` | `empty` — matches frozen Continue quality copy.
 */
confidence_band: string, 
/**
 * True when at least one place/focus item will be attempted.
 */
restore_eligible: boolean, };

export type ResumePlanPreview = { saved_context_id: string, saved_context_name: string, 
/**
 * User-authored intended next action (PP-P01A). Not an Action effect.
 */
handoff_note: string, plan: ActionPlan, 
/**
 * Compatibility / confidence derived from plan dispositions.
 */
compatibility: RestoreCompatibilitySummary, };

export type ActionItemOutcome = { item_id: string, action_type: string, target_summary: string, disposition: ItemDisposition, what: string, why: string, reason: string | null, error_code: string | null, user_action_available: string, };

export type RestoreExecutionSummary = { 
/**
 * Effect items that completed (place and/or focus).
 */
restored_windows: number, 
/**
 * Skipped unsupported, unresolvable, or not attempted.
 */
skipped_windows: number, 
/**
 * Closed / absent windows (`ACTION_TARGET_NOT_FOUND`).
 */
missing_applications: number, 
/**
 * Failed, refused-changed, or outcome-unknown items.
 */
failed_operations: number, 
/**
 * Wall-clock duration of the execute pass.
 */
duration_ms: number, };

export type ActionOperationResult = { operation_id: string, outcome: OperationOutcome, items: Array<ActionItemOutcome>, summary: RestoreExecutionSummary, };

export type PilotScopeItem = { key: string, summary: string, };

export type PilotMeasurementScope = { id: string, purpose: string, measured: Array<PilotScopeItem>, not_measured: Array<PilotScopeItem>, };

export type PilotConsent = { scope_id: string, consented_at: string, withdrawn_at: string | null, };

export type PilotBaseline = { return_minutes: number, recorded_at: string, notes: string, };

export type PilotLeaveResumeRecord = { id: string, recorded_at: string, 
/**
 * Local calendar day (YYYY-MM-DD) for week-four habit counting.
 */
local_day: string, return_minutes: number, correction_needed: boolean, correction_note: string, };

export type PilotInterviewRecord = { phase: PilotInterviewPhase, recorded_at: string, responses: string, };

export type PilotMeasurementSnapshot = { scope: PilotMeasurementScope, consent: PilotConsent | null, baseline: PilotBaseline | null, leave_resume: Array<PilotLeaveResumeRecord>, interview_baseline: PilotInterviewRecord | null, interview_week_four: PilotInterviewRecord | null, distinct_resume_days: number, median_return_minutes: number | null, };

