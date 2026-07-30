/**
 * Purpose: Applications as observed desktop objects first; library optional.
 * Owner: Frontend product shell (Product Contract V5 / Product Foundation V14)
 * Inputs: Active workspace, shared busy/banner callbacks
 * Outputs: WorkspaceState for running apps; focus_desktop_window; list/create/launch registry
 * Dependencies: Existing application + desktop focus IPC; workspaceStateClient
 * Non-responsibilities: Assistant, OS discovery, geometry apply, minimize APIs,
 *   parallel activeApps/activeWindows slices (hold WorkspaceState)
 */

import { useCallback, useEffect, useState } from "react";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  launchRegisteredApplication,
  launchSuccessMessage,
} from "../lib/applicationLaunch";
import {
  activeApplicationName,
  applicationsEmptyCopy,
} from "../lib/applicationsUi";
import { refreshObservedWorkspaceState } from "../lib/workspaceStateClient";
import type {
  ApplicationReference,
  Workspace,
  WorkspaceActiveApplication,
  WorkspaceState,
} from "../types/domain";
import {
  ActiveApplicationsView,
  resolveActiveApplicationHwnd,
} from "./ActiveApplicationsView";
import { ApplicationList } from "./ApplicationList";
import type { DesktopWindowFocusResult } from "../types/desktopArrangement";

interface ApplicationsPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

export function ApplicationsPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
}: ApplicationsPanelProps) {
  const [applications, setApplications] = useState<ApplicationReference[]>([]);
  const [workspaceState, setWorkspaceState] = useState<WorkspaceState | null>(
    null,
  );
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [activeLoading, setActiveLoading] = useState(false);
  const [name, setName] = useState("");
  const [identifier, setIdentifier] = useState("");
  const [executablePath, setExecutablePath] = useState("");
  const [localHint, setLocalHint] = useState<string | null>(null);
  const [showRegister, setShowRegister] = useState(false);
  const runtime = isIpcRuntimeAvailable();
  const empty = applicationsEmptyCopy(Boolean(workspace));
  const activeApps = workspaceState?.active_applications ?? [];
  const activeWindows = workspaceState?.windows ?? [];

  const selected =
    applications.find((item) => item.id === selectedId) ?? null;

  const run = useCallback(
    async (okMessage: string, action: () => Promise<void>) => {
      onBusy(true);
      onError(null);
      setLocalHint(null);
      try {
        await action();
        onMessage(okMessage);
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    },
    [onBusy, onError, onMessage],
  );

  const refreshRegistry = useCallback(async () => {
    if (!workspace) {
      setApplications([]);
      setSelectedId(null);
      return;
    }
    if (!runtime) {
      setApplications([]);
      setLocalHint("Library needs the desktop app.");
      return;
    }
    setLoading(true);
    try {
      const listed = await invokeIpc<ApplicationReference[]>(
        "list_applications",
        { workspaceId: workspace.id },
      );
      setApplications(listed);
      setSelectedId((prev) => {
        if (prev && listed.some((item) => item.id === prev)) {
          return prev;
        }
        return listed[0]?.id ?? null;
      });
      setLocalHint(null);
    } finally {
      setLoading(false);
    }
  }, [runtime, workspace]);

  const refreshActive = useCallback(async () => {
    if (!runtime) {
      setWorkspaceState(null);
      return;
    }
    setActiveLoading(true);
    try {
      const state = await refreshObservedWorkspaceState(
        "workspace_applications",
      );
      setWorkspaceState(state);
    } finally {
      setActiveLoading(false);
    }
  }, [runtime]);

  useEffect(() => {
    void refreshRegistry().catch((err: unknown) => {
      onError(err instanceof Error ? err.message : String(err));
    });
  }, [refreshRegistry, onError]);

  useEffect(() => {
    void refreshActive().catch(() => {
      setWorkspaceState(null);
    });
  }, [refreshActive, workspace?.id]);
  const registerApplication = () => {
    if (!workspace) {
      onError("Create a profile under Profiles to save library apps.");
      return;
    }
    const trimmed = name.trim();
    if (!trimmed) {
      onError("Give the application a name.");
      return;
    }
    void run("Application added", async () => {
      const created = await invokeIpc<ApplicationReference>(
        "create_application",
        {
          workspaceId: workspace.id,
          name: trimmed,
          identifier: identifier.trim() || null,
          executablePath: executablePath.trim() || null,
        },
      );
      setApplications((prev) => [
        created,
        ...prev.filter((a) => a.id !== created.id),
      ]);
      setSelectedId(created.id);
      setName("");
      setIdentifier("");
      setExecutablePath("");
      setShowRegister(false);
    });
  };

  const launchApplication = (app: ApplicationReference) => {
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const result = await launchRegisteredApplication(app);
        await refreshActive();
        onMessage(launchSuccessMessage(result));
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  const focusActiveApplication = (app: WorkspaceActiveApplication) => {
    const hwnd = resolveActiveApplicationHwnd(
      app,
      activeWindows,
      workspaceState?.focused_window ?? null,
    );
    if (!hwnd) {
      onError("No observed window handle for that application.");
      return;
    }
    const label = activeApplicationName(app);
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const result = await invokeIpc<DesktopWindowFocusResult>(
          "focus_desktop_window",
          { hwnd },
        );
        await refreshActive();
        onMessage(
          result.simulated
            ? `Focused ${label} (simulated)`
            : `Focused ${label}`,
        );
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  return (
    <section className="product-panel applications-panel" aria-label="Applications">
      <header className="product-panel-hero">
        <h2>Running</h2>
      </header>

      <section aria-label="Running on the desktop">
        <div className="row section-heading-row">
          <button
            type="button"
            className="ghost"
            disabled={busy || activeLoading || !runtime}
            onClick={() => {
              void run("Desktop apps refreshed", refreshActive);
            }}
          >
            Refresh
          </button>
        </div>
        <ActiveApplicationsView
          applications={activeApps}
          windows={activeWindows}
          focusedWindow={workspaceState?.focused_window ?? null}
          loading={activeLoading}
          busy={busy}
          onFocusApplication={focusActiveApplication}
        />
      </section>

      <details className="applications-library-details">
        <summary>Optional library</summary>
        {!workspace ? (
          <p className="muted">{empty.body}</p>
        ) : (
          <>
            <div className="row section-heading-row">
              <div className="row">
                <button
                  type="button"
                  className="ghost"
                  disabled={busy || loading || !runtime}
                  onClick={() => {
                    void run("Applications refreshed", refreshRegistry);
                  }}
                >
                  Refresh
                </button>
                <button
                  type="button"
                  className="ghost"
                  disabled={busy || !runtime}
                  onClick={() => setShowRegister((open) => !open)}
                >
                  {showRegister ? "Hide form" : "Add"}
                </button>
              </div>
            </div>
            {localHint ? <p className="muted">{localHint}</p> : null}
            {loading && applications.length === 0 ? (
              <p className="muted">Loading…</p>
            ) : applications.length === 0 ? (
              <p className="muted">{empty.body}</p>
            ) : (
              <ApplicationList
                applications={applications}
                selectedId={selectedId}
                busy={busy || !runtime}
                onSelect={setSelectedId}
                onLaunch={launchApplication}
              />
            )}
            {selected ? (
              <p className="muted arrangement-permission-note">
                Selected: <strong>{selected.name}</strong>
              </p>
            ) : null}
            {showRegister ? (
              <section aria-label="Add application">
                <label className="arrangement-field">
                  <span>Name</span>
                  <input
                    type="text"
                    value={name}
                    disabled={busy || !runtime}
                    placeholder="Code"
                    onChange={(event) => setName(event.target.value)}
                  />
                </label>
                <label className="arrangement-field">
                  <span>Identity (optional)</span>
                  <input
                    type="text"
                    value={identifier}
                    disabled={busy || !runtime}
                    placeholder="editor"
                    onChange={(event) => setIdentifier(event.target.value)}
                  />
                </label>
                <label className="arrangement-field">
                  <span>Executable path</span>
                  <input
                    type="text"
                    value={executablePath}
                    disabled={busy || !runtime}
                    placeholder="C:\\Program Files\\App\\app.exe"
                    onChange={(event) => setExecutablePath(event.target.value)}
                  />
                </label>
                <button
                  type="button"
                  disabled={busy || !runtime}
                  onClick={registerApplication}
                >
                  Save
                </button>
              </section>
            ) : null}
          </>
        )}
      </details>
    </section>
  );
}
