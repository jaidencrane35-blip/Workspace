import { useCallback, useEffect, useState } from "react";
import { HomeWorkspacePanel } from "./components/HomeWorkspacePanel";
import { PilotHelpPanel } from "./components/PilotHelpPanel";
import { PilotMeasurementPanel } from "./components/PilotMeasurementPanel";
import { ResumeContextPanel } from "./components/ResumeContextPanel";
import { SaveContextPanel } from "./components/SaveContextPanel";
import { invokeIpc } from "./lib/ipc";
import {
  PILOT_DEFAULT_VIEW,
  PILOT_PRIMARY_VIEWS,
  PILOT_VIEW_LABELS,
  type PilotPrimaryView,
} from "./lib/pilotChrome";
import type { Workspace } from "./types/domain";
import type {
  WorkspaceHealth,
  WorkspaceSettings,
  WorkspaceStatus,
} from "./types/workspace";

const LEGACY_WORKSPACE_ID_KEY = "workspace.active_id";

function formatError(err: unknown): string {
  if (err instanceof Error) {
    return err.message;
  }
  return String(err);
}

async function persistActiveWorkspaceId(id: string | null): Promise<void> {
  await invokeIpc<WorkspaceSettings>("update_settings", {
    update: {
      active_workspace_id: id ?? "",
    },
  });
  localStorage.removeItem(LEGACY_WORKSPACE_ID_KEY);
}

/**
 * Experience shell — companion-first chrome for Product Proof.
 *
 * Primary navigation: Home / Save / Continue / Check-in / Guide.
 * Engine surfaces remain in the codebase but are not default tabs.
 */
export default function App() {
  const [view, setView] = useState<PilotPrimaryView>(PILOT_DEFAULT_VIEW);
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [bootstrapped, setBootstrapped] = useState(false);
  const [busy, setBusy] = useState(false);
  const [focusContextId, setFocusContextId] = useState<string | null>(null);

  const onError = useCallback((next: string | null) => {
    setError(next);
  }, []);

  const onMessage = useCallback((next: string | null) => {
    setMessage(next);
  }, []);

  const activateWorkspace = useCallback(async (next: Workspace) => {
    setWorkspace(next);
    await persistActiveWorkspaceId(next.id);
  }, []);

  useEffect(() => {
    Promise.all([
      invokeIpc<WorkspaceStatus>("get_workspace_status"),
      invokeIpc<WorkspaceHealth>("get_workspace_health"),
      invokeIpc<WorkspaceSettings>("get_settings"),
    ])
      .then(async ([, , settings]) => {
        const storedId =
          settings.active_workspace_id?.trim() ||
          localStorage.getItem(LEGACY_WORKSPACE_ID_KEY);
        if (!storedId) {
          return;
        }
        const loaded = await invokeIpc<Workspace>("get_workspace", {
          id: storedId,
        });
        if (!settings.active_workspace_id) {
          await persistActiveWorkspaceId(loaded.id);
        } else {
          localStorage.removeItem(LEGACY_WORKSPACE_ID_KEY);
        }
        setWorkspace(loaded);
      })
      .catch((err: unknown) => {
        localStorage.removeItem(LEGACY_WORKSPACE_ID_KEY);
        setError(formatError(err));
      })
      .finally(() => setBootstrapped(true));
  }, []);

  const createWorkspace = () => {
    setBusy(true);
    setError(null);
    void (async () => {
      try {
        const created = await invokeIpc<Workspace>("create_workspace", {
          name: "My Workspace",
        });
        await activateWorkspace(created);
        setMessage("Your workspace is ready.");
        setView("home");
      } catch (err: unknown) {
        setError(formatError(err));
      } finally {
        setBusy(false);
      }
    })();
  };

  const goContinue = (contextId?: string) => {
    setFocusContextId(contextId ?? null);
    setView("resume");
  };

  return (
    <main className="app-shell exp-shell">
      <header className="app-chrome exp-chrome">
        <div className="exp-brand">
          <span className="exp-brand-mark" aria-hidden="true" />
          <h1>Workspace</h1>
        </div>
        <nav className="tabs exp-nav" aria-label="Workspace" role="tablist">
          {PILOT_PRIMARY_VIEWS.map((id) => (
            <button
              key={id}
              type="button"
              role="tab"
              className={view === id ? "tab active" : "tab"}
              aria-current={view === id ? "page" : undefined}
              aria-selected={view === id}
              onClick={() => {
                setFocusContextId(null);
                setView(id);
              }}
            >
              {PILOT_VIEW_LABELS[id]}
            </button>
          ))}
        </nav>
      </header>

      {error && (
        <p
          className="error banner"
          role="status"
          aria-live="assertive"
          aria-atomic="true"
        >
          {error}
        </p>
      )}
      {message && (
        <p
          className="ok banner"
          role="status"
          aria-live="polite"
          aria-atomic="true"
        >
          {message}
        </p>
      )}

      <div className="exp-body">
        {!bootstrapped ? (
          <p className="muted exp-loading">Opening your workspace…</p>
        ) : view === "home" ? (
          <HomeWorkspacePanel
            workspace={workspace}
            busy={busy}
            onCreateWorkspace={createWorkspace}
            onGoToSave={() => setView("save")}
            onGoToContinue={() => goContinue()}
            onContinueContext={(id) => goContinue(id)}
          />
        ) : view === "save" ? (
          <SaveContextPanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
            onCreateWorkspace={createWorkspace}
          />
        ) : view === "resume" ? (
          <ResumeContextPanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
            onGoToPilot={() => setView("pilot")}
            onGoHome={() => setView("home")}
            focusContextId={focusContextId}
          />
        ) : view === "pilot" ? (
          <PilotMeasurementPanel
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        ) : (
          <PilotHelpPanel />
        )}
      </div>
    </main>
  );
}
