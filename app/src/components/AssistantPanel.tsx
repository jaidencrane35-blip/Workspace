import { useState } from "react";
import { invokeIpc } from "../lib/ipc";
import { DecisionReasonList, DisplayReasonList } from "./DisplayReasonList";
import type {
  AiAssistantActionPreview,
  AiAssistantPlanComparison,
  AiAssistantProductState,
  AiAssistantWorkflow,
  Workspace,
  WorkspaceExperienceState,
  WorkspaceWorkContextState,
  WorkspaceNavigationState,
  WorkspaceMilestoneState,
  WorkspaceWorkingStyleState,
  WorkspaceTransitionState,
  WorkspaceInteractionState,
  WorkspaceProfileState,
  WorkspaceIntelligenceState,
} from "../types/domain";
import { assistantProductState } from "../types/domain";

interface AssistantPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

const PRODUCT_STATES: AiAssistantProductState[] = [
  "idle",
  "understanding",
  "planning",
  "evaluating",
  "awaiting_confirmation",
  "awaiting_permission",
  "executing",
  "completed",
  "failed",
  "cancelled",
];

function formatStateLabel(state: AiAssistantProductState): string {
  return state.replace(/_/g, " ");
}

export function AssistantPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
}: AssistantPanelProps) {
  const [goal, setGoal] = useState("Prepare my coding workspace");
  const [workflow, setWorkflow] = useState<AiAssistantWorkflow | null>(null);
  const [comparison, setComparison] = useState<AiAssistantPlanComparison | null>(
    null,
  );
  const [expandedStepId, setExpandedStepId] = useState<string | null>(null);
  const [workspaceIntel, setWorkspaceIntel] =
    useState<WorkspaceIntelligenceState | null>(null);
  const [experienceState, setExperienceState] =
    useState<WorkspaceExperienceState | null>(null);
  const [workContextState, setWorkContextState] =
    useState<WorkspaceWorkContextState | null>(null);
  const [navigationState, setNavigationState] =
    useState<WorkspaceNavigationState | null>(null);
  const [milestoneState, setMilestoneState] =
    useState<WorkspaceMilestoneState | null>(null);
  const [workingStyleState, setWorkingStyleState] =
    useState<WorkspaceWorkingStyleState | null>(null);
  const [transitionState, setTransitionState] =
    useState<WorkspaceTransitionState | null>(null);
  const [interactionState, setInteractionState] =
    useState<WorkspaceInteractionState | null>(null);
  const [profileState, setProfileState] =
    useState<WorkspaceProfileState | null>(null);

  const productState = assistantProductState(workflow?.state);
  const canConfirm = workflow?.state === "awaiting_confirmation";
  const canRevise =
    workflow?.state === "awaiting_confirmation" ||
    workflow?.state === "cancelled" ||
    workflow?.state === "failed";
  const canRegenerate = workflow?.state === "awaiting_confirmation";
  const canResume = workflow?.state === "waiting_for_permission";
  const canCancel =
    !!workflow &&
    workflow.state !== "completed" &&
    workflow.state !== "cancelled";
  const canCompare =
    !!workflow &&
    (workflow.plan_revisions?.length ?? 0) > 0 &&
    !!workflow.plan_preview;

  async function run(
    okMessage: string,
    action: () => Promise<void>,
  ): Promise<void> {
    onBusy(true);
    onError(null);
    try {
      await action();
      onMessage(okMessage);
    } catch (err: unknown) {
      onError(err instanceof Error ? err.message : String(err));
    } finally {
      onBusy(false);
    }
  }

  async function toggleExplanation(action: AiAssistantActionPreview) {
    if (!workflow) return;
    const next = expandedStepId === action.step_id ? null : action.step_id;
    setExpandedStepId(next);
    if (next) {
      try {
        await invokeIpc("record_assistant_explanation_viewed", {
          workflowId: workflow.id,
          stepId: action.step_id,
        });
      } catch {
        // Viewing still works if audit fails; surface is informational only.
      }
    }
  }

  return (
    <div className="assistant-panel">
      <header className="assistant-hero">
        <p className="assistant-kicker">Governed assistant</p>
        <h2>Ask for work. Review the plan. Approve what runs.</h2>
        <p className="lede">
          A governed interface into Workspace Intelligence. Understanding comes
          from the Workspace; every action still passes the Permission Gateway.
        </p>
      </header>

      <section>
        <h3>Shared workspace understanding</h3>
        <p className="muted">
          Same Experience + Intelligence paths as the Work tab — explain only;
          never edit or own state.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Workspace understanding loaded", async () => {
                if (!workspace) return;
                const [
                  intel,
                  experience,
                  workContext,
                  navigation,
                  milestones,
                  workingStyle,
                  transitions,
                  interactions,
                  profiles,
                ] =
                  await Promise.all([
                  invokeIpc<WorkspaceIntelligenceState>(
                    "generate_workspace_intelligence",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceExperienceState>(
                    "generate_workspace_experience",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceWorkContextState>(
                    "generate_workspace_work_context",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceNavigationState>(
                    "generate_workspace_navigation",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceMilestoneState>(
                    "generate_workspace_milestones",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceWorkingStyleState>(
                    "generate_workspace_working_style",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceTransitionState>(
                    "generate_workspace_transitions",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceInteractionState>(
                    "generate_workspace_interactions",
                    { workspaceId: workspace.id },
                  ),
                  invokeIpc<WorkspaceProfileState>(
                    "generate_workspace_profile_state",
                    { workspaceId: workspace.id },
                  ),
                ]);
                setWorkspaceIntel(intel);
                setExperienceState(experience);
                setWorkContextState(workContext);
                setNavigationState(navigation);
                setMilestoneState(milestones);
                setWorkingStyleState(workingStyle);
                setTransitionState(transitions);
                setInteractionState(interactions);
                setProfileState(profiles);
              })
            }
          >
            Load workspace intelligence
          </button>
        </div>
        {profileState && (
          <dl>
            <dt>Workspace Profiles</dt>
            <dd>
              {profileState.profile_summary.narrative}{" "}
              {profileState.best_profile_name
                ? `Closest: ${profileState.best_profile_name}. `
                : ""}
              Assistant explains profiles only — cannot activate them.
            </dd>
          </dl>
        )}
        {interactionState && (
          <dl>
            <dt>Interactions</dt>
            <dd>
              {interactionState.interaction_summary.narrative} Assistant
              explains opportunities only — never silently executes. Selecting
              an interaction creates Intent handoff; Gateway remains required.
            </dd>
          </dl>
        )}
        {transitionState && (
          <dl>
            <dt>Transitions</dt>
            <dd>
              {transitionState.transition_summary.narrative} Assistant explains
              movement between states — never restores or executes.
            </dd>
          </dl>
        )}
        {workingStyleState && (
          <dl>
            <dt>Working Style</dt>
            <dd>
              {workingStyleState.style_summary.narrative} Assistant explains
              how work usually happens — never claims certainty, never
              profiles, never executes.
            </dd>
          </dl>
        )}
        {milestoneState && (
          <dl>
            <dt>Milestones</dt>
            <dd>
              {milestoneState.milestone_summary.narrative} Assistant explains
              progress toward outcomes — never plans, completes, or executes.
            </dd>
          </dl>
        )}
        {navigationState && (
          <dl>
            <dt>Navigation</dt>
            <dd>
              {navigationState.navigation_summary.narrative} Assistant explains
              where to look — never routes or executes.
            </dd>
          </dl>
        )}
        {workContextState && (
          <dl>
            <dt>Work Context</dt>
            <dd>
              {workContextState.summary} Assistant explains what kind of work
              this is — never edits or owns Work Context.
            </dd>
          </dl>
        )}
        {experienceState && (
          <dl>
            <dt>Experience</dt>
            <dd>
              {experienceState.experience_summary.narrative} Assistant explains
              this calm Work view — never edits Experience or Session.
            </dd>
            <dt>Focus / next</dt>
            <dd>
              {experienceState.experience_summary.focus_line} Next:{" "}
              {experienceState.experience_summary.next_line}
            </dd>
          </dl>
        )}
        {workspaceIntel && (
          <dl>
            <dt>Summary</dt>
            <dd>{workspaceIntel.summary}</dd>
            <dt>Project / task</dt>
            <dd>
              {workspaceIntel.current_project?.name ?? "None"} /{" "}
              {workspaceIntel.current_task?.title ?? "None"}
            </dd>
            <dt>Needs attention</dt>
            <dd>
              Decision Queue: {workspaceIntel.decision_queue.pending_count}{" "}
              decision(s) needing attention (
              {workspaceIntel.decision_queue.high_priority_count} high priority)
              · history {workspaceIntel.decision_queue.history_count} terminal
              overlay(s) (not actionable) ·{" "}
              {workspaceIntel.blocked_actions.length} blocked action(s)
              projected from the same queue.
            </dd>
            <dt>Automation Contracts</dt>
            <dd>
              {workspaceIntel.automation_contracts.length === 0
                ? "None"
                : workspaceIntel.automation_contracts
                    .map(
                      (c) =>
                        `${c.name} (${c.status}/${c.approval_state})`,
                    )
                    .join("; ")}
              . Informational only — Assistant cannot approve or authorize them.
            </dd>
            <dt>Intent Proposals</dt>
            <dd>
              {workspaceIntel.pending_automation_proposals.length === 0
                ? "None pending"
                : workspaceIntel.pending_automation_proposals
                    .map(
                      (p) =>
                        `${p.intent_statement.slice(0, 48)}… (${p.status})`,
                    )
                    .join("; ")}
              . Explain-only — Assistant cannot accept, execute, or bypass
              governance for Intent Proposals.
            </dd>
            <dt>Attention</dt>
            <dd>
              {workspaceIntel.attention.summary}{" "}
              Same Attention Engine as the Work tab. Assistant may explain why
              something is prioritized — never change attention, approve, or
              execute.
              {workspaceIntel.attention.top_items.length > 0 ? (
                <div className="assistant-attention-reasons">
                  {workspaceIntel.attention.top_items.slice(0, 4).map((item) => (
                    <div key={item.id}>
                      <strong>{item.title}</strong>
                      <DisplayReasonList reasons={item.reasons} />
                    </div>
                  ))}
                </div>
              ) : null}
            </dd>
            <dt>Decision Engine</dt>
            <dd>
              {workspaceIntel.decision_engine.summary}{" "}
              {workspaceIntel.decision_engine.top_candidates.length === 0
                ? "No open recommendations."
                : workspaceIntel.decision_engine.top_candidates
                    .slice(0, 2)
                    .map((c) => c.title)
                    .join("; ")}{" "}
              History evidence: {workspaceIntel.decision_engine.history_count}{" "}
              terminal artifact(s) projected (not actionable). Explain-only here
              — accept/dismiss/postpone live on the Work tab. Never executes.
              {workspaceIntel.decision_engine.top_candidates[0] ? (
                <DecisionReasonList
                  reasons={
                    workspaceIntel.decision_engine.top_candidates[0].explanation
                      .reasons
                  }
                />
              ) : null}
            </dd>
            <dt>Task Graph</dt>
            <dd>
              {workspaceIntel.task_graph.summary}{" "}
              {workspaceIntel.task_graph.top_nodes.length === 0
                ? "No graph nodes."
                : `Active work: ${workspaceIntel.task_graph.top_nodes
                    .slice(0, 3)
                    .map((n) => n.task.title)
                    .join("; ")}.`}{" "}
              Same Task Graph as the Work tab. Assistant may explain — never
              mutate or execute.
            </dd>
            <dt>Environment</dt>
            <dd>
              {workspaceIntel.environment.summary}{" "}
              {workspaceIntel.environment.focused_window_title
                ? `Focused: ${workspaceIntel.environment.focused_window_title}. `
                : ""}
              {workspaceIntel.environment.disconnected_work
                ? "Active work may be disconnected from open windows. "
                : ""}
              Same Environment Model as the Work tab. Explain-only — never
              moves windows.
            </dd>
            <dt>Composition</dt>
            <dd>
              {workspaceIntel.composition.summary}{" "}
              {workspaceIntel.composition.focus_label
                ? `Focus: ${workspaceIntel.composition.focus_label}. `
                : ""}
              {workspaceIntel.composition.gap_count > 0
                ? `${workspaceIntel.composition.gap_count} gap(s) in the working environment. `
                : ""}
              Same Composition Engine as the Work tab. Assistant may explain why
              resources belong together — never edits compositions.
            </dd>
            <dt>Purpose</dt>
            <dd>
              {workspaceIntel.purpose.summary}{" "}
              {workspaceIntel.purpose.recent_progress.length > 0
                ? `Recent: ${workspaceIntel.purpose.recent_progress
                    .slice(0, 2)
                    .join("; ")}. `
                : ""}
              Same Purpose Model as the Work tab. Assistant may explain what you
              are working toward — never creates or modifies purpose silently.
            </dd>
            <dt>Evolution</dt>
            <dd>
              {workspaceIntel.evolution.summary}{" "}
              {workspaceIntel.evolution.top_insights.length > 0
                ? `Insight: ${workspaceIntel.evolution.top_insights[0].title}. `
                : ""}
              Same Evolution Model as the Work tab. Assistant may explain what
              changed — never rewrites history or acts on patterns.
            </dd>
            <dt>Recommendation Engine</dt>
            <dd>
              {workspaceIntel.recommendation_engine.summary}{" "}
              {workspaceIntel.recommendation_engine.top_candidates.length > 0
                ? `Top: ${workspaceIntel.recommendation_engine.top_candidates[0].title}. `
                : ""}
              History evidence:{" "}
              {workspaceIntel.recommendation_engine.history_count ?? 0} terminal
              record(s) projected (not actionable). Same Recommendation Engine
              as the Work tab. Assistant may explain and compare options — never
              accept, execute, or convert into actions silently.
              {workspaceIntel.recommendation_engine.top_candidates[0]
                ?.attention_reasons.length ? (
                <DisplayReasonList
                  reasons={
                    workspaceIntel.recommendation_engine.top_candidates[0]
                      .attention_reasons
                  }
                />
              ) : null}
            </dd>
            <dt>Recommended actions</dt>
            <dd>
              {workspaceIntel.recommended_actions.length === 0
                ? "None projected from Attention."
                : workspaceIntel.recommended_actions.slice(0, 3).map((rec) => (
                    <div key={rec.id}>
                      <strong>{rec.title}</strong>
                      <DisplayReasonList reasons={rec.reasons} />
                    </div>
                  ))}
            </dd>
            <dt>Operating State</dt>
            <dd>
              {workspaceIntel.operating_state.summary}{" "}
              {workspaceIntel.operating_state.operating_summary.headline
                ? `${workspaceIntel.operating_state.operating_summary.headline}. `
                : ""}
              Same Operating State as the Work tab. Assistant may summarize the
              current situation and explain relationships — never alter state or
              execute from it.
            </dd>
            <dt>Pattern Model</dt>
            <dd>
              {workspaceIntel.pattern.summary}{" "}
              {workspaceIntel.pattern.top_patterns.length > 0
                ? `Example: ${workspaceIntel.pattern.top_patterns[0].title}. `
                : ""}
              Same Pattern Model as the Work tab. Assistant may explain recurring
              structures — never act on patterns or create automation.
            </dd>
            <dt>Adaptation Proposals</dt>
            <dd>
              {workspaceIntel.adaptation.summary}{" "}
              {workspaceIntel.adaptation.top_proposals.length > 0
                ? `Would you like to consider: ${workspaceIntel.adaptation.top_proposals[0].title}? `
                : ""}
              Same Adaptation layer as the Work tab. Assistant may explain and
              compare — never accept or apply adaptations.
            </dd>
            <dt>Workspace Readiness</dt>
            <dd>
              {workspaceIntel.readiness.readiness_summary.status_line}{" "}
              {workspaceIntel.readiness.top_gaps.length > 0
                ? `Gap: ${workspaceIntel.readiness.top_gaps[0].title}. `
                : ""}
              Same Readiness Model as the Work tab. Assistant may explain blockers
              and missing context — never prepare or fix the Workspace.
            </dd>
            <dt>Working Session</dt>
            <dd>
              Session is the canonical runtime model behind Experience.
              Assistant may explain what you were doing, what is blocked, and
              what is ready — never restore, launch, or execute from Session.
            </dd>
            <dt>Workspace Experience</dt>
            <dd>
              Same Experience Layer as the Work tab. Assistant consumes it to
              explain focus, blockers, waiting, and next steps — never edits or
              owns presentation state.
            </dd>
            <dt>Work Context</dt>
            <dd>
              {workspaceIntel.work_context.summary} Same Work Context Engine as
              the Work tab. Assistant may explain the kind of work — never plans
              or executes from Context.
            </dd>
            <dt>Navigation</dt>
            <dd>
              {workspaceIntel.navigation.summary} Same Navigation Engine as the
              Work tab. Assistant may explain paths and next inspections — never
              routes or executes.
            </dd>
            <dt>Milestones</dt>
            <dd>
              {workspaceIntel.milestones.summary} Same Milestone Engine as the
              Work tab. Assistant may explain progress and blockers — never
              plans, schedules, or executes.
            </dd>
            <dt>Working Style</dt>
            <dd>
              {workspaceIntel.working_style.summary} Same Working Style Model as
              the Work tab. Assistant may explain observed patterns and
              explicit preferences separately — never claims certainty or
              changes behaviour.
            </dd>
            <dt>Transitions</dt>
            <dd>
              {workspaceIntel.transition.summary} Same Transition Engine as the
              Work tab. Assistant may explain where you left off and what
              changed — never restores or executes.
            </dd>
            <dt>Continuity</dt>
            <dd>
              {workspaceIntel.continuity.summary}{" "}
              {workspaceIntel.continuity.suggested_next_step
                ? `Suggested next: ${workspaceIntel.continuity.suggested_next_step.title}. `
                : ""}
              Same Continuity Engine as the Work tab. Assistant may explain —
              never resume or execute.
            </dd>
            <dt>Decision Queue</dt>
            <dd>
              {workspaceIntel.decision_queue.pending_count} pending (
              {workspaceIntel.decision_queue.high_priority_count} high
              priority)
              {workspaceIntel.decision_queue.items.length === 0
                ? "."
                : `: ${workspaceIntel.decision_queue.items
                    .map((i) => `[${i.priority}] ${i.title}`)
                    .join("; ")}.`}{" "}
              History evidence:{" "}
              {workspaceIntel.decision_queue.history_count} terminal
              overlay(s) projected (not actionable). Same queue as the Work
              tab. Assistant may explain and prioritize — never accept, reject,
              defer, or execute.
            </dd>
            <dt>Activity Graph</dt>
            <dd>
              {workspaceIntel.activity_graph.activity_count} activities,{" "}
              {workspaceIntel.activity_graph.unresolved_count} unresolved,{" "}
              {workspaceIntel.activity_graph.relationship_count} relationships
              {workspaceIntel.activity_graph.recent_timeline.length === 0
                ? "."
                : `: ${workspaceIntel.activity_graph.recent_timeline
                    .slice(0, 3)
                    .map((a) => a.summary)
                    .join("; ")}.`}{" "}
              Same graph as the Work tab. Assistant may explain history and
              blockers — never mutate or execute.
            </dd>
            <dt>Authority effect</dt>
            <dd>{workspaceIntel.authority_effect}</dd>
          </dl>
        )}
      </section>

      <div
        className="assistant-states"
        role="status"
        aria-label={`Workflow status: ${formatStateLabel(productState)}`}
      >
        {PRODUCT_STATES.map((state) => (
          <span
            key={state}
            className={
              state === productState
                ? `assistant-state active state-${state}`
                : "assistant-state"
            }
          >
            {formatStateLabel(state)}
          </span>
        ))}
      </div>

      <section className="assistant-goal-block">
        <label htmlFor="assistant-goal-input">What should the workspace do?</label>
        <textarea
          id="assistant-goal-input"
          value={goal}
          disabled={busy}
          rows={3}
          onChange={(event) => setGoal(event.target.value)}
          placeholder="e.g. Prepare my coding workspace"
        />
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace || !goal.trim()}
            onClick={() =>
              void run("Plan ready for review", async () => {
                if (!workspace) return;
                const next = await invokeIpc<AiAssistantWorkflow>(
                  "submit_assistant_goal",
                  {
                    goal: goal.trim(),
                    workspaceId: workspace.id,
                  },
                );
                setWorkflow(next);
                setComparison(null);
                setExpandedStepId(null);
              })
            }
          >
            Prepare plan
          </button>
          <button
            type="button"
            disabled={busy || !workflow || !canRevise || !goal.trim()}
            onClick={() =>
              void run("Goal updated — plan regenerated", async () => {
                if (!workflow || !workspace) return;
                const next = await invokeIpc<AiAssistantWorkflow>(
                  "revise_assistant_goal",
                  {
                    workflowId: workflow.id,
                    goal: goal.trim(),
                    workspaceId: workspace.id,
                  },
                );
                setWorkflow(next);
                setComparison(null);
              })
            }
          >
            Revise goal
          </button>
          <button
            type="button"
            disabled={busy || !workflow || !canRegenerate}
            onClick={() =>
              void run("Plan regenerated", async () => {
                if (!workflow || !workspace) return;
                const next = await invokeIpc<AiAssistantWorkflow>(
                  "regenerate_assistant_plan",
                  {
                    workflowId: workflow.id,
                    workspaceId: workspace.id,
                  },
                );
                setWorkflow(next);
                setComparison(null);
              })
            }
          >
            Regenerate plan
          </button>
        </div>
      </section>

      {workflow && (
        <section className="assistant-summary">
          <h3>Current goal</h3>
          <p>{workflow.user_goal}</p>
          <p className="muted">{workflow.status_message}</p>
          {workflow.plan_preview?.influence_summary && (
            <p className="assistant-influence">
              {workflow.plan_preview.influence_summary}
            </p>
          )}
        </section>
      )}

      {workflow?.plan_preview && (
        <section className="assistant-plan">
          <div className="assistant-plan-header">
            <h3>Plan preview</h3>
            <p className="assistant-permission-note">
              {workflow.plan_preview.permission_note}
            </p>
          </div>
          <ol className="assistant-actions">
            {workflow.plan_preview.actions.map((action) => {
              const detail = action.structured_explanation;
              const open = expandedStepId === action.step_id;
              return (
                <li key={action.step_id} className="assistant-action">
                  <div className="assistant-action-main">
                    <strong>
                      {action.ordinal + 1}. {action.command_name}
                    </strong>
                    <span className="muted">
                      {action.target ?? "workspace"} · {action.step_state}
                    </span>
                    <button
                      type="button"
                      className="linkish"
                      disabled={busy}
                      aria-expanded={open}
                      aria-controls={`assistant-explanation-${action.step_id}`}
                      onClick={() => void toggleExplanation(action)}
                    >
                      {open ? "Hide explanation" : "Why this?"}
                    </button>
                  </div>
                  {open && detail && (
                    <dl
                      id={`assistant-explanation-${action.step_id}`}
                      className="assistant-explanation"
                    >
                      <dt>Why suggested</dt>
                      <dd>{detail.why_suggested}</dd>
                      <dt>Why permission</dt>
                      <dd>{detail.why_permission}</dd>
                      <dt>If you approve</dt>
                      <dd>{detail.what_if_approve}</dd>
                      {detail.influence_tags.length > 0 && (
                        <>
                          <dt>Influences</dt>
                          <dd>{detail.influence_tags.join(", ")}</dd>
                        </>
                      )}
                    </dl>
                  )}
                </li>
              );
            })}
          </ol>
        </section>
      )}

      <section className="assistant-controls">
        <div className="row">
          <button
            type="button"
            disabled={busy || !canConfirm}
            onClick={() =>
              void run("Plan confirmed — awaiting Permission Gateway", async () => {
                if (!workflow) return;
                const next = await invokeIpc<AiAssistantWorkflow>(
                  "confirm_assistant_workflow",
                  { workflowId: workflow.id },
                );
                setWorkflow(next);
              })
            }
          >
            Confirm execution
          </button>
          <button
            type="button"
            disabled={busy || !canResume}
            onClick={() =>
              void run("Workflow resumed after permission", async () => {
                if (!workflow) return;
                const next = await invokeIpc<AiAssistantWorkflow>(
                  "resume_assistant_workflow",
                  { workflowId: workflow.id },
                );
                setWorkflow(next);
              })
            }
          >
            Resume paused plan
          </button>
          <button
            type="button"
            disabled={busy || !canCancel}
            onClick={() =>
              void run("Workflow cancelled", async () => {
                if (!workflow) return;
                const next = await invokeIpc<AiAssistantWorkflow>(
                  "cancel_assistant_workflow",
                  { workflowId: workflow.id },
                );
                setWorkflow(next);
              })
            }
          >
            Cancel
          </button>
          <button
            type="button"
            disabled={busy || !canCompare}
            onClick={() =>
              void run("Plan revisions compared", async () => {
                if (!workflow) return;
                const revisions = workflow.plan_revisions ?? [];
                const latestPrior = revisions[revisions.length - 1];
                if (!latestPrior) return;
                const result = await invokeIpc<AiAssistantPlanComparison>(
                  "compare_assistant_plan_revisions",
                  {
                    workflowId: workflow.id,
                    leftRevision: latestPrior.revision,
                    rightRevision: null,
                  },
                );
                setComparison(result);
              })
            }
          >
            Compare with previous
          </button>
        </div>
        {workflow?.state === "failed" && (
          <p className="error">
            An action was blocked. Revise the goal or cancel — authority was not
            bypassed.
          </p>
        )}
      </section>

      {comparison && (
        <section className="assistant-compare">
          <h3>Plan comparison</h3>
          <p className="muted">
            Revision {comparison.left.revision} → current (revision{" "}
            {comparison.right.revision})
          </p>
          <ul>
            {comparison.differences.map((diff, index) => (
              <li key={`comparison-diff-${index}`}>{diff}</li>
            ))}
          </ul>
        </section>
      )}

      {!workspace && (
        <p className="muted">
          Create or activate a workspace on Canvas before asking the assistant
          for work.
        </p>
      )}
    </div>
  );
}
