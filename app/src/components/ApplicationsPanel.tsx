/**
 * Purpose: Optional application library — running desktop lives on Stage.
 * Owner: Frontend product shell (Product Contract V5 / Product Foundation V15)
 * Inputs: Active workspace, shared busy/banner callbacks, optional open-Stage
 * Outputs: list/create/launch registry; refreshes shared WorkspaceState after launch
 * Dependencies: application IPC; workspaceStateClient
 * Non-responsibilities: Parallel running-app focus grid (Stage owns that),
 *   Assistant, OS discovery, geometry apply, minimize APIs
 */

import { useCallback, useEffect, useState } from "react";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  launchRegisteredApplication,
  launchSuccessMessage,
} from "../lib/applicationLaunch";
import { applicationsEmptyCopy } from "../lib/applicationsUi";
import { useObservedWorkspaceState } from "../lib/useObservedWorkspaceState";
import type { ApplicationReference, Workspace } from "../types/domain";
import { ApplicationList } from "./ApplicationList";

interface ApplicationsPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  /** Navigate to Stage — the continuous desktop for running apps. */
  onOpenStage?: () => void;
}

export function ApplicationsPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
  onOpenStage,
}: ApplicationsPanelProps) {
  const [applications, setApplications] = useState<ApplicationReference[]>([]);
  const { refreshWorkspaceState } = useObservedWorkspaceState();
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
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

  useEffect(() => {
    void refreshRegistry().catch((err: unknown) => {
      onError(err instanceof Error ? err.message : String(err));
    });
  }, [refreshRegistry, onError]);

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
        if (runtime) {
          try {
            await refreshWorkspaceState("workspace_applications");
          } catch {
            // Stage will refresh on next observation epoch / open.
          }
        }
        onMessage(launchSuccessMessage(result));
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  return (
    <section className="product-panel applications-panel" aria-label="Library">
      <header className="product-panel-hero">
        <h2>Library</h2>
        <p className="muted">
          Optional saved apps. Running windows live on Desktop.
        </p>
      </header>

      {onOpenStage ? (
        <p className="applications-stage-link">
          <button
            type="button"
            className="ghost"
            disabled={busy}
            onClick={onOpenStage}
          >
            Open Desktop
          </button>
          <span className="muted"> — see and Restore Arrangements</span>
        </p>
      ) : null}

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
                Save to library
              </button>
            </section>
          ) : null}
        </>
      )}
    </section>
  );
}
