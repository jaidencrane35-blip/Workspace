import { useCallback, useEffect, useState } from "react";
import { PilotHelpPanel } from "./components/PilotHelpPanel";
import { PilotMeasurementPanel } from "./components/PilotMeasurementPanel";
import { ResumeContextPanel } from "./components/ResumeContextPanel";
import { SaveContextPanel } from "./components/SaveContextPanel";
import { invokeIpc } from "./lib/ipc";
import {
  PILOT_DEFAULT_VIEW,
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
 * PP-P01D / PP-P01E — Pilot-safe chrome with consented measurement surface.
 *
 * Primary navigation is Save / Resume / Pilot / Help. Canvas, Work, Assistant,
 * and Diagnostic remain in the codebase but are not default pilot surfaces.
 */
export default function App() {
  const [view, setView] = useState<PilotPrimaryView>(PILOT_DEFAULT_VIEW);
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [bootstrapped, setBootstrapped] = useState(false);
  const [busy, setBusy] = useState(false);

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
          name: "Pilot Workspace",
        });
        await activateWorkspace(created);
        setMessage("Workspace created. You can save a context now.");
        setView("save");
      } catch (err: unknown) {
        setError(formatError(err));
      } finally {
        setBusy(false);
      }
    })();
  };

  return (
    <main className="app-shell">
      <header className="app-chrome">
        <h1>Workspace</h1>
        <nav className="tabs" aria-label="Pilot workspace views" role="tablist">
          <button
            type="button"
            role="tab"
            className={view === "save" ? "tab active" : "tab"}
            aria-current={view === "save" ? "page" : undefined}
            aria-selected={view === "save"}
            onClick={() => setView("save")}
          >
            Save
          </button>
          <button
            type="button"
            role="tab"
            className={view === "resume" ? "tab active" : "tab"}
            aria-current={view === "resume" ? "page" : undefined}
            aria-selected={view === "resume"}
            onClick={() => setView("resume")}
          >
            Resume
          </button>
          <button
            type="button"
            role="tab"
            className={view === "pilot" ? "tab active" : "tab"}
            aria-current={view === "pilot" ? "page" : undefined}
            aria-selected={view === "pilot"}
            onClick={() => setView("pilot")}
          >
            Pilot
          </button>
          <button
            type="button"
            role="tab"
            className={view === "help" ? "tab active" : "tab"}
            aria-current={view === "help" ? "page" : undefined}
            aria-selected={view === "help"}
            onClick={() => setView("help")}
          >
            Help
          </button>
        </nav>
      </header>

      {error && (
        <p
          className="error banner"
          role="status"
          aria-live="assertive"
          aria-atomic="true"
        >
          Error: {error}
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

      {view === "save" ? (
        <div className="container assistant-container">
          {bootstrapped ? (
            <SaveContextPanel
              workspace={workspace}
              busy={busy}
              onBusy={setBusy}
              onError={onError}
              onMessage={onMessage}
              onCreateWorkspace={createWorkspace}
            />
          ) : (
            <p className="muted">Loading…</p>
          )}
        </div>
      ) : view === "resume" ? (
        <div className="container assistant-container">
          {bootstrapped ? (
            <ResumeContextPanel
              workspace={workspace}
              busy={busy}
              onBusy={setBusy}
              onError={onError}
              onMessage={onMessage}
              onGoToPilot={() => setView("pilot")}
            />
          ) : (
            <p className="muted">Loading…</p>
          )}
        </div>
      ) : view === "pilot" ? (
        <div className="container assistant-container">
          <PilotMeasurementPanel
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      ) : (
        <div className="container assistant-container">
          <PilotHelpPanel />
        </div>
      )}
    </main>
  );
}
