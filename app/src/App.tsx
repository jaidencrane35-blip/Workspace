import { useCallback, useEffect, useState } from "react";
import { HomeWorkspacePanel } from "./components/HomeWorkspacePanel";
import { PilotHelpPanel } from "./components/PilotHelpPanel";
import { PilotMeasurementPanel } from "./components/PilotMeasurementPanel";
import { ResumeContextPanel } from "./components/ResumeContextPanel";
import { SaveContextPanel } from "./components/SaveContextPanel";
import { WorkspaceShell } from "./components/WorkspaceShell";
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
 * Product root — mounts one persistent WorkspaceShell.
 * Destinations swap as content inside the shell (never as separate pages).
 * Pilot chrome: Home / Save / Continue / Check-in / Guide via PILOT_PRIMARY_VIEWS.
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

  const navigate = (next: PilotPrimaryView) => {
    if (!PILOT_PRIMARY_VIEWS.includes(next)) {
      return;
    }
    setFocusContextId(null);
    setView(next);
  };

  useEffect(() => {
    document.title = `Workspace · ${PILOT_VIEW_LABELS[view]}`;
  }, [view]);

  let content = (
    <p className="muted exp-loading">Opening your workspace…</p>
  );
  if (bootstrapped) {
    if (view === "home") {
      content = (
        <HomeWorkspacePanel
          workspace={workspace}
          busy={busy}
          onCreateWorkspace={createWorkspace}
          onGoToSave={() => setView("save")}
          onGoToContinue={() => goContinue()}
          onContinueContext={(id) => goContinue(id)}
        />
      );
    } else if (view === "save") {
      content = (
        <SaveContextPanel
          workspace={workspace}
          busy={busy}
          onBusy={setBusy}
          onError={onError}
          onMessage={onMessage}
          onCreateWorkspace={createWorkspace}
        />
      );
    } else if (view === "resume") {
      content = (
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
      );
    } else if (view === "pilot") {
      content = (
        <PilotMeasurementPanel
          busy={busy}
          onBusy={setBusy}
          onError={onError}
          onMessage={onMessage}
        />
      );
    } else {
      content = <PilotHelpPanel />;
    }
  }

  const status = (
    <>
      {error && (
        <p className="error banner ws-toast" role="status" aria-atomic="true">
          {error}
        </p>
      )}
      {message && !error && (
        <p className="ok banner ws-toast" role="status" aria-atomic="true">
          {message}
        </p>
      )}
    </>
  );

  return (
    <WorkspaceShell view={view} onNavigate={navigate} status={status}>
      {content}
    </WorkspaceShell>
  );
}
