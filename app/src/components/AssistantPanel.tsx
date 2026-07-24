import { useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  AiAssistantActionPreview,
  AiAssistantPlanComparison,
  AiAssistantProductState,
  AiAssistantWorkflow,
  Workspace,
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
          Same generate_workspace_intelligence path as the Work tab — read-only.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Workspace understanding loaded", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceIntelligenceState>(
                  "generate_workspace_intelligence",
                  { workspaceId: workspace.id },
                );
                setWorkspaceIntel(state);
              })
            }
          >
            Load workspace intelligence
          </button>
        </div>
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
              {workspaceIntel.pending_approvals.length} pending decision(s),{" "}
              {workspaceIntel.blocked_actions.length} blocked action(s)
            </dd>
            <dt>Automation contracts</dt>
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
            <dt>Automation proposals</dt>
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
              governance for proposals.
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
                      onClick={() => void toggleExplanation(action)}
                    >
                      {open ? "Hide explanation" : "Why this?"}
                    </button>
                  </div>
                  {open && detail && (
                    <dl className="assistant-explanation">
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
            {comparison.differences.map((diff) => (
              <li key={diff}>{diff}</li>
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
