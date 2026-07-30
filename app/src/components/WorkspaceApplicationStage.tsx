/**
 * Purpose: Desktop Interaction Layer Stage — runtime objects, relationships, interaction.
 * Owner: Frontend product shell (Product Contract V6)
 * Inputs: optional profile, registry apps, work mode, launch + navigate;
 *   get_workspace_state / ensure_observation_freshness / focus_desktop_window /
 *   list_desktop_arrangements
 * Outputs: Spatial desktop objects with select≠activate, multi-select, keyboard;
 *   Flow relationships; Focus dock; arrangement working-set overlay
 * Dependencies: stageDesktopUi, layoutsStageUi, ipc, applicationLaunch helpers
 * Non-goals: Fake windows, Assistant-owned control, minimize APIs, OS geometry apply
 */

import { useCallback, useEffect, useMemo, useState, type KeyboardEvent } from "react";
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
  nextStageSelectionKey,
  organiseStageForWorkMode,
  primaryStageSelectionKey,
  relatedStageObjectKeys,
  replaceStageSelection,
  stageArrangementMemberKeys,
  stageDesktopMetaLine,
  stageDesktopPlaneMessage,
  toggleStageSelection,
  type StageDesktopLoadState,
  type StageDesktopWindowTile,
} from "../lib/stageDesktopUi";
import { refreshObservedWorkspaceState } from "../lib/workspaceStateClient";
import type { WorkMode } from "../lib/workMode";
import type {
  DesktopArrangement,
  DesktopWindowFocusResult,
} from "../types/desktopArrangement";
import type {
  ApplicationReference,
  DesktopWindowGroup,
  Workspace,
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
  /** Bump after restore/launch so Stage re-reads desktop reality. */
  observationEpoch?: number;
}

function matchLibraryApp(
  tile: StageDesktopWindowTile,
  applications: ApplicationReference[],
): ApplicationReference | null {
  const process = tile.title.replace(/\.exe$/i, "").toLowerCase();
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
  observationEpoch = 0,
}: WorkspaceApplicationStageProps) {
  const runtime = isIpcRuntimeAvailable();
  const registryEmpty = layoutsStageEmptyAppsCopy();
  const [loadState, setLoadState] = useState<StageDesktopLoadState>(
    runtime ? "loading" : "runtime_unavailable",
  );
  const [windows, setWindows] = useState<WorkspaceStateWindow[]>([]);
  const [windowGroups, setWindowGroups] = useState<DesktopWindowGroup[]>([]);
  const [monitorCount, setMonitorCount] = useState(0);
  const [focusedTitle, setFocusedTitle] = useState<string | null>(null);
  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const [arrangements, setArrangements] = useState<DesktopArrangement[]>([]);
  const [workingSetId, setWorkingSetId] = useState<string>("");

  const selectedKey = primaryStageSelectionKey(selectedKeys);

  const refreshDesktop = useCallback(async () => {
    if (!isIpcRuntimeAvailable()) {
      setLoadState("runtime_unavailable");
      setWindows([]);
      setWindowGroups([]);
      setMonitorCount(0);
      setFocusedTitle(null);
      return;
    }
    setLoadState("loading");
    try {
      const state = await refreshObservedWorkspaceState("workspace_stage");
      setWindows(state.windows);
      setWindowGroups(state.window_groups ?? []);
      setMonitorCount(state.metadata.monitor_count);
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
      setWindowGroups([]);
      setMonitorCount(0);
      setFocusedTitle(null);
      setLoadState("error");
    }
  }, []);

  const refreshArrangements = useCallback(async () => {
    if (!workspace || !isIpcRuntimeAvailable()) {
      setArrangements([]);
      setWorkingSetId("");
      return;
    }
    try {
      const listed = await invokeIpc<DesktopArrangement[]>(
        "list_desktop_arrangements",
        { workspaceId: workspace.id, limit: 50 },
      );
      setArrangements(listed);
      setWorkingSetId((prev) => {
        if (prev && listed.some((item) => item.id === prev)) {
          return prev;
        }
        return "";
      });
    } catch {
      setArrangements([]);
    }
  }, [workspace]);

  useEffect(() => {
    void refreshDesktop();
  }, [refreshDesktop, observationEpoch]);

  useEffect(() => {
    void refreshArrangements();
  }, [refreshArrangements, observationEpoch]);

  const tiles = useMemo(
    () => layoutStageDesktopWindows(windows),
    [windows],
  );

  useEffect(() => {
    setSelectedKeys((prev) => {
      const next = prev.filter((key) => tiles.some((tile) => tile.key === key));
      if (next.length === prev.length && next.every((key, i) => key === prev[i])) {
        return prev;
      }
      return next;
    });
  }, [tiles]);

  const selectedTile =
    tiles.find((tile) => tile.key === selectedKey) ??
    tiles.find((tile) => tile.focused) ??
    null;

  const relatedKeys = useMemo(
    () => relatedStageObjectKeys(tiles, selectedKey, workMode, windowGroups),
    [tiles, selectedKey, workMode, windowGroups],
  );

  const workingSet = arrangements.find((item) => item.id === workingSetId) ?? null;
  const workingSetKeys = useMemo(
    () => stageArrangementMemberKeys(tiles, workingSet?.entries),
    [tiles, workingSet],
  );

  const organisation = useMemo(
    () => organiseStageForWorkMode(windows, workMode, selectedKey, windowGroups),
    [windows, workMode, selectedKey, windowGroups],
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

  const saveSelectionAsWorkingSet = () => {
    if (!workspace) {
      onError("Create a profile under Profiles to save a working set.");
      return;
    }
    const hwnds = selectedKeys
      .map((key) => tiles.find((tile) => tile.key === key)?.hwnd)
      .filter((hwnd): hwnd is string => Boolean(hwnd));
    if (hwnds.length === 0) {
      onError("Select one or more windows first.");
      return;
    }
    const label =
      selectedKeys.length === 1
        ? selectedTile?.title || "Selection"
        : `Selection (${selectedKeys.length})`;
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const saved = await invokeIpc<DesktopArrangement>(
          "capture_desktop_arrangement",
          {
            workspaceId: workspace.id,
            name: label,
            description: "Working set from Stage selection",
            arrangementId: null,
            refreshObservation: false,
            memberHwnds: hwnds,
          },
        );
        setArrangements((prev) => {
          const without = prev.filter((item) => item.id !== saved.id);
          return [saved, ...without];
        });
        setWorkingSetId(saved.id);
        onMessage(
          `Saved working set “${saved.name}” · ${saved.entries.length} window${
            saved.entries.length === 1 ? "" : "s"
          }`,
        );
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

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

  const selectTile = (
    tile: StageDesktopWindowTile,
    mode: "replace" | "toggle",
  ) => {
    setSelectedKeys((prev) =>
      mode === "toggle"
        ? toggleStageSelection(prev, tile.key)
        : replaceStageSelection(tile.key),
    );
  };

  const activateTile = (tile: StageDesktopWindowTile) => {
    setSelectedKeys(replaceStageSelection(tile.key));
    void focusSelectedWindow(tile);
  };

  const onMapKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (planeCalm || mapTiles.length === 0) {
      return;
    }
    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      const next = nextStageSelectionKey(mapTiles, selectedKey, "next");
      if (next) {
        setSelectedKeys(replaceStageSelection(next));
      }
      return;
    }
    if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      const next = nextStageSelectionKey(mapTiles, selectedKey, "previous");
      if (next) {
        setSelectedKeys(replaceStageSelection(next));
      }
      return;
    }
    if (event.key === "Home") {
      event.preventDefault();
      const next = nextStageSelectionKey(mapTiles, selectedKey, "home");
      if (next) {
        setSelectedKeys(replaceStageSelection(next));
      }
      return;
    }
    if (event.key === "End") {
      event.preventDefault();
      const next = nextStageSelectionKey(mapTiles, selectedKey, "end");
      if (next) {
        setSelectedKeys(replaceStageSelection(next));
      }
      return;
    }
    if (event.key === "Enter" && selectedTile) {
      event.preventDefault();
      void focusSelectedWindow(selectedTile);
    }
  };

  const renderTile = (tile: StageDesktopWindowTile) => {
    const related = relatedKeys.has(tile.key);
    const selected = selectedKeys.includes(tile.key);
    const inWorkingSet = workingSetKeys.size > 0 && workingSetKeys.has(tile.key);
    const outsideWorkingSet =
      workingSetKeys.size > 0 && !workingSetKeys.has(tile.key);
    const className = [
      "stage-desktop-window",
      `relation-${tile.relationIndex}`,
      tile.focused ? "focused" : null,
      tile.minimized ? "minimized" : null,
      !tile.visible ? "hidden-object" : null,
      selected ? "selected" : null,
      related && !selected ? "related" : null,
      inWorkingSet ? "working-set-member" : null,
      outsideWorkingSet ? "working-set-outside" : null,
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
        title={`${tile.title} — click to select · double-click or Enter to focus`}
        aria-pressed={selected}
        disabled={busy}
        onClick={(event) => {
          selectTile(
            tile,
            event.metaKey || event.ctrlKey ? "toggle" : "replace",
          );
        }}
        onDoubleClick={() => {
          activateTile(tile);
        }}
      >
        <span className="stage-desktop-window-title">{tile.title}</span>
        <span className="stage-desktop-window-meta muted">
          {[
            tile.processLabel || null,
            tile.monitorLabel,
            tile.focused ? "Focused" : null,
            tile.minimized ? "Minimized" : null,
            !tile.visible ? "Hidden" : null,
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
            void refreshArrangements();
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
            focusedTitle,
            selectedCount: selectedKeys.length,
          })}
        </p>
      ) : null}

      {showDesktopMap && arrangements.length > 0 ? (
        <div className="stage-working-set-row">
          <label htmlFor="stage-working-set">
            Working set
            <select
              id="stage-working-set"
              value={workingSetId}
              disabled={busy}
              onChange={(event) => {
                setWorkingSetId(event.target.value);
              }}
            >
              <option value="">All observed windows</option>
              {arrangements.map((item) => (
                <option key={item.id} value={item.id}>
                  {item.name}
                </option>
              ))}
            </select>
          </label>
        </div>
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
        tabIndex={planeCalm ? undefined : 0}
        onKeyDown={onMapKeyDown}
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
                    selectTile(tile, "replace");
                  }
                }}
                onDoubleClick={() => {
                  const tile = tiles.find((item) => item.key === entry.tileKey);
                  if (tile) {
                    activateTile(tile);
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
            {selectedKeys.length > 1
              ? `${selectedKeys.length} selected · ${selectedTile.title}`
              : relatedKeys.size > 1
                ? `${selectedTile.title} · ${relatedKeys.size} related`
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
              Focus
            </button>
            <button
              type="button"
              className="ghost"
              disabled={busy || !runtime || !workspace}
              onClick={saveSelectionAsWorkingSet}
            >
              Save selection
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
