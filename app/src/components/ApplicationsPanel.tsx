/**
 * Purpose: Applications as observed desktop objects first; library optional.
 * Owner: Frontend product shell
 * Inputs: Active workspace, work mode, shared busy/banner callbacks
 * Outputs: get_workspace_state / list_applications / create / launch
 * Dependencies: Existing application IPC + WorkspaceState + workMode
 * Non-responsibilities: Permission policy, WindowController, Assistant,
 *   OS app discovery, auto-layout, AI recommendations, OS geometry apply
 */

import { useCallback, useEffect, useState } from "react";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  launchRegisteredApplication,
  launchSuccessMessage,
} from "../lib/applicationLaunch";
import {
  applicationsEmptyCopy,
  applicationsLayoutsRelationCopy,
} from "../lib/applicationsUi";
import type { WorkMode } from "../lib/workMode";
import type {
  ApplicationReference,
  Workspace,
  WorkspaceActiveApplication,
  WorkspaceState,
} from "../types/domain";
import { ActiveApplicationsView } from "./ActiveApplicationsView";
import { ApplicationList } from "./ApplicationList";

interface ApplicationsPanelProps {
  workspace: Workspace | null;
  workMode: WorkMode;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

export function ApplicationsPanel({
  workspace,
  workMode,
  busy,
  onBusy,
  onError,
  onMessage,
}: ApplicationsPanelProps) {
  const [applications, setApplications] = useState<ApplicationReference[]>([]);
  const [activeApps, setActiveApps] = useState<WorkspaceActiveApplication[]>(
    [],
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
      setLocalHint(
        "Preview mode: library cards appear here once registered in the desktop app.",
      );
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
      setActiveApps([]);
      return;
    }
    setActiveLoading(true);
    try {
      const state = await invokeIpc<WorkspaceState>("get_workspace_state");
      setActiveApps(state.active_applications);
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
      setActiveApps([]);
    });
  }, [refreshActive, workspace?.id]);

  const registerApplication = () => {
    if (!workspace) {
      onError(
        "Select or create a named profile under Profiles to save library apps.",
      );
      return;
    }
    const trimmed = name.trim();
    if (!trimmed) {
      onError("Give the application a name.");
      return;
    }
    void run("Application added to this workspace", async () => {
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

  return (
    <section
      className={
        workMode === "focus"
          ? "product-panel applications-panel mode-focus"
          : "product-panel applications-panel mode-flow"
      }
      aria-label="Applications"
      data-work-mode={workMode}
    >
      <header className="product-panel-hero">
        <p className="arrangement-eyebrow">Apps</p>
        <h2>{applicationsLayoutsRelationCopy()}</h2>
      </header>

      <section
        aria-label="Running on the desktop"
        className={workMode === "focus" ? "applications-active quiet" : undefined}
      >
        <div className="row section-heading-row">
          <h3>Running now</h3>
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
          loading={activeLoading}
        />
      </section>

      <details className="applications-library-details">
        <summary>Optional library</summary>
        <p className="muted">
          Register apps only when you want launch shortcuts tied to a named
          profile. Observed windows do not need this.
        </p>
        {!workspace ? (
          <div className="arrangement-empty" aria-live="polite">
            <h3>{empty.title}</h3>
            <p className="muted">{empty.body}</p>
          </div>
        ) : (
          <>
            <div className="row section-heading-row">
              <h3>Library</h3>
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
                  disabled={busy || !runtime}
                  onClick={() => setShowRegister((open) => !open)}
                >
                  {showRegister ? "Hide add form" : "Add application"}
                </button>
              </div>
            </div>
            {localHint ? <p className="muted">{localHint}</p> : null}
            {loading && applications.length === 0 ? (
              <p className="muted">Loading…</p>
            ) : applications.length === 0 ? (
              <div className="arrangement-empty" aria-live="polite">
                <h4>{empty.title}</h4>
                <p className="muted">{empty.body}</p>
              </div>
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
                Launch asks Workspace for permission before starting the app.
                Selected: <strong>{selected.name}</strong>
              </p>
            ) : null}
            {showRegister ? (
              <section aria-label="Add application">
                <h3>Add application</h3>
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
                  <span>Short identity (optional)</span>
                  <input
                    type="text"
                    value={identifier}
                    disabled={busy || !runtime}
                    placeholder="editor"
                    onChange={(event) => setIdentifier(event.target.value)}
                  />
                </label>
                <label className="arrangement-field">
                  <span>Executable path (needed to launch)</span>
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
                  Save to workspace
                </button>
              </section>
            ) : null}
          </>
        )}
      </details>
    </section>
  );
}
