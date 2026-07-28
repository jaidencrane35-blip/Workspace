import { useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import {
  DecisionReasonList,
  DisplayReasonList,
} from "./DisplayReasonList";
import {
  isActiveRecommendation,
  RecommendationExplanationBlock,
  RecommendationHistoryList,
} from "./RecommendationExplanationView";
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
  DecisionEngineActionResult,
  DecisionEngineState,
  TaskGraph,
  WorkspaceEnvironmentState,
  WorkspaceCompositionState,
  WorkspacePurposeState,
  WorkspaceEvolutionState,
  WorkspaceRecommendationEngineState,
  WorkspaceOperatingState,
  WorkspacePatternState,
  WorkspaceAdaptationState,
  AdaptationActionResult,
  WorkspaceReadinessState,
  WorkspaceSessionState,
  WorkspaceSessionComparison,
  WorkspaceExperienceState,
  WorkspaceExperienceComparison,
  WorkspaceWorkContextState,
  WorkspaceWorkContextComparison,
  WorkspaceNavigationState,
  WorkspaceNavigationComparison,
  WorkspaceMilestoneState,
  WorkspaceMilestoneComparison,
  WorkspaceWorkingStyleState,
  WorkspaceWorkingStyleComparison,
  WorkspaceTransitionState,
  WorkspaceTransitionComparison,
  WorkspaceInteractionState,
  WorkspaceInteractionComparison,
  InteractionSelectResult,
  WorkspaceProfileState,
  WorkspaceProfileComparison,
  WorkspaceProfile,
  WorkspaceProfileMemberInput,
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
  const [evolution, setEvolution] =
    useState<WorkspaceEvolutionState | null>(null);
  const [recommendationEngine, setRecommendationEngine] =
    useState<WorkspaceRecommendationEngineState | null>(null);
  const [operatingState, setOperatingState] =
    useState<WorkspaceOperatingState | null>(null);
  const [patternState, setPatternState] =
    useState<WorkspacePatternState | null>(null);
  const [adaptationState, setAdaptationState] =
    useState<WorkspaceAdaptationState | null>(null);
  const [readinessState, setReadinessState] =
    useState<WorkspaceReadinessState | null>(null);
  const [sessionState, setSessionState] =
    useState<WorkspaceSessionState | null>(null);
  const [sessionCompareNote, setSessionCompareNote] = useState<string | null>(
    null,
  );
  const [experienceState, setExperienceState] =
    useState<WorkspaceExperienceState | null>(null);
  const [experienceCompareNote, setExperienceCompareNote] = useState<
    string | null
  >(null);
  const [workContextState, setWorkContextState] =
    useState<WorkspaceWorkContextState | null>(null);
  const [workContextCompareNote, setWorkContextCompareNote] = useState<
    string | null
  >(null);
  const [navigationState, setNavigationState] =
    useState<WorkspaceNavigationState | null>(null);
  const [navigationCompareNote, setNavigationCompareNote] = useState<
    string | null
  >(null);
  const [milestoneState, setMilestoneState] =
    useState<WorkspaceMilestoneState | null>(null);
  const [milestoneCompareNote, setMilestoneCompareNote] = useState<
    string | null
  >(null);
  const [workingStyleState, setWorkingStyleState] =
    useState<WorkspaceWorkingStyleState | null>(null);
  const [workingStyleCompareNote, setWorkingStyleCompareNote] = useState<
    string | null
  >(null);
  const [transitionState, setTransitionState] =
    useState<WorkspaceTransitionState | null>(null);
  const [transitionCompareNote, setTransitionCompareNote] = useState<
    string | null
  >(null);
  const [interactionState, setInteractionState] =
    useState<WorkspaceInteractionState | null>(null);
  const [interactionCompareNote, setInteractionCompareNote] = useState<
    string | null
  >(null);
  const [profileState, setProfileState] =
    useState<WorkspaceProfileState | null>(null);
  const [profileCompareNote, setProfileCompareNote] = useState<string | null>(
    null,
  );
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
      setEvolution(null);
      setRecommendationEngine(null);
      setOperatingState(null);
      setPatternState(null);
      setAdaptationState(null);
      setReadinessState(null);
      setSessionState(null);
      setSessionCompareNote(null);
      setExperienceState(null);
      setExperienceCompareNote(null);
      setWorkContextState(null);
      setWorkContextCompareNote(null);
      setNavigationState(null);
      setNavigationCompareNote(null);
      setMilestoneState(null);
      setMilestoneCompareNote(null);
      setWorkingStyleState(null);
      setWorkingStyleCompareNote(null);
      setTransitionState(null);
      setTransitionCompareNote(null);
      setInteractionState(null);
      setInteractionCompareNote(null);
      setProfileState(null);
      setProfileCompareNote(null);
      setLastHandoff(null);
      setState(null);
      return;
    }
    let cancelled = false;
    void (async () => {
      try {
        // Phase 5.5 hardening: do not Promise.all every standalone aggregator.
        // Intelligence is the shared-input path; full projections refresh on demand.
        // Interactive surfaces (Decision Queue / Engine / Adaptation / Task Graph) load once.
        const [
          listed,
          listedContracts,
          listedProposals,
          queue,
          intel,
          session,
          experience,
          workContext,
          navigation,
          milestones,
          workingStyle,
          transitions,
          interactions,
          profileSnapshot,
          decisions,
          graphTasks,
          adapt,
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
            invokeIpc<WorkspaceIntelligenceState>(
              "generate_workspace_intelligence",
              { workspaceId: workspace.id },
            ),
            invokeIpc<WorkspaceSessionState>("generate_workspace_session", {
              workspaceId: workspace.id,
            }),
            invokeIpc<WorkspaceExperienceState>("generate_workspace_experience", {
              workspaceId: workspace.id,
            }),
            invokeIpc<WorkspaceWorkContextState>(
              "generate_workspace_work_context",
              { workspaceId: workspace.id },
            ),
            invokeIpc<WorkspaceNavigationState>("generate_workspace_navigation", {
              workspaceId: workspace.id,
            }),
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
            invokeIpc<DecisionEngineState>("generate_decision_engine", {
              workspaceId: workspace.id,
            }),
            invokeIpc<TaskGraph>("generate_task_graph", {
              workspaceId: workspace.id,
            }),
            invokeIpc<WorkspaceAdaptationState>("generate_workspace_adaptation", {
              workspaceId: workspace.id,
            }),
          ]);
        if (!cancelled) {
          setProjects(listed);
          setContracts(listedContracts);
          setProposals(listedProposals);
          setDecisionQueue(queue);
          setState(intel);
          setSessionState(session);
          setExperienceState(experience);
          setWorkContextState(workContext);
          setNavigationState(navigation);
          setMilestoneState(milestones);
          setWorkingStyleState(workingStyle);
          setTransitionState(transitions);
          setInteractionState(interactions);
          setProfileState(profileSnapshot);
          setDecisionEngine(decisions);
          setTaskGraph(graphTasks);
          setAdaptationState(adapt);
          // Pure projections: show Intelligence summaries until explicit Refresh.
          setActivityGraph(null);
          setContinuity(null);
          setAttention(null);
          setEnvironment(null);
          setComposition(null);
          setPurpose(null);
          setEvolution(null);
          setRecommendationEngine(null);
          setOperatingState(null);
          setPatternState(null);
          setReadinessState(null);
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
          Experience presents one calm product view over your Session. Nothing
          here executes or grants permission.
        </p>
      </header>

      <section>
        <h3>Your workspace</h3>
        <p className="muted">
          Presentation over Session — Immediate and Highlighted first; collapsed
          and deferred details stay out of the way.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Experience refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceExperienceState>(
                  "generate_workspace_experience",
                  { workspaceId: workspace.id },
                );
                setExperienceState(next);
                setExperienceCompareNote(null);
              })
            }
          >
            Refresh experience
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !experienceState}
            onClick={() =>
              void run("Experience compared", async () => {
                if (!workspace || !experienceState) return;
                const next = await invokeIpc<WorkspaceExperienceState>(
                  "generate_workspace_experience",
                  { workspaceId: workspace.id },
                );
                const comparison =
                  await invokeIpc<WorkspaceExperienceComparison>(
                    "compare_workspace_experiences",
                    { left: experienceState, right: next },
                  );
                setExperienceState(next);
                setExperienceCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare experience
          </button>
        </div>
        {experienceState ? (
          <>
            <p>
              <strong>{experienceState.experience_summary.headline}</strong>
            </p>
            <p>{experienceState.experience_summary.focus_line}</p>
            <p className="muted">
              {experienceState.experience_summary.matters_line}
            </p>
            <p className="muted">
              {experienceState.experience_summary.blocked_line}
            </p>
            <p className="muted">
              {experienceState.experience_summary.ready_line}
            </p>
            <p className="muted">
              Next: {experienceState.experience_summary.next_line}
            </p>
            <ul className="intelligence-list">
              {experienceState.sections
                .filter(
                  (section) =>
                    section.visibility === "immediate" ||
                    section.visibility === "highlighted",
                )
                .map((section) => (
                  <li key={section.kind}>
                    <strong>
                      {section.title}
                      {section.visibility === "highlighted" ? " · highlighted" : ""}
                    </strong>
                    {section.items.length === 0 ? (
                      <div className="muted">
                        {section.collapsed_hint ?? "Nothing here right now."}
                      </div>
                    ) : (
                      section.items.slice(0, 3).map((item) => (
                        <div key={item.id} className="muted">
                          {item.title}
                          {item.summary ? ` — ${item.summary}` : ""}
                        </div>
                      ))
                    )}
                  </li>
                ))}
            </ul>
            <details>
              <summary className="muted">
                Collapsed / deferred ({experienceState.collapsed_count +
                  experienceState.deferred_count})
              </summary>
              <ul className="intelligence-list">
                {experienceState.sections
                  .filter(
                    (section) =>
                      section.visibility === "collapsed" ||
                      section.visibility === "deferred",
                  )
                  .map((section) => (
                    <li key={section.kind}>
                      <strong>{section.title}</strong>
                      <div className="muted">
                        {section.collapsed_hint ??
                          `${section.item_count} item(s) · from ${section.source_session_field}`}
                      </div>
                    </li>
                  ))}
              </ul>
            </details>
            <p className="muted">
              authority: {experienceState.authority_effect} · from Session{" "}
              {experienceState.session_generated_at}
            </p>
            {experienceCompareNote && (
              <p className="muted">Compare: {experienceCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No experience snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Current work context</h3>
        <p className="muted">
          What kind of work this is — semantic projection over Session,
          Experience, and Intelligence. Never plans or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Work context refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceWorkContextState>(
                  "generate_workspace_work_context",
                  { workspaceId: workspace.id },
                );
                setWorkContextState(next);
                setWorkContextCompareNote(null);
              })
            }
          >
            Refresh work context
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !workContextState}
            onClick={() =>
              void run("Work context compared", async () => {
                if (!workspace || !workContextState) return;
                const next = await invokeIpc<WorkspaceWorkContextState>(
                  "generate_workspace_work_context",
                  { workspaceId: workspace.id },
                );
                const comparison =
                  await invokeIpc<WorkspaceWorkContextComparison>(
                    "compare_workspace_work_contexts",
                    { left: workContextState, right: next },
                  );
                setWorkContextState(next);
                setWorkContextCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare work context
          </button>
        </div>
        {workContextState ? (
          <>
            <p>
              <strong>
                {workContextState.contexts.find(
                  (c) => c.id === workContextState.primary_context_id,
                )?.name ?? workContextState.summary}
              </strong>
            </p>
            <p className="muted">{workContextState.summary}</p>
            <ul className="intelligence-list">
              {workContextState.contexts
                .filter((c) => c.current_status === "active" || c.current_status === "blocked")
                .map((ctx) => (
                  <li key={ctx.id}>
                    <strong>
                      {ctx.name} · {ctx.context_type} · {ctx.current_status}
                    </strong>
                    <div className="muted">Focus: {ctx.suggested_focus}</div>
                    {ctx.blocked_reasons[0] && (
                      <div className="muted">Blocked: {ctx.blocked_reasons[0]}</div>
                    )}
                    <div className="muted">
                      Evidence:{" "}
                      {ctx.evidence
                        .slice(0, 3)
                        .map((e) => e.label)
                        .join(", ")}
                    </div>
                    <div className="muted">
                      Projects:{" "}
                      {ctx.associated_projects.map((p) => p.label).join(", ") ||
                        "none"}
                    </div>
                    <div className="muted">
                      Tasks:{" "}
                      {ctx.associated_tasks.map((t) => t.label).join(", ") ||
                        "none"}
                    </div>
                    <div className="muted">
                      Apps:{" "}
                      {ctx.associated_applications
                        .map((a) => a.label)
                        .join(", ") || "none"}
                    </div>
                  </li>
                ))}
            </ul>
            <details>
              <summary className="muted">
                Dormant / candidate ({workContextState.dormant_count +
                  workContextState.candidate_count})
              </summary>
              <ul className="intelligence-list">
                {workContextState.contexts
                  .filter(
                    (c) =>
                      c.current_status === "dormant" ||
                      c.current_status === "candidate",
                  )
                  .map((ctx) => (
                    <li key={ctx.id}>
                      <strong>{ctx.name}</strong>
                      <div className="muted">
                        {ctx.current_status} · {ctx.why}
                      </div>
                    </li>
                  ))}
              </ul>
            </details>
            <p className="muted">
              authority: {workContextState.authority_effect} · from Session{" "}
              {workContextState.session_generated_at}
            </p>
            {workContextCompareNote && (
              <p className="muted">Compare: {workContextCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No work context snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Workspace navigation</h3>
        <p className="muted">
          Where to go next — interaction paths over existing understanding.
          Inspection only; never executes or routes autonomously.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Navigation refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceNavigationState>(
                  "generate_workspace_navigation",
                  { workspaceId: workspace.id },
                );
                setNavigationState(next);
                setNavigationCompareNote(null);
              })
            }
          >
            Refresh navigation
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !navigationState}
            onClick={() =>
              void run("Navigation compared", async () => {
                if (!workspace || !navigationState) return;
                const next = await invokeIpc<WorkspaceNavigationState>(
                  "generate_workspace_navigation",
                  { workspaceId: workspace.id },
                );
                const comparison =
                  await invokeIpc<WorkspaceNavigationComparison>(
                    "compare_workspace_navigation",
                    { left: navigationState, right: next },
                  );
                setNavigationState(next);
                setNavigationCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare navigation
          </button>
        </div>
        {navigationState ? (
          <>
            <p>
              <strong>{navigationState.navigation_summary.headline}</strong>
            </p>
            <p className="muted">
              Path: {navigationState.navigation_summary.breadcrumb_line}
            </p>
            <p>{navigationState.navigation_summary.current_path_line}</p>
            <p className="muted">
              Related: {navigationState.navigation_summary.related_line}
            </p>
            <p className="muted">
              Blocked: {navigationState.navigation_summary.blocked_line}
            </p>
            <p className="muted">
              Next inspection:{" "}
              {navigationState.navigation_summary.next_inspection_line}
            </p>
            <ul className="intelligence-list">
              {navigationState.paths
                .filter((p) =>
                  [
                    "current_focus",
                    "blocking_item",
                    "suggested_destination",
                    "possible_next_inspection",
                    "related_work",
                    "dependency_chain",
                  ].includes(p.kind),
                )
                .map((path) => (
                  <li key={path.kind}>
                    <strong>{path.title}</strong>
                    {path.nodes.slice(0, 3).map((node) => (
                      <div key={node.id} className="muted">
                        {node.label}
                        {node.summary ? ` — ${node.summary}` : ""}
                      </div>
                    ))}
                  </li>
                ))}
            </ul>
            <p className="muted">
              authority: {navigationState.authority_effect} ·{" "}
              {navigationState.node_count} stop(s)
            </p>
            {navigationCompareNote && (
              <p className="muted">Compare: {navigationCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No navigation snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Progress toward outcomes</h3>
        <p className="muted">
          Milestone coordination — current, upcoming, blocked, and completed
          outcomes projected from existing understanding. Never plans or
          executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Milestones refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceMilestoneState>(
                  "generate_workspace_milestones",
                  { workspaceId: workspace.id },
                );
                setMilestoneState(next);
                setMilestoneCompareNote(null);
              })
            }
          >
            Refresh milestones
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !milestoneState}
            onClick={() =>
              void run("Milestones compared", async () => {
                if (!workspace || !milestoneState) return;
                const next = await invokeIpc<WorkspaceMilestoneState>(
                  "generate_workspace_milestones",
                  { workspaceId: workspace.id },
                );
                const comparison =
                  await invokeIpc<WorkspaceMilestoneComparison>(
                    "compare_workspace_milestones",
                    { left: milestoneState, right: next },
                  );
                setMilestoneState(next);
                setMilestoneCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare milestones
          </button>
        </div>
        {milestoneState ? (
          <>
            <p>
              <strong>{milestoneState.milestone_summary.headline}</strong>
            </p>
            <p>
              Current: {milestoneState.milestone_summary.current_line}
            </p>
            <p className="muted">
              Closest: {milestoneState.milestone_summary.closest_line}
            </p>
            <p className="muted">
              Blocked: {milestoneState.milestone_summary.blocked_line}
            </p>
            <p className="muted">
              Completed: {milestoneState.milestone_summary.completed_line}
            </p>
            <p className="muted">
              Next attention:{" "}
              {milestoneState.milestone_summary.next_attention_line}
            </p>
            <ul className="intelligence-list">
              {milestoneState.milestones
                .filter((m) =>
                  ["current", "upcoming", "blocked", "completed"].includes(
                    m.status,
                  ),
                )
                .map((m) => (
                  <li key={m.id}>
                    <strong>
                      {m.status}: {m.title}
                    </strong>
                    <div className="muted">
                      {m.progress_percent}% · {m.readiness} — {m.why}
                    </div>
                    {m.dependencies.length > 0 && (
                      <div className="muted">
                        Depends on:{" "}
                        {m.dependencies.map((d) => d.label).join(", ")}
                      </div>
                    )}
                  </li>
                ))}
            </ul>
            {milestoneState.relationships.length > 0 && (
              <details>
                <summary>Milestone dependencies</summary>
                <ul className="muted">
                  {milestoneState.relationships.map((r) => (
                    <li key={r.id}>
                      {r.kind}: {r.from_milestone_id} → {r.to_milestone_id} —{" "}
                      {r.why}
                    </li>
                  ))}
                </ul>
              </details>
            )}
            <p className="muted">
              authority: {milestoneState.authority_effect} ·{" "}
              {milestoneState.milestone_count} milestone(s)
            </p>
            {milestoneCompareNote && (
              <p className="muted">Compare: {milestoneCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No milestone snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>How this Workspace usually works</h3>
        <p className="muted">
          Working Style — observable operating patterns. Observed behaviour is
          kept separate from explicit preferences. Never profiles or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Working style refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceWorkingStyleState>(
                  "generate_workspace_working_style",
                  { workspaceId: workspace.id },
                );
                setWorkingStyleState(next);
                setWorkingStyleCompareNote(null);
              })
            }
          >
            Refresh working style
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !workingStyleState}
            onClick={() =>
              void run("Working style compared", async () => {
                if (!workspace || !workingStyleState) return;
                const next = await invokeIpc<WorkspaceWorkingStyleState>(
                  "generate_workspace_working_style",
                  { workspaceId: workspace.id },
                );
                const comparison =
                  await invokeIpc<WorkspaceWorkingStyleComparison>(
                    "compare_workspace_working_styles",
                    { left: workingStyleState, right: next },
                  );
                setWorkingStyleState(next);
                setWorkingStyleCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare working style
          </button>
        </div>
        {workingStyleState ? (
          <>
            <p>
              <strong>{workingStyleState.style_summary.headline}</strong>
            </p>
            <p className="muted">
              Common workflows: {workingStyleState.style_summary.workflow_line}
            </p>
            <p className="muted">
              Typical contexts:{" "}
              {workingStyleState.style_summary.context_switching_line}
            </p>
            <p className="muted">
              Frequent arrangements:{" "}
              {workingStyleState.style_summary.organization_line}
            </p>
            <p className="muted">
              Observed patterns: {workingStyleState.style_summary.rhythm_line}
            </p>
            <p className="muted">
              Explicit preferences:{" "}
              {workingStyleState.style_summary.preference_line}
            </p>
            <p>
              Observed vs preferred:{" "}
              {workingStyleState.style_summary.observed_vs_preferred_line}
            </p>
            <ul className="intelligence-list">
              {workingStyleState.observations.slice(0, 8).map((obs) => (
                <li key={obs.id}>
                  <strong>
                    [{obs.origin}] {obs.title}
                  </strong>
                  <div className="muted">
                    {obs.confidence} · {obs.kind} — {obs.summary}
                  </div>
                  <div className="muted">{obs.why}</div>
                </li>
              ))}
            </ul>
            <p className="muted">
              authority: {workingStyleState.authority_effect} ·{" "}
              {workingStyleState.observation_count} observation(s) ·{" "}
              {workingStyleState.preference_count} explicit preference(s). Never
              certainty.
            </p>
            {workingStyleCompareNote && (
              <p className="muted">Compare: {workingStyleCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No working style snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Recent transitions</h3>
        <p className="muted">
          Movement between work states — where you left off, what changed, what
          you are entering. Explanation only; never restores or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Transitions refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceTransitionState>(
                  "generate_workspace_transitions",
                  { workspaceId: workspace.id },
                );
                setTransitionState(next);
                setTransitionCompareNote(null);
              })
            }
          >
            Refresh transitions
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !transitionState}
            onClick={() =>
              void run("Transitions compared", async () => {
                if (!workspace || !transitionState) return;
                const next = await invokeIpc<WorkspaceTransitionState>(
                  "generate_workspace_transitions",
                  { workspaceId: workspace.id },
                );
                const comparison =
                  await invokeIpc<WorkspaceTransitionComparison>(
                    "compare_workspace_transitions",
                    { left: transitionState, right: next },
                  );
                setTransitionState(next);
                setTransitionCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare transitions
          </button>
        </div>
        {transitionState ? (
          <>
            <p>
              <strong>{transitionState.transition_summary.headline}</strong>
            </p>
            <p>Where you left off: {transitionState.transition_summary.left_off_line}</p>
            <p className="muted">
              Current transition:{" "}
              {transitionState.transition_summary.current_transition_line}
            </p>
            <p className="muted">
              Changed since last session:{" "}
              {transitionState.transition_summary.changed_line}
            </p>
            <p className="muted">
              Returned work: {transitionState.transition_summary.returned_line}
            </p>
            <p className="muted">
              Context switches:{" "}
              {transitionState.transition_summary.context_switch_line}
            </p>
            <p className="muted">
              Interrupted work:{" "}
              {transitionState.transition_summary.interrupted_line}
            </p>
            <ul className="intelligence-list">
              {transitionState.transitions.slice(0, 6).map((t) => (
                <li key={t.id}>
                  <strong>
                    {t.kind}: {t.title}
                  </strong>
                  <div className="muted">
                    {t.previous_state} → {t.current_state}
                  </div>
                  <div className="muted">{t.why}</div>
                </li>
              ))}
            </ul>
            <p className="muted">
              authority: {transitionState.authority_effect} ·{" "}
              {transitionState.transition_count} transition(s)
            </p>
            {transitionCompareNote && (
              <p className="muted">Compare: {transitionCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No transition snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Things you can do</h3>
        <p className="muted">
          Unified interaction opportunities over existing understanding. Shows
          why each item exists, its source, and the expected next step. Selecting
          creates an Intent handoff only — never executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Interactions refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceInteractionState>(
                  "generate_workspace_interactions",
                  { workspaceId: workspace.id },
                );
                setInteractionState(next);
                setInteractionCompareNote(null);
              })
            }
          >
            Refresh interactions
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !interactionState}
            onClick={() =>
              void run("Interactions compared", async () => {
                if (!workspace || !interactionState) return;
                const next = await invokeIpc<WorkspaceInteractionState>(
                  "generate_workspace_interactions",
                  { workspaceId: workspace.id },
                );
                const comparison =
                  await invokeIpc<WorkspaceInteractionComparison>(
                    "compare_workspace_interactions",
                    { left: interactionState, right: next },
                  );
                setInteractionState(next);
                setInteractionCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare interactions
          </button>
        </div>
        {interactionState ? (
          <>
            <p>
              <strong>{interactionState.interaction_summary.headline}</strong>
            </p>
            <p className="muted">
              {interactionState.interaction_summary.narrative}
            </p>
            <ul className="intelligence-list">
              {interactionState.items.slice(0, 8).map((item) => (
                <li key={item.id}>
                  <strong>{item.title}</strong>
                  <div className="muted">{item.why}</div>
                  <div className="muted">
                    Source: {item.source_projection} · Next:{" "}
                    {item.available_action}
                  </div>
                  <button
                    type="button"
                    disabled={busy || !workspace}
                    onClick={() =>
                      void run("Interaction handoff created", async () => {
                        if (!workspace) return;
                        const result =
                          await invokeIpc<InteractionSelectResult>(
                            "select_workspace_interaction",
                            {
                              workspaceId: workspace.id,
                              interactionId: item.id,
                            },
                          );
                        setLastHandoff(
                          result.handoff
                            ? `${result.handoff.next_command}: ${result.handoff.intent_statement}`
                            : "No handoff",
                        );
                        const next =
                          await invokeIpc<WorkspaceInteractionState>(
                            "generate_workspace_interactions",
                            { workspaceId: workspace.id },
                          );
                        setInteractionState(next);
                      })
                    }
                  >
                    Select (Intent handoff)
                  </button>
                </li>
              ))}
            </ul>
            <p className="muted">
              authority: {interactionState.authority_effect} ·{" "}
              {interactionState.item_count} opportunity(ies)
            </p>
            {interactionCompareNote && (
              <p className="muted">Compare: {interactionCompareNote}</p>
            )}
            {lastHandoff && (
              <p className="muted">Last handoff: {lastHandoff}</p>
            )}
          </>
        ) : (
          <p className="muted">No interaction snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Workspace Profiles</h3>
        <p className="muted">
          User-owned preferred setups. Shows available profiles, alignment,
          differences, and evidence. Describes environments — never activates,
          launches, or restores.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Profiles refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceProfileState>(
                  "generate_workspace_profile_state",
                  { workspaceId: workspace.id },
                );
                setProfileState(next);
                setProfileCompareNote(null);
              })
            }
          >
            Refresh profiles
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Development profile created", async () => {
                if (!workspace) return;
                const members: WorkspaceProfileMemberInput[] = [];
                if (projects[0]) {
                  members.push({
                    member_type: "project",
                    reference_id: projects[0].id,
                    relationship: "preferred",
                    evidence: "User included active project in Development setup",
                    label: projects[0].name,
                  });
                }
                members.push({
                  member_type: "application",
                  reference_id: "vscode",
                  relationship: "expected",
                  evidence: "Development setups usually include VS Code",
                  label: "VS Code",
                });
                members.push({
                  member_type: "application",
                  reference_id: "terminal",
                  relationship: "expected",
                  evidence: "Development setups usually include a terminal",
                  label: "Terminal",
                });
                await invokeIpc<WorkspaceProfile>("create_workspace_profile", {
                  workspaceId: workspace.id,
                  name: "Development setup",
                  description: "Preferred coding environment",
                  members,
                });
                const next = await invokeIpc<WorkspaceProfileState>(
                  "generate_workspace_profile_state",
                  { workspaceId: workspace.id },
                );
                setProfileState(next);
              })
            }
          >
            Create Development setup
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !profileState?.profiles[0]}
            onClick={() =>
              void run("Profile compared", async () => {
                if (!workspace || !profileState?.profiles[0]) return;
                const comparison =
                  await invokeIpc<WorkspaceProfileComparison>(
                    "compare_workspace_profile",
                    {
                      workspaceId: workspace.id,
                      profileId: profileState.profiles[0].id,
                    },
                  );
                setProfileCompareNote(
                  `${comparison.alignment}: matched ${comparison.matched_count}, missing ${comparison.missing_count}`,
                );
              })
            }
          >
            Compare first profile
          </button>
        </div>
        {profileState ? (
          <>
            <p>
              <strong>{profileState.profile_summary.headline}</strong>
            </p>
            <p className="muted">{profileState.profile_summary.alignment_line}</p>
            <p className="muted">{profileState.profile_summary.missing_line}</p>
            <ul className="intelligence-list">
              {profileState.profiles.slice(0, 6).map((profile) => {
                const cmp = profileState.comparisons.find(
                  (c) => c.profile_id === profile.id,
                );
                return (
                  <li key={profile.id}>
                    <strong>{profile.name}</strong>
                    <div className="muted">{profile.description || "No description"}</div>
                    <div className="muted">
                      Members: {profile.members.map((m) => m.label || m.reference_id).join(", ") || "none"}
                    </div>
                    {cmp && (
                      <div className="muted">
                        Alignment: {cmp.alignment} · missing{" "}
                        {cmp.missing_members
                          .map((m) => m.label || m.reference_id)
                          .join(", ") || "none"}
                      </div>
                    )}
                  </li>
                );
              })}
            </ul>
            <p className="muted">
              authority: {profileState.authority_effect} ·{" "}
              {profileState.profile_count} profile(s)
            </p>
            {profileCompareNote && (
              <p className="muted">Compare: {profileCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No profile snapshot yet.</p>
        )}
      </section>

      <section>
        <h3>Working session</h3>
        <p className="muted">
          Runtime orchestration over Workspace Intelligence. Owns no source
          data. Decision Queue, Recommendations, Readiness, and Adaptations
          remain independent.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Session refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceSessionState>(
                  "generate_workspace_session",
                  { workspaceId: workspace.id },
                );
                setSessionState(next);
                setSessionCompareNote(null);
              })
            }
          >
            Refresh session
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !sessionState}
            onClick={() =>
              void run("Session compared", async () => {
                if (!workspace || !sessionState) return;
                const next = await invokeIpc<WorkspaceSessionState>(
                  "generate_workspace_session",
                  { workspaceId: workspace.id },
                );
                const comparison = await invokeIpc<WorkspaceSessionComparison>(
                  "compare_workspace_sessions",
                  { left: sessionState, right: next },
                );
                setSessionState(next);
                setSessionCompareNote(comparison.differences.join(" · "));
              })
            }
          >
            Compare session
          </button>
        </div>
        {sessionState ? (
          <>
            <p>
              <strong>{sessionState.session_summary.headline}</strong>
            </p>
            <p>{sessionState.session_summary.doing_line}</p>
            <p className="muted">{sessionState.session_summary.matters_line}</p>
            <p className="muted">{sessionState.session_summary.blocked_line}</p>
            <p className="muted">{sessionState.session_summary.ready_line}</p>
            <p className="muted">{sessionState.session_summary.changed_line}</p>
            <p className="muted">
              Focus: {sessionState.focus.project_label ?? "no project"} /{" "}
              {sessionState.focus.task_label ?? "no task"} · Decisions{" "}
              {sessionState.decision_count} · Risks {sessionState.risk_count} ·
              Readiness {sessionState.readiness.overall_status} · authority:{" "}
              {sessionState.authority_effect}
            </p>
            {sessionState.risks.length > 0 && (
              <ul className="intelligence-list">
                {sessionState.risks.slice(0, 4).map((risk) => (
                  <li key={risk.id}>
                    <strong>{risk.title}</strong>
                    <div className="muted">
                      {risk.explanation} · from {risk.source_projection}
                    </div>
                  </li>
                ))}
              </ul>
            )}
            {sessionCompareNote && (
              <p className="muted">Compare: {sessionCompareNote}</p>
            )}
          </>
        ) : (
          <p className="muted">No session snapshot yet.</p>
        )}
      </section>

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
              Health: {state.workspace_health} (kernel lifecycle) · Readiness:{" "}
              {state.readiness.overall_status} · Authority effect:{" "}
              {state.authority_effect} · Active:{" "}
              {state.current_project?.name ?? "no project"} /{" "}
              {state.current_task?.title ?? "no task"}
            </p>
            <p className="muted">
              Load uses Intelligence once (shared-input path). Pure projections
              refresh on demand — avoids regenerating Operating State diamonds.
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
          <p className="muted">
            {state?.continuity.summary
              ? `Intelligence summary: ${state.continuity.summary} Refresh for full Continuity detail.`
              : "No continuity snapshot yet."}
          </p>
        )}
      </section>

      <section>
        <h3>What deserves attention</h3>
        <p className="muted">
          Governed prioritization over Workspace context — explainable scores
          only. Attention never observes the desktop, never executes, and never
          grants permission.
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
                      {item.category} · {item.attention_state}
                    </div>
                    {item.reasons.length === 0 ? (
                      <div className="muted">{item.explanation}</div>
                    ) : null}
                    <DisplayReasonList reasons={item.reasons} />
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">
            {state?.attention.summary
              ? `Intelligence summary: ${state.attention.summary} Refresh for full Attention detail.`
              : "No attention snapshot yet."}
          </p>
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
          <p className="muted">
            {state?.environment.summary
              ? `Intelligence summary: ${state.environment.summary} Refresh for full Environment detail.`
              : "No environment snapshot yet."}
          </p>
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
          <p className="muted">
            {state?.composition.summary
              ? `Intelligence summary: ${state.composition.summary} Refresh for full Composition detail.`
              : "No composition snapshot yet."}
          </p>
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
          <p className="muted">
            {state?.purpose.summary
              ? `Intelligence summary: ${state.purpose.summary} Refresh for full Purpose detail.`
              : "No purpose snapshot yet."}
          </p>
        )}
      </section>

      <section>
        <h3>Evolution</h3>
        <p className="muted">
          How work changed — projected from Activity Graph, Task Graph, Purpose,
          Composition, and Continuity. Not a second history store; never
          predicts.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Evolution refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceEvolutionState>(
                  "generate_workspace_evolution",
                  { workspaceId: workspace.id },
                );
                setEvolution(next);
              })
            }
          >
            Refresh evolution
          </button>
        </div>
        {evolution ? (
          <>
            <p>
              <strong>{evolution.label}</strong>
            </p>
            <p>{evolution.summary}</p>
            <p className="muted">{evolution.explanation}</p>
            <p className="muted">
              {evolution.event_count} event(s) · {evolution.insight_count}{" "}
              insight(s) · authority: {evolution.authority_effect}
            </p>
            {evolution.insights.length > 0 && (
              <ul className="intelligence-list">
                {evolution.insights.slice(0, 5).map((insight) => (
                  <li key={insight.id}>
                    <strong>{insight.title}</strong>
                    <div className="muted">{insight.explanation}</div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">
            {state?.evolution.summary
              ? `Intelligence summary: ${state.evolution.summary} Refresh for full Evolution detail.`
              : "No evolution snapshot yet."}
          </p>
        )}
      </section>

      <section>
        <h3>Operating State</h3>
        <p className="muted">
          What is happening right now — unified snapshot over Purpose,
          Environment, Composition, Task Graph, Continuity, Attention,
          Decision Queue, and Recommendations. Aggregates only; never executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Operating State refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceOperatingState>(
                  "generate_workspace_operating_state",
                  { workspaceId: workspace.id },
                );
                setOperatingState(next);
              })
            }
          >
            Refresh operating state
          </button>
        </div>
        {operatingState ? (
          <>
            <p>
              <strong>{operatingState.operating_summary.headline}</strong>
            </p>
            <p>{operatingState.summary}</p>
            <p className="muted">{operatingState.operating_summary.narrative}</p>
            <ul className="muted">
              <li>{operatingState.operating_summary.purpose_line}</li>
              <li>{operatingState.operating_summary.environment_line}</li>
              <li>{operatingState.operating_summary.progress_line}</li>
              <li>{operatingState.operating_summary.pending_line}</li>
              <li>{operatingState.operating_summary.suggested_line}</li>
            </ul>
            <p className="muted">
              {operatingState.signal_count} signal(s) · authority:{" "}
              {operatingState.authority_effect}
            </p>
          </>
        ) : (
          <p className="muted">
            {state?.operating_state.summary
              ? `Intelligence summary: ${state.operating_state.summary} Refresh for full Operating State detail.`
              : "No operating state snapshot yet."}
          </p>
        )}
      </section>

      <section>
        <h3>Pattern Model</h3>
        <p className="muted">
          Things this workspace often does — recurring structures from Activity,
          Evolution, Operating State, Composition, and Task Graph. Observations
          only; not prediction, profiling, or automation.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Pattern Model refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspacePatternState>(
                  "generate_workspace_pattern",
                  { workspaceId: workspace.id },
                );
                setPatternState(next);
              })
            }
          >
            Refresh patterns
          </button>
        </div>
        {patternState ? (
          <>
            <p>
              <strong>{patternState.pattern_summary.headline}</strong>
            </p>
            <p>{patternState.summary}</p>
            <p className="muted">{patternState.pattern_summary.narrative}</p>
            <p className="muted">
              {patternState.pattern_count} pattern(s) · authority:{" "}
              {patternState.authority_effect}
            </p>
            {patternState.patterns.length > 0 && (
              <ul className="intelligence-list">
                {patternState.patterns.slice(0, 5).map((item) => (
                  <li key={item.id}>
                    <strong>
                      [{item.kind}] {item.title}
                    </strong>
                    <div className="muted">
                      Observation: {item.observation} · Impact: {item.impact} ·
                      Confidence: {item.confidence}
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">
            {state?.pattern.summary
              ? `Intelligence summary: ${state.pattern.summary} Refresh for full Pattern detail.`
              : "No pattern snapshot yet."}
          </p>
        )}
      </section>

      <section>
        <h3>Can I Continue Working?</h3>
        <p className="muted">
          Readiness — preparedness for current work from Operating State,
          Environment, Composition, Task Graph, Purpose, Continuity, Evolution,
          Patterns, and Decision Queue. Describes gaps only; never prepares or
          executes. Distinct from runtime health.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Readiness refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceReadinessState>(
                  "generate_workspace_readiness",
                  { workspaceId: workspace.id },
                );
                setReadinessState(next);
              })
            }
          >
            Refresh readiness
          </button>
        </div>
        {readinessState ? (
          <>
            <p>
              <strong>{readinessState.readiness_summary.headline}</strong>
            </p>
            <p>{readinessState.readiness_summary.status_line}</p>
            <p className="muted">{readinessState.readiness_summary.narrative}</p>
            <p className="muted">
              Status: {readinessState.overall_status} · {readinessState.gap_count}{" "}
              gap(s) · authority: {readinessState.authority_effect}
            </p>
            {readinessState.assessments.length > 0 && (
              <ul className="intelligence-list">
                {readinessState.assessments.map((item) => (
                  <li key={item.id}>
                    <strong>
                      [{item.kind}] {item.title} — {item.status}
                    </strong>
                    <div className="muted">
                      Why: {item.reason} · Impact: {item.impact}
                      {item.gaps.length > 0
                        ? ` · Gaps: ${item.gaps.map((g) => g.title).join("; ")}`
                        : ""}
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </>
        ) : (
          <p className="muted">
            {state?.readiness.summary
              ? `Intelligence summary: ${state.readiness.summary} Refresh for full Readiness detail.`
              : "No readiness snapshot yet."}
          </p>
        )}
      </section>

      <section>
        <h3>Possible Improvements</h3>
        <p className="muted">
          Adaptation proposals — possible Workspace improvements grounded in
          Pattern, Recommendations, Operating State, and Composition. Proposals
          only; review then accept hands off to Intent. Never applies changes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Adaptation proposals refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceAdaptationState>(
                  "generate_workspace_adaptation",
                  { workspaceId: workspace.id },
                );
                setAdaptationState(next);
              })
            }
          >
            Refresh adaptations
          </button>
        </div>
        {adaptationState ? (
          <>
            <p>
              <strong>{adaptationState.adaptation_summary.headline}</strong>
            </p>
            <p>{adaptationState.summary}</p>
            <p className="muted">
              {adaptationState.adaptation_summary.narrative}
            </p>
            <p className="muted">
              {adaptationState.open_count} open · authority:{" "}
              {adaptationState.authority_effect}
            </p>
            {adaptationState.proposals.length > 0 && (
              <ul className="intelligence-list">
                {adaptationState.proposals.slice(0, 5).map((item) => (
                  <li key={item.id}>
                    <strong>
                      [{item.kind}] {item.title}
                    </strong>
                    <div className="muted">
                      Why: {item.reason} · Benefit: {item.impact.benefit} ·
                      Risk: {item.impact.risk} · Status: {item.status}
                    </div>
                    <div className="row">
                      <button
                        type="button"
                        disabled={busy || !workspace || item.status !== "proposed"}
                        onClick={() =>
                          void run("Adaptation reviewed", async () => {
                            if (!workspace) return;
                            await invokeIpc<AdaptationActionResult>(
                              "review_adaptation_proposal",
                              {
                                workspaceId: workspace.id,
                                proposalId: item.id,
                              },
                            );
                            const next =
                              await invokeIpc<WorkspaceAdaptationState>(
                                "generate_workspace_adaptation",
                                { workspaceId: workspace.id },
                              );
                            setAdaptationState(next);
                          })
                        }
                      >
                        Review
                      </button>
                      <button
                        type="button"
                        disabled={
                          busy || !workspace || item.status !== "reviewed"
                        }
                        onClick={() =>
                          void run("Adaptation accepted (Intent handoff)", async () => {
                            if (!workspace) return;
                            const result =
                              await invokeIpc<AdaptationActionResult>(
                                "accept_adaptation_proposal",
                                {
                                  workspaceId: workspace.id,
                                  proposalId: item.id,
                                },
                              );
                            if (result.handoff) {
                              setLastHandoff(
                                `Intent handoff via ${result.handoff.next_command}`,
                              );
                            }
                            const next =
                              await invokeIpc<WorkspaceAdaptationState>(
                                "generate_workspace_adaptation",
                                { workspaceId: workspace.id },
                              );
                            setAdaptationState(next);
                          })
                        }
                      >
                        Accept (handoff)
                      </button>
                      <button
                        type="button"
                        disabled={
                          busy ||
                          !workspace ||
                          (item.status !== "proposed" &&
                            item.status !== "reviewed")
                        }
                        onClick={() =>
                          void run("Adaptation rejected", async () => {
                            if (!workspace) return;
                            await invokeIpc<AdaptationActionResult>(
                              "reject_adaptation_proposal",
                              {
                                workspaceId: workspace.id,
                                proposalId: item.id,
                              },
                            );
                            const next =
                              await invokeIpc<WorkspaceAdaptationState>(
                                "generate_workspace_adaptation",
                                { workspaceId: workspace.id },
                              );
                            setAdaptationState(next);
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
          </>
        ) : (
          <p className="muted">No adaptation proposals yet.</p>
        )}
      </section>

      <section>
        <h3>Recommendation Engine</h3>
        <p className="muted">
          What might help next — suggestions grounded in Attention, Continuity,
          Evolution, Purpose, Task Graph, Composition, Decision Queue, and
          Environment. Explanation shows why a suggestion is shown (evidence,
          lifecycle, Experience catalog keys). Present / Accept / Reject record
          recommendation agreement only — not a Decision Engine object, intent,
          or execution authority. Confirm future decision is separate and still
          creates nothing. Distinct from Decision Engine accept/handoff below.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Recommendation Engine refreshed", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceRecommendationEngineState>(
                  "generate_workspace_recommendation_engine",
                  { workspaceId: workspace.id },
                );
                setRecommendationEngine(next);
              })
            }
          >
            Refresh recommendations
          </button>
        </div>
        {recommendationEngine ? (
          <>
            <p>
              <strong>{recommendationEngine.label}</strong>
            </p>
            <p>{recommendationEngine.summary}</p>
            <p className="muted">{recommendationEngine.explanation}</p>
            <p className="muted">
              Active{" "}
              {
                recommendationEngine.candidates.filter(isActiveRecommendation)
                  .length
              }{" "}
              · history {recommendationEngine.history_count ?? 0} · authority:{" "}
              {recommendationEngine.authority_effect}
            </p>
            {recommendationEngine.candidates.filter(isActiveRecommendation)
              .length > 0 && (
              <ul className="intelligence-list">
                {recommendationEngine.candidates
                  .filter(isActiveRecommendation)
                  .slice(0, 6)
                  .map((item) => (
                    <li key={item.id}>
                      <strong>
                        [{item.kind}] {item.title}
                      </strong>
                      <RecommendationExplanationBlock item={item} />
                      <div className="row">
                        <button
                          type="button"
                          disabled={
                            busy ||
                            !workspace ||
                            item.lifecycle_state === "presented"
                          }
                          onClick={() =>
                            void run("Recommendation presented", async () => {
                              if (!workspace) return;
                              await invokeIpc("present_recommendation", {
                                workspaceId: workspace.id,
                                recommendationId: item.id,
                              });
                              const next =
                                await invokeIpc<WorkspaceRecommendationEngineState>(
                                  "generate_workspace_recommendation_engine",
                                  { workspaceId: workspace.id },
                                );
                              setRecommendationEngine(next);
                            })
                          }
                        >
                          Present
                        </button>
                        <button
                          type="button"
                          disabled={busy || !workspace}
                          onClick={() =>
                            void run(
                              "Recommendation accepted (decision only)",
                              async () => {
                                if (!workspace) return;
                                await invokeIpc("accept_recommendation", {
                                  workspaceId: workspace.id,
                                  recommendationId: item.id,
                                });
                                const next =
                                  await invokeIpc<WorkspaceRecommendationEngineState>(
                                    "generate_workspace_recommendation_engine",
                                    { workspaceId: workspace.id },
                                  );
                                setRecommendationEngine(next);
                              },
                            )
                          }
                        >
                          Accept
                        </button>
                        <button
                          type="button"
                          disabled={busy || !workspace}
                          onClick={() =>
                            void run(
                              "Recommendation rejected (decision only)",
                              async () => {
                                if (!workspace) return;
                                await invokeIpc("reject_recommendation", {
                                  workspaceId: workspace.id,
                                  recommendationId: item.id,
                                });
                                const next =
                                  await invokeIpc<WorkspaceRecommendationEngineState>(
                                    "generate_workspace_recommendation_engine",
                                    { workspaceId: workspace.id },
                                  );
                                setRecommendationEngine(next);
                              },
                            )
                          }
                        >
                          Reject
                        </button>
                        {item.decision_confirmation?.confirmation_state ===
                          "required" && (
                          <>
                            <button
                              type="button"
                              disabled={busy || !workspace}
                              onClick={() =>
                                void run(
                                  "Future decision consideration confirmed (no DE object)",
                                  async () => {
                                    if (!workspace) return;
                                    await invokeIpc(
                                      "confirm_recommendation_decision",
                                      {
                                        workspaceId: workspace.id,
                                        recommendationId: item.id,
                                        confirmationIntent:
                                          "create_future_decision",
                                      },
                                    );
                                    const next =
                                      await invokeIpc<WorkspaceRecommendationEngineState>(
                                        "generate_workspace_recommendation_engine",
                                        { workspaceId: workspace.id },
                                      );
                                    setRecommendationEngine(next);
                                  },
                                )
                              }
                            >
                              Confirm future decision
                            </button>
                            <button
                              type="button"
                              disabled={busy || !workspace}
                              onClick={() =>
                                void run(
                                  "Future decision consideration declined",
                                  async () => {
                                    if (!workspace) return;
                                    await invokeIpc(
                                      "decline_recommendation_decision",
                                      {
                                        workspaceId: workspace.id,
                                        recommendationId: item.id,
                                      },
                                    );
                                    const next =
                                      await invokeIpc<WorkspaceRecommendationEngineState>(
                                        "generate_workspace_recommendation_engine",
                                        { workspaceId: workspace.id },
                                      );
                                    setRecommendationEngine(next);
                                  },
                                )
                              }
                            >
                              Decline future decision
                            </button>
                          </>
                        )}
                      </div>
                    </li>
                  ))}
              </ul>
            )}
            {(recommendationEngine.history?.length ?? 0) > 0 && (
              <>
                <p className="muted">
                  Outcome history (immutable feedback — not actionable)
                </p>
                <RecommendationHistoryList
                  history={(recommendationEngine.history ?? []).slice(0, 5)}
                />
              </>
            )}
          </>
        ) : (
          <p className="muted">
            {state?.recommendation_engine.summary
              ? `Intelligence summary: ${state.recommendation_engine.summary} Refresh for full Recommendation Engine detail.`
              : "No recommendation engine snapshot yet."}
          </p>
        )}
      </section>

      <section>
        <h3>Recommended Actions</h3>
        <p className="muted">
          Decision Engine synthesizes Attention, memory, preferences, and goals
          into ranked candidates. Accept hands off to the Planner — never
          executes. Separate from the Recommendation Engine above.
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
                      Confidence {candidate.explanation.confidence} · {candidate.outcome}
                    </div>
                    <DecisionReasonList
                      reasons={candidate.explanation.reasons}
                    />
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
                            const result = await invokeIpc<DecisionEngineActionResult>(
                              "select_decision_candidate",
                              {
                              workspaceId: workspace.id,
                              candidateId: candidate.id,
                              },
                            );
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
          <p className="muted">
            {state && state.activity_graph.activity_count > 0
              ? `Intelligence summary: ${state.activity_graph.activity_count} activit(ies), ${state.activity_graph.unresolved_count} unresolved. Refresh for full Activity Graph detail.`
              : "No activity yet for this workspace."}
          </p>
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
                  {rec.reasons.length === 0 ? (
                    <div className="muted">{rec.explanation}</div>
                  ) : null}
                  <DisplayReasonList reasons={rec.reasons} />
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
