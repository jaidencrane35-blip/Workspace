import { useCallback, useEffect, useState } from "react";
import { ApplicationsPanel } from "./components/ApplicationsPanel";
import { AssistantIntelligencePanel } from "./components/AssistantIntelligencePanel";
import { AssistantPanel } from "./components/AssistantPanel";
import { CanvasShell } from "./components/CanvasShell";
import { DesktopArrangementPanel } from "./components/DesktopArrangementPanel";
import { OperatorConsole } from "./components/OperatorConsole";
import {
  WorkspaceHome,
  type ProductPrimaryView,
} from "./components/WorkspaceHome";
import { WorkspaceIntelligencePanel } from "./components/WorkspaceIntelligencePanel";
import { WorkspaceSwitcher } from "./components/WorkspaceSwitcher";
import { invokeIpc } from "./lib/ipc";
import type { Workspace, WorkspaceContext, Zone } from "./types/domain";
import type { Layout } from "./types/layout";
import type {
  WorkspaceHealth,
  WorkspaceSettings,
  WorkspaceStatus,
} from "./types/workspace";

const LEGACY_WORKSPACE_ID_KEY = "workspace.active_id";

type AppView =
  | ProductPrimaryView
  | "assistant"
  | "operator"
  | "work";

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
  const [view, setView] = useState<AppView>("home");
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

  const primaryTab = (
    id: ProductPrimaryView,
    label: string,
  ) => (
    <button
      type="button"
      role="tab"
      className={view === id ? "tab active" : "tab"}
      aria-current={view === id ? "page" : undefined}
      aria-selected={view === id}
      onClick={() => setView(id)}
    >
      {label}
    </button>
  );

  return (
    <main className="app-shell">
      <header className="app-chrome">
        <h1>Workspace</h1>
        <nav className="tabs" aria-label="Primary workspace views" role="tablist">
          {primaryTab("home", "Home")}
          {primaryTab("workspaces", "Workspaces")}
          {primaryTab("applications", "Applications")}
          {primaryTab("layouts", "Layouts")}
          <button
            type="button"
            role="tab"
            className={
              view === "assistant" ? "tab secondary active" : "tab secondary"
            }
            aria-current={view === "assistant" ? "page" : undefined}
            aria-selected={view === "assistant"}
            onClick={() => setView("assistant")}
          >
            Assistant
          </button>
          <button
            type="button"
            role="tab"
            className={view === "work" ? "tab quiet active" : "tab quiet"}
            aria-current={view === "work" ? "page" : undefined}
            aria-selected={view === "work"}
            onClick={() => setView("work")}
          >
            Work
          </button>
          <button
            type="button"
            role="tab"
            className={view === "operator" ? "tab quiet active" : "tab quiet"}
            aria-current={view === "operator" ? "page" : undefined}
            aria-selected={view === "operator"}
            onClick={() => setView("operator")}
          >
            Diagnostic
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

      {view === "home" ? (
        <div className="container product-container">
          <WorkspaceHome
            workspace={workspace}
            zoneCount={zones.length}
            bootstrapped={bootstrapped}
            busy={busy}
            onNavigate={setView}
            onCreateWorkspace={createWorkspaceFromCanvas}
          />
        </div>
      ) : view === "workspaces" ? (
        <div className="workspace-stage">
          <div className="workspace-stage-main">
            <WorkspaceSwitcher
              activeWorkspace={workspace}
              busy={busy}
              onBusy={setBusy}
              onError={onError}
              onMessage={onMessage}
              onActivate={activateWorkspace}
              onCreated={activateWorkspace}
            />
          </div>
          <DesktopArrangementPanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      ) : view === "applications" ? (
        <div className="container product-container">
          <ApplicationsPanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      ) : view === "layouts" ? (
        <div className="workspace-stage">
          <div className="workspace-stage-main">
            {!bootstrapped ? (
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
                  No active workspace. Create one here or use Workspaces to
                  switch to a saved environment.
                </p>
                <button
                  type="button"
                  disabled={busy}
                  onClick={createWorkspaceFromCanvas}
                >
                  Create workspace
                </button>
              </div>
            )}
          </div>
          <DesktopArrangementPanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      ) : view === "work" ? (
        <div className="container assistant-container">
          <WorkspaceIntelligencePanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      ) : view === "assistant" ? (
        <div className="container assistant-container assistant-intel-container">
          <p className="lede assistant-tool-note">
            Assistant is a supporting tool inside Workspace — not the primary
            product surface.
          </p>
          <AssistantIntelligencePanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
          <section
            className="assistant-legacy-section"
            aria-label="Governed workflow legacy"
          >
            <h2>Governed workflow (legacy)</h2>
            <p className="lede">
              Goal → plan → permission path for mutations. Prefer the
              intelligence panel above for read-only composition.
            </p>
            <AssistantPanel
              workspace={workspace}
              busy={busy}
              onBusy={setBusy}
              onError={onError}
              onMessage={onMessage}
            />
          </section>
        </div>
      ) : (
        <div className="container">
          <p className="lede">
            <span className="badge">Diagnostic</span> Operator console —
            validates Work intelligence and the governed Assistant pipeline.
            Prefer <strong>Home</strong>, <strong>Applications</strong>, and{" "}
            <strong>Layouts</strong> for product workflows.
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
