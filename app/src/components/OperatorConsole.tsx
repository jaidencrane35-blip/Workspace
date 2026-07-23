import { useCallback, useEffect, useState } from "react";
import { IpcCommandError, invokeIpc } from "../lib/ipc";
import type {
  ActionCatalog,
  AiPlanSubmissionResult,
  ApplicationLaunchResult,
  ApplicationReference,
  ApprovalDecisionResult,
  CancellationRequest,
  DesktopWindowSnapshot,
  ExecutionOutcome,
  ExecutionReconciliation,
  IntentExecutionRequest,
  PermissionApprovalRequest,
  Suggestion,
  SuggestionIntentRequest,
  SuggestionLifecycleRecord,
  Workspace,
  WorkspaceContext,
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
  const [desktopWindows, setDesktopWindows] = useState<DesktopWindowSnapshot[]>(
    [],
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

  const refreshDesktopWindows = () =>
    run("Desktop windows refreshed", async () => {
      const windows = await invokeIpc<DesktopWindowSnapshot[]>(
        "get_desktop_windows",
        { limit: 50 },
      );
      setDesktopWindows(windows);
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
        <h2>Desktop windows</h2>
        <p className="muted">
          Win32 enumeration via Windows Integration Layer (observation only).
        </p>
        <div className="row">
          <button
            type="button"
            disabled={busy}
            onClick={() => void refreshDesktopWindows()}
          >
            Refresh windows
          </button>
        </div>
        {desktopWindows.length === 0 ? (
          <p className="muted">No windows listed yet — click refresh.</p>
        ) : (
          <ul className="list compact">
            {desktopWindows.map((w) => (
              <li key={w.hwnd}>
                <strong>{w.title}</strong>{" "}
                <span className="mono">pid {w.process_id}</span>
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
