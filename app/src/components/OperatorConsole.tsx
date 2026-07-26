import { useCallback, useEffect, useState } from "react";
import { IpcCommandError, invokeIpc } from "../lib/ipc";
import { DisplayReasonList } from "./DisplayReasonList";
import type {
  ActionCatalog,
  AiAssistantPlanComparison,
  AiAssistantWorkflow,
  AiMemoryAwareness,
  Project,
  Task,
  WorkspaceIntelligenceState,
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
  WorkspaceReadinessState,
  WorkspaceRuntimeOperatorView,
  WorkspaceSessionState,
  WorkspaceSessionComparison,
  WorkspaceExperienceState,
  WorkspaceExperienceComparison,
  WorkspaceWorkContextState,
  WorkspaceWorkContextComparison,
  WorkspaceWorkContextValidation,
  WorkspaceNavigationState,
  WorkspaceNavigationComparison,
  WorkspaceNavigationValidation,
  WorkspaceMilestoneState,
  WorkspaceMilestoneComparison,
  WorkspaceMilestoneValidation,
  WorkspaceWorkingStyleState,
  WorkspaceWorkingStyleComparison,
  WorkspaceWorkingStyleValidation,
  WorkspaceTransitionState,
  WorkspaceTransitionComparison,
  WorkspaceTransitionValidation,
  WorkspaceInteractionState,
  WorkspaceInteractionComparison,
  WorkspaceInteractionValidation,
  InteractionSelectResult,
  WorkspaceProfileState,
  WorkspaceProfileStateComparison,
  WorkspaceProfileValidation,
  WorkspaceProfile,
  WorkspaceProfileMemberInput,
  AiOrchestratedPlan,
  AiPlan,
  AiPlanEvaluationReport,
  AiPlanSubmissionResult,
  AiProposalEvaluation,
  ApplicationLaunchResult,
  ApplicationReference,
  ApprovalDecisionResult,
  CancellationRequest,
  ExecutionOutcome,
  ExecutionReconciliation,
  IntentExecutionRequest,
  MemoryEntry,
  ModelProviderDescriptor,
  ModelResponse,
  PersonalizedPlanComparison,
  PermissionApprovalRequest,
  UserPreference,
  UserPreferenceProfile,
  Suggestion,
  SuggestionIntentRequest,
  SuggestionLifecycleRecord,
  Workspace,
  WorkspaceContext,
  WorkspaceState,
  WorkspaceStateWindow,
  Zone,
} from "../types/domain";
import type {
  WorkspaceHealth,
  WorkspaceSettings,
  WorkspaceStatus,
} from "../types/workspace";

function formatError(err: unknown): string {
  if (err instanceof Error) {
    return err.message;
  }
  return String(err);
}

function formatLaunchError(err: unknown): string {
  if (err instanceof IpcCommandError) {
    if (err.code === "permission_denied") {
      const reason = err.message.replace(/^This action was not permitted:\s*/i, "");
      return `Launch denied: ${reason || "this actor does not have permission to open this application."}`;
    }
    if (err.code === "approval_required") {
      return err.message.startsWith("This action requires approval")
        ? err.message
        : `This action requires approval. ${err.message}`;
    }
    if (err.code === "invalid_launch_target") {
      return `Launch failed: ${err.message}`;
    }
    return `${err.message} (${err.code})`;
  }
  return formatError(err);
}

function executionIdFor(suggestionId: string): string {
  return `execution:${suggestionId}`;
}

export interface OperatorConsoleProps {
  workspace: Workspace | null;
  zones: Zone[];
  onWorkspaceChange: (workspace: Workspace | null) => void;
  onZonesChange: (zones: Zone[]) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

export function OperatorConsole({
  workspace,
  zones,
  onWorkspaceChange,
  onZonesChange,
  onError,
  onMessage,
}: OperatorConsoleProps) {
  const [status, setStatus] = useState<WorkspaceStatus | null>(null);
  const [health, setHealth] = useState<WorkspaceHealth | null>(null);
  const [settings, setSettings] = useState<WorkspaceSettings | null>(null);
  const [suggestions, setSuggestions] = useState<Suggestion[]>([]);
  const [lifecycle, setLifecycle] = useState<SuggestionLifecycleRecord[]>([]);
  const [context, setContext] = useState<WorkspaceContext | null>(null);
  const [outcomes, setOutcomes] = useState<ExecutionOutcome[]>([]);
  const [executionStates, setExecutionStates] = useState<
    ExecutionReconciliation[]
  >([]);
  const [workspaceStateWindows, setWorkspaceStateWindows] = useState<
    WorkspaceStateWindow[]
  >([]);
  const [workspaceStateMeta, setWorkspaceStateMeta] = useState<string | null>(
    null,
  );
  const [lastIntent, setLastIntent] = useState<SuggestionIntentRequest | null>(
    null,
  );
  const [lastExecution, setLastExecution] =
    useState<IntentExecutionRequest | null>(null);
  const [lastCancellation, setLastCancellation] =
    useState<CancellationRequest | null>(null);

  const [workspaceName, setWorkspaceName] = useState("Operator Workspace");
  const [zoneName, setZoneName] = useState("Zone");
  const [appName, setAppName] = useState("Notepad");
  const [executablePath, setExecutablePath] = useState("notepad.exe");
  const [lastLaunch, setLastLaunch] = useState<ApplicationLaunchResult | null>(
    null,
  );
  const [lastRegisteredApp, setLastRegisteredApp] =
    useState<ApplicationReference | null>(null);
  const [approvals, setApprovals] = useState<PermissionApprovalRequest[]>([]);
  const [lastAiPlan, setLastAiPlan] = useState<AiPlanSubmissionResult | null>(
    null,
  );
  const [actionCatalog, setActionCatalog] = useState<ActionCatalog | null>(
    null,
  );
  const [lastEvaluation, setLastEvaluation] =
    useState<AiPlanEvaluationReport | null>(null);
  const [evaluationHistory, setEvaluationHistory] = useState<
    AiProposalEvaluation[]
  >([]);
  const [orchestratedPlan, setOrchestratedPlan] =
    useState<AiOrchestratedPlan | null>(null);
  const [assistantWorkflow, setAssistantWorkflow] =
    useState<AiAssistantWorkflow | null>(null);
  const [assistantComparison, setAssistantComparison] =
    useState<AiAssistantPlanComparison | null>(null);
  const [assistantGoal, setAssistantGoal] = useState(
    "Prepare my coding workspace",
  );
  const [workspaceIntelligence, setWorkspaceIntelligence] =
    useState<WorkspaceIntelligenceState | null>(null);
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
  const [runtimeOverview, setRuntimeOverview] =
    useState<WorkspaceRuntimeOperatorView | null>(null);
  const [sessionState, setSessionState] =
    useState<WorkspaceSessionState | null>(null);
  const [priorSession, setPriorSession] =
    useState<WorkspaceSessionState | null>(null);
  const [experienceState, setExperienceState] =
    useState<WorkspaceExperienceState | null>(null);
  const [priorExperience, setPriorExperience] =
    useState<WorkspaceExperienceState | null>(null);
  const [workContextState, setWorkContextState] =
    useState<WorkspaceWorkContextState | null>(null);
  const [priorWorkContext, setPriorWorkContext] =
    useState<WorkspaceWorkContextState | null>(null);
  const [navigationState, setNavigationState] =
    useState<WorkspaceNavigationState | null>(null);
  const [priorNavigation, setPriorNavigation] =
    useState<WorkspaceNavigationState | null>(null);
  const [milestoneState, setMilestoneState] =
    useState<WorkspaceMilestoneState | null>(null);
  const [priorMilestones, setPriorMilestones] =
    useState<WorkspaceMilestoneState | null>(null);
  const [workingStyleState, setWorkingStyleState] =
    useState<WorkspaceWorkingStyleState | null>(null);
  const [priorWorkingStyle, setPriorWorkingStyle] =
    useState<WorkspaceWorkingStyleState | null>(null);
  const [transitionState, setTransitionState] =
    useState<WorkspaceTransitionState | null>(null);
  const [priorTransitions, setPriorTransitions] =
    useState<WorkspaceTransitionState | null>(null);
  const [interactionState, setInteractionState] =
    useState<WorkspaceInteractionState | null>(null);
  const [priorInteractions, setPriorInteractions] =
    useState<WorkspaceInteractionState | null>(null);
  const [profileState, setProfileState] =
    useState<WorkspaceProfileState | null>(null);
  const [priorProfileState, setPriorProfileState] =
    useState<WorkspaceProfileState | null>(null);
  const [activeProject, setActiveProject] = useState<Project | null>(null);
  const [activeTask, setActiveTask] = useState<Task | null>(null);
  const [memoryEntries, setMemoryEntries] = useState<MemoryEntry[]>([]);
  const [memoryContext, setMemoryContext] = useState<AiMemoryAwareness | null>(
    null,
  );
  const [memoryPlan, setMemoryPlan] = useState<AiPlan | null>(null);
  const [modelProviders, setModelProviders] = useState<
    ModelProviderDescriptor[]
  >([]);
  const [selectedProvider, setSelectedProvider] =
    useState<ModelProviderDescriptor | null>(null);
  const [modelResponse, setModelResponse] = useState<ModelResponse | null>(
    null,
  );
  const [modelPlan, setModelPlan] = useState<AiPlan | null>(null);
  const [preferenceProfile, setPreferenceProfile] =
    useState<UserPreferenceProfile | null>(null);
  const [personalizedPlan, setPersonalizedPlan] = useState<AiPlan | null>(null);
  const [planComparison, setPlanComparison] =
    useState<PersonalizedPlanComparison | null>(null);
  const [busy, setBusy] = useState(false);

  const run = useCallback(
    async (label: string, action: () => Promise<void>) => {
      setBusy(true);
      onError(null);
      onMessage(null);
      try {
        await action();
        onMessage(label);
      } catch (err: unknown) {
        onError(formatError(err));
      } finally {
        setBusy(false);
      }
    },
    [onError, onMessage],
  );

  const refreshReads = useCallback(
    async (workspaceId: string) => {
      const [
        nextSuggestions,
        nextLifecycle,
        nextContext,
        nextOutcomes,
        nextStates,
        nextApprovals,
      ] = await Promise.all([
        invokeIpc<Suggestion[]>("get_suggestions", {
          workspaceId,
          limit: 50,
        }),
        invokeIpc<SuggestionLifecycleRecord[]>("get_suggestion_lifecycle", {
          limit: 50,
        }),
        invokeIpc<WorkspaceContext>("get_workspace_context", {
          workspaceId,
          limit: 200,
        }),
        invokeIpc<ExecutionOutcome[]>("get_execution_outcomes", {
          limit: 50,
        }),
        invokeIpc<ExecutionReconciliation[]>("get_execution_states", {
          limit: 50,
        }),
        invokeIpc<PermissionApprovalRequest[]>("get_permission_approvals", {
          limit: 50,
        }),
      ]);
      setSuggestions(nextSuggestions);
      setLifecycle(nextLifecycle);
      setContext(nextContext);
      setOutcomes(nextOutcomes);
      setExecutionStates(nextStates);
      setApprovals(nextApprovals);
      onZonesChange(
        nextContext.snapshot.zones.map((z) => {
          const summary = z as {
            resource_ref?: { id?: string };
            name?: string;
          };
          return {
            id: String(summary.resource_ref?.id ?? ""),
            workspace_id: workspaceId,
            name: String(summary.name ?? ""),
            position_metadata: null,
          };
        }),
      );
    },
    [onZonesChange],
  );

  useEffect(() => {
    Promise.all([
      invokeIpc<WorkspaceStatus>("get_workspace_status"),
      invokeIpc<WorkspaceHealth>("get_workspace_health"),
      invokeIpc<WorkspaceSettings>("get_settings"),
    ])
      .then(([nextStatus, nextHealth, nextSettings]) => {
        setStatus(nextStatus);
        setHealth(nextHealth);
        setSettings(nextSettings);
      })
      .catch((err: unknown) => onError(formatError(err)));
  }, [onError]);

  useEffect(() => {
    if (!workspace) {
      return;
    }
    void refreshReads(workspace.id).catch((err: unknown) =>
      onError(formatError(err)),
    );
  }, [workspace?.id, onError, refreshReads]);

  const createWorkspace = () =>
    run("Workspace created", async () => {
      const created = await invokeIpc<Workspace>("create_workspace", {
        name: workspaceName.trim() || "Operator Workspace",
      });
      await invokeIpc<WorkspaceSettings>("update_settings", {
        update: { active_workspace_id: created.id },
      });
      localStorage.removeItem("workspace.active_id");
      onWorkspaceChange(created);
      onZonesChange([]);
      await refreshReads(created.id);
    });

  const seedZones = () =>
    run("Zones seeded", async () => {
      if (!workspace) {
        throw new Error("Create or load a workspace first.");
      }
      const created: Zone[] = [];
      for (let i = 1; i <= 3; i += 1) {
        const zone = await invokeIpc<Zone>("create_zone", {
          workspaceId: workspace.id,
          name: `${zoneName.trim() || "Zone"} ${i}`,
          positionMetadata: null,
        });
        created.push(zone);
      }
      onZonesChange([...zones, ...created]);
      await refreshReads(workspace.id);
    });

  const accept = (suggestionId: string) =>
    run("Suggestion accepted", async () => {
      if (!workspace) {
        throw new Error("Create or load a workspace first.");
      }
      await invokeIpc<Suggestion>("accept_suggestion", {
        workspaceId: workspace.id,
        suggestionId,
      });
      await refreshReads(workspace.id);
    });

  const reject = (suggestionId: string) =>
    run("Suggestion rejected", async () => {
      if (!workspace) {
        throw new Error("Create or load a workspace first.");
      }
      await invokeIpc<Suggestion>("reject_suggestion", {
        workspaceId: workspace.id,
        suggestionId,
      });
      await refreshReads(workspace.id);
    });

  const createIntent = (suggestionId: string) =>
    run("Intent request created", async () => {
      if (!workspace) {
        throw new Error("Create or load a workspace first.");
      }
      const intent = await invokeIpc<SuggestionIntentRequest>(
        "create_suggestion_intent_request",
        { workspaceId: workspace.id, suggestionId },
      );
      setLastIntent(intent);
      await refreshReads(workspace.id);
    });

  const execute = (suggestionId: string) =>
    run("Intent executed", async () => {
      if (!workspace) {
        throw new Error("Create or load a workspace first.");
      }
      const execution = await invokeIpc<IntentExecutionRequest>(
        "execute_intent_request",
        { workspaceId: workspace.id, suggestionId },
      );
      setLastExecution(execution);
      await refreshReads(workspace.id);
    });

  const cancelExecution = (suggestionId: string) =>
    run("Cancellation requested", async () => {
      if (!workspace) {
        throw new Error("Create or load a workspace first.");
      }
      const cancellation = await invokeIpc<CancellationRequest>(
        "request_execution_cancellation",
        {
          workspaceId: workspace.id,
          executionRequestId: executionIdFor(suggestionId),
          reason: "operator_console",
        },
      );
      setLastCancellation(cancellation);
      await refreshReads(workspace.id);
    });

  const updateTheme = (theme: string) =>
    run("Settings updated", async () => {
      const next = await invokeIpc<WorkspaceSettings>("update_settings", {
        update: { theme, first_run: false },
      });
      setSettings(next);
    });

  const refreshWorkspaceState = () =>
    run("WorkspaceState refreshed", async () => {
      const state = await invokeIpc<WorkspaceState>("get_workspace_state");
      setWorkspaceStateWindows(state.windows);
      setWorkspaceStateMeta(
        `${state.metadata.window_count} windows · pass ${
          state.metadata.observation_pass_id ?? "none"
        }`,
      );
    });

  const registerAndLaunch = () => {
    setBusy(true);
    onError(null);
    onMessage(null);
    void (async () => {
      try {
        if (!workspace) {
          throw new Error("Create or load a workspace first.");
        }
        const application = await invokeIpc<ApplicationReference>(
          "create_application",
          {
            workspaceId: workspace.id,
            name: appName,
            identifier: null,
            executablePath,
          },
        );
        setLastRegisteredApp(application);
        const result = await invokeIpc<ApplicationLaunchResult>(
          "launch_application",
          { id: application.id },
        );
        setLastLaunch(result);
        const mode = result.simulated ? "simulated" : `pid ${result.process_id ?? "?"}`;
        onMessage(
          `Launch allowed: ${result.name} (${result.executable_path}) — ${mode}`,
        );
      } catch (err: unknown) {
        onError(formatLaunchError(err));
      } finally {
        setBusy(false);
      }
    })();
  };

  return (
    <div className="operator-console">
      {busy && <p className="muted">Working…</p>}

      {status && (
        <section>
          <h2>Runtime</h2>
          <dl>
            <dt>Status</dt>
            <dd>{status.status}</dd>
            <dt>Version</dt>
            <dd>{status.version}</dd>
            <dt>Initialized</dt>
            <dd>{status.initialized ? "yes" : "no"}</dd>
          </dl>
        </section>
      )}

      {health && (
        <section>
          <h2>Health</h2>
          <dl>
            <dt>Status</dt>
            <dd>{health.status}</dd>
            <dt>Services</dt>
            <dd>{health.services.join(", ")}</dd>
          </dl>
        </section>
      )}

      <section>
        <h2>Governed application launch</h2>
        <p className="muted">
          Registers an application resource, then launches through Permission
          Gateway (Allow / Deny / ApprovalRequired).
        </p>
        <div className="row">
          <input
            value={appName}
            disabled={busy}
            onChange={(event) => setAppName(event.target.value)}
            placeholder="Application name"
          />
          <input
            value={executablePath}
            disabled={busy}
            onChange={(event) => setExecutablePath(event.target.value)}
            placeholder="Executable path"
          />
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() => registerAndLaunch()}
          >
            Register &amp; launch
          </button>
        </div>
        {lastLaunch && (
          <dl>
            <dt>Last launch</dt>
            <dd>
              {lastLaunch.name} → {lastLaunch.executable_path}
              {lastLaunch.simulated
                ? " (simulated)"
                : lastLaunch.process_id != null
                  ? ` (pid ${lastLaunch.process_id})`
                  : ""}
            </dd>
          </dl>
        )}
        <div className="row">
          <button
            type="button"
            disabled={busy || !lastRegisteredApp}
            onClick={() =>
              void (async () => {
                if (!lastRegisteredApp || !workspace) return;
                setBusy(true);
                onError(null);
                onMessage(null);
                try {
                  const result = await invokeIpc<ApplicationLaunchResult>(
                    "request_ai_application_launch",
                    { id: lastRegisteredApp.id },
                  );
                  setLastLaunch(result);
                  onMessage(
                    `AI launch allowed: ${result.name} (grant consumed)`,
                  );
                  await refreshReads(workspace.id);
                } catch (err: unknown) {
                  onError(formatLaunchError(err));
                  await refreshReads(workspace.id);
                } finally {
                  setBusy(false);
                }
              })()
            }
          >
            Propose as AI (governed path)
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void (async () => {
                if (!workspace) return;
                setBusy(true);
                onError(null);
                onMessage(null);
                try {
                  const result = await invokeIpc<AiPlanSubmissionResult>(
                    "diagnose_ai_workspace_plan",
                    {
                      goal: "Prepare my workspace",
                      workspaceId: workspace.id,
                    },
                  );
                  setLastAiPlan(result);
                  const outcomes = result.submissions
                    .map((s) => s.outcome.kind)
                    .join(", ");
                  onMessage(
                    `AI plan: ${result.plan.proposals.length} proposal(s) → ${outcomes || "none"} (no auto-retry)`,
                  );
                  if (workspace) {
                    await refreshReads(workspace.id);
                  }
                } catch (err: unknown) {
                  onError(formatError(err));
                } finally {
                  setBusy(false);
                }
              })()
            }
          >
            Plan as AI (prepare workspace)
          </button>
        </div>
        {lastAiPlan && (
          <dl>
            <dt>Last AI plan</dt>
            <dd>
              {lastAiPlan.plan.goal.statement} —{" "}
              {lastAiPlan.plan.proposals.length} proposal(s)
              {lastAiPlan.submissions.map((s) => (
                <div key={s.proposal.id}>
                  {s.proposal.command_name}: {s.outcome.kind}
                </div>
              ))}
            </dd>
          </dl>
        )}
      </section>

      <section>
        <h2>Model providers</h2>
        <p className="muted">
          Providers supply intelligence only. Model output becomes proposals —
          Permission Gateway remains the sole authority boundary.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Model providers listed", async () => {
                const providers = await invokeIpc<ModelProviderDescriptor[]>(
                  "list_model_providers",
                );
                setModelProviders(providers);
              })
            }
          >
            List available model providers
          </button>
          <button
            type="button"
            disabled={busy || modelProviders.length === 0}
            onClick={() =>
              void run("Model metadata loaded", async () => {
                const id =
                  modelProviders.find((p) => p.availability === "available")
                    ?.provider_id ?? modelProviders[0]?.provider_id;
                if (!id) return;
                const metadata = await invokeIpc<ModelProviderDescriptor>(
                  "get_model_provider_metadata",
                  { providerId: id },
                );
                setSelectedProvider(metadata);
              })
            }
          >
            Inspect model metadata
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Provider request tested", async () => {
                const response = await invokeIpc<ModelResponse>(
                  "test_model_provider_request",
                  {
                    task: "Prepare my coding workspace",
                    applicationIds: lastRegisteredApp
                      ? [lastRegisteredApp.id]
                      : [],
                    preferredProviderId: "deterministic",
                  },
                );
                setModelResponse(response);
              })
            }
          >
            Test provider request
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Proposal generation tested", async () => {
                if (!workspace) return;
                const plan = await invokeIpc<AiPlan>(
                  "diagnose_model_proposal_generation",
                  {
                    goal: "Prepare my coding workspace",
                    workspaceId: workspace.id,
                    applicationIds: lastRegisteredApp
                      ? [lastRegisteredApp.id]
                      : [],
                  },
                );
                setModelPlan(plan);
              })
            }
          >
            Run proposal generation test
          </button>
        </div>
        {modelProviders.length > 0 && (
          <dl>
            <dt>Providers</dt>
            <dd>
              {modelProviders.map((provider) => (
                <div key={provider.provider_id}>
                  {provider.display_name} ({provider.provider_id}/
                  {provider.model_id}) — {provider.availability} v
                  {provider.version}
                </div>
              ))}
            </dd>
          </dl>
        )}
        {selectedProvider && (
          <dl>
            <dt>Selected provider metadata</dt>
            <dd>
              {selectedProvider.display_name} — capabilities:{" "}
              {selectedProvider.capabilities.join(", ") || "none"}
              {selectedProvider.runtime_metadata
                ? ` — ${selectedProvider.runtime_metadata}`
                : ""}
            </dd>
          </dl>
        )}
        {modelResponse && (
          <dl>
            <dt>Provider response</dt>
            <dd>
              {modelResponse.provider_id}/{modelResponse.model_id} —{" "}
              {modelResponse.status} (
              {modelResponse.proposal_candidates.length} candidate(s))
              {modelResponse.text_output ? ` — ${modelResponse.text_output}` : ""}
            </dd>
          </dl>
        )}
        {modelPlan && (
          <dl>
            <dt>Provider → proposal plan (no execution)</dt>
            <dd>
              {modelPlan.goal.statement} — {modelPlan.proposals.length}{" "}
              proposal(s)
              {modelPlan.model_invocation && (
                <div>
                  via {modelPlan.model_invocation.provider_id}/
                  {modelPlan.model_invocation.model_id} (
                  {modelPlan.model_invocation.status})
                </div>
              )}
              {modelPlan.proposals.map((proposal) => (
                <div key={proposal.id}>
                  {proposal.command_name}:{" "}
                  {proposal.explanation ?? "(no explanation)"}
                </div>
              ))}
            </dd>
          </dl>
        )}
      </section>

      <section>
        <h2>Governed personalization</h2>
        <p className="muted">
          Explicit preferences improve ranking and explanations only. Disable
          anytime for neutral planning — never grants permissions.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace || !lastRegisteredApp}
            onClick={() =>
              void run("Preference created", async () => {
                if (!workspace || !lastRegisteredApp) return;
                await invokeIpc<UserPreference>("create_user_preference", {
                  category: "application",
                  key: "default_editor",
                  value: `${lastRegisteredApp.name} is my default editor`,
                  source: "user_defined",
                  workspaceId: workspace.id,
                  label: lastRegisteredApp.name,
                  attributes: JSON.stringify({
                    application_id: lastRegisteredApp.id,
                  }),
                });
                const profile = await invokeIpc<UserPreferenceProfile>(
                  "get_preference_profile",
                  { workspaceId: workspace.id, limit: 20 },
                );
                setPreferenceProfile(profile);
              })
            }
          >
            Create preference
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Preferences listed", async () => {
                const profile = await invokeIpc<UserPreferenceProfile>(
                  "get_preference_profile",
                  {
                    workspaceId: workspace?.id ?? null,
                    limit: 20,
                  },
                );
                setPreferenceProfile(profile);
              })
            }
          >
            List preferences
          </button>
          <button
            type="button"
            disabled={busy || !preferenceProfile?.preferences[0]}
            onClick={() =>
              void run("Preference edited", async () => {
                const first = preferenceProfile?.preferences[0];
                if (!first) return;
                await invokeIpc<UserPreference>("update_user_preference", {
                  id: first.id,
                  value: `${first.value} (edited)`,
                  label: first.label,
                });
                const profile = await invokeIpc<UserPreferenceProfile>(
                  "get_preference_profile",
                  {
                    workspaceId: workspace?.id ?? null,
                    limit: 20,
                  },
                );
                setPreferenceProfile(profile);
              })
            }
          >
            Edit preference
          </button>
          <button
            type="button"
            disabled={busy || !preferenceProfile?.preferences[0]}
            onClick={() =>
              void run("Preference deleted", async () => {
                const first = preferenceProfile?.preferences[0];
                if (!first) return;
                await invokeIpc<UserPreference>("delete_user_preference", {
                  id: first.id,
                });
                const profile = await invokeIpc<UserPreferenceProfile>(
                  "get_preference_profile",
                  {
                    workspaceId: workspace?.id ?? null,
                    limit: 20,
                  },
                );
                setPreferenceProfile(profile);
              })
            }
          >
            Delete preference
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Personalization toggled", async () => {
                const enabled = !(
                  preferenceProfile?.personalization_enabled ?? true
                );
                await invokeIpc<boolean>("set_personalization_enabled", {
                  enabled,
                });
                const profile = await invokeIpc<UserPreferenceProfile>(
                  "get_preference_profile",
                  {
                    workspaceId: workspace?.id ?? null,
                    limit: 20,
                  },
                );
                setPreferenceProfile(profile);
              })
            }
          >
            Toggle personalization
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Personalized plan generated", async () => {
                if (!workspace) return;
                const plan = await invokeIpc<AiPlan>(
                  "diagnose_ai_plan_with_personalization",
                  {
                    goal: "Prepare my coding workspace",
                    workspaceId: workspace.id,
                    applicationIds: lastRegisteredApp
                      ? [lastRegisteredApp.id]
                      : [],
                    personalizationEnabled: true,
                  },
                );
                setPersonalizedPlan(plan);
              })
            }
          >
            Generate plan with personalization
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Personalized vs neutral compared", async () => {
                if (!workspace) return;
                const comparison = await invokeIpc<PersonalizedPlanComparison>(
                  "compare_personalized_vs_neutral_plan",
                  {
                    goal: "Prepare my coding workspace",
                    workspaceId: workspace.id,
                    applicationIds: lastRegisteredApp
                      ? [lastRegisteredApp.id]
                      : [],
                  },
                );
                setPlanComparison(comparison);
              })
            }
          >
            Compare personalized vs neutral
          </button>
        </div>
        {preferenceProfile && (
          <dl>
            <dt>Preference profile</dt>
            <dd>
              enabled={String(preferenceProfile.personalization_enabled)} —{" "}
              {preferenceProfile.preferences.length} preference(s)
              {preferenceProfile.preferences.map((preference) => (
                <div key={preference.id}>
                  [{preference.category}] {preference.key}: {preference.value}
                  {preference.label ? ` (${preference.label})` : ""}
                </div>
              ))}
            </dd>
          </dl>
        )}
        {personalizedPlan && (
          <dl>
            <dt>Personalized plan</dt>
            <dd>
              {personalizedPlan.proposals.map((proposal) => (
                <div key={proposal.id}>
                  {proposal.command_name}:{" "}
                  {proposal.explanation ?? "(no explanation)"}
                </div>
              ))}
            </dd>
          </dl>
        )}
        {planComparison && (
          <dl>
            <dt>Personalized vs neutral</dt>
            <dd>
              <div>
                Personalized first:{" "}
                {planComparison.personalized.proposals[0]?.explanation ?? "none"}
              </div>
              <div>
                Neutral first:{" "}
                {planComparison.neutral.proposals[0]?.explanation ?? "none"}
              </div>
            </dd>
          </dl>
        )}
      </section>

      <section>
        <h2>Governed memory</h2>
        <p className="muted">
          Memory improves planning context only. It never grants permissions or
          executes actions — Permission Gateway remains the authority boundary.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace || !lastRegisteredApp}
            onClick={() =>
              void run("Test memory created", async () => {
                if (!workspace || !lastRegisteredApp) return;
                const entry = await invokeIpc<MemoryEntry>(
                  "create_memory_entry",
                  {
                    memoryType: "workspace",
                    key: "preferred_application",
                    summary: `Prefer ${lastRegisteredApp.name} for workspace prep`,
                    source: "operator_console",
                    workspaceId: workspace.id,
                    attributes: JSON.stringify({
                      application_id: lastRegisteredApp.id,
                    }),
                  },
                );
                setMemoryEntries((prev) => [entry, ...prev]);
              })
            }
          >
            Create test memory
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Memory context loaded", async () => {
                const context = await invokeIpc<AiMemoryAwareness>(
                  "get_memory_context",
                  {
                    workspaceId: workspace?.id ?? null,
                    limit: 20,
                  },
                );
                setMemoryContext(context);
                setMemoryEntries(context.entries);
              })
            }
          >
            View memory context
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Plan generated with memory", async () => {
                if (!workspace) return;
                const plan = await invokeIpc<AiPlan>(
                  "diagnose_ai_plan_preview",
                  {
                    goal: "Prepare my coding workspace",
                    workspaceId: workspace.id,
                    applicationIds: lastRegisteredApp
                      ? [lastRegisteredApp.id]
                      : [],
                  },
                );
                setMemoryPlan(plan);
              })
            }
          >
            Generate plan with memory
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Test memory cleared", async () => {
                await invokeIpc<number>("clear_memory_entries", {
                  memoryType: null,
                  workspaceId: workspace?.id ?? null,
                });
                setMemoryEntries([]);
                setMemoryContext(null);
                setMemoryPlan(null);
              })
            }
          >
            Clear test memory
          </button>
        </div>
        {memoryContext && (
          <dl>
            <dt>Memory context</dt>
            <dd>
              {memoryContext.entries.length} active entr
              {memoryContext.entries.length === 1 ? "y" : "ies"} (assembled{" "}
              {memoryContext.assembled_at})
              {memoryContext.entries.map((entry) => (
                <div key={entry.id}>
                  [{entry.memory_type}] {entry.key}: {entry.summary} (
                  {entry.source})
                </div>
              ))}
            </dd>
          </dl>
        )}
        {!memoryContext && memoryEntries.length > 0 && (
          <dl>
            <dt>Recent memory</dt>
            <dd>
              {memoryEntries.map((entry) => (
                <div key={entry.id}>
                  [{entry.memory_type}] {entry.summary}
                </div>
              ))}
            </dd>
          </dl>
        )}
        {memoryPlan && (
          <dl>
            <dt>Memory-aware plan (no execution)</dt>
            <dd>
              {memoryPlan.goal.statement} — {memoryPlan.proposals.length}{" "}
              proposal(s)
              {memoryPlan.proposals.map((proposal) => (
                <div key={proposal.id}>
                  {proposal.command_name}:{" "}
                  {proposal.explanation ?? "(no explanation)"}
                </div>
              ))}
            </dd>
          </dl>
        )}
      </section>

      <section>
        <h2>Workspace intelligence (diagnostics)</h2>
        <p className="muted">
          Same WorkspaceIntelligenceService as the product Work tab — aggregate
          only, authority_effect none.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Workspace intelligence generated", async () => {
                if (!workspace) return;
                const project = await invokeIpc<Project>("create_project", {
                  workspaceId: workspace.id,
                  name: "Diagnostic Project",
                  description: "Operator console seed",
                  metadata: null,
                });
                const task = await invokeIpc<Task>("create_task", {
                  projectId: project.id,
                  workspaceId: workspace.id,
                  title: "Inspect workspace context",
                  priority: "medium",
                });
                await invokeIpc("set_active_work", {
                  workspaceId: workspace.id,
                  projectId: project.id,
                  taskId: task.id,
                });
                setActiveProject(project);
                setActiveTask(task);
                const state = await invokeIpc<WorkspaceIntelligenceState>(
                  "generate_workspace_intelligence",
                  { workspaceId: workspace.id },
                );
                setWorkspaceIntelligence(state);
              })
            }
          >
            Seed + generate intelligence
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Workspace intelligence refreshed", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceIntelligenceState>(
                  "generate_workspace_intelligence",
                  { workspaceId: workspace.id },
                );
                setWorkspaceIntelligence(state);
              })
            }
          >
            Refresh intelligence (shared path)
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Workflow context inspected", async () => {
                if (!workspace) return;
                const context = await invokeIpc("get_workflow_context", {
                  workspaceId: workspace.id,
                });
                onMessage(
                  `Context: project=${(context as { active_project_id?: string }).active_project_id ?? "none"} task=${(context as { active_task_id?: string }).active_task_id ?? "none"}`,
                );
              })
            }
          >
            Inspect Workspace Context
          </button>
          <button
            type="button"
            disabled={busy || !activeProject}
            onClick={() =>
              void run("Active project inspected", async () => {
                if (!activeProject) return;
                const project = await invokeIpc<Project>("get_project", {
                  projectId: activeProject.id,
                });
                setActiveProject(project);
              })
            }
          >
            Inspect Active Project
          </button>
          <button
            type="button"
            disabled={busy || !activeTask}
            onClick={() =>
              void run("Active task inspected", async () => {
                if (!activeTask) return;
                const task = await invokeIpc<Task>("get_task", {
                  taskId: activeTask.id,
                });
                setActiveTask(task);
              })
            }
          >
            Inspect Active Task
          </button>
        </div>
        {workspaceIntelligence && (
          <ul className="muted">
            <li>{workspaceIntelligence.summary}</li>
            <li>
              Project: {workspaceIntelligence.current_project?.name ?? "none"} /
              Task: {workspaceIntelligence.current_task?.title ?? "none"}
            </li>
            <li>
              Pending approvals: {workspaceIntelligence.pending_approvals.length}{" "}
              · Blocked: {workspaceIntelligence.blocked_actions.length} · Recs:{" "}
              {workspaceIntelligence.recommended_actions.length}
            </li>
            {workspaceIntelligence.recommended_actions.slice(0, 3).map((rec) => (
              <li key={rec.id}>
                {rec.title} — {rec.explanation}
                <DisplayReasonList
                  reasons={rec.reasons}
                  showUnresolvedKey
                />
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Decision Engine (diagnostics)</h2>
        <p className="muted">
          Generate decisions, inspect scoring, compare recommendations. Never
          executes — accept hands off to submit_assistant_goal only.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Decision Engine generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<DecisionEngineState>(
                  "generate_decision_engine",
                  { workspaceId: workspace.id },
                );
                setDecisionEngine(state);
              })
            }
          >
            Generate decisions
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !decisionEngine}
            onClick={() =>
              void run("Decision graph inspected", async () => {
                if (!decisionEngine) return;
                onMessage(
                  `Candidates=${decisionEngine.candidates.length} top=${decisionEngine.top_candidates
                    .map((c) => `${c.title}:${c.score.total}`)
                    .join(" | ")}`,
                );
              })
            }
          >
            Inspect decision graph
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !decisionEngine}
            onClick={() =>
              void run("Scoring inspected", async () => {
                if (!decisionEngine) return;
                const lines = decisionEngine.candidates.slice(0, 5).map((c) => {
                  const s = c.score;
                  return `${c.title}: total=${s.total} attn=${s.attention_contribution} mem=${s.memory_contribution} pref=${s.personalization_contribution} goal=${s.goal_contribution}`;
                });
                onMessage(lines.join(" · "));
              })
            }
          >
            Inspect scoring
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Decision generation replayed", async () => {
                if (!workspace) return;
                const first = await invokeIpc<DecisionEngineState>(
                  "generate_decision_engine",
                  { workspaceId: workspace.id },
                );
                const second = await invokeIpc<DecisionEngineState>(
                  "generate_decision_engine",
                  { workspaceId: workspace.id },
                );
                setDecisionEngine(second);
                onMessage(
                  `Replay: first=${first.top_candidates[0]?.score.total ?? 0} second=${second.top_candidates[0]?.score.total ?? 0} (deterministic ranks expected)`,
                );
              })
            }
          >
            Replay decision generation
          </button>
        </div>
        {decisionEngine && (
          <ul className="muted">
            <li>{decisionEngine.summary}</li>
            <li>
              Context: attention={decisionEngine.context.attention_item_count}{" "}
              memory={decisionEngine.context.memory_highlight_count} prefs=
              {decisionEngine.context.preference_highlight_count} approvals=
              {decisionEngine.context.pending_approval_count}
            </li>
            {decisionEngine.top_candidates.map((c) => (
              <li key={c.id}>
                [{c.explanation.confidence}] {c.title} — score {c.score.total} —{" "}
                {c.explanation.headline}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Task Graph (diagnostics)</h2>
        <p className="muted">
          Create / inspect / validate the Workspace Task Graph. Never executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Task Graph created/refreshed", async () => {
                if (!workspace) return;
                await invokeIpc("create_workspace_task", {
                  workspaceId: workspace.id,
                  title: "Diagnostic graph task",
                  projectId: activeProject?.id ?? null,
                  priority: "high",
                });
                const graph = await invokeIpc<TaskGraph>("generate_task_graph", {
                  workspaceId: workspace.id,
                });
                setTaskGraph(graph);
              })
            }
          >
            Create graph
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Task Graph inspected", async () => {
                if (!workspace) return;
                const graph = await invokeIpc<TaskGraph>("generate_task_graph", {
                  workspaceId: workspace.id,
                });
                setTaskGraph(graph);
                onMessage(
                  `${graph.nodes.length} nodes · ${graph.relationships.length} relationships · ${graph.progress_percent}%`,
                );
              })
            }
          >
            Inspect graph
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Task Graph integrity validated", async () => {
                if (!workspace) return;
                const graph = await invokeIpc<TaskGraph>("validate_task_graph", {
                  workspaceId: workspace.id,
                });
                setTaskGraph(graph);
                onMessage(
                  graph.integrity_ok
                    ? "Integrity ok"
                    : `Issues: ${graph.integrity_notes.join("; ")}`,
                );
              })
            }
          >
            Validate graph integrity
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !taskGraph}
            onClick={() =>
              void run("Dependencies inspected", async () => {
                if (!taskGraph) return;
                const deps = taskGraph.relationships
                  .filter((r) => r.kind === "depends_on" || r.kind === "blocks")
                  .map((r) => `${r.kind}:${r.from_task_id.slice(0, 8)}→${r.to_task_id.slice(0, 8)}`);
                onMessage(deps.length ? deps.join(" · ") : "No dependency edges");
              })
            }
          >
            Inspect dependencies
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Planning inputs inspected", async () => {
                if (!workspace) return;
                const inputs = await invokeIpc<{ id: string; title: string; status: string }[]>(
                  "get_task_graph_planning_inputs",
                  { workspaceId: workspace.id },
                );
                onMessage(
                  inputs.length
                    ? inputs.map((t) => `${t.title}(${t.status})`).join(" · ")
                    : "No open graph nodes for planner",
                );
              })
            }
          >
            Inspect task lifecycle / planner inputs
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Graph history replayed", async () => {
                if (!workspace) return;
                const first = await invokeIpc<TaskGraph>("generate_task_graph", {
                  workspaceId: workspace.id,
                });
                const second = await invokeIpc<TaskGraph>("generate_task_graph", {
                  workspaceId: workspace.id,
                });
                setTaskGraph(second);
                onMessage(
                  `Replay nodes ${first.nodes.length}→${second.nodes.length} (durable regenerate)`,
                );
              })
            }
          >
            Replay graph history
          </button>
        </div>
        {taskGraph && (
          <ul className="muted">
            <li>{taskGraph.summary}</li>
            {taskGraph.nodes.slice(0, 5).map((n) => (
              <li key={n.task.id}>
                [{n.task.status}] {n.task.title}
                {n.waiting_reason ? ` — ${n.waiting_reason}` : ""}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Environment Model (diagnostics)</h2>
        <p className="muted">
          Aggregate live desktop windows with Workspace apps and active work.
          Never moves windows or grants authority.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Environment generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceEnvironmentState>(
                  "generate_workspace_environment",
                  { workspaceId: workspace.id },
                );
                setEnvironment(state);
              })
            }
          >
            Inspect environment
          </button>
          <button
            type="button"
            disabled={busy || !environment}
            onClick={() =>
              void run("Environment gaps inspected", async () => {
                if (!environment) return;
                onMessage(
                  environment.gaps.length
                    ? environment.gaps.map((g) => g.title).join(" · ")
                    : "No gaps",
                );
              })
            }
          >
            Inspect gaps
          </button>
          <button
            type="button"
            disabled={busy || !environment}
            onClick={() =>
              void run("Window groups inspected", async () => {
                if (!environment) return;
                onMessage(
                  environment.window_groups.length
                    ? environment.window_groups
                        .map((g) => `${g.label}(${g.window_ids.length})`)
                        .join(" · ")
                    : "No groups",
                );
              })
            }
          >
            Inspect window groups
          </button>
        </div>
        {environment && (
          <ul className="muted">
            <li>{environment.summary}</li>
            <li>
              Running {environment.running_application_count} · missing{" "}
              {environment.missing_application_count} · disconnected=
              {String(environment.disconnected_work)}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Composition Engine (diagnostics)</h2>
        <p className="muted">
          Logical working environment — how apps, projects, tasks, and desktop
          state belong together. Never launches, groups, or moves windows.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Composition generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceCompositionState>(
                  "generate_workspace_composition",
                  { workspaceId: workspace.id },
                );
                setComposition(state);
              })
            }
          >
            Inspect composition
          </button>
          <button
            type="button"
            disabled={busy || !composition}
            onClick={() =>
              void run("Composition members inspected", async () => {
                if (!composition) return;
                onMessage(
                  composition.members.length
                    ? composition.members
                        .slice(0, 8)
                        .map(
                          (m) =>
                            `${m.present ? "✓" : "○"} ${m.label} (${m.kind})`,
                        )
                        .join(" · ")
                    : "No members",
                );
              })
            }
          >
            Inspect members
          </button>
          <button
            type="button"
            disabled={busy || !composition}
            onClick={() =>
              void run("Composition gaps inspected", async () => {
                if (!composition) return;
                onMessage(
                  composition.gaps.length
                    ? composition.gaps.map((g) => g.title).join(" · ")
                    : "No gaps",
                );
              })
            }
          >
            Inspect composition gaps
          </button>
        </div>
        {composition && (
          <ul className="muted">
            <li>{composition.summary}</li>
            <li>
              Present {composition.present_application_count} · missing{" "}
              {composition.missing_application_count} · tasks{" "}
              {composition.task_node_count} · decisions{" "}
              {composition.outstanding_decision_count}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Purpose Model (diagnostics)</h2>
        <p className="muted">
          Why work exists — projected from WorkGoals, projects, Task Graph,
          Composition, Continuity. Never executes or owns goals.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Purpose generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspacePurposeState>(
                  "generate_workspace_purpose",
                  { workspaceId: workspace.id },
                );
                setPurpose(state);
              })
            }
          >
            Inspect purpose
          </button>
          <button
            type="button"
            disabled={busy || !purpose}
            onClick={() =>
              void run("Purpose relationships inspected", async () => {
                if (!purpose) return;
                onMessage(
                  purpose.relationships.length
                    ? purpose.relationships
                        .slice(0, 6)
                        .map((r) => `${r.kind}: ${r.explanation}`)
                        .join(" · ")
                    : "No relationships",
                );
              })
            }
          >
            Inspect relationships
          </button>
          <button
            type="button"
            disabled={busy || !purpose}
            onClick={() =>
              void run("Purpose obstacles inspected", async () => {
                if (!purpose) return;
                onMessage(
                  purpose.obstacles.length
                    ? purpose.obstacles.map((o) => o.title).join(" · ")
                    : "No obstacles",
                );
              })
            }
          >
            Inspect obstacles
          </button>
        </div>
        {purpose && (
          <ul className="muted">
            <li>{purpose.summary}</li>
            <li>
              Progress {purpose.progress_percent}% · open{" "}
              {purpose.open_task_count} · obstacles {purpose.obstacles.length}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Evolution Model (diagnostics)</h2>
        <p className="muted">
          How work changed — projected from Activity Graph and related models.
          Never stores a second history or predicts.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Evolution generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceEvolutionState>(
                  "generate_workspace_evolution",
                  { workspaceId: workspace.id },
                );
                setEvolution(state);
              })
            }
          >
            Inspect evolution
          </button>
          <button
            type="button"
            disabled={busy || !evolution}
            onClick={() =>
              void run("Evolution insights inspected", async () => {
                if (!evolution) return;
                onMessage(
                  evolution.insights.length
                    ? evolution.insights.map((i) => i.title).join(" · ")
                    : "No insights",
                );
              })
            }
          >
            Inspect insights
          </button>
        </div>
        {evolution && (
          <ul className="muted">
            <li>{evolution.summary}</li>
            <li>
              Events {evolution.event_count} · insights {evolution.insight_count}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Recommendation Engine (diagnostics)</h2>
        <p className="muted">
          What might help next — suggestions only. Aggregates Attention,
          Continuity, Evolution, Purpose, Task Graph, Composition, Decision
          Queue, Environment. Never executes or grants authority.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Recommendation Engine generated", async () => {
                if (!workspace) return;
                const state =
                  await invokeIpc<WorkspaceRecommendationEngineState>(
                    "generate_workspace_recommendation_engine",
                    { workspaceId: workspace.id },
                  );
                setRecommendationEngine(state);
              })
            }
          >
            Inspect recommendations
          </button>
          <button
            type="button"
            disabled={busy || !recommendationEngine}
            onClick={() =>
              void run("Recommendation evidence inspected", async () => {
                if (!recommendationEngine) return;
                onMessage(
                  recommendationEngine.candidates.length
                    ? recommendationEngine.candidates
                        .slice(0, 5)
                        .map(
                          (c) =>
                            `${c.kind}: ${c.title} (${c.confidence}) — ${c.reason}`,
                        )
                        .join(" · ")
                    : "No candidates",
                );
              })
            }
          >
            Inspect candidates
          </button>
        </div>
        {recommendationEngine && (
          <ul className="muted">
            <li>{recommendationEngine.summary}</li>
            <li>
              Candidates {recommendationEngine.candidate_count} · authority{" "}
              {recommendationEngine.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Operating State (diagnostics)</h2>
        <p className="muted">
          What is happening right now — unified current-situation snapshot.
          Aggregates existing understanding systems; never executes or grants
          authority.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Operating State generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceOperatingState>(
                  "generate_workspace_operating_state",
                  { workspaceId: workspace.id },
                );
                setOperatingState(state);
              })
            }
          >
            Inspect operating state
          </button>
          <button
            type="button"
            disabled={busy || !operatingState}
            onClick={() =>
              void run("Operating narrative inspected", async () => {
                if (!operatingState) return;
                onMessage(operatingState.operating_summary.narrative);
              })
            }
          >
            Inspect narrative
          </button>
        </div>
        {operatingState && (
          <ul className="muted">
            <li>{operatingState.summary}</li>
            <li>{operatingState.operating_summary.purpose_line}</li>
            <li>
              Signals {operatingState.signal_count} · authority{" "}
              {operatingState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Pattern Model (diagnostics)</h2>
        <p className="muted">
          Recurring structures — observations only. Never predicts, profiles, or
          executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Pattern Model generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspacePatternState>(
                  "generate_workspace_pattern",
                  { workspaceId: workspace.id },
                );
                setPatternState(state);
              })
            }
          >
            Inspect patterns
          </button>
          <button
            type="button"
            disabled={busy || !patternState}
            onClick={() =>
              void run("Pattern observations inspected", async () => {
                if (!patternState) return;
                onMessage(
                  patternState.patterns.length
                    ? patternState.patterns
                        .slice(0, 5)
                        .map((p) => `${p.kind}: ${p.observation}`)
                        .join(" · ")
                    : "No patterns",
                );
              })
            }
          >
            Inspect observations
          </button>
        </div>
        {patternState && (
          <ul className="muted">
            <li>{patternState.summary}</li>
            <li>
              Patterns {patternState.pattern_count} · authority{" "}
              {patternState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Adaptation Proposals (diagnostics)</h2>
        <p className="muted">
          Possible improvements — proposals only. Review / accept hands off to
          Intent; never applies changes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Adaptation proposals generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceAdaptationState>(
                  "generate_workspace_adaptation",
                  { workspaceId: workspace.id },
                );
                setAdaptationState(state);
              })
            }
          >
            Inspect adaptation proposals
          </button>
          <button
            type="button"
            disabled={busy || !adaptationState}
            onClick={() =>
              void run("Adaptation narratives inspected", async () => {
                if (!adaptationState) return;
                onMessage(
                  adaptationState.proposals.length
                    ? adaptationState.proposals
                        .slice(0, 5)
                        .map((p) => `${p.kind}: ${p.title} (${p.status})`)
                        .join(" · ")
                    : "No proposals",
                );
              })
            }
          >
            Inspect proposals
          </button>
        </div>
        {adaptationState && (
          <ul className="muted">
            <li>{adaptationState.summary}</li>
            <li>
              Open {adaptationState.open_count} · authority{" "}
              {adaptationState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Readiness (diagnostics)</h2>
        <p className="muted">
          Preparedness for current work — informational only. Never prepares,
          launches, or executes. Distinct from runtime health.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Readiness assessed", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceReadinessState>(
                  "generate_workspace_readiness",
                  { workspaceId: workspace.id },
                );
                setReadinessState(state);
              })
            }
          >
            Inspect readiness
          </button>
          <button
            type="button"
            disabled={busy || !readinessState}
            onClick={() =>
              void run("Readiness assessments inspected", async () => {
                if (!readinessState) return;
                onMessage(
                  readinessState.assessments.length
                    ? readinessState.assessments
                        .map((a) => `${a.kind}: ${a.status}`)
                        .join(" · ")
                    : readinessState.summary,
                );
              })
            }
          >
            Inspect assessments
          </button>
        </div>
        {readinessState && (
          <ul className="muted">
            <li>{readinessState.readiness_summary.status_line}</li>
            <li>
              {readinessState.overall_status} · {readinessState.gap_count} gap(s)
              · authority {readinessState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Runtime (diagnostics)</h2>
        <p className="muted">
          Live operating context, observational health, and operator overview —
          never executes, scores, or unblocks publication. Distinct from
          readiness preparedness and kernel lifecycle health.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Runtime overview projected", async () => {
                if (!workspace) return;
                const view = await invokeIpc<WorkspaceRuntimeOperatorView>(
                  "generate_workspace_runtime_overview",
                  { workspaceId: workspace.id },
                );
                setRuntimeOverview(view);
              })
            }
          >
            Inspect runtime overview
          </button>
          <button
            type="button"
            disabled={busy || !runtimeOverview}
            onClick={() =>
              void run("Runtime governance labels inspected", async () => {
                if (!runtimeOverview) return;
                onMessage(
                  `${runtimeOverview.overview.governance_summary} · review ${runtimeOverview.operator_context.review_status_label} · publication blocked=${runtimeOverview.publication_blocked}`,
                );
              })
            }
          >
            Inspect governance labels
          </button>
        </div>
        {runtimeOverview && (
          <ul className="muted">
            <li>
              Health {runtimeOverview.health.overall} · coherence{" "}
              {runtimeOverview.coherence_ok ? "ok" : "fail"} · review{" "}
              {runtimeOverview.architecture_review_passed ? "passed" : "fail"}
            </li>
            <li>
              {runtimeOverview.overview.dependency_summary} ·{" "}
              {runtimeOverview.overview.capability_summary}
            </li>
            <li>
              Snapshot {runtimeOverview.diagnostic_snapshot_id} · authority{" "}
              {runtimeOverview.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Navigation (diagnostics)</h2>
        <p className="muted">
          Interaction paths — Generate / Inspect / Compare / Validate. Never
          plans, routes autonomously, or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Navigation generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceNavigationState>(
                  "generate_workspace_navigation",
                  { workspaceId: workspace.id },
                );
                if (navigationState) setPriorNavigation(navigationState);
                setNavigationState(state);
              })
            }
          >
            Generate navigation
          </button>
          <button
            type="button"
            disabled={busy || !navigationState}
            onClick={() =>
              void run("Navigation inspected", async () => {
                if (!navigationState) return;
                onMessage(
                  `${navigationState.navigation_summary.narrative} Evidence: ${navigationState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Inspect navigation
          </button>
          <button
            type="button"
            disabled={busy || !navigationState || !priorNavigation}
            onClick={() =>
              void run("Navigations compared", async () => {
                if (!navigationState || !priorNavigation) return;
                const comparison =
                  await invokeIpc<WorkspaceNavigationComparison>(
                    "compare_workspace_navigation",
                    { left: priorNavigation, right: navigationState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare navigation
          </button>
          <button
            type="button"
            disabled={busy || !navigationState}
            onClick={() =>
              void run("Navigation validated", async () => {
                if (!navigationState) return;
                const report = await invokeIpc<WorkspaceNavigationValidation>(
                  "validate_workspace_navigation",
                  { state: navigationState },
                );
                onMessage(report.messages.join(" · "));
              })
            }
          >
            Validate navigation
          </button>
        </div>
        {navigationState && (
          <ul className="muted">
            <li>{navigationState.navigation_summary.headline}</li>
            <li>{navigationState.navigation_summary.breadcrumb_line}</li>
            <li>
              Nodes {navigationState.node_count} · Blocked{" "}
              {navigationState.blocked_count} · Suggested{" "}
              {navigationState.suggested_count} · authority{" "}
              {navigationState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Milestones (diagnostics)</h2>
        <p className="muted">
          Progress coordination — Generate / Inspect / Compare / Validate. Never
          plans, schedules, completes, or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Milestones generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceMilestoneState>(
                  "generate_workspace_milestones",
                  { workspaceId: workspace.id },
                );
                if (milestoneState) setPriorMilestones(milestoneState);
                setMilestoneState(state);
              })
            }
          >
            Generate milestones
          </button>
          <button
            type="button"
            disabled={busy || !milestoneState}
            onClick={() =>
              void run("Milestones inspected", async () => {
                if (!milestoneState) return;
                onMessage(
                  `${milestoneState.milestone_summary.narrative} Evidence: ${milestoneState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Inspect milestones
          </button>
          <button
            type="button"
            disabled={busy || !milestoneState || !priorMilestones}
            onClick={() =>
              void run("Milestones compared", async () => {
                if (!milestoneState || !priorMilestones) return;
                const comparison =
                  await invokeIpc<WorkspaceMilestoneComparison>(
                    "compare_workspace_milestones",
                    { left: priorMilestones, right: milestoneState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare milestones
          </button>
          <button
            type="button"
            disabled={busy || !milestoneState}
            onClick={() =>
              void run("Milestones validated", async () => {
                if (!milestoneState) return;
                const report = await invokeIpc<WorkspaceMilestoneValidation>(
                  "validate_workspace_milestones",
                  { state: milestoneState },
                );
                onMessage(report.messages.join(" · "));
              })
            }
          >
            Validate milestones
          </button>
        </div>
        {milestoneState && (
          <ul className="muted">
            <li>{milestoneState.milestone_summary.headline}</li>
            <li>{milestoneState.milestone_summary.current_line}</li>
            <li>
              Current {milestoneState.current_count} · Upcoming{" "}
              {milestoneState.upcoming_count} · Blocked{" "}
              {milestoneState.blocked_count} · Completed{" "}
              {milestoneState.completed_count} · authority{" "}
              {milestoneState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Working Style (diagnostics)</h2>
        <p className="muted">
          Operating patterns — Generate / Inspect / Compare / Validate. Never
          profiles, predicts, or executes. Observed ≠ preferred.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Working style generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceWorkingStyleState>(
                  "generate_workspace_working_style",
                  { workspaceId: workspace.id },
                );
                if (workingStyleState) setPriorWorkingStyle(workingStyleState);
                setWorkingStyleState(state);
              })
            }
          >
            Generate working style
          </button>
          <button
            type="button"
            disabled={busy || !workingStyleState}
            onClick={() =>
              void run("Working style inspected", async () => {
                if (!workingStyleState) return;
                onMessage(
                  `${workingStyleState.style_summary.narrative} Evidence: ${workingStyleState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Inspect working style
          </button>
          <button
            type="button"
            disabled={busy || !workingStyleState || !priorWorkingStyle}
            onClick={() =>
              void run("Working styles compared", async () => {
                if (!workingStyleState || !priorWorkingStyle) return;
                const comparison =
                  await invokeIpc<WorkspaceWorkingStyleComparison>(
                    "compare_workspace_working_styles",
                    { left: priorWorkingStyle, right: workingStyleState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare working styles
          </button>
          <button
            type="button"
            disabled={busy || !workingStyleState}
            onClick={() =>
              void run("Working style validated", async () => {
                if (!workingStyleState) return;
                const report =
                  await invokeIpc<WorkspaceWorkingStyleValidation>(
                    "validate_workspace_working_style",
                    { state: workingStyleState },
                  );
                onMessage(report.messages.join(" · "));
              })
            }
          >
            Validate working style
          </button>
        </div>
        {workingStyleState && (
          <ul className="muted">
            <li>{workingStyleState.style_summary.headline}</li>
            <li>{workingStyleState.style_summary.observed_vs_preferred_line}</li>
            <li>
              Observed {workingStyleState.observed_count} · Explicit prefs{" "}
              {workingStyleState.preference_count} · authority{" "}
              {workingStyleState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Transitions (diagnostics)</h2>
        <p className="muted">
          Movement explanation — Generate / Inspect / Compare / Validate. Never
          restores, launches, or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Transitions generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceTransitionState>(
                  "generate_workspace_transitions",
                  { workspaceId: workspace.id },
                );
                if (transitionState) setPriorTransitions(transitionState);
                setTransitionState(state);
              })
            }
          >
            Generate transitions
          </button>
          <button
            type="button"
            disabled={busy || !transitionState}
            onClick={() =>
              void run("Transitions inspected", async () => {
                if (!transitionState) return;
                onMessage(
                  `${transitionState.transition_summary.narrative} Evidence: ${transitionState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Inspect transitions
          </button>
          <button
            type="button"
            disabled={busy || !transitionState || !priorTransitions}
            onClick={() =>
              void run("Transitions compared", async () => {
                if (!transitionState || !priorTransitions) return;
                const comparison =
                  await invokeIpc<WorkspaceTransitionComparison>(
                    "compare_workspace_transitions",
                    { left: priorTransitions, right: transitionState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare transitions
          </button>
          <button
            type="button"
            disabled={busy || !transitionState}
            onClick={() =>
              void run("Transitions validated", async () => {
                if (!transitionState) return;
                const report = await invokeIpc<WorkspaceTransitionValidation>(
                  "validate_workspace_transitions",
                  { state: transitionState },
                );
                onMessage(report.messages.join(" · "));
              })
            }
          >
            Validate transitions
          </button>
        </div>
        {transitionState && (
          <ul className="muted">
            <li>{transitionState.transition_summary.headline}</li>
            <li>{transitionState.transition_summary.left_off_line}</li>
            <li>
              Returning {transitionState.returning_count} · Switching{" "}
              {transitionState.switching_count} · authority{" "}
              {transitionState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Interactions (diagnostics)</h2>
        <p className="muted">
          Unified opportunities — Generate / Inspect / Validate / Compare. Select
          creates Intent handoff only. Never executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Interactions generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceInteractionState>(
                  "generate_workspace_interactions",
                  { workspaceId: workspace.id },
                );
                if (interactionState) setPriorInteractions(interactionState);
                setInteractionState(state);
              })
            }
          >
            Generate interactions
          </button>
          <button
            type="button"
            disabled={busy || !interactionState}
            onClick={() =>
              void run("Interactions inspected", async () => {
                if (!interactionState) return;
                onMessage(
                  `${interactionState.interaction_summary.narrative} Evidence: ${interactionState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Inspect interactions
          </button>
          <button
            type="button"
            disabled={busy || !interactionState || !priorInteractions}
            onClick={() =>
              void run("Interactions compared", async () => {
                if (!interactionState || !priorInteractions) return;
                const comparison =
                  await invokeIpc<WorkspaceInteractionComparison>(
                    "compare_workspace_interactions",
                    { left: priorInteractions, right: interactionState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare interactions
          </button>
          <button
            type="button"
            disabled={busy || !interactionState}
            onClick={() =>
              void run("Interactions validated", async () => {
                if (!interactionState) return;
                const report =
                  await invokeIpc<WorkspaceInteractionValidation>(
                    "validate_workspace_interactions",
                    { state: interactionState },
                  );
                onMessage(report.messages.join(" · "));
              })
            }
          >
            Validate interactions
          </button>
          <button
            type="button"
            disabled={busy || !workspace || !interactionState?.items[0]}
            onClick={() =>
              void run("Interaction selected (handoff)", async () => {
                if (!workspace || !interactionState?.items[0]) return;
                const result = await invokeIpc<InteractionSelectResult>(
                  "select_workspace_interaction",
                  {
                    workspaceId: workspace.id,
                    interactionId: interactionState.items[0].id,
                  },
                );
                onMessage(
                  result.handoff
                    ? `Handoff → ${result.handoff.next_command}: ${result.handoff.intent_statement}`
                    : "No handoff",
                );
              })
            }
          >
            Select first (handoff)
          </button>
        </div>
        {interactionState && (
          <ul className="muted">
            <li>{interactionState.interaction_summary.headline}</li>
            <li>{interactionState.interaction_summary.continue_line}</li>
            <li>
              Items {interactionState.item_count} · Decisions{" "}
              {interactionState.decision_count} · Recommendations{" "}
              {interactionState.recommendation_count} · authority{" "}
              {interactionState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Profiles (diagnostics)</h2>
        <p className="muted">
          Durable user-owned setups — Create / Inspect / Compare / Validate.
          Never launches, restores, or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Profile created", async () => {
                if (!workspace) return;
                const members: WorkspaceProfileMemberInput[] = [
                  {
                    member_type: "application",
                    reference_id: "vscode",
                    relationship: "expected",
                    evidence: "Operator-created Development setup",
                    label: "VS Code",
                  },
                  {
                    member_type: "application",
                    reference_id: "terminal",
                    relationship: "expected",
                    evidence: "Operator-created Development setup",
                    label: "Terminal",
                  },
                ];
                if (activeProject) {
                  members.push({
                    member_type: "project",
                    reference_id: activeProject.id,
                    relationship: "preferred",
                    evidence: "Active project reference",
                    label: activeProject.name,
                  });
                }
                await invokeIpc<WorkspaceProfile>("create_workspace_profile", {
                  workspaceId: workspace.id,
                  name: "Development setup",
                  description: "Operator diagnostic profile",
                  members,
                });
                const state = await invokeIpc<WorkspaceProfileState>(
                  "generate_workspace_profile_state",
                  { workspaceId: workspace.id },
                );
                if (profileState) setPriorProfileState(profileState);
                setProfileState(state);
              })
            }
          >
            Create profile
          </button>
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Profiles generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceProfileState>(
                  "generate_workspace_profile_state",
                  { workspaceId: workspace.id },
                );
                if (profileState) setPriorProfileState(profileState);
                setProfileState(state);
              })
            }
          >
            Inspect profiles
          </button>
          <button
            type="button"
            disabled={busy || !profileState || !priorProfileState}
            onClick={() =>
              void run("Profiles compared", async () => {
                if (!profileState || !priorProfileState) return;
                const comparison =
                  await invokeIpc<WorkspaceProfileStateComparison>(
                    "compare_workspace_profile_states",
                    { left: priorProfileState, right: profileState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare profiles
          </button>
          <button
            type="button"
            disabled={busy || !profileState}
            onClick={() =>
              void run("Profiles validated", async () => {
                if (!profileState) return;
                const report = await invokeIpc<WorkspaceProfileValidation>(
                  "validate_workspace_profile_state",
                  { state: profileState },
                );
                onMessage(report.messages.join(" · "));
              })
            }
          >
            Validate profiles
          </button>
        </div>
        {profileState && (
          <ul className="muted">
            <li>{profileState.profile_summary.headline}</li>
            <li>{profileState.profile_summary.alignment_line}</li>
            <li>
              Profiles {profileState.profile_count} · Active{" "}
              {profileState.active_count} · authority{" "}
              {profileState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Work Context (diagnostics)</h2>
        <p className="muted">
          Semantic kind-of-work — Generate / Inspect / Compare / Validate. Never
          plans, launches, or executes.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Work context generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceWorkContextState>(
                  "generate_workspace_work_context",
                  { workspaceId: workspace.id },
                );
                if (workContextState) setPriorWorkContext(workContextState);
                setWorkContextState(state);
              })
            }
          >
            Generate work context
          </button>
          <button
            type="button"
            disabled={busy || !workContextState}
            onClick={() =>
              void run("Work context inspected", async () => {
                if (!workContextState) return;
                const primary = workContextState.contexts.find(
                  (c) => c.id === workContextState.primary_context_id,
                );
                onMessage(
                  `${workContextState.summary} Primary: ${primary?.name ?? "none"} (${primary?.context_type ?? "n/a"}). Evidence: ${workContextState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Inspect work context
          </button>
          <button
            type="button"
            disabled={busy || !workContextState || !priorWorkContext}
            onClick={() =>
              void run("Work contexts compared", async () => {
                if (!workContextState || !priorWorkContext) return;
                const comparison =
                  await invokeIpc<WorkspaceWorkContextComparison>(
                    "compare_workspace_work_contexts",
                    { left: priorWorkContext, right: workContextState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare work contexts
          </button>
          <button
            type="button"
            disabled={busy || !workContextState}
            onClick={() =>
              void run("Work context validated", async () => {
                if (!workContextState) return;
                const report = await invokeIpc<WorkspaceWorkContextValidation>(
                  "validate_workspace_work_context",
                  { state: workContextState },
                );
                onMessage(report.messages.join(" · "));
              })
            }
          >
            Validate work context
          </button>
        </div>
        {workContextState && (
          <ul className="muted">
            <li>{workContextState.summary}</li>
            <li>
              Contexts {workContextState.context_count} · Active{" "}
              {workContextState.active_count} · Blocked{" "}
              {workContextState.blocked_count} · Dormant{" "}
              {workContextState.dormant_count} · authority{" "}
              {workContextState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Experience (diagnostics)</h2>
        <p className="muted">
          Presentation over Session — Generate / Explain / Validate / Compare.
          Never executes, prepares, restores, or owns data.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Experience generated", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceExperienceState>(
                  "generate_workspace_experience",
                  { workspaceId: workspace.id },
                );
                if (experienceState) setPriorExperience(experienceState);
                setExperienceState(state);
              })
            }
          >
            Generate experience
          </button>
          <button
            type="button"
            disabled={busy || !experienceState}
            onClick={() =>
              void run("Experience explained", async () => {
                if (!experienceState) return;
                onMessage(
                  `${experienceState.experience_summary.narrative} Evidence: ${experienceState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Explain experience
          </button>
          <button
            type="button"
            disabled={busy || !experienceState || !priorExperience}
            onClick={() =>
              void run("Experiences compared", async () => {
                if (!experienceState || !priorExperience) return;
                const comparison =
                  await invokeIpc<WorkspaceExperienceComparison>(
                    "compare_workspace_experiences",
                    { left: priorExperience, right: experienceState },
                  );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare experiences
          </button>
          <button
            type="button"
            disabled={busy || !experienceState}
            onClick={() =>
              void run("Experience validated", async () => {
                if (!experienceState) return;
                onMessage(
                  experienceState.authority_effect === "none" &&
                    experienceState.explanation.includes("owns no")
                    ? `Valid: authority none · ${experienceState.section_count} sections · immediate ${experienceState.immediate_count} · from Session ${experienceState.session_generated_at}`
                    : "Experience validation concerns found.",
                );
              })
            }
          >
            Validate experience
          </button>
        </div>
        {experienceState && (
          <ul className="muted">
            <li>{experienceState.experience_summary.headline}</li>
            <li>{experienceState.experience_summary.focus_line}</li>
            <li>
              Immediate {experienceState.immediate_count} · Highlighted{" "}
              {experienceState.highlighted_count} · Collapsed{" "}
              {experienceState.collapsed_count} · Deferred{" "}
              {experienceState.deferred_count} · authority{" "}
              {experienceState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Workspace Session (diagnostics)</h2>
        <p className="muted">
          Runtime orchestration over Intelligence — inspect / compare / explain.
          Never executes, prepares, or restores. Canonical input to Experience.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Session inspected", async () => {
                if (!workspace) return;
                const state = await invokeIpc<WorkspaceSessionState>(
                  "generate_workspace_session",
                  { workspaceId: workspace.id },
                );
                if (sessionState) setPriorSession(sessionState);
                setSessionState(state);
              })
            }
          >
            Inspect session
          </button>
          <button
            type="button"
            disabled={busy || !sessionState}
            onClick={() =>
              void run("Session explained", async () => {
                if (!sessionState) return;
                onMessage(
                  `${sessionState.session_summary.narrative} Evidence: ${sessionState.evidence.slice(0, 3).join(" · ")}`,
                );
              })
            }
          >
            Explain session
          </button>
          <button
            type="button"
            disabled={busy || !sessionState || !priorSession}
            onClick={() =>
              void run("Sessions compared", async () => {
                if (!sessionState || !priorSession) return;
                const comparison = await invokeIpc<WorkspaceSessionComparison>(
                  "compare_workspace_sessions",
                  { left: priorSession, right: sessionState },
                );
                onMessage(comparison.differences.join(" · "));
              })
            }
          >
            Compare sessions
          </button>
          <button
            type="button"
            disabled={busy || !sessionState}
            onClick={() =>
              void run("Session validated", async () => {
                if (!sessionState) return;
                onMessage(
                  sessionState.authority_effect === "none" &&
                    sessionState.explanation.includes("owns no source data")
                    ? `Valid: authority none · ${sessionState.member_count} members · from Intelligence ${sessionState.intelligence_generated_at}`
                    : "Session validation concerns found.",
                );
              })
            }
          >
            Validate session
          </button>
        </div>
        {sessionState && (
          <ul className="muted">
            <li>{sessionState.session_summary.headline}</li>
            <li>{sessionState.session_summary.doing_line}</li>
            <li>
              Decisions {sessionState.decision_count} · Risks{" "}
              {sessionState.risk_count} · Readiness{" "}
              {sessionState.readiness.overall_status} · authority{" "}
              {sessionState.authority_effect}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Governed AI assistant (diagnostics)</h2>
        <p className="muted">
          Same CommandHandler pipeline as the product Assistant tab — regenerate,
          compare, explanation rendering, and recovery checks.
        </p>
        <div className="row">
          <input
            value={assistantGoal}
            disabled={busy}
            onChange={(event) => setAssistantGoal(event.target.value)}
            aria-label="Assistant goal"
            style={{ minWidth: "16rem" }}
          />
          <button
            type="button"
            disabled={busy || !workspace || !assistantGoal.trim()}
            onClick={() =>
              void run("Assistant goal submitted", async () => {
                if (!workspace) return;
                const workflow = await invokeIpc<AiAssistantWorkflow>(
                  "submit_assistant_goal",
                  {
                    goal: assistantGoal.trim(),
                    workspaceId: workspace.id,
                  },
                );
                setAssistantWorkflow(workflow);
                setAssistantComparison(null);
              })
            }
          >
            Submit assistant goal
          </button>
          <button
            type="button"
            disabled={
              busy ||
              !assistantWorkflow ||
              !workspace ||
              !assistantGoal.trim() ||
              !(
                assistantWorkflow.state === "awaiting_confirmation" ||
                assistantWorkflow.state === "cancelled" ||
                assistantWorkflow.state === "failed"
              )
            }
            onClick={() =>
              void run("Assistant goal revised", async () => {
                if (!assistantWorkflow || !workspace) return;
                const workflow = await invokeIpc<AiAssistantWorkflow>(
                  "revise_assistant_goal",
                  {
                    workflowId: assistantWorkflow.id,
                    goal: assistantGoal.trim(),
                    workspaceId: workspace.id,
                  },
                );
                setAssistantWorkflow(workflow);
              })
            }
          >
            Revise goal
          </button>
          <button
            type="button"
            disabled={
              busy ||
              !assistantWorkflow ||
              !workspace ||
              assistantWorkflow.state !== "awaiting_confirmation"
            }
            onClick={() =>
              void run("Assistant plan regenerated", async () => {
                if (!assistantWorkflow || !workspace) return;
                const workflow = await invokeIpc<AiAssistantWorkflow>(
                  "regenerate_assistant_plan",
                  {
                    workflowId: assistantWorkflow.id,
                    workspaceId: workspace.id,
                  },
                );
                setAssistantWorkflow(workflow);
              })
            }
          >
            Regenerate plan
          </button>
          <button
            type="button"
            disabled={
              busy ||
              !assistantWorkflow ||
              !(assistantWorkflow.plan_revisions?.length) ||
              !assistantWorkflow.plan_preview
            }
            onClick={() =>
              void run("Assistant plans compared", async () => {
                if (!assistantWorkflow) return;
                const revisions = assistantWorkflow.plan_revisions ?? [];
                const prior = revisions[revisions.length - 1];
                if (!prior) return;
                const result = await invokeIpc<AiAssistantPlanComparison>(
                  "compare_assistant_plan_revisions",
                  {
                    workflowId: assistantWorkflow.id,
                    leftRevision: prior.revision,
                    rightRevision: null,
                  },
                );
                setAssistantComparison(result);
              })
            }
          >
            Compare plan revisions
          </button>
          <button
            type="button"
            disabled={
              busy ||
              !assistantWorkflow ||
              !assistantWorkflow.plan_preview?.actions[0]
            }
            onClick={() =>
              void run("Explanation viewed (audit)", async () => {
                if (!assistantWorkflow?.plan_preview?.actions[0]) return;
                await invokeIpc("record_assistant_explanation_viewed", {
                  workflowId: assistantWorkflow.id,
                  stepId: assistantWorkflow.plan_preview.actions[0].step_id,
                });
              })
            }
          >
            Record explanation view
          </button>
          <button
            type="button"
            disabled={
              busy ||
              !assistantWorkflow ||
              assistantWorkflow.state !== "awaiting_confirmation"
            }
            onClick={() =>
              void run("Assistant workflow confirmed", async () => {
                if (!assistantWorkflow) return;
                const workflow = await invokeIpc<AiAssistantWorkflow>(
                  "confirm_assistant_workflow",
                  { workflowId: assistantWorkflow.id },
                );
                setAssistantWorkflow(workflow);
              })
            }
          >
            Confirm governed workflow
          </button>
          <button
            type="button"
            disabled={
              busy ||
              !assistantWorkflow ||
              assistantWorkflow.state !== "waiting_for_permission"
            }
            onClick={() =>
              void run("Assistant workflow resumed", async () => {
                if (!assistantWorkflow) return;
                const workflow = await invokeIpc<AiAssistantWorkflow>(
                  "resume_assistant_workflow",
                  { workflowId: assistantWorkflow.id },
                );
                setAssistantWorkflow(workflow);
              })
            }
          >
            Resume after permission
          </button>
          <button
            type="button"
            disabled={busy || !assistantWorkflow}
            onClick={() =>
              void run("Assistant workflow cancelled", async () => {
                if (!assistantWorkflow) return;
                const workflow = await invokeIpc<AiAssistantWorkflow>(
                  "cancel_assistant_workflow",
                  { workflowId: assistantWorkflow.id },
                );
                setAssistantWorkflow(workflow);
              })
            }
          >
            Cancel / recover
          </button>
        </div>
        {assistantWorkflow && (
          <ul className="muted">
            <li>
              Goal: {assistantWorkflow.user_goal} — state:{" "}
              {assistantWorkflow.state} (product:{" "}
              {assistantWorkflow.state === "submitting_actions"
                ? "executing"
                : assistantWorkflow.state === "waiting_for_permission"
                  ? "awaiting_permission"
                  : assistantWorkflow.state === "generating_plan"
                    ? "planning"
                    : assistantWorkflow.state}
              )
            </li>
            <li>{assistantWorkflow.status_message}</li>
            {assistantWorkflow.plan_preview?.influence_summary && (
              <li>{assistantWorkflow.plan_preview.influence_summary}</li>
            )}
            {assistantWorkflow.plan_preview?.actions.map((action) => (
              <li key={action.step_id}>
                {action.ordinal + 1}. {action.command_name} → {action.step_state}
                {action.capability_hint ? ` (${action.capability_hint})` : ""}
                {action.structured_explanation && (
                  <div>
                    Why: {action.structured_explanation.why_suggested} | Perm:{" "}
                    {action.structured_explanation.why_permission} | Approve:{" "}
                    {action.structured_explanation.what_if_approve}
                  </div>
                )}
              </li>
            ))}
            {assistantWorkflow.plan_preview && (
              <li>{assistantWorkflow.plan_preview.permission_note}</li>
            )}
            <li>
              Revisions archived:{" "}
              {assistantWorkflow.plan_revisions?.length ?? 0}
            </li>
          </ul>
        )}
        {assistantComparison && (
          <ul className="muted">
            <li>
              Compare rev {assistantComparison.left.revision} →{" "}
              {assistantComparison.right.revision}
            </li>
            {assistantComparison.differences.map((diff) => (
              <li key={diff}>{diff}</li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Governed multi-step AI plan</h2>
        <p className="muted">
          Orchestration organizes proposals. Each step still passes through the
          Permission Gateway — no shortcut path, no silent continue after deny.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Multi-step AI plan created", async () => {
                if (!workspace) return;
                const plan = await invokeIpc<AiOrchestratedPlan>(
                  "create_orchestrated_ai_plan",
                  {
                    goal: "Prepare my coding workspace",
                    workspaceId: workspace.id,
                  },
                );
                setOrchestratedPlan(plan);
              })
            }
          >
            Create multi-step plan
          </button>
          <button
            type="button"
            disabled={busy || !orchestratedPlan}
            onClick={() =>
              void run("Plan advanced through gateway", async () => {
                if (!orchestratedPlan) return;
                const plan = await invokeIpc<AiOrchestratedPlan>(
                  "advance_orchestrated_ai_plan",
                  { planId: orchestratedPlan.id },
                );
                setOrchestratedPlan(plan);
              })
            }
          >
            Advance plan (gateway)
          </button>
          <button
            type="button"
            disabled={busy || !orchestratedPlan}
            onClick={() =>
              void run("Plan resumed after approval decision", async () => {
                if (!orchestratedPlan) return;
                const plan = await invokeIpc<AiOrchestratedPlan>(
                  "resume_orchestrated_ai_plan",
                  { planId: orchestratedPlan.id },
                );
                setOrchestratedPlan(plan);
              })
            }
          >
            Resume after approval
          </button>
          <button
            type="button"
            disabled={busy || !orchestratedPlan}
            onClick={() =>
              void run("Plan cancelled", async () => {
                if (!orchestratedPlan) return;
                const plan = await invokeIpc<AiOrchestratedPlan>(
                  "cancel_orchestrated_ai_plan",
                  { planId: orchestratedPlan.id },
                );
                setOrchestratedPlan(plan);
              })
            }
          >
            Cancel plan
          </button>
        </div>
        {orchestratedPlan && (
          <ul className="muted">
            <li>
              {orchestratedPlan.goal.statement} — state: {orchestratedPlan.state}{" "}
              ({orchestratedPlan.steps.length} steps)
            </li>
            {orchestratedPlan.steps.map((step) => (
              <li key={step.id}>
                {step.ordinal + 1}. {step.proposal.command_name} → {step.state}
                {step.approval_request_id
                  ? ` (approval ${step.approval_request_id.slice(0, 8)}…)`
                  : ""}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>AI plan evaluation (measurement only)</h2>
        <p className="muted">
          Was the proposal useful / necessary / appropriate? Evaluation does not
          authorize actions — the Permission Gateway still decides.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("AI plan evaluated", async () => {
                if (!workspace) return;
                const report = await invokeIpc<AiPlanEvaluationReport>(
                  "diagnose_ai_plan_evaluation",
                  {
                    goal: "Prepare my workspace",
                    workspaceId: workspace.id,
                  },
                );
                setLastEvaluation(report);
              })
            }
          >
            Evaluate plan (no submit)
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Evaluation history loaded", async () => {
                const history = await invokeIpc<AiProposalEvaluation[]>(
                  "get_ai_evaluation_history",
                  { limit: 20 },
                );
                setEvaluationHistory(history);
              })
            }
          >
            Load evaluation history
          </button>
        </div>
        {lastEvaluation && (
          <ul className="muted">
            <li>
              {lastEvaluation.goal_statement}:{" "}
              {lastEvaluation.summary.proposal_count} proposal(s),{" "}
              {lastEvaluation.summary.unnecessary_count} unnecessary,{" "}
              {lastEvaluation.summary.duplicate_count} duplicate
            </li>
            {lastEvaluation.evaluations.slice(0, 5).map((evaluation) => (
              <li key={evaluation.proposal_id}>
                {evaluation.command_name} → {evaluation.outcome}
                {evaluation.quality_issues.length > 0
                  ? ` (${evaluation.quality_issues.map((i) => i.kind).join(", ")})`
                  : ""}
              </li>
            ))}
            <li>{lastEvaluation.authority_note}</li>
          </ul>
        )}
        {evaluationHistory.length > 0 && (
          <ul className="muted">
            {evaluationHistory.slice(0, 5).map((evaluation) => (
              <li key={`${evaluation.proposal_id}-${evaluation.evaluated_at}`}>
                {evaluation.command_name}: {evaluation.outcome}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Action catalog (informational)</h2>
        <p className="muted">
          What actions exist and which capability they require. Seeing an action
          does not authorize it — the Permission Gateway still decides.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              void run("Action catalog loaded", async () => {
                const catalog = await invokeIpc<ActionCatalog>(
                  "get_action_catalog",
                );
                setActionCatalog(catalog);
              })
            }
          >
            Explore actions
          </button>
        </div>
        {actionCatalog && (
          <ul className="muted">
            {actionCatalog.entries
              .filter((entry) =>
                ["LaunchApplication", "CreateApplication", "CreateZone"].includes(
                  entry.command_name,
                ),
              )
              .map((entry) => (
                <li key={entry.intent_id}>
                  {entry.name} → requires{" "}
                  <code>{entry.capability_required.id}</code>
                </li>
              ))}
            <li>
              … {actionCatalog.entries.length} actions total (catalog only)
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Permission approvals</h2>
        <p className="muted">
          Human control loop: Allow once issues a consumable grant; Deny leaves
          the action blocked.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Approvals refreshed", async () => {
                if (!workspace) return;
                await refreshReads(workspace.id);
              })
            }
          >
            Refresh approvals
          </button>
        </div>
        {approvals.length === 0 ? (
          <p className="muted">No approval requests yet.</p>
        ) : (
          <ul className="list">
            {approvals.map((item) => (
              <li key={item.id}>
                <strong>{item.status}</strong> — {item.command_name} /{" "}
                {item.capability} ({item.requesting_actor_type})
                <div className="muted">{item.reason}</div>
                {item.status === "pending" && (
                  <div className="row">
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() =>
                        void run("Allowed once", async () => {
                          await invokeIpc<ApprovalDecisionResult>(
                            "decide_approval",
                            {
                              requestId: item.id,
                              decision: "allow_once",
                            },
                          );
                          if (workspace) {
                            await refreshReads(workspace.id);
                          }
                        })
                      }
                    >
                      Allow once
                    </button>
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() =>
                        void run("Denied", async () => {
                          await invokeIpc<ApprovalDecisionResult>(
                            "decide_approval",
                            {
                              requestId: item.id,
                              decision: "deny",
                            },
                          );
                          if (workspace) {
                            await refreshReads(workspace.id);
                          }
                        })
                      }
                    >
                      Deny
                    </button>
                  </div>
                )}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>WorkspaceState</h2>
        <p className="muted">
          Canonical runtime desktop projection via WorkspaceStateEngine
          (observation + delta). Not live Win32 enumeration.
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy}
            onClick={() => void refreshWorkspaceState()}
          >
            Refresh WorkspaceState
          </button>
        </div>
        {workspaceStateMeta ? (
          <p className="muted mono">{workspaceStateMeta}</p>
        ) : null}
        {workspaceStateWindows.length === 0 ? (
          <p className="muted">
            No projected windows yet — capture an observation, then refresh.
          </p>
        ) : (
          <ul className="list compact">
            {workspaceStateWindows.map((w) => (
              <li key={w.hwnd}>
                <strong>{w.title}</strong>{" "}
                <span className="mono">pid {w.process_id}</span>
                {w.focused ? " · focused" : ""}
                {w.minimized ? " · minimized" : ""}
              </li>
            ))}
          </ul>
        )}
      </section>

      {settings && (
        <section>
          <h2>Settings</h2>
          <dl>
            <dt>Theme</dt>
            <dd>{settings.theme}</dd>
            <dt>First run</dt>
            <dd>{settings.first_run ? "yes" : "no"}</dd>
          </dl>
          <div className="row">
            {["system", "dark", "light"].map((theme) => (
              <button
                key={theme}
                type="button"
                disabled={busy || settings.theme === theme}
                onClick={() => void updateTheme(theme)}
              >
                {theme}
              </button>
            ))}
          </div>
        </section>
      )}

      <section>
        <h2>Workspace</h2>
        <div className="row">
          <input
            value={workspaceName}
            onChange={(e) => setWorkspaceName(e.target.value)}
            placeholder="Workspace name"
            disabled={busy}
          />
          <button
            type="button"
            disabled={busy}
            onClick={() => void createWorkspace()}
          >
            Create
          </button>
          {workspace && (
            <button
              type="button"
              disabled={busy}
              onClick={() =>
                void run("Refreshed", () => refreshReads(workspace.id))
              }
            >
              Refresh
            </button>
          )}
        </div>
        {workspace ? (
          <dl>
            <dt>Id</dt>
            <dd className="mono">{workspace.id}</dd>
            <dt>Name</dt>
            <dd>{workspace.name}</dd>
            <dt>Zones (snapshot)</dt>
            <dd>{zones.length}</dd>
          </dl>
        ) : (
          <p className="muted">No active workspace. Create one to begin.</p>
        )}
      </section>

      <section>
        <h2>Seed zones</h2>
        <p className="muted">
          ResourceGrowth suggestions require at least 3 resource creations.
          Zones appear on the Canvas tab.
        </p>
        <div className="row">
          <input
            value={zoneName}
            onChange={(e) => setZoneName(e.target.value)}
            placeholder="Zone name prefix"
            disabled={busy || !workspace}
          />
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() => void seedZones()}
          >
            Create 3 zones
          </button>
        </div>
      </section>

      <section>
        <h2>Suggestions</h2>
        {suggestions.length === 0 ? (
          <p className="muted">No proposals yet.</p>
        ) : (
          <ul className="list">
            {suggestions.map((s) => (
              <li key={s.id}>
                <div className="item-head">
                  <strong>{s.title}</strong>
                  <span className="badge">{s.status}</span>
                  <span className="badge">{s.suggestion_type}</span>
                </div>
                <p>{s.description}</p>
                <div className="row">
                  {s.status === "pending" && (
                    <>
                      <button
                        type="button"
                        disabled={busy}
                        onClick={() => void accept(s.id)}
                      >
                        Accept
                      </button>
                      <button
                        type="button"
                        disabled={busy}
                        onClick={() => void reject(s.id)}
                      >
                        Reject
                      </button>
                    </>
                  )}
                  {s.status === "accepted" && (
                    <>
                      <button
                        type="button"
                        disabled={busy}
                        onClick={() => void createIntent(s.id)}
                      >
                        Create intent
                      </button>
                      <button
                        type="button"
                        disabled={busy}
                        onClick={() => void execute(s.id)}
                      >
                        Execute
                      </button>
                      <button
                        type="button"
                        disabled={busy}
                        onClick={() => void cancelExecution(s.id)}
                      >
                        Request cancel
                      </button>
                    </>
                  )}
                </div>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Execution path</h2>
        {lastIntent && (
          <dl>
            <dt>Last intent</dt>
            <dd className="mono">
              {lastIntent.suggestion_id} → {lastIntent.intent_id}
            </dd>
          </dl>
        )}
        {lastExecution && (
          <dl>
            <dt>Last execution</dt>
            <dd className="mono">
              {lastExecution.id} ({lastExecution.status})
            </dd>
          </dl>
        )}
        {lastCancellation && (
          <dl>
            <dt>Last cancellation</dt>
            <dd className="mono">
              {lastCancellation.execution_request_id} ({lastCancellation.status})
            </dd>
          </dl>
        )}
        {!lastIntent && !lastExecution && (
          <p className="muted">
            Accept a suggestion, then create intent / execute.
          </p>
        )}
      </section>

      <section>
        <h2>Outcomes</h2>
        {outcomes.length === 0 ? (
          <p className="muted">No execution outcomes.</p>
        ) : (
          <ul className="list compact">
            {outcomes.map((o) => (
              <li key={`${o.execution_request_id}-${o.completed_at}`}>
                <span className="badge">{o.status}</span>{" "}
                <span className="mono">{o.execution_request_id}</span> —{" "}
                {o.command_name}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Reconciled states</h2>
        {executionStates.length === 0 ? (
          <p className="muted">No reconciled states.</p>
        ) : (
          <ul className="list compact">
            {executionStates.map((s) => (
              <li key={s.execution_request_id}>
                <span className="mono">{s.execution_request_id}</span> —{" "}
                {s.current_state}
                {s.dispatch_allowed ? " · dispatch ok" : " · dispatch blocked"}
                {s.cancellation_allowed ? " · cancel ok" : " · cancel blocked"}
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Lifecycle</h2>
        {lifecycle.length === 0 ? (
          <p className="muted">No lifecycle records.</p>
        ) : (
          <ul className="list compact">
            {lifecycle.slice(0, 20).map((r, i) => (
              <li key={`${r.suggestion_id}-${r.occurred_at}-${i}`}>
                <span className="badge">{r.state}</span>{" "}
                <span className="mono">{r.suggestion_id}</span>
              </li>
            ))}
          </ul>
        )}
      </section>

      {context && (
        <section>
          <h2>Context</h2>
          <dl>
            <dt>Generated</dt>
            <dd className="mono">{context.generated_at}</dd>
            <dt>Workspace</dt>
            <dd>{context.snapshot.workspace_name}</dd>
            <dt>Observations</dt>
            <dd>{context.observations.length}</dd>
            <dt>Exec completed</dt>
            <dd>{context.execution_context.recent_completed_count}</dd>
            <dt>Exec failed</dt>
            <dd>{context.execution_context.recent_failed_count}</dd>
            <dt>Exec cancelled</dt>
            <dd>{context.execution_context.recent_cancelled_count}</dd>
            <dt>Context states</dt>
            <dd>{context.execution_states.length}</dd>
          </dl>
        </section>
      )}
    </div>
  );
}
