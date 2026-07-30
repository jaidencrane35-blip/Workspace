/**
 * Purpose: Desktop Reality Stage — spatial representation of the observed desktop.
 * Owner: Frontend product shell (Product Contract V3)
 * Inputs: optional profile, registry apps, work mode, launch + navigate callbacks;
 *   get_workspace_state IPC
 * Outputs: Desktop plane (empty or observed windows); optional library details
 * Dependencies: stageDesktopUi, layoutsStageUi, workMode, ipc
 * Non-goals: OS geometry apply, fake windows, companion canvas, new engines/AI
 *
 * Contract: Desktop Reality → Representation. Flow/Focus = density of same map.
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
  layoutsStageEyebrow,
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
      <header className="stage-hero stage-hero-compact">
        <div>
          <p className="arrangement-eyebrow">{layoutsStageEyebrow()}</p>
          <h2>{layoutsStageTitle(workspace?.name)}</h2>
        </div>
        <div className="stage-hero-controls">
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
        </div>
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
          tiles.map((tile) => (
            <article
              key={tile.key}
              className={
                tile.focused
                  ? "stage-desktop-window focused"
                  : tile.minimized
                    ? "stage-desktop-window minimized"
                    : "stage-desktop-window"
              }
              style={{
                left: `${tile.leftPct}%`,
                top: `${tile.topPct}%`,
                width: `${tile.widthPct}%`,
                height: `${tile.heightPct}%`,
              }}
              title={`${tile.title} — ${tile.boundsLabel}`}
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
            </article>
          ))
        )}
      </div>

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
            Applications
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
                          : "Add an executable path under Applications first"
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
