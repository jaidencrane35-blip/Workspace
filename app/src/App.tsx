import { useCallback, useEffect, useState, type ReactNode } from "react";
import { HomeWorkspacePanel } from "./components/HomeWorkspacePanel";
import { OperatorRoot } from "./components/operator/OperatorRoot";
import { PilotHelpPanel } from "./components/PilotHelpPanel";
import { PilotMeasurementPanel } from "./components/PilotMeasurementPanel";
import { ResumeContextPanel } from "./components/ResumeContextPanel";
import { SaveContextPanel } from "./components/SaveContextPanel";
import { isExperienceDemoActive } from "./demo/demoMode";
import { trackNavigate } from "./dev";
import { invokeIpc } from "./lib/ipc";
import {
  PILOT_DEFAULT_VIEW,
  PILOT_PRIMARY_VIEWS,
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

function isTransportLeak(message: string): boolean {
  return (
    /invoke/i.test(message) ||
    /Cannot read properties/i.test(message) ||
    /__TAURI__/i.test(message)
  );
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
 * Main shell (Modes 1–3) — conversation is the product.
 * Former Product Proof surfaces are Mode 3 specialized tools only (no dock IA).
 */
export default function App() {
  const [view, setView] = useState<PilotPrimaryView>(PILOT_DEFAULT_VIEW);
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [bootstrapped, setBootstrapped] = useState(false);
  const [busy, setBusy] = useState(false);
  const [focusContextId, setFocusContextId] = useState<string | null>(null);
  const demo = isExperienceDemoActive();

  const onError = useCallback((next: string | null) => {
    if (next && isExperienceDemoActive() && isTransportLeak(next)) {
      setError(null);
      return;
    }
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
        if (isExperienceDemoActive()) {
          setError(null);
          return;
        }
        const text = formatError(err);
        if (isTransportLeak(text)) {
          setError(null);
          return;
        }
        setError(text);
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
        setMessage("Workspace ready.");
        setView("save");
      } catch (err: unknown) {
        onError(formatError(err));
      } finally {
        setBusy(false);
      }
    })();
  };

  const goContinue = (contextId?: string) => {
    setFocusContextId(contextId ?? null);
    trackNavigate("resume", { commandId: "go_continue" });
    setView("resume");
  };

  const navigate = (
    next: PilotPrimaryView,
    opts?: { focusContextId?: string | null },
  ) => {
    if (!PILOT_PRIMARY_VIEWS.includes(next)) {
      return;
    }
    if (opts && "focusContextId" in opts) {
      setFocusContextId(opts.focusContextId ?? null);
    } else {
      setFocusContextId(null);
    }
    trackNavigate(next, { commandId: "dock_navigate" });
    setView(next);
  };

  const listMoments = useCallback(async () => {
    if (!workspace) {
      return [];
    }
    try {
      const list = await invokeIpc<Array<{ id: string; name: string }>>(
        "list_saved_contexts",
        {
          workspaceId: workspace.id,
        },
      );
      return list.map((item) => ({ id: item.id, name: item.name }));
    } catch {
      return [];
    }
  }, [workspace]);

  useEffect(() => {
    if (!demo) {
      return;
    }
    document.documentElement.dataset.experienceDemo = "on";
    return () => {
      delete document.documentElement.dataset.experienceDemo;
    };
  }, [demo]);

  let tool: ReactNode = <p className="muted exp-loading">Opening…</p>;
  if (bootstrapped) {
    if (view === "home") {
      tool = (
        <div className="op-tool">
          <HomeWorkspacePanel
            workspace={workspace}
            busy={busy}
            onCreateWorkspace={createWorkspace}
            onGoToSave={() => {
              trackNavigate("save", { commandId: "go_save" });
              setView("save");
            }}
            onContinueContext={(id) => goContinue(id)}
          />
        </div>
      );
    } else if (view === "save") {
      tool = (
        <div className="op-tool">
          <SaveContextPanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
            onCreateWorkspace={createWorkspace}
          />
        </div>
      );
    } else if (view === "resume") {
      tool = (
        <div className="op-tool">
          <ResumeContextPanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
            onGoToPilot={() => {
              trackNavigate("pilot", { commandId: "dock_navigate" });
              setView("pilot");
            }}
            onGoHome={() => {
              trackNavigate("home", { commandId: "dock_navigate" });
              setView("home");
            }}
            focusContextId={focusContextId}
          />
        </div>
      );
    } else if (view === "pilot") {
      tool = (
        <div className="op-tool">
          <PilotMeasurementPanel
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      );
    } else {
      tool = (
        <div className="op-tool">
          <PilotHelpPanel />
        </div>
      );
    }
  }

  const specializedSurface = (
    <div className="op-specialized">
      {(error || message) && (
        <div className="op-specialized__status" role="status" aria-atomic="true">
          {error && <p className="error banner ws-toast">{error}</p>}
          {message && !error && <p className="ok banner ws-toast">{message}</p>}
        </div>
      )}
      {tool}
    </div>
  );

  return (
    <OperatorRoot
      specializedSurface={specializedSurface}
      onNavigateProduct={navigate}
      listMoments={listMoments}
      activeSpecialized={view}
    />
  );
}
