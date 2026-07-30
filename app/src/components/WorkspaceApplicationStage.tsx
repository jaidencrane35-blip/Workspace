/**
 * Purpose: Desktop Interaction Layer Stage — observe, select, focus, organise.
 * Owner: Frontend product shell (Product Contract V5)
 * Inputs: optional profile, registry apps, work mode, launch + navigate;
 *   get_workspace_state / ensure_observation_freshness / focus_desktop_window
 * Outputs: Spatial desktop objects with interaction; optional library
 * Dependencies: stageDesktopUi, layoutsStageUi, ipc, applicationLaunch helpers
 * Non-goals: Fake windows, Assistant-owned control, minimize APIs (deferred)
 */

import { useCallback, useEffect, useMemo, useState } from "react";
import {
  applicationIdentityLine,
  applicationStatusLabel,
  canLaunchApplication,
} from "../lib/applicationsUi";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  layoutsStageEmptyAppsCopy,
  layoutsStageRegistryHeading,
  layoutsStageTitle,
} from "../lib/layoutsStageUi";
import { monogramFromName } from "../lib/productShellUi";
import {
  layoutStageDesktopWindows,
  organiseStageForWorkMode,
  stageDesktopMetaLine,
  stageDesktopPlaneMessage,
  type StageDesktopLoadState,
  type StageDesktopWindowTile,
} from "../lib/stageDesktopUi";
import type { WorkMode } from "../lib/workMode";
import type { DesktopWindowFocusResult } from "../types/desktopArrangement";
import type {
  ApplicationReference,
  Workspace,
  WorkspaceState,
  WorkspaceStateWindow,
} from "../types/domain";

interface WorkspaceApplicationStageProps {
  workspace: Workspace | null;
  applications: ApplicationReference[];
  appsLoading: boolean;
  workMode: WorkMode;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  onManageApplications: () => void;
  onLaunchApplication: (app: ApplicationReference) => void;
}

function matchLibraryApp(
  tile: StageDesktopWindowTile,
  applications: ApplicationReference[],
): ApplicationReference | null {
  const process = tile.processKey.replace(/\.exe$/i, "").toLowerCase();
  if (!process) {
    return null;
  }
  return (
    applications.find((app) => {
      const name = app.name.trim().toLowerCase();
      const id = app.identifier?.trim().toLowerCase() ?? "";
      const exe = app.executable_path?.trim().toLowerCase() ?? "";
      return (
        name.includes(process) ||
        process.includes(name) ||
        id === process ||
        exe.includes(process)
      );
    }) ?? null
  );
}

export function WorkspaceApplicationStage({
  workspace,
  applications,
  appsLoading,
  workMode,
  busy,
  onBusy,
  onError,
  onMessage,
  onManageApplications,
  onLaunchApplication,
}: WorkspaceApplicationStageProps) {
  const runtime = isIpcRuntimeAvailable();
  const registryEmpty = layoutsStageEmptyAppsCopy();
  const [loadState, setLoadState] = useState<StageDesktopLoadState>(
    runtime ? "loading" : "runtime_unavailable",
  );
  const [windows, setWindows] = useState<WorkspaceStateWindow[]>([]);
  const [monitorCount, setMonitorCount] = useState(0);
  const [observationPassId, setObservationPassId] = useState<string | null>(
    null,
  );
  const [focusedTitle, setFocusedTitle] = useState<string | null>(null);
  const [selectedKey, setSelectedKey] = useState<string | null>(null);

  const refreshDesktop = useCallback(async () => {
    if (!isIpcRuntimeAvailable()) {
      setLoadState("runtime_unavailable");
      setWindows([]);
      setMonitorCount(0);
      setObservationPassId(null);
      setFocusedTitle(null);
      return;
    }
    setLoadState("loading");
    try {
      try {
        await invokeIpc("ensure_observation_freshness", {
          consumerId: "workspace_stage",
        });
      } catch {
        // Freshness is best-effort; still read projected state.
      }
      const state = await invokeIpc<WorkspaceState>("get_workspace_state");
      setWindows(state.windows);
      setMonitorCount(state.metadata.monitor_count);
      setObservationPassId(state.metadata.observation_pass_id);
      {
        const focused = state.focused_window;
        const title = focused?.title.trim();
        setFocusedTitle(
          title || (focused ? `Window ${focused.hwnd}` : null),
        );
      }
      setLoadState("ready");
    } catch {
      setWindows([]);
      setMonitorCount(0);
      setObservationPassId(null);
      setFocusedTitle(null);
      setLoadState("error");
    }
  }, []);

  useEffect(() => {
    void refreshDesktop();
  }, [refreshDesktop]);

  const tiles = useMemo(
    () => layoutStageDesktopWindows(windows),
    [windows],
  );

  useEffect(() => {
    if (selectedKey && !tiles.some((tile) => tile.key === selectedKey)) {
      setSelectedKey(null);
    }
  }, [tiles, selectedKey]);

  const selectedTile =
    tiles.find((tile) => tile.key === selectedKey) ??
    tiles.find((tile) => tile.focused) ??
    null;

  const relatedKeys = useMemo(() => {
    if (!selectedTile?.processKey) {
      return new Set<string>();
    }
    return new Set(
      tiles
        .filter((tile) => tile.processKey === selectedTile.processKey)
        .map((tile) => tile.key),
    );
  }, [tiles, selectedTile]);

  const organisation = useMemo(
    () => organiseStageForWorkMode(windows, workMode, selectedKey),
    [windows, workMode, selectedKey],
  );

  const mapTiles = useMemo(() => {
    if (workMode === "focus") {
      return layoutStageDesktopWindows(organisation.mapWindows);
    }
    return tiles;
  }, [workMode, organisation.mapWindows, tiles]);

  const showDesktopMap = loadState === "ready" && tiles.length > 0;
  const planeCalm = !showDesktopMap;
  const planeMessage = stageDesktopPlaneMessage(loadState);
  const matchedLibrary = selectedTile
    ? matchLibraryApp(selectedTile, applications)
    : null;

  const focusSelectedWindow = async (tile: StageDesktopWindowTile) => {
    if (!runtime) {
      onError("Open the desktop app to focus windows.");
      return;
    }
    onBusy(true);
    onError(null);
    try {
      const result = await invokeIpc<DesktopWindowFocusResult>(
        "focus_desktop_window",
        { hwnd: tile.hwnd },
      );
      onMessage(
        result.simulated
          ? `Focused ${tile.title} (simulated)`
          : `Focused ${tile.title}`,
      );
      await refreshDesktop();
    } catch (err: unknown) {
      onError(err instanceof Error ? err.message : String(err));
    } finally {
      onBusy(false);
    }
  };

  const onTileActivate = (tile: StageDesktopWindowTile) => {
    setSelectedKey(tile.key);
    void focusSelectedWindow(tile);
  };

  const renderTile = (tile: StageDesktopWindowTile) => {
    const related = relatedKeys.has(tile.key);
    const selected = tile.key === selectedKey;
    const className = [
      "stage-desktop-window",
      `relation-${tile.relationIndex}`,
      tile.focused ? "focused" : null,
      tile.minimized ? "minimized" : null,
      selected ? "selected" : null,
      related && !selected ? "related" : null,
    ]
      .filter(Boolean)
      .join(" ");
    return (
      <button
        key={tile.key}
        type="button"
        className={className}
        style={{
          left: `${tile.leftPct}%`,
          top: `${tile.topPct}%`,
          width: `${tile.widthPct}%`,
          height: `${tile.heightPct}%`,
        }}
        title={`${tile.title} — click to focus`}
        aria-pressed={selected}
        disabled={busy}
        onClick={() => {
          onTileActivate(tile);
        }}
      >
        <span className="stage-desktop-window-title">{tile.title}</span>
        <span className="stage-desktop-window-meta muted">
          {[
            tile.processLabel || null,
            tile.monitorLabel,
            tile.focused ? "Focused" : null,
            tile.minimized ? "Minimized" : null,
          ]
            .filter(Boolean)
            .join(" · ")}
        </span>
      </button>
    );
  };

  const stageClass = [
    "workspace-application-stage",
    workMode === "focus" ? "mode-focus" : "mode-flow",
    planeCalm ? "stage-plane-calm" : "stage-plane-live",
  ].join(" ");

  return (
    <section
      className={stageClass}
      aria-label="Desktop reality stage"
      data-work-mode={workMode}
      data-stage-plane={planeCalm ? "calm" : "live"}
    >
      <header className="stage-hero stage-hero-minimal">
        <h2>{layoutsStageTitle(workspace?.name)}</h2>
        <button
          type="button"
          className="ghost stage-refresh"
          disabled={busy || loadState === "loading" || !runtime}
          onClick={() => {
            void refreshDesktop();
          }}
        >
          Refresh
        </button>
      </header>

      {showDesktopMap ? (
        <p className="stage-meta stage-meta-live muted">
          {stageDesktopMetaLine({
            windowCount: windows.length,
            monitorCount,
            observationPassId,
            focusedTitle,
          })}
        </p>
      ) : null}

      <div
        className={
          planeCalm
            ? "stage-desktop-map stage-desktop-plane empty"
            : workMode === "focus"
              ? "stage-desktop-map stage-focus-primary"
              : "stage-desktop-map"
        }
        aria-label={
          planeCalm ? "Desktop surface" : "Observed desktop windows"
        }
      >
        {planeCalm ? (
          <div className="stage-desktop-plane-message">
            <p>{planeMessage}</p>
            {runtime && loadState !== "loading" ? (
              <button
                type="button"
                className="ghost"
                disabled={busy}
                onClick={() => {
                  void refreshDesktop();
                }}
              >
                Retry
              </button>
            ) : null}
          </div>
        ) : (
          mapTiles.map(renderTile)
        )}
      </div>

      {workMode === "focus" && organisation.dockEntries.length > 0 ? (
        <ul className="stage-process-dock" aria-label="Other applications">
          {organisation.dockEntries.map((entry) => (
            <li key={entry.processKey}>
              <button
                type="button"
                className="stage-dock-object"
                disabled={busy}
                title={
                  entry.windowCount > 1
                    ? `${entry.label} · ${entry.windowCount} windows`
                    : entry.label
                }
                onClick={() => {
                  const tile = tiles.find((item) => item.key === entry.tileKey);
                  if (tile) {
                    onTileActivate(tile);
                  }
                }}
              >
                <span className="application-monogram" aria-hidden="true">
                  {monogramFromName(entry.label)}
                </span>
                <span className="stage-dock-name">{entry.label}</span>
                {entry.windowCount > 1 ? (
                  <span className="stage-dock-count muted">
                    {entry.windowCount}
                  </span>
                ) : null}
              </button>
            </li>
          ))}
        </ul>
      ) : null}

      {selectedTile ? (
        <div className="stage-interaction-bar" aria-live="polite">
          <p className="stage-selection muted">
            {relatedKeys.size > 1
              ? `${selectedTile.title} · ${relatedKeys.size} windows`
              : selectedTile.title}
          </p>
          <div className="row">
            <button
              type="button"
              disabled={busy || !runtime}
              onClick={() => {
                void focusSelectedWindow(selectedTile);
              }}
            >
              {selectedTile.minimized ? "Restore" : "Focus"}
            </button>
            {matchedLibrary && canLaunchApplication(matchedLibrary) ? (
              <button
                type="button"
                className="ghost"
                disabled={busy}
                onClick={() => onLaunchApplication(matchedLibrary)}
              >
                Launch {matchedLibrary.name}
              </button>
            ) : null}
          </div>
        </div>
      ) : null}

      <details className="stage-registry-secondary quiet">
        <summary>
          <span>{layoutsStageRegistryHeading()}</span>
          <button
            type="button"
            className="ghost"
            onClick={(event) => {
              event.preventDefault();
              onManageApplications();
            }}
          >
            Apps
          </button>
        </summary>
        {!workspace ? (
          <p className="muted">Optional.</p>
        ) : appsLoading ? (
          <p className="muted">Loading…</p>
        ) : applications.length === 0 ? (
          <p className="muted">{registryEmpty.title}</p>
        ) : (
          <ul className="stage-tile-grid compact" aria-label="Library apps">
            {applications.map((app) => {
              const launchable = canLaunchApplication(app);
              return (
                <li key={app.id}>
                  <article className="stage-tile">
                    <span
                      className="application-monogram large"
                      aria-hidden="true"
                    >
                      {monogramFromName(app.name)}
                    </span>
                    <span className="stage-tile-name">{app.name}</span>
                    <span className="product-list-meta muted">
                      {applicationIdentityLine(app)}
                    </span>
                    <span className="stage-tile-status">
                      {applicationStatusLabel(app)}
                    </span>
                    <button
                      type="button"
                      className="stage-tile-launch"
                      disabled={busy || !launchable}
                      onClick={() => onLaunchApplication(app)}
                    >
                      Launch
                    </button>
                  </article>
                </li>
              );
            })}
          </ul>
        )}
      </details>
    </section>
  );
}
