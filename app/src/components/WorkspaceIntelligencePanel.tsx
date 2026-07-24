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
      setLastHandoff(null);
      setState(null);
      return;
    }
    let cancelled = false;
    void (async () => {
      try {
        const [listed, listedContracts, listedProposals, queue, graph, intel] =
          await Promise.all([
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
          ]);
        if (!cancelled) {
          setProjects(listed);
          setContracts(listedContracts);
          setProposals(listedProposals);
          setDecisionQueue(queue);
          setActivityGraph(graph);
          setState(intel);
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
          One Workspace operating environment — current work, Decision Queue,
          Activity Graph, then Recommendations. Nothing here executes or grants
          permission.
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
                const [listedContracts, listedProposals, queue, graph] =
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
                  ]);
                setContracts(listedContracts);
                setProposals(listedProposals);
                setDecisionQueue(queue);
                setActivityGraph(graph);
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
