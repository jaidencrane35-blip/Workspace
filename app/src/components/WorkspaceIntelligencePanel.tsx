import { useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  AutomationContract,
  AutomationIntentProposal,
  DecisionActionResult,
  DecisionItem,
  DecisionQueue,
  Project,
  Task,
  TriggerEvaluationResult,
  Workspace,
  WorkspaceActivityGraph,
  WorkspaceAttentionState,
  WorkspaceContinuityState,
  DecisionEngineState,
  TaskGraph,
  WorkspaceEnvironmentState,
  WorkspaceCompositionState,
  WorkspacePurposeState,
  WorkspaceIntelligenceState,
} from "../types/domain";

interface WorkspaceIntelligencePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

export function WorkspaceIntelligencePanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
}: WorkspaceIntelligencePanelProps) {
  const [state, setState] = useState<WorkspaceIntelligenceState | null>(null);
  const [projectName, setProjectName] = useState("Workspace AI");
  const [taskTitle, setTaskTitle] = useState(
    "Build Workspace Intelligence Layer",
  );
  const [projects, setProjects] = useState<Project[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [contracts, setContracts] = useState<AutomationContract[]>([]);
  const [proposals, setProposals] = useState<AutomationIntentProposal[]>([]);
  const [lastEvaluation, setLastEvaluation] =
    useState<TriggerEvaluationResult | null>(null);
  const [decisionQueue, setDecisionQueue] = useState<DecisionQueue | null>(
    null,
  );
  const [activityGraph, setActivityGraph] =
    useState<WorkspaceActivityGraph | null>(null);
  const [continuity, setContinuity] = useState<WorkspaceContinuityState | null>(
    null,
  );
  const [attention, setAttention] = useState<WorkspaceAttentionState | null>(
    null,
  );
  const [decisionEngine, setDecisionEngine] =
    useState<DecisionEngineState | null>(null);
  const [taskGraph, setTaskGraph] = useState<TaskGraph | null>(null);
  const [environment, setEnvironment] =
    useState<WorkspaceEnvironmentState | null>(null);
  const [composition, setComposition] =
    useState<WorkspaceCompositionState | null>(null);
  const [purpose, setPurpose] = useState<WorkspacePurposeState | null>(null);
  const [lastHandoff, setLastHandoff] = useState<string | null>(null);
  const [contractName, setContractName] = useState(
    "Prepare coding environment",
  );
  const [contractIntent, setContractIntent] = useState(
    "When ready, request opening my development environment.",
  );

  async function run(ok: string, action: () => Promise<void>) {
    onBusy(true);
    onError(null);
    try {
      await action();
      onMessage(ok);
    } catch (err: unknown) {
      onError(err instanceof Error ? err.message : String(err));
    } finally {
      onBusy(false);
    }
  }

  useEffect(() => {
    if (!workspace) {
      setProjects([]);
      setTasks([]);
      setContracts([]);
      setProposals([]);
      setLastEvaluation(null);
      setDecisionQueue(null);
      setActivityGraph(null);
      setContinuity(null);
      setAttention(null);
      setDecisionEngine(null);
      setTaskGraph(null);
      setEnvironment(null);
      setComposition(null);
      setPurpose(null);
      setLastHandoff(null);
      setState(null);
      return;
    }
    let cancelled = false;
    void (async () => {
      try {
        const [
          listed,
          listedContracts,
          listedProposals,
          queue,
          graph,
          intel,
          cont,
          attn,
          decisions,
          graphTasks,
          env,
          comp,
          purp,
        ] = await Promise.all([
            invokeIpc<Project[]>("list_projects", {
              workspaceId: workspace.id,
              limit: 50,
            }),
            invokeIpc<AutomationContract[]>("list_automation_contracts", {
              workspaceId: workspace.id,
              limit: 50,
            }),
            invokeIpc<AutomationIntentProposal[]>(
              "list_automation_intent_proposals",
              { workspaceId: workspace.id, limit: 50 },
            ),
            invokeIpc<DecisionQueue>("generate_decision_queue", {
              workspaceId: workspace.id,
            }),
            invokeIpc<WorkspaceActivityGraph>(
              "generate_workspace_activity_graph",
              { workspaceId: workspace.id },
            ),
            invokeIpc<WorkspaceIntelligenceState>(
              "generate_workspace_intelligence",
              { workspaceId: workspace.id },
            ),
            invokeIpc<WorkspaceContinuityState>("generate_workspace_continuity", {
              workspaceId: workspace.id,
            }),
            invokeIpc<WorkspaceAttentionState>("generate_workspace_attention", {
              workspaceId: workspace.id,
            }),
            invokeIpc<DecisionEngineState>("generate_decision_engine", {
              workspaceId: workspace.id,
            }),
            invokeIpc<TaskGraph>("generate_task_graph", {
              workspaceId: workspace.id,
            }),
            invokeIpc<WorkspaceEnvironmentState>(
              "generate_workspace_environment",
              { workspaceId: workspace.id },
            ),
            invokeIpc<WorkspaceCompositionState>(
              "generate_workspace_composition",
              { workspaceId: workspace.id },
            ),
            invokeIpc<WorkspacePurposeState>("generate_workspace_purpose", {
              workspaceId: workspace.id,
            }),
          ]);
        if (!cancelled) {
          setProjects(listed);
          setContracts(listedContracts);
          setProposals(listedProposals);
          setDecisionQueue(queue);
          setActivityGraph(graph);
          setState(intel);
          setContinuity(cont);
          setAttention(attn);
          setDecisionEngine(decisions);
          setTaskGraph(graphTasks);
          setEnvironment(env);
          setComposition(comp);
          setPurpose(purp);
        }
      } catch (err: unknown) {
        if (!cancelled) {
          onError(err instanceof Error ? err.message : String(err));
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [workspace, onError]);

  return (
    <div className="intelligence-panel">
      <header className="intelligence-hero">
        <p className="assistant-kicker">Work</p>
        <h2>Where you are. What needs attention. How work connects.</h2>
        <p className="lede">
          One Workspace operating environment — Continuity, Attention, Task
          Graph, Environment, Composition, Purpose, Decision Engine, Decision
          Queue, then Activity. Nothing here executes or grants permission.
        </p>
      </header>

      <section>
        <h3>Current work</h3>
        {projects.length > 0 && (
          <p className="muted">
            Durable projects:{" "}
            {projects.map((project) => project.name).join(", ")}
          </p>
        )}
        <div className="row">
          <input
            value={projectName}
            disabled={busy || !workspace}
            onChange={(e) => setProjectName(e.target.value)}
            aria-label="Project name"
            placeholder="Project name"
          />
          <button
            type="button"
            disabled={busy || !workspace || !projectName.trim()}
            onClick={() =>
              void run("Project created", async () => {
                if (!workspace) return;
                const project = await invokeIpc<Project>("create_project", {
                  workspaceId: workspace.id,
                  name: projectName.trim(),
                  description: null,
                  metadata: null,
                });
                setProjects((prev) => [project, ...prev]);
                await invokeIpc("set_active_work", {
                  workspaceId: workspace.id,
                  projectId: project.id,
                  taskId: null,
                });
              })
            }
          >
            Create project
          </button>
          <input
            value={taskTitle}
            disabled={busy || !workspace}
            onChange={(e) => setTaskTitle(e.target.value)}
            aria-label="Task title"
            placeholder="Task title"
          />
          <button
            type="button"
            disabled={
              busy || !workspace || !taskTitle.trim() || projects.length === 0
            }
            onClick={() =>
              void run("Task created", async () => {
                if (!workspace || projects.length === 0) return;
                const project = projects[0];
                const task = await invokeIpc<Task>("create_task", {
                  projectId: project.id,
                  workspaceId: workspace.id,
                  title: taskTitle.trim(),
                  priority: "high",
                });
                setTasks((prev) => [task, ...prev]);
                await invokeIpc("set_active_work", {
                  workspaceId: workspace.id,
                  projectId: project.id,
                  taskId: task.id,
                });
              })
            }
          >
            Create task
          </button>
        </div>
      </section>

      <section>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Workspace understanding refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceIntelligenceState>(
                  "generate_workspace_intelligence",
                  { workspaceId: workspace.id },
                );
                setState(next);
                const [listedContracts, listedProposals, queue, graph, cont, attn] =
                  await Promise.all([
                    invokeIpc<AutomationContract[]>("list_automation_contracts", {
                      workspaceId: workspace.id,
                      limit: 50,
                    }),
                    invokeIpc<AutomationIntentProposal[]>(
                      "list_automation_intent_proposals",
                      { workspaceId: workspace.id, limit: 50 },
                    ),
                    invokeIpc<DecisionQueue>("generate_decision_queue", {
                      workspaceId: workspace.id,
                    }),
                    invokeIpc<WorkspaceActivityGraph>(
                      "generate_workspace_activity_graph",
                      { workspaceId: workspace.id },
                    ),
                    invokeIpc<WorkspaceContinuityState>(
                      "generate_workspace_continuity",
                      { workspaceId: workspace.id },
                    ),
                    invokeIpc<WorkspaceAttentionState>(
                      "generate_workspace_attention",
                      { workspaceId: workspace.id },
                    ),
                  ]);
                setContracts(listedContracts);
                setProposals(listedProposals);
                setDecisionQueue(queue);
                setActivityGraph(graph);
                setContinuity(cont);
                setAttention(attn);
              })
            }
          >
            Refresh workspace understanding
          </button>
        </div>
        {state && (
          <>
            <p>{state.summary}</p>
            <p className="muted">
              Health: {state.workspace_health} · Authority effect:{" "}
              {state.authority_effect} · Active:{" "}
              {state.current_project?.name ?? "no project"} /{" "}
              {state.current_task?.title ?? "no task"}
            </p>
          </>
        )}
      </section>

      <section>
        <h3>Pick up where you left off</h3>
        <p className="muted">
          Continuity explains focus, what changed, and what remains — without
          executing or granting permission.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Continuity refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceContinuityState>(
                  "generate_workspace_continuity",
                  { workspaceId: workspace.id },
                );
                setContinuity(next);
              })
            }
          >
            Refresh continuity
          </button>
        </div>
        {continuity ? (
          <>
            <p>{continuity.summary}</p>
            <p className="muted">
              Session anchor: {continuity.session_anchor || "none"} · authority:{" "}
              {continuity.authority_effect}
            </p>
            {continuity.current_focus && (
              <p>
                <strong>{continuity.current_focus.title}</strong>
                <span className="muted"> — {continuity.current_focus.why}</span>
              </p>
            )}
            {continuity.suggested_next_step && (
              <p>
                <strong>Suggested next:</strong>{" "}
                {continuity.suggested_next_step.title}
                <div className="muted">{continuity.suggested_next_step.why}</div>
              </p>
            )}
            <p className="muted">
              {continuity.outstanding_decisions.length} outstanding decision(s) ·{" "}
              {continuity.interrupted_work.length} interrupted ·{" "}
              {continuity.resumable_work.length} resumable ·{" "}
              {continuity.blockers.length} blocker(s)
            </p>
            {continuity.recent_progress.length > 0 && (
              <ul className="intelligence-list">
                {continuity.recent_progress.slice(0, 5).map((item) => (
                  <li key={item.id}>
                    <strong>{item.title}</strong>
                    <div className="muted">{item.what_changed}</div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">No continuity snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>What deserves attention</h3>
        <p className="muted">
          Canonical prioritization — explainable scores only. Attention never
          executes or grants permission.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Attention refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceAttentionState>(
                  "generate_workspace_attention",
                  { workspaceId: workspace.id },
                );
                setAttention(next);
              })
            }
          >
            Refresh attention
          </button>
        </div>
        {attention ? (
          <>
            <p>{attention.summary}</p>
            <p className="muted">
              {attention.requires_decision_count} require decision ·{" "}
              {attention.blocker_count} blocker(s) · {attention.can_wait_count}{" "}
              can wait · authority: {attention.authority_effect}
            </p>
            {attention.top_items.length === 0 ? (
              <p className="muted">Nothing needs attention right now.</p>
            ) : (
              <ul className="intelligence-list">
                {attention.top_items.map((item) => (
                  <li key={item.id}>
                    <strong>
                      [{item.priority}/{item.urgency}] {item.title}
                    </strong>
                    <div className="muted">
                      score {item.score} · {item.category} · {item.attention_state}
                    </div>
                    <div className="muted">{item.explanation}</div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">No attention snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Workspace Task Graph</h3>
        <p className="muted">
          Canonical model of work — dependencies, blockers, and progress.
          Informational only; never executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Task Graph refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<TaskGraph>("generate_task_graph", {
                  workspaceId: workspace.id,
                });
                setTaskGraph(next);
              })
            }
          >
            Refresh task graph
          </button>
        </div>
        {taskGraph ? (
          <>
            <p>{taskGraph.summary}</p>
            <p className="muted">
              {taskGraph.active_count} active · {taskGraph.blocked_count} blocked ·{" "}
              {taskGraph.waiting_count} waiting · {taskGraph.completed_count}{" "}
              completed · {taskGraph.progress_percent}% · integrity{" "}
              {taskGraph.integrity_ok ? "ok" : "issues"} · authority:{" "}
              {taskGraph.authority_effect}
            </p>
            {taskGraph.nodes.length === 0 ? (
              <p className="muted">No graph tasks yet.</p>
            ) : (
              <ul className="intelligence-list">
                {taskGraph.nodes.slice(0, 12).map((node) => (
                  <li key={node.task.id}>
                    <strong>
                      [{node.task.status}/{node.task.priority}] {node.task.title}
                    </strong>
                    <div className="muted">
                      {node.task.progress_percent}% · deps{" "}
                      {node.dependency_ids.length} · blockers{" "}
                      {node.blocker_ids.length}
                    </div>
                    {node.waiting_reason && (
                      <div>{node.waiting_reason}</div>
                    )}
                    {!node.waiting_reason && (
                      <div className="muted">{node.task.explanation}</div>
                    )}
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">No Task Graph snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Desktop environment</h3>
        <p className="muted">
          Live desktop read model — which windows and apps belong to this
          Workspace. Observation only; never moves windows.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Environment refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceEnvironmentState>(
                  "generate_workspace_environment",
                  { workspaceId: workspace.id },
                );
                setEnvironment(next);
              })
            }
          >
            Refresh environment
          </button>
        </div>
        {environment ? (
          <>
            <p>{environment.summary}</p>
            <p className="muted">
              {environment.running_application_count} running ·{" "}
              {environment.missing_application_count} missing ·{" "}
              {environment.windows.length} window(s) ·{" "}
              {environment.disconnected_work
                ? "work appears disconnected"
                : "work linked"}{" "}
              · authority: {environment.authority_effect}
            </p>
            {environment.gaps.length > 0 && (
              <ul className="intelligence-list">
                {environment.gaps.slice(0, 5).map((gap) => (
                  <li key={`${gap.kind}-${gap.title}`}>
                    <strong>{gap.title}</strong>
                    <div className="muted">{gap.explanation}</div>
                  </li>
                ))}
              </ul>
            )}
            {environment.window_groups.length > 0 && (
              <ul className="intelligence-list">
                {environment.window_groups.slice(0, 5).map((group) => (
                  <li key={group.id}>
                    <strong>{group.label}</strong>
                    <div className="muted">
                      {group.window_ids.length} window(s) · {group.explanation}
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">No environment snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Working environment</h3>
        <p className="muted">
          Composition Engine — how applications, projects, tasks, and desktop
          state belong together as meaning. Never launches or groups windows.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Composition refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceCompositionState>(
                  "generate_workspace_composition",
                  { workspaceId: workspace.id },
                );
                setComposition(next);
              })
            }
          >
            Refresh composition
          </button>
        </div>
        {composition ? (
          <>
            <p>
              <strong>{composition.label}</strong>
            </p>
            <p>{composition.summary}</p>
            <p className="muted">{composition.explanation}</p>
            <p className="muted">
              {composition.present_application_count} present ·{" "}
              {composition.missing_application_count} missing ·{" "}
              {composition.task_node_count} task node(s) ·{" "}
              {composition.outstanding_decision_count} outstanding decision(s)
              {composition.focus_label
                ? ` · focus: ${composition.focus_label}`
                : ""}{" "}
              · authority: {composition.authority_effect}
            </p>
            {composition.members.filter((m) => m.kind === "application")
              .length > 0 && (
              <ul className="intelligence-list">
                {composition.members
                  .filter((m) => m.kind === "application")
                  .map((member) => (
                    <li key={member.id}>
                      <strong>
                        {member.present ? "✓" : "○"} {member.label}
                      </strong>
                      <div className="muted">{member.explanation}</div>
                    </li>
                  ))}
              </ul>
            )}
            {composition.gaps.length > 0 && (
              <ul className="intelligence-list">
                {composition.gaps.slice(0, 5).map((gap) => (
                  <li key={`${gap.kind}-${gap.title}`}>
                    <strong>{gap.title}</strong>
                    <div className="muted">{gap.explanation}</div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">No composition snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Purpose</h3>
        <p className="muted">
          Why this work exists — outcomes projected from WorkGoals, projects,
          Task Graph, Composition, and Continuity. Never executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Purpose refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspacePurposeState>(
                  "generate_workspace_purpose",
                  { workspaceId: workspace.id },
                );
                setPurpose(next);
              })
            }
          >
            Refresh purpose
          </button>
        </div>
        {purpose ? (
          <>
            <p>
              <strong>{purpose.label}</strong>
            </p>
            <p>{purpose.summary}</p>
            <p className="muted">{purpose.explanation}</p>
            <p className="muted">
              {purpose.progress_percent}% progress · {purpose.open_task_count}{" "}
              open · {purpose.completed_task_count} completed ·{" "}
              {purpose.outstanding_decision_count} outstanding decision(s)
              {purpose.composition_label
                ? ` · environment: ${purpose.composition_label}`
                : ""}{" "}
              · authority: {purpose.authority_effect}
            </p>
            {purpose.recent_progress.length > 0 && (
              <ul className="intelligence-list">
                {purpose.recent_progress.slice(0, 5).map((line) => (
                  <li key={line}>
                    <div className="muted">{line}</div>
                  </li>
                ))}
              </ul>
            )}
            {purpose.obstacles.length > 0 && (
              <ul className="intelligence-list">
                {purpose.obstacles.slice(0, 5).map((obstacle) => (
                  <li key={`${obstacle.kind}-${obstacle.title}`}>
                    <strong>{obstacle.title}</strong>
                    <div className="muted">{obstacle.explanation}</div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">No purpose snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Recommended Actions</h3>
        <p className="muted">
          Decision Engine synthesizes Attention, memory, preferences, and goals
          into ranked recommendations. Accept hands off to the Planner — never
          executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Recommendations regenerated", async () => {
                if (!workspace) return;
                const next = await invokeIpc<DecisionEngineState>(
                  "generate_decision_engine",
                  { workspaceId: workspace.id },
                );
                setDecisionEngine(next);
              })
            }
          >
            Regenerate recommendations
          </button>
        </div>
        {decisionEngine ? (
          <>
            <p>{decisionEngine.summary}</p>
            <p className="muted">
              Why this matters now · {decisionEngine.context.attention_item_count}{" "}
              attention · {decisionEngine.context.pending_approval_count} pending
              approvals · authority: {decisionEngine.authority_effect}
            </p>
            {decisionEngine.top_candidates.length === 0 ? (
              <p className="muted">No open recommendations.</p>
            ) : (
              <ul className="intelligence-list">
                {decisionEngine.top_candidates.map((candidate, index) => (
                  <li key={candidate.id}>
                    <strong>
                      {index === 0 ? "Top · " : "Alternative · "}
                      {candidate.title}
                    </strong>
                    <div className="muted">
                      Confidence {candidate.explanation.confidence} · score{" "}
                      {candidate.score.total} · {candidate.outcome}
                    </div>
                    <div>{candidate.explanation.headline}</div>
                    <ul className="muted">
                      {candidate.explanation.reasons.map((reason) => (
                        <li key={`${candidate.id}-${reason.kind}-${reason.summary}`}>
                          {reason.summary}
                        </li>
                      ))}
                    </ul>
                    {candidate.related_goal_ids.length > 0 && (
                      <div className="muted">
                        Related goals: {candidate.related_goal_ids.length}
                      </div>
                    )}
                    {candidate.pending_approval_ids.length > 0 && (
                      <div className="muted">
                        Pending approvals: {candidate.pending_approval_ids.length}
                      </div>
                    )}
                    <div className="row">
                      <button
                        type="button"
                        disabled={busy || !workspace || candidate.outcome !== "open"}
                        onClick={() =>
                          void run("Recommendation accepted → planner", async () => {
                            if (!workspace) return;
                            const result = await invokeIpc<{
                              handoff: {
                                next_command: string;
                                goal_statement: string;
                                workspace_id: string;
                              } | null;
                            }>("select_decision_candidate", {
                              workspaceId: workspace.id,
                              candidateId: candidate.id,
                            });
                            if (result.handoff?.next_command === "submit_assistant_goal") {
                              try {
                                await invokeIpc("submit_assistant_goal", {
                                  goal: result.handoff.goal_statement,
                                  applicationIds: [],
                                  workspaceId: result.handoff.workspace_id,
                                });
                                setLastHandoff(
                                  `Planner received recommendation via ${result.handoff.next_command}`,
                                );
                              } catch {
                                setLastHandoff(
                                  `Selected — planner handoff ready (${result.handoff.next_command}). Open Assistant to plan.`,
                                );
                              }
                            } else {
                              setLastHandoff(
                                result.handoff
                                  ? `Handoff: ${result.handoff.next_command}`
                                  : "Selected",
                              );
                            }
                            const next = await invokeIpc<DecisionEngineState>(
                              "generate_decision_engine",
                              { workspaceId: workspace.id },
                            );
                            setDecisionEngine(next);
                          })
                        }
                      >
                        Accept
                      </button>
                      <button
                        type="button"
                        disabled={busy || !workspace}
                        onClick={() =>
                          void run("Recommendation postponed", async () => {
                            if (!workspace) return;
                            await invokeIpc("postpone_decision_candidate", {
                              workspaceId: workspace.id,
                              candidateId: candidate.id,
                            });
                            const next = await invokeIpc<DecisionEngineState>(
                              "generate_decision_engine",
                              { workspaceId: workspace.id },
                            );
                            setDecisionEngine(next);
                          })
                        }
                      >
                        Postpone
                      </button>
                      <button
                        type="button"
                        disabled={busy || !workspace}
                        onClick={() =>
                          void run("Recommendation dismissed", async () => {
                            if (!workspace) return;
                            await invokeIpc("dismiss_decision_candidate", {
                              workspaceId: workspace.id,
                              candidateId: candidate.id,
                            });
                            const next = await invokeIpc<DecisionEngineState>(
                              "generate_decision_engine",
                              { workspaceId: workspace.id },
                            );
                            setDecisionEngine(next);
                          })
                        }
                      >
                        Dismiss
                      </button>
                    </div>
                  </li>
                ))}
              </ul>
            )}
            {lastHandoff && <p className="muted">{lastHandoff}</p>}
          </>
        ) : (
          <p className="muted">No Decision Engine snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Activity</h3>
        <p className="muted">
          How work connects — projects, contracts, triggers, decisions, and
          outcomes. Informational only; the Activity Graph never executes or
          grants permissions.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Activity graph refreshed", async () => {
                if (!workspace) return;
                const graph = await invokeIpc<WorkspaceActivityGraph>(
                  "generate_workspace_activity_graph",
                  { workspaceId: workspace.id },
                );
                setActivityGraph(graph);
              })
            }
          >
            Refresh activity
          </button>
        </div>
        {activityGraph && (
          <p className="muted">
            {activityGraph.activities.length} activities ·{" "}
            {activityGraph.relationship_count} relationships ·{" "}
            {activityGraph.unresolved_count} still need attention · authority:{" "}
            {activityGraph.authority_effect}
          </p>
        )}
        {!activityGraph || activityGraph.timeline.length === 0 ? (
          <p className="muted">No activity yet for this workspace.</p>
        ) : (
          <ul className="intelligence-list">
            {[...activityGraph.timeline].reverse().slice(0, 20).map((item) => (
              <li key={item.id}>
                <strong>{item.summary}</strong>
                <div className="muted">
                  {item.activity_type}
                  {item.unresolved ? " · needs attention" : ""}
                  {item.project_id
                    ? ` · project ${item.project_id.slice(0, 8)}…`
                    : ""}
                </div>
                <div className="muted">{item.explanation}</div>
                {item.related_activity_ids.length > 0 && (
                  <div className="muted">
                    Connected to {item.related_activity_ids.length} related
                    item(s)
                    {item.parent_activity_id
                      ? ` · follows ${item.parent_activity_id.split(":")[1] ?? "parent"}`
                      : ""}
                  </div>
                )}
              </li>
            ))}
          </ul>
        )}
      </section>
      <section>
        <h3>Workspace Decision Queue</h3>
        <p className="muted">
          Canonical inbox for governed decisions. Sources remain authoritative —
          the queue aggregates, explains, and prioritizes. It never executes or
          grants permissions.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Decision Queue refreshed", async () => {
                if (!workspace) return;
                const queue = await invokeIpc<DecisionQueue>(
                  "generate_decision_queue",
                  { workspaceId: workspace.id },
                );
                setDecisionQueue(queue);
                setLastHandoff(null);
              })
            }
          >
            Refresh Decision Queue
          </button>
        </div>
        {decisionQueue && (
          <p className="muted">
            {decisionQueue.pending_count} needing attention ·{" "}
            {decisionQueue.high_priority_count} high priority · authority:{" "}
            {decisionQueue.authority_effect}
          </p>
        )}
        {lastHandoff && <p className="muted">{lastHandoff}</p>}
        {!decisionQueue || decisionQueue.items.length === 0 ? (
          <p className="muted">No decisions require attention.</p>
        ) : (
          <ul className="intelligence-list">
            {decisionQueue.items.map((item) => (
              <li key={item.id}>
                <strong>
                  [{item.priority}] {item.title}
                </strong>
                <div className="muted">
                  {item.category} · {item.decision_state} · {item.source_type}
                </div>
                <div className="muted">{item.explanation}</div>
                <div className="muted">
                  If accepted: {item.recommended_action}
                </div>
                {item.required_capabilities.length > 0 && (
                  <div className="muted">
                    Permissions eventually required:{" "}
                    {item.required_capabilities.join(", ")}
                  </div>
                )}
                <div className="row">
                  <button
                    type="button"
                    disabled={busy || item.decision_state === "dismissed"}
                    onClick={() =>
                      void run("Decision marked viewed", async () => {
                        if (!workspace) return;
                        const next = await invokeIpc<DecisionItem>(
                          "mark_decision_item_viewed",
                          {
                            workspaceId: workspace.id,
                            decisionItemId: item.id,
                          },
                        );
                        setDecisionQueue((prev) =>
                          prev
                            ? {
                                ...prev,
                                items: prev.items.map((i) =>
                                  i.id === next.id ? next : i,
                                ),
                              }
                            : prev,
                        );
                      })
                    }
                  >
                    Mark viewed
                  </button>
                  <button
                    type="button"
                    disabled={busy || item.decision_state === "dismissed"}
                    onClick={() =>
                      void run("Decision deferred", async () => {
                        if (!workspace) return;
                        const next = await invokeIpc<DecisionItem>(
                          "defer_decision_item",
                          {
                            workspaceId: workspace.id,
                            decisionItemId: item.id,
                          },
                        );
                        setDecisionQueue((prev) =>
                          prev
                            ? {
                                ...prev,
                                items: prev.items.map((i) =>
                                  i.id === next.id ? next : i,
                                ),
                              }
                            : prev,
                        );
                      })
                    }
                  >
                    Defer
                  </button>
                  <button
                    type="button"
                    disabled={busy || item.decision_state === "dismissed"}
                    onClick={() =>
                      void run("Decision dismissed (source unchanged)", async () => {
                        if (!workspace) return;
                        const next = await invokeIpc<DecisionItem>(
                          "dismiss_decision_item",
                          {
                            workspaceId: workspace.id,
                            decisionItemId: item.id,
                          },
                        );
                        setDecisionQueue((prev) =>
                          prev
                            ? {
                                ...prev,
                                items: prev.items.map((i) =>
                                  i.id === next.id ? next : i,
                                ),
                              }
                            : prev,
                        );
                      })
                    }
                  >
                    Dismiss
                  </button>
                  <button
                    type="button"
                    disabled={
                      busy ||
                      item.decision_state === "dismissed" ||
                      item.decision_state === "accepted" ||
                      item.decision_state === "rejected"
                    }
                    onClick={() =>
                      void run("Decision accept attempted", async () => {
                        if (!workspace) return;
                        const result = await invokeIpc<DecisionActionResult>(
                          "accept_decision_item",
                          {
                            workspaceId: workspace.id,
                            decisionItemId: item.id,
                          },
                        );
                        if (result.handoff) {
                          setLastHandoff(
                            `Handoff: use ${result.handoff.next_command} — ${result.handoff.note}`,
                          );
                        } else {
                          setLastHandoff(
                            result.delegated
                              ? "Delegated to source subsystem (not executed)."
                              : null,
                          );
                        }
                        const queue = await invokeIpc<DecisionQueue>(
                          "generate_decision_queue",
                          { workspaceId: workspace.id },
                        );
                        setDecisionQueue(queue);
                      })
                    }
                  >
                    Accept
                  </button>
                  <button
                    type="button"
                    disabled={
                      busy ||
                      item.decision_state === "dismissed" ||
                      item.decision_state === "accepted" ||
                      item.decision_state === "rejected"
                    }
                    onClick={() =>
                      void run("Decision reject attempted", async () => {
                        if (!workspace) return;
                        const result = await invokeIpc<DecisionActionResult>(
                          "reject_decision_item",
                          {
                            workspaceId: workspace.id,
                            decisionItemId: item.id,
                          },
                        );
                        if (result.handoff) {
                          setLastHandoff(
                            `Handoff: use ${result.handoff.next_command} — ${result.handoff.note}`,
                          );
                        }
                        const queue = await invokeIpc<DecisionQueue>(
                          "generate_decision_queue",
                          { workspaceId: workspace.id },
                        );
                        setDecisionQueue(queue);
                      })
                    }
                  >
                    Reject
                  </button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </section>
      {state && (
        <section>
          <h3>Recommendations</h3>
          <p className="muted">
            Advisory only — Recommendations never execute or grant permission.
            Act on Decisions in the Decision Queue above.
          </p>
          {state.recommended_actions.length === 0 ? (
            <p className="muted">No recommendations right now.</p>
          ) : (
            <ul className="intelligence-list">
              {state.recommended_actions.map((rec) => (
                <li key={rec.id}>
                  <strong>{rec.title}</strong>
                  <div className="muted">{rec.explanation}</div>
                </li>
              ))}
            </ul>
          )}
        </section>
      )}

      {state && (state.memory_highlights.length > 0 || state.preference_highlights.length > 0) && (
        <section>
          <h3>Memory & preferences</h3>
          <p className="muted">Informational influence only — never authority.</p>
          <ul className="intelligence-list">
            {state.memory_highlights.map((item) => (
              <li key={item.id}>
                <strong>{item.label}</strong>
                <div className="muted">{item.summary}</div>
              </li>
            ))}
            {state.preference_highlights.map((item) => (
              <li key={`pref-${item.id}`}>
                <strong>{item.label}</strong>
                <div className="muted">{item.summary}</div>
              </li>
            ))}
          </ul>
        </section>
      )}

      <section>
        <h3>Automation setup</h3>
        <p className="muted">
          Define Automation Contracts and review Intent Proposals. Contract
          Approval is definition consent — not Permission Approval and not
          execution.
        </p>
      </section>

      <section>
        <h3>Automation Contracts</h3>
        <p className="muted">
          These are stored intent definitions you may approve for later use.
          Approval is consent for this exact definition — not a permanent
          permission, not guaranteed execution, and not autonomous AI. Changing
          intent, trigger, or capabilities clears approval. Nothing runs unless
          it later enters the Command Pipeline and Permission Gateway.
        </p>
        <div className="row">
          <input
            value={contractName}
            disabled={busy || !workspace || projects.length === 0}
            onChange={(e) => setContractName(e.target.value)}
            aria-label="Contract name"
            placeholder="Contract name"
          />
          <input
            value={contractIntent}
            disabled={busy || !workspace || projects.length === 0}
            onChange={(e) => setContractIntent(e.target.value)}
            aria-label="Intent statement"
            placeholder="Intent statement"
          />
          <button
            type="button"
            disabled={
              busy ||
              !workspace ||
              projects.length === 0 ||
              !contractName.trim() ||
              !contractIntent.trim()
            }
            onClick={() =>
              void run("Automation contract created (draft)", async () => {
                if (!workspace || projects.length === 0) return;
                const project = projects[0];
                const created = await invokeIpc<AutomationContract>(
                  "create_automation_contract",
                  {
                    workspaceId: workspace.id,
                    projectId: project.id,
                    taskId: tasks[0]?.id ?? null,
                    name: contractName.trim(),
                    description: "User-defined automation contract",
                    triggerKind: "manual",
                    triggerDefinition: null,
                    intentStatement: contractIntent.trim(),
                    requiredCapabilities: ["application.launch"],
                  },
                );
                setContracts((prev) => [created, ...prev]);
              })
            }
          >
            Create draft contract
          </button>
        </div>
        {contracts.length === 0 ? (
          <p className="muted">No automation contracts yet.</p>
        ) : (
          <ul className="intelligence-list">
            {contracts.map((contract) => (
              <li key={contract.id}>
                <strong>{contract.name}</strong>
                <div className="muted">
                  Project {contract.project_id.slice(0, 8)}… · Status:{" "}
                  {contract.status} · Approval: {contract.approval_state}
                </div>
                <div className="muted">
                  Intent: {contract.intent_definition.statement}
                </div>
                <div className="muted">
                  Required capabilities (still checked at execution):{" "}
                  {contract.required_capabilities.join(", ") || "none listed"}
                </div>
                <div className="muted">
                  Created by: {contract.created_by_actor}
                  {contract.approved_by_actor
                    ? ` · Definition approved by: ${contract.approved_by_actor}`
                    : " · Definition not approved"}
                </div>
                <div className="row">
                  <button
                    type="button"
                    disabled={busy || contract.status !== "draft"}
                    onClick={() =>
                      void run("Approval requested", async () => {
                        const next = await invokeIpc<AutomationContract>(
                          "request_automation_contract_approval",
                          { contractId: contract.id },
                        );
                        setContracts((prev) =>
                          prev.map((c) => (c.id === next.id ? next : c)),
                        );
                      })
                    }
                  >
                    Request approval
                  </button>
                  <button
                    type="button"
                    disabled={
                      busy ||
                      (contract.status !== "pending_approval" &&
                        contract.status !== "draft")
                    }
                    onClick={() =>
                      void run("Definition approved (not execution)", async () => {
                        const next = await invokeIpc<AutomationContract>(
                          "approve_automation_contract",
                          { contractId: contract.id },
                        );
                        setContracts((prev) =>
                          prev.map((c) => (c.id === next.id ? next : c)),
                        );
                      })
                    }
                  >
                    Approve definition
                  </button>
                  <button
                    type="button"
                    disabled={busy || contract.status !== "approved"}
                    onClick={() =>
                      void run("Contract paused", async () => {
                        const next = await invokeIpc<AutomationContract>(
                          "pause_automation_contract",
                          { contractId: contract.id },
                        );
                        setContracts((prev) =>
                          prev.map((c) => (c.id === next.id ? next : c)),
                        );
                      })
                    }
                  >
                    Pause
                  </button>
                  <button
                    type="button"
                    disabled={busy || contract.status !== "paused"}
                    onClick={() =>
                      void run("Contract resumed", async () => {
                        const next = await invokeIpc<AutomationContract>(
                          "resume_automation_contract",
                          { contractId: contract.id },
                        );
                        setContracts((prev) =>
                          prev.map((c) => (c.id === next.id ? next : c)),
                        );
                      })
                    }
                  >
                    Resume
                  </button>
                  <button
                    type="button"
                    disabled={
                      busy ||
                      contract.status === "revoked" ||
                      contract.status === "completed"
                    }
                    onClick={() =>
                      void run("Contract revoked", async () => {
                        const next = await invokeIpc<AutomationContract>(
                          "revoke_automation_contract",
                          { contractId: contract.id },
                        );
                        setContracts((prev) =>
                          prev.map((c) => (c.id === next.id ? next : c)),
                        );
                      })
                    }
                  >
                    Revoke
                  </button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </section>
      <section>
        <h3>Trigger evaluation (Intent Proposals only)</h3>
        <p className="muted">
          Manual evaluation notices when an approved contract may be relevant.
          It creates intent proposals — never executes. Accepting a proposal is
          still not authority; prepare + Command Pipeline + Permission Gateway
          remain required.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace || projects.length === 0}
            onClick={() =>
              void run("Trigger evaluation complete", async () => {
                if (!workspace || projects.length === 0) return;
                const evaluation = await invokeIpc<TriggerEvaluationResult>(
                  "record_and_evaluate_triggers",
                  {
                    workspaceId: workspace.id,
                    eventType: "manual_evaluation_requested",
                    source: "workspace_intelligence_panel",
                    context: JSON.stringify({ reason: "user_requested" }),
                    projectId: projects[0]?.id ?? null,
                    taskId: tasks[0]?.id ?? null,
                  },
                );
                setLastEvaluation(evaluation);
                const listedProposals = await invokeIpc<
                  AutomationIntentProposal[]
                >("list_automation_intent_proposals", {
                  workspaceId: workspace.id,
                  limit: 50,
                });
                setProposals(listedProposals);
              })
            }
          >
            Evaluate approved contracts now
          </button>
        </div>
        {lastEvaluation && (
          <p className="muted">
            Last event: {lastEvaluation.trigger_event.event_type} ·{" "}
            {lastEvaluation.proposals.length} proposal(s) ·{" "}
            {lastEvaluation.rejections.length} rejection(s) · authority:{" "}
            {lastEvaluation.authority_effect}
          </p>
        )}
        {lastEvaluation && lastEvaluation.rejections.length > 0 && (
          <ul className="intelligence-list">
            {lastEvaluation.rejections.map((rejection) => (
              <li key={`${rejection.contract_id}-${rejection.reason}`}>
                <strong>{rejection.contract_name}</strong>
                <div className="muted">{rejection.reason}</div>
              </li>
            ))}
          </ul>
        )}
        {proposals.length === 0 ? (
          <p className="muted">No Intent Proposals yet.</p>
        ) : (
          <ul className="intelligence-list">
            {proposals.map((proposal) => (
              <li key={proposal.id}>
                <strong>Intent Proposal · {proposal.status}</strong>
                <div className="muted">
                  Contract {proposal.contract_id.slice(0, 8)}… · Event{" "}
                  {proposal.trigger_event_id.slice(0, 8)}…
                </div>
                <div className="muted">
                  Intent that would be prepared:{" "}
                  {proposal.intent_definition.statement}
                </div>
                <div className="muted">{proposal.explanation}</div>
                <div className="muted">
                  Requires (still gateway-checked):{" "}
                  {proposal.required_capabilities.join(", ") || "none listed"}
                </div>
                <div className="row">
                  <button
                    type="button"
                    disabled={busy || proposal.status !== "pending_review"}
                    onClick={() =>
                      void run(
                        "Intent Proposal accepted for review (not executed)",
                        async () => {
                          const next =
                            await invokeIpc<AutomationIntentProposal>(
                              "accept_automation_intent_proposal",
                              { proposalId: proposal.id },
                            );
                          setProposals((prev) =>
                            prev.map((p) => (p.id === next.id ? next : p)),
                          );
                        },
                      )
                    }
                  >
                    Accept (review only)
                  </button>
                  <button
                    type="button"
                    disabled={busy || proposal.status !== "pending_review"}
                    onClick={() =>
                      void run("Intent Proposal rejected", async () => {
                        const next = await invokeIpc<AutomationIntentProposal>(
                          "reject_automation_intent_proposal",
                          { proposalId: proposal.id },
                        );
                        setProposals((prev) =>
                          prev.map((p) => (p.id === next.id ? next : p)),
                        );
                      })
                    }
                  >
                    Reject
                  </button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </section>
      {!workspace && (
        <p className="muted">
          Create or activate a workspace on Canvas before generating
          intelligence.
        </p>
      )}

      {tasks.length > 0 && (
        <p className="muted">Tasks created this session: {tasks.length}</p>
      )}
    </div>
  );
}
