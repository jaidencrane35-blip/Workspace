/**
 * Purpose: Desktop Reality Stage — plane-first spatial desktop representation.
 * Owner: Frontend product shell (Product Contract V4)
 * Inputs: optional profile, registry apps, work mode, launch + navigate;
 *   get_workspace_state IPC
 * Outputs: Desktop plane with observed app objects; optional library details
 * Dependencies: stageDesktopUi, layoutsStageUi, ipc
 * Non-goals: OS geometry apply, fake windows, arrangements forms on Stage
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
  stageDesktopMetaLine,
  stageDesktopPlaneMessage,
  type StageDesktopLoadState,
} from "../lib/stageDesktopUi";
import type { WorkMode } from "../lib/workMode";
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
  onManageApplications: () => void;
  onLaunchApplication: (app: ApplicationReference) => void;
}

export function WorkspaceApplicationStage({
  workspace,
  applications,
  appsLoading,
  workMode,
  busy,
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
    tiles.find((tile) => tile.key === selectedKey) ?? null;
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

  const showDesktopMap = loadState === "ready" && tiles.length > 0;
  const planeCalm = !showDesktopMap;
  const planeMessage = stageDesktopPlaneMessage(loadState);

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
          tiles.map((tile) => {
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
                title={`${tile.title} — ${tile.boundsLabel}`}
                aria-pressed={selected}
                onClick={() => {
                  setSelectedKey((prev) =>
                    prev === tile.key ? null : tile.key,
                  );
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
          })
        )}
      </div>

      {selectedTile ? (
        <p className="stage-selection muted" aria-live="polite">
          {relatedKeys.size > 1
            ? `${selectedTile.title} · ${relatedKeys.size} windows`
            : selectedTile.title}
        </p>
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
                      title={
                        launchable
                          ? "Launch this application"
                          : "Add an executable path under Apps first"
                      }
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
