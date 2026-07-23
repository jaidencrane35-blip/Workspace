import { useCallback, useEffect, useState } from "react";
import { CanvasShell } from "./components/CanvasShell";
import { OperatorConsole } from "./components/OperatorConsole";
import { invokeIpc } from "./lib/ipc";
import type { Workspace, WorkspaceContext, Zone } from "./types/domain";
import type { Layout } from "./types/layout";
import type {
  WorkspaceHealth,
  WorkspaceSettings,
  WorkspaceStatus,
} from "./types/workspace";

const LEGACY_WORKSPACE_ID_KEY = "workspace.active_id";

type AppView = "canvas" | "operator";

function formatError(err: unknown): string {
  if (err instanceof Error) {
    return err.message;
  }
  return String(err);
}

function zonesFromContext(
  workspaceId: string,
  context: WorkspaceContext,
): Zone[] {
  return context.snapshot.zones.map((z) => {
    const summary = z as {
      resource_ref?: { id?: string };
      name?: string;
    };
    return {
      id: String(summary.resource_ref?.id ?? ""),
      workspace_id: workspaceId,
      name: String(summary.name ?? ""),
      position_metadata: null,
    };
  });
}

async function persistActiveWorkspaceId(id: string | null): Promise<void> {
  await invokeIpc<WorkspaceSettings>("update_settings", {
    update: {
      active_workspace_id: id ?? "",
    },
  });
  localStorage.removeItem(LEGACY_WORKSPACE_ID_KEY);
}

async function loadZones(workspaceId: string): Promise<Zone[]> {
  const context = await invokeIpc<WorkspaceContext>("get_workspace_context", {
    workspaceId,
    limit: 200,
  });
  return zonesFromContext(workspaceId, context);
}

export default function App() {
  const [view, setView] = useState<AppView>("canvas");
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [zones, setZones] = useState<Zone[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [bootstrapped, setBootstrapped] = useState(false);
  const [busy, setBusy] = useState(false);

  const onWorkspaceChange = useCallback((next: Workspace | null) => {
    setWorkspace(next);
  }, []);

  const onZonesChange = useCallback((next: Zone[]) => {
    setZones(next);
  }, []);

  const onError = useCallback((next: string | null) => {
    setError(next);
  }, []);

  const onMessage = useCallback((next: string | null) => {
    setMessage(next);
  }, []);

  const onLayoutSaved = useCallback((layout: Layout) => {
    setMessage(`Layout saved (${layout.nodes.length} nodes)`);
    setError(null);
  }, []);

  const activateWorkspace = useCallback(async (next: Workspace) => {
    setWorkspace(next);
    await persistActiveWorkspaceId(next.id);
    const nextZones = await loadZones(next.id);
    setZones(nextZones);
  }, []);

  useEffect(() => {
    Promise.all([
      invokeIpc<WorkspaceStatus>("get_workspace_status"),
      invokeIpc<WorkspaceHealth>("get_workspace_health"),
      invokeIpc<WorkspaceSettings>("get_settings"),
    ])
      .then(async ([, , settings]) => {
        let storedId =
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
        setZones(await loadZones(loaded.id));
      })
      .catch((err: unknown) => {
        localStorage.removeItem(LEGACY_WORKSPACE_ID_KEY);
        setError(formatError(err));
      })
      .finally(() => setBootstrapped(true));
  }, []);

  const createWorkspaceFromCanvas = () => {
    setBusy(true);
    setError(null);
    void (async () => {
      try {
        const created = await invokeIpc<Workspace>("create_workspace", {
          name: "Canvas Workspace",
        });
        await activateWorkspace(created);
        setMessage("Workspace created");
      } catch (err: unknown) {
        setError(formatError(err));
      } finally {
        setBusy(false);
      }
    })();
  };

  const addZoneFromCanvas = () => {
    if (!workspace) {
      setError("Create a workspace first.");
      return;
    }
    setBusy(true);
    setError(null);
    void (async () => {
      try {
        const zone = await invokeIpc<Zone>("create_zone", {
          workspaceId: workspace.id,
          name: `Zone ${zones.length + 1}`,
          positionMetadata: null,
        });
        setZones((prev) => [...prev, zone]);
        setMessage(`Zone created: ${zone.name}`);
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
        <nav className="tabs" aria-label="Primary">
          <button
            type="button"
            className={view === "canvas" ? "tab active" : "tab"}
            onClick={() => setView("canvas")}
          >
            Canvas
          </button>
          <button
            type="button"
            className={view === "operator" ? "tab active" : "tab"}
            onClick={() => setView("operator")}
          >
            Diagnostic
          </button>
        </nav>
      </header>

      {error && <p className="error banner">Error: {error}</p>}
      {message && <p className="ok banner">{message}</p>}

      {view === "canvas" ? (
        !bootstrapped ? (
          <div className="canvas-shell">
            <p className="muted">Loading…</p>
          </div>
        ) : workspace ? (
          <CanvasShell
            workspaceId={workspace.id}
            workspaceName={workspace.name}
            zones={zones}
            busy={busy}
            onError={(msg) => setError(msg)}
            onSaved={onLayoutSaved}
            onCreateWorkspace={createWorkspaceFromCanvas}
            onAddZone={addZoneFromCanvas}
          />
        ) : (
          <div className="canvas-shell canvas-bootstrap">
            <p className="lede">
              No active workspace. Create one here to start arranging zones.
            </p>
            <button
              type="button"
              disabled={busy}
              onClick={createWorkspaceFromCanvas}
            >
              Create workspace
            </button>
          </div>
        )
      ) : (
        <div className="container">
          <p className="lede">
            <span className="badge">Diagnostic</span> Operator console — not the
            product shell. Use for pipeline inspection (suggestions → execute),
            settings, and desktop window enumeration. Prefer Canvas for layout
            work.
          </p>
          <OperatorConsole
            workspace={workspace}
            zones={zones}
            onWorkspaceChange={onWorkspaceChange}
            onZonesChange={onZonesChange}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      )}
    </main>
  );
}
