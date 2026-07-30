/**
 * Purpose: Product shell root — Desktop Reality Stage landing, view routing,
 *   Flow/Focus density, persistent Assistant companion rail.
 * Owner: Frontend product shell (Milestone R)
 * Inputs: Tauri IPC (settings, workspace, zones, applications, workspace state);
 *   local UI prefs
 * Outputs: Navigation, banners, chrome; Stage shows observed desktop without
 *   requiring a named workspace profile
 * Dependencies: Product panels, DesktopArrangementPanel, companion rail, prefs
 * Non-goals: OS geometry apply, grouping, audio, new AI/engines, setup-first gate
 */

import { useCallback, useEffect, useRef, useState } from "react";
import { ApplicationsPanel } from "./components/ApplicationsPanel";
import { AssistantCompanionRail } from "./components/AssistantCompanionRail";
import { CanvasShell } from "./components/CanvasShell";
import { DesktopArrangementPanel } from "./components/DesktopArrangementPanel";
import { OperatorConsole } from "./components/OperatorConsole";
import { WorkspaceApplicationStage } from "./components/WorkspaceApplicationStage";
import {
  WorkspaceHome,
  type ProductPrimaryView,
} from "./components/WorkspaceHome";
import { WorkspaceIntelligencePanel } from "./components/WorkspaceIntelligencePanel";
import { WorkspaceSwitcher } from "./components/WorkspaceSwitcher";
import { WorkModeSwitch } from "./components/WorkModeSwitch";
import {
  ASSISTANT_COMPANION_RAIL_ID,
  assistantRailToggleLabel,
} from "./lib/assistantRail";
import {
  launchRegisteredApplication,
  launchSuccessMessage,
} from "./lib/applicationLaunch";
import {
  invokeIpc,
  IpcRuntimeUnavailableError,
  isIpcRuntimeAvailable,
} from "./lib/ipc";
import {
  classifyBanner,
  ZONE_CONTEXT_LIMIT,
} from "./lib/productShellUi";
import { useAssistantRail } from "./lib/useAssistantRail";
import { useWorkMode } from "./lib/useWorkMode";
import type {
  ApplicationReference,
  Workspace,
  WorkspaceContext,
  Zone,
} from "./types/domain";
import type { Layout } from "./types/layout";
import type { WorkspaceSettings } from "./types/workspace";

const LEGACY_WORKSPACE_ID_KEY = "workspace.active_id";

type ToolView = "diagnostics" | "developer";
type AppView = ProductPrimaryView | ToolView;

function isPrimaryView(view: AppView): view is ProductPrimaryView {
  return (
    view === "home" ||
    view === "workspaces" ||
    view === "applications" ||
    view === "layouts"
  );
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
    limit: ZONE_CONTEXT_LIMIT,
  });
  return zonesFromContext(workspaceId, context);
}

export default function App() {
  // Milestone D: land on the Workspace stage (Layouts) — apps first.
  const [view, setView] = useState<AppView>("layouts");
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [zones, setZones] = useState<Zone[]>([]);
  const [homeApps, setHomeApps] = useState<ApplicationReference[]>([]);
  const [homeAppsLoading, setHomeAppsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [errorKind, setErrorKind] = useState<"error" | "runtime">("error");
  const [message, setMessage] = useState<string | null>(null);
  const [bootstrapped, setBootstrapped] = useState(false);
  const [busy, setBusy] = useState(false);
  const onStatus = useCallback((text: string) => {
    setMessage(text);
  }, []);
  const { workMode, onWorkModeChange } = useWorkMode(onStatus);
  // Rail toggle is visual (aria-pressed); avoid banner noise on every click.
  const { assistantRailOpen, onAssistantRailOpenChange } = useAssistantRail();
  const [lastPrimaryView, setLastPrimaryView] =
    useState<ProductPrimaryView>("layouts");
  const assistantToggleRef = useRef<HTMLButtonElement>(null);

  const navigatePrimary = useCallback((next: ProductPrimaryView) => {
    setLastPrimaryView(next);
    setView(next);
  }, []);

  const launchFromStage = useCallback(
    (app: ApplicationReference) => {
      setBusy(true);
      setError(null);
      void (async () => {
        try {
          const result = await launchRegisteredApplication(app);
          setMessage(launchSuccessMessage(result));
        } catch (err: unknown) {
          const classified = classifyBanner(err);
          setErrorKind(classified.kind === "runtime" ? "runtime" : "error");
          setError(classified.text);
        } finally {
          setBusy(false);
        }
      })();
    },
    [],
  );

  const setAssistantRailOpen = useCallback(
    (open: boolean) => {
      onAssistantRailOpenChange(open);
      if (!open) {
        // Return keyboard focus to the chrome control after Escape / Hide.
        queueMicrotask(() => assistantToggleRef.current?.focus());
      }
    },
    [onAssistantRailOpenChange],
  );

  const onWorkspaceChange = useCallback((next: Workspace | null) => {
    setWorkspace(next);
  }, []);

  const onZonesChange = useCallback((next: Zone[]) => {
    setZones(next);
  }, []);

  const onError = useCallback((next: string | null) => {
    if (!next) {
      setError(null);
      return;
    }
    const classified = classifyBanner(next);
    setErrorKind(classified.kind === "runtime" ? "runtime" : "error");
    setError(classified.text);
  }, []);

  const onMessage = useCallback((next: string | null) => {
    setMessage(next);
  }, []);

  const onLayoutSaved = useCallback((layout: Layout) => {
    setMessage(`Layout saved (${layout.nodes.length} nodes)`);
    setError(null);
  }, []);

  const refreshHomeApps = useCallback(async (workspaceId: string | null) => {
    if (!workspaceId || !isIpcRuntimeAvailable()) {
      setHomeApps([]);
      setHomeAppsLoading(false);
      return;
    }
    setHomeAppsLoading(true);
    try {
      const listed = await invokeIpc<ApplicationReference[]>(
        "list_applications",
        { workspaceId },
      );
      setHomeApps(listed);
    } catch {
      setHomeApps([]);
    } finally {
      setHomeAppsLoading(false);
    }
  }, []);

  const activateWorkspace = useCallback(
    async (next: Workspace) => {
      setWorkspace(next);
      await persistActiveWorkspaceId(next.id);
      const nextZones = await loadZones(next.id);
      setZones(nextZones);
      await refreshHomeApps(next.id);
    },
    [refreshHomeApps],
  );

  useEffect(() => {
    if (!isIpcRuntimeAvailable()) {
      setErrorKind("runtime");
      setError(
        classifyBanner(new IpcRuntimeUnavailableError()).text,
      );
      setBootstrapped(true);
      return;
    }

    invokeIpc<WorkspaceSettings>("get_settings")
      .then(async (settings) => {
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
        await refreshHomeApps(loaded.id);
      })
      .catch((err: unknown) => {
        localStorage.removeItem(LEGACY_WORKSPACE_ID_KEY);
        const classified = classifyBanner(err);
        setErrorKind(classified.kind === "runtime" ? "runtime" : "error");
        setError(classified.text);
      })
      .finally(() => setBootstrapped(true));
  }, [refreshHomeApps]);

  useEffect(() => {
    if (view !== "layouts" && view !== "home") {
      return;
    }
    void refreshHomeApps(workspace?.id ?? null);
  }, [view, workspace?.id, refreshHomeApps]);

  const createWorkspaceFromHome = () => {
    setBusy(true);
    setError(null);
    void (async () => {
      try {
        const created = await invokeIpc<Workspace>("create_workspace", {
          name: "My workspace",
        });
        await activateWorkspace(created);
        setMessage("Workspace created");
        setView("workspaces");
      } catch (err: unknown) {
        const classified = classifyBanner(err);
        setErrorKind(classified.kind === "runtime" ? "runtime" : "error");
        setError(classified.text);
      } finally {
        setBusy(false);
      }
    })();
  };

  const addZoneFromCanvas = () => {
    if (!workspace) {
      onError("Create a workspace first.");
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
        const classified = classifyBanner(err);
        setErrorKind(classified.kind === "runtime" ? "runtime" : "error");
        setError(classified.text);
      } finally {
        setBusy(false);
      }
    })();
  };

  const primaryTab = (id: ProductPrimaryView, label: string) => (
    <button
      type="button"
      role="tab"
      className={view === id ? "tab active" : "tab"}
      aria-current={view === id ? "page" : undefined}
      aria-selected={view === id}
      onClick={() => navigatePrimary(id)}
    >
      {label}
    </button>
  );

  const toolTab = (id: ToolView, label: string) => (
    <button
      type="button"
      role="tab"
      className={view === id ? "tab tool active" : "tab tool"}
      aria-current={view === id ? "page" : undefined}
      aria-selected={view === id}
      onClick={() => setView(id)}
    >
      {label}
    </button>
  );

  const showAssistantRail = isPrimaryView(view) && assistantRailOpen;

  const primaryStage = (() => {
    if (view === "home") {
      return (
        <div className="container product-container">
          <WorkspaceHome
            workspace={workspace}
            zoneCount={zones.length}
            applications={homeApps}
            appsLoading={homeAppsLoading}
            bootstrapped={bootstrapped}
            busy={busy}
            workMode={workMode}
            onNavigate={navigatePrimary}
            onCreateWorkspace={createWorkspaceFromHome}
          />
        </div>
      );
    }
    if (view === "workspaces") {
      return (
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
      );
    }
    if (view === "applications") {
      return (
        <div className="container product-container applications-wide">
          <ApplicationsPanel
            workspace={workspace}
            workMode={workMode}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      );
    }
    if (view === "layouts") {
      return (
        <div className="workspace-stage workspace-stage-apps-first">
          <div className="workspace-stage-main">
            {!bootstrapped ? (
              <div className="canvas-shell">
                <p className="muted">Loading…</p>
              </div>
            ) : (
              <>
                <WorkspaceApplicationStage
                  workspace={workspace}
                  applications={homeApps}
                  appsLoading={homeAppsLoading}
                  zoneCount={zones.length}
                  workMode={workMode}
                  busy={busy}
                  onWorkModeChange={onWorkModeChange}
                  onManageApplications={() => navigatePrimary("applications")}
                  onLaunchApplication={launchFromStage}
                />
                {workspace && workMode === "flow" ? (
                  <div className="stage-secondary-canvas">
                    <p className="stage-secondary-label muted">
                      Companion canvas (optional board practice)
                    </p>
                    <CanvasShell
                      workspaceId={workspace.id}
                      workspaceName={workspace.name}
                      zones={zones}
                      busy={busy}
                      onError={(msg) => onError(msg)}
                      onSaved={onLayoutSaved}
                      onCreateWorkspace={createWorkspaceFromHome}
                      onAddZone={addZoneFromCanvas}
                    />
                  </div>
                ) : null}
                {workspace && workMode === "focus" ? (
                  <div className="focus-canvas-suppressed" aria-live="polite">
                    <p className="muted">
                      Companion canvas is hidden in Focus. Switch to Flow to
                      edit zones. Desktop reality and arrangements are
                      unchanged.
                    </p>
                    <button
                      type="button"
                      className="ghost"
                      onClick={() => onWorkModeChange("flow")}
                    >
                      Return to Flow
                    </button>
                  </div>
                ) : null}
              </>
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
      );
    }
    if (view === "developer") {
      return (
        <div className="container assistant-container">
          <p className="lede">
            <span className="badge">Developer</span> Engineering presentation of
            work intelligence. Prefer <strong>Home</strong> and{" "}
            <strong>Applications</strong> for product use.
          </p>
          <WorkspaceIntelligencePanel
            workspace={workspace}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
          />
        </div>
      );
    }
    return (
      <div className="container">
        <p className="lede">
          <span className="badge">Diagnostics</span> Operator console for
          engineering validation. Prefer <strong>Home</strong>,{" "}
          <strong>Applications</strong>, and <strong>Layouts</strong> for
          product workflows.
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
    );
  })();

  return (
    <main className="app-shell" data-work-mode={workMode}>
      <header className="app-chrome">
        <div className="chrome-brand">
          <h1>Workspace</h1>
          <p className="chrome-tagline">Desktop workspace environment</p>
        </div>
        <div className="chrome-nav-groups">
          <nav
            className="tabs primary-tabs"
            aria-label="Primary workspace views"
            role="tablist"
          >
            {primaryTab("layouts", "Stage")}
            {primaryTab("applications", "Applications")}
            {primaryTab("workspaces", "Workspaces")}
            {primaryTab("home", "Home")}
          </nav>
          <WorkModeSwitch mode={workMode} onChange={onWorkModeChange} />
          <nav
            className="tabs tool-tabs"
            aria-label="Supporting tools"
          >
            <button
              ref={assistantToggleRef}
              type="button"
              className={
                showAssistantRail ? "tab tool active" : "tab tool"
              }
              aria-pressed={showAssistantRail}
              aria-controls={ASSISTANT_COMPANION_RAIL_ID}
              aria-label={assistantRailToggleLabel(assistantRailOpen)}
              title={assistantRailToggleLabel(assistantRailOpen)}
              onClick={() => {
                if (!isPrimaryView(view)) {
                  navigatePrimary(lastPrimaryView);
                  setAssistantRailOpen(true);
                  return;
                }
                setAssistantRailOpen(!assistantRailOpen);
              }}
            >
              Assistant
            </button>
            {toolTab("diagnostics", "Diagnostics")}
            {toolTab("developer", "Developer")}
          </nav>
        </div>
      </header>

      {error && (
        <p
          className={
            errorKind === "runtime" ? "runtime banner" : "error banner"
          }
          role="status"
          aria-live={errorKind === "runtime" ? "polite" : "assertive"}
          aria-atomic="true"
        >
          {errorKind === "runtime" ? error : `Error: ${error}`}
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

      <div
        className={
          showAssistantRail ? "app-body with-companion-rail" : "app-body"
        }
      >
        <div className="app-body-main">
          {isPrimaryView(view) && !assistantRailOpen ? (
            <div className="assistant-rail-reopen-bar">
              <button
                type="button"
                className="ghost"
                onClick={() => setAssistantRailOpen(true)}
              >
                Show Assistant companion
              </button>
            </div>
          ) : null}
          {primaryStage}
        </div>
        {showAssistantRail ? (
          <AssistantCompanionRail
            workspace={workspace}
            workMode={workMode}
            busy={busy}
            onBusy={setBusy}
            onError={onError}
            onMessage={onMessage}
            onCollapse={() => setAssistantRailOpen(false)}
          />
        ) : null}
      </div>
    </main>
  );
}
