/**
 * Purpose: Product Applications surface — registry, identity, governed launch,
 *   and observed active apps.
 * Owner: Frontend product shell (Milestone A)
 * Inputs: Active workspace + shared busy/banner callbacks
 * Outputs: create_application / launch_application / list_applications /
 *   get_workspace_state invocations; user-facing status copy
 * Dependencies: Existing application IPC + WorkspaceState projection
 * Non-responsibilities: Permission policy, WindowController, Assistant,
 *   OS app discovery, auto-layout
 */

import { useCallback, useEffect, useState } from "react";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  applicationsEmptyCopy,
  canLaunchApplication,
} from "../lib/applicationsUi";
import type {
  ApplicationLaunchResult,
  ApplicationReference,
  Workspace,
  WorkspaceActiveApplication,
  WorkspaceState,
} from "../types/domain";
import { ActiveApplicationsView } from "./ActiveApplicationsView";
import { ApplicationList } from "./ApplicationList";

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
        "Application registry needs the Workspace desktop runtime. Browse UI still works; register and launch stay disabled.",
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
      // Observation may be empty before first capture — keep panel usable.
      setActiveApps([]);
    });
  }, [refreshActive, workspace?.id]);

  const registerApplication = () => {
    if (!workspace) {
      onError("Create or select a workspace first.");
      return;
    }
    const trimmed = name.trim();
    if (!trimmed) {
      onError("Give the application a name.");
      return;
    }
    void run("Application registered", async () => {
      const created = await invokeIpc<ApplicationReference>(
        "create_application",
        {
          workspaceId: workspace.id,
          name: trimmed,
          identifier: identifier.trim() || null,
          executablePath: executablePath.trim() || null,
        },
      );
      setApplications((prev) => [created, ...prev.filter((a) => a.id !== created.id)]);
      setSelectedId(created.id);
      setName("");
      setIdentifier("");
      setExecutablePath("");
    });
  };

  const launchApplication = (app: ApplicationReference) => {
    if (!canLaunchApplication(app)) {
      onError("Add an executable path before launching this application.");
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const result = await invokeIpc<ApplicationLaunchResult>(
          "launch_application",
          { id: app.id },
        );
        await refreshActive();
        onMessage(
          result.simulated
            ? `Launch recorded for ${result.name} (simulated on this platform)`
            : `Launched ${result.name}`,
        );
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  return (
    <section className="product-panel" aria-label="Applications">
      <header className="product-panel-hero">
        <p className="arrangement-eyebrow">Applications</p>
        <h2>Workspace applications</h2>
        <p className="lede">
          See registered apps, understand identity, and launch through the
          existing governed path. Active desktop processes come from observation
          — not Assistant suggestions.
        </p>
      </header>

      {!workspace ? (
        <div className="arrangement-empty" aria-live="polite">
          <h3>{empty.title}</h3>
          <p className="muted">{empty.body}</p>
        </div>
      ) : (
        <>
          <section aria-label="Register application">
            <h3>Register application</h3>
            {localHint ? <p className="muted">{localHint}</p> : null}
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
              <span>Identifier (optional)</span>
              <input
                type="text"
                value={identifier}
                disabled={busy || !runtime}
                placeholder="com.example.code"
                onChange={(event) => setIdentifier(event.target.value)}
              />
            </label>
            <label className="arrangement-field">
              <span>Executable path (optional, required to launch)</span>
              <input
                type="text"
                value={executablePath}
                disabled={busy || !runtime}
                placeholder="C:\\Program Files\\App\\app.exe"
                onChange={(event) => setExecutablePath(event.target.value)}
              />
            </label>
            <div className="row">
              <button
                type="button"
                disabled={busy || !runtime}
                onClick={registerApplication}
              >
                Register
              </button>
              <button
                type="button"
                className="ghost"
                disabled={busy || loading || !runtime}
                onClick={() => {
                  void run("Applications refreshed", refreshRegistry);
                }}
              >
                Refresh registry
              </button>
            </div>
          </section>

          <section aria-label="Registered applications">
            <h3>Registered in this workspace</h3>
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
                Launch uses Permission Gateway (`application_launch`). Selected:{" "}
                <span className="mono">{selected.id}</span>
              </p>
            ) : null}
          </section>

          <section aria-label="Active desktop applications">
            <div className="row section-heading-row">
              <h3>Active on desktop</h3>
              <button
                type="button"
                className="ghost"
                disabled={busy || activeLoading || !runtime}
                onClick={() => {
                  void run("Active applications refreshed", refreshActive);
                }}
              >
                Refresh observed
              </button>
            </div>
            <ActiveApplicationsView
              applications={activeApps}
              loading={activeLoading}
            />
          </section>
        </>
      )}
    </section>
  );
}
