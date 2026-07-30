/**
 * Purpose: Optional named profiles — list first, create secondary.
 * Owner: Frontend product shell (Product Contract V3)
 * Inputs: active workspace, busy flags, activate/create callbacks from App
 * Outputs: Profile selection; optional create-with-name
 * Dependencies: list_workspaces IPC
 * Non-responsibilities: Window control, Stage observation, Assistant
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
      onError("Give the profile a name before creating it.");
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
        onMessage(`Profile ready: ${created.name}`);
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
    <section className="product-panel" aria-label="Profiles">
      <header className="product-panel-hero">
        <p className="arrangement-eyebrow">Profiles</p>
        <h2>Named profiles</h2>
      </header>

      {activeWorkspace ? (
        <p className="product-current" aria-live="polite">
          Current: <strong>{activeWorkspace.name}</strong>
        </p>
      ) : (
        <p className="muted">None selected — Stage still shows your desktop.</p>
      )}

      <section aria-label="Saved profiles">
        <div className="row section-heading-row">
          <h3>Profiles</h3>
          <button
            type="button"
            className="ghost"
            disabled={busy || loading || !runtime}
            onClick={() => {
              void refresh()
                .then(() => onMessage("Profiles refreshed"))
                .catch((err: unknown) => {
                  onError(err instanceof Error ? err.message : String(err));
                });
            }}
          >
            Refresh
          </button>
        </div>
        {localHint ? <p className="muted">{localHint}</p> : null}
        {loading && workspaces.length === 0 ? (
          <p className="muted">Loading…</p>
        ) : workspaces.length === 0 ? (
          <p className="muted">{empty.body}</p>
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

      <details className="profile-create-details">
        <summary>Add profile</summary>
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
        <button
          type="button"
          disabled={busy || !runtime}
          onClick={createWorkspace}
        >
          Add profile
        </button>
      </details>
    </section>
  );
}
