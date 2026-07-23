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

const WORKSPACE_ID_KEY = "workspace.active_id";

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

export default function App() {
  const [view, setView] = useState<AppView>("canvas");
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [zones, setZones] = useState<Zone[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [bootstrapped, setBootstrapped] = useState(false);

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

  useEffect(() => {
    Promise.all([
      invokeIpc<WorkspaceStatus>("get_workspace_status"),
      invokeIpc<WorkspaceHealth>("get_workspace_health"),
      invokeIpc<WorkspaceSettings>("get_settings"),
    ])
      .then(async () => {
        const storedId = localStorage.getItem(WORKSPACE_ID_KEY);
        if (!storedId) {
          return;
        }
        const loaded = await invokeIpc<Workspace>("get_workspace", {
          id: storedId,
        });
        setWorkspace(loaded);
        const context = await invokeIpc<WorkspaceContext>(
          "get_workspace_context",
          { workspaceId: loaded.id, limit: 200 },
        );
        setZones(zonesFromContext(loaded.id, context));
      })
      .catch((err: unknown) => {
        localStorage.removeItem(WORKSPACE_ID_KEY);
        setError(formatError(err));
      })
      .finally(() => setBootstrapped(true));
  }, []);

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
            Operator
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
            onError={(msg) => setError(msg)}
            onSaved={onLayoutSaved}
          />
        ) : (
          <div className="canvas-shell">
            <p className="muted">
              No active workspace. Open the Operator tab to create one and seed
              zones, then return here.
            </p>
          </div>
        )
      ) : (
        <div className="container">
          <p className="lede">
            Operator console — create workspace, seed zones, approve
            suggestions, bridge intent, execute, inspect outcomes.
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
