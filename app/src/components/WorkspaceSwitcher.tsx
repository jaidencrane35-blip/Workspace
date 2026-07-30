/**
 * Purpose: Product switcher for saved workspaces (list / create / activate).
 * Owner: Frontend product shell (Milestone A / A.1)
 * Inputs: active workspace, busy flags, activate/create callbacks from App
 * Outputs: User selection of workspace; create-with-name requests
 * Dependencies: list_workspaces IPC, existing create/activate paths in App
 * Non-responsibilities: Window control, permissions, arrangement restore engine,
 *   Assistant reasoning, zone layout logic
 */

import { useCallback, useEffect, useState } from "react";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  formatWorkspaceMeta,
  workspaceRowLabel,
  workspaceSwitcherEmptyCopy,
} from "../lib/workspaceSwitcherUi";
import type { Workspace } from "../types/domain";

interface WorkspaceSwitcherProps {
  activeWorkspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  onActivate: (workspace: Workspace) => Promise<void>;
  onCreated: (workspace: Workspace) => Promise<void>;
}

export function WorkspaceSwitcher({
  activeWorkspace,
  busy,
  onBusy,
  onError,
  onMessage,
  onActivate,
  onCreated,
}: WorkspaceSwitcherProps) {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const [loading, setLoading] = useState(false);
  const [name, setName] = useState("");
  const [localHint, setLocalHint] = useState<string | null>(null);
  const runtime = isIpcRuntimeAvailable();
  const empty = workspaceSwitcherEmptyCopy(runtime);

  const refresh = useCallback(async () => {
    if (!runtime) {
      setWorkspaces([]);
      setLocalHint(empty.body);
      return;
    }
    setLoading(true);
    try {
      const listed = await invokeIpc<Workspace[]>("list_workspaces");
      setWorkspaces(listed);
      setLocalHint(null);
    } finally {
      setLoading(false);
    }
  }, [empty.body, runtime]);

  useEffect(() => {
    void refresh().catch((err: unknown) => {
      onError(err instanceof Error ? err.message : String(err));
    });
  }, [refresh, onError, activeWorkspace?.id]);

  const createWorkspace = () => {
    const trimmed = name.trim();
    if (!trimmed) {
      onError("Give the workspace a name before creating it.");
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const created = await invokeIpc<Workspace>("create_workspace", {
          name: trimmed,
        });
        await onCreated(created);
        setName("");
        await refresh();
        onMessage(`Workspace ready: ${created.name}`);
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  const switchTo = (workspace: Workspace) => {
    if (activeWorkspace?.id === workspace.id) {
      onMessage(`${workspace.name} is already current`);
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        await onActivate(workspace);
        onMessage(`Switched to ${workspace.name}`);
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  return (
    <section className="product-panel" aria-label="Workspace switcher">
      <header className="product-panel-hero">
        <p className="arrangement-eyebrow">Profiles</p>
        <h2>Named profiles (optional)</h2>
        <p className="lede">
          Optional labels for saved arrangements and library apps. Your desktop
          on Stage does not require one.
        </p>
      </header>

      {activeWorkspace ? (
        <p className="product-current" aria-live="polite">
          Current profile: <strong>{activeWorkspace.name}</strong>
        </p>
      ) : (
        <p className="muted">No profile selected — Stage still shows your desktop.</p>
      )}

      <section aria-label="Create profile">
        <h3>Add profile</h3>
        <label className="arrangement-field">
          <span>Name</span>
          <input
            type="text"
            value={name}
            disabled={busy || !runtime}
            placeholder="Deep work"
            onChange={(event) => setName(event.target.value)}
          />
        </label>
        <div className="row">
          <button
            type="button"
            disabled={busy || !runtime}
            onClick={createWorkspace}
          >
            Add profile
          </button>
          <button
            type="button"
            className="ghost"
            disabled={busy || loading || !runtime}
            onClick={() => {
              void refresh()
                .then(() => onMessage("Workspaces refreshed"))
                .catch((err: unknown) => {
                  onError(err instanceof Error ? err.message : String(err));
                });
            }}
          >
            Refresh
          </button>
        </div>
      </section>

      <section aria-label="Saved workspaces">
        <h3>Switch workspace</h3>
        {localHint ? <p className="muted">{localHint}</p> : null}
        {loading && workspaces.length === 0 ? (
          <p className="muted">Loading…</p>
        ) : workspaces.length === 0 ? (
          <div className="arrangement-empty" aria-live="polite">
            <h4>{empty.title}</h4>
            <p className="muted">{empty.body}</p>
          </div>
        ) : (
          <ul className="product-list">
            {workspaces.map((workspace) => {
              const active = activeWorkspace?.id === workspace.id;
              return (
                <li key={workspace.id}>
                  <button
                    type="button"
                    className={
                      active ? "product-list-item active" : "product-list-item"
                    }
                    disabled={busy}
                    aria-current={active ? "true" : undefined}
                    onClick={() => switchTo(workspace)}
                  >
                    <span className="product-list-title">
                      {workspaceRowLabel(
                        workspace,
                        activeWorkspace?.id ?? null,
                      )}
                    </span>
                    <span className="product-list-meta muted">
                      {formatWorkspaceMeta(workspace)}
                    </span>
                  </button>
                </li>
              );
            })}
          </ul>
        )}
      </section>
    </section>
  );
}
