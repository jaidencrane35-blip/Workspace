/**
 * Purpose: Desktop Reality Stage — spatial representation of observed windows.
 * Owner: Frontend product shell (Milestone R — first slice)
 * Inputs: optional workspace profile, registry apps, zone count, work mode,
 *   launch + navigate callbacks; get_workspace_state IPC
 * Outputs: Observed desktop map, Focus emphasis, optional library section
 * Dependencies: stageDesktopUi, layoutsStageUi, workMode, ipc, applicationsUi
 * Non-goals: OS geometry apply (G), arrangement editing (F), grouping (E),
 *   audio (H), new AI/engines, fake window thumbnails, create-workspace gate
 *
 * Hierarchy: Desktop reality → controls → Assistant (elsewhere).
 */

import { useCallback, useEffect, useMemo, useState } from "react";
import {
  applicationIdentityLine,
  applicationStatusLabel,
  canLaunchApplication,
} from "../lib/applicationsUi";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  layoutsStageCanvasNote,
  layoutsStageEmptyAppsCopy,
  layoutsStageEyebrow,
  layoutsStageFocusHint,
  layoutsStageFocusNote,
  layoutsStageFlowHint,
  layoutsStageRegistryHeading,
  layoutsStageRegistryNote,
  layoutsStageTitle,
} from "../lib/layoutsStageUi";
import { monogramFromName } from "../lib/productShellUi";
import {
  layoutStageDesktopWindows,
  stageDesktopEmptyCopy,
  stageDesktopMetaLine,
  type StageDesktopLoadState,
} from "../lib/stageDesktopUi";
import {
  partitionFocusApplications,
  workModeStageLede,
  type WorkMode,
} from "../lib/workMode";
import type {
  ApplicationReference,
  Workspace,
  WorkspaceState,
  WorkspaceStateWindow,
} from "../types/domain";
import { FocusSupportingAppChips } from "./FocusSupportingAppChips";
import { WorkModeSwitch } from "./WorkModeSwitch";

interface WorkspaceApplicationStageProps {
  workspace: Workspace | null;
  applications: ApplicationReference[];
  appsLoading: boolean;
  zoneCount: number;
  workMode: WorkMode;
  busy: boolean;
  onWorkModeChange: (mode: WorkMode) => void;
  onManageApplications: () => void;
  onLaunchApplication: (app: ApplicationReference) => void;
}

export function WorkspaceApplicationStage({
  workspace,
  applications,
  appsLoading,
  zoneCount,
  workMode,
  busy,
  onWorkModeChange,
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
  const [primaryId, setPrimaryId] = useState<string | null>(null);

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
          title ||
            (focused
              ? `Window ${focused.hwnd}`
              : null),
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
    if (applications.length === 0) {
      setPrimaryId(null);
      return;
    }
    setPrimaryId((prev) => {
      if (prev && applications.some((app) => app.id === prev)) {
        return prev;
      }
      return applications[0]?.id ?? null;
    });
  }, [applications]);

  const { primary, supporting } = partitionFocusApplications(
    applications,
    primaryId,
  );

  const emptyDesktop = stageDesktopEmptyCopy(loadState);
  const showDesktopMap = loadState === "ready" && tiles.length > 0;
  const focusedTile =
    tiles.find((tile) => tile.focused) ?? tiles[0] ?? null;
  const supportingTiles = focusedTile
    ? tiles.filter((tile) => tile.key !== focusedTile.key)
    : tiles;

  return (
    <section
      className={
        workMode === "focus"
          ? "workspace-application-stage mode-focus"
          : "workspace-application-stage mode-flow"
      }
      aria-label="Desktop reality stage"
      data-work-mode={workMode}
    >
      <header className="stage-hero stage-hero-with-mode">
        <div>
          <p className="arrangement-eyebrow">{layoutsStageEyebrow()}</p>
          <h2>{layoutsStageTitle(workspace?.name)}</h2>
          <p className="lede">{workModeStageLede(workMode)}</p>
        </div>
        <WorkModeSwitch
          mode={workMode}
          onChange={onWorkModeChange}
          density="stage"
        />
      </header>

      <div className="row stage-desktop-toolbar">
        <p className="stage-meta muted">
          {loadState === "ready"
            ? stageDesktopMetaLine({
                windowCount: windows.length,
                monitorCount,
                observationPassId,
                focusedTitle,
              })
            : emptyDesktop.title}
        </p>
        <button
          type="button"
          className="ghost"
          disabled={busy || loadState === "loading" || !runtime}
          onClick={() => {
            void refreshDesktop();
          }}
        >
          Refresh desktop
        </button>
      </div>

      {workMode === "flow" ? (
        <p className="stage-meta muted">{layoutsStageCanvasNote(zoneCount)}</p>
      ) : (
        <p className="stage-meta muted">{layoutsStageFocusNote()}</p>
      )}

      {showDesktopMap ? (
        workMode === "flow" ? (
          <div
            className="stage-desktop-map"
            aria-label="Observed desktop windows"
          >
            {tiles.map((tile) => (
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
                  {tile.processLabel}
                  {tile.monitorLabel ? ` · ${tile.monitorLabel}` : ""}
                  {tile.focused ? " · Focused" : ""}
                  {tile.minimized ? " · Minimized" : ""}
                </span>
              </article>
            ))}
          </div>
        ) : (
          <div
            className="focus-stage-layout"
            aria-label="Focus desktop reality stage"
          >
            {focusedTile ? (
              <article className="stage-tile stage-tile-primary">
                <p className="home-current-label">Focused on desktop</p>
                <span className="stage-tile-name">{focusedTile.title}</span>
                <span className="product-list-meta muted">
                  {focusedTile.processLabel}
                </span>
                <span className="stage-tile-status">
                  Observed window · {focusedTile.boundsLabel}
                </span>
              </article>
            ) : null}
            {supportingTiles.length > 0 ? (
              <ul
                className="focus-supporting-apps"
                aria-label="Other observed windows"
              >
                {supportingTiles.slice(0, 8).map((tile) => (
                  <li key={tile.key}>
                    <span className="focus-supporting-chip static">
                      {tile.title}
                    </span>
                  </li>
                ))}
              </ul>
            ) : null}
          </div>
        )
      ) : (
        <div className="arrangement-empty stage-empty">
          <h3>{emptyDesktop.title}</h3>
          <p className="muted">{emptyDesktop.body}</p>
          {runtime ? (
            <button
              type="button"
              className="ghost"
              disabled={busy || loadState === "loading"}
              onClick={() => {
                void refreshDesktop();
              }}
            >
              Retry observation
            </button>
          ) : null}
        </div>
      )}

      <div className="row stage-actions">
        <p className="muted stage-hint">
          {workMode === "flow"
            ? layoutsStageFlowHint()
            : layoutsStageFocusHint()}
        </p>
      </div>

      <section
        className="stage-registry-secondary"
        aria-label="Optional application library"
      >
        <div className="row section-heading-row">
          <h3>{layoutsStageRegistryHeading()}</h3>
          <button
            type="button"
            className="ghost"
            onClick={onManageApplications}
          >
            Applications
          </button>
        </div>
        <p className="muted">{layoutsStageRegistryNote()}</p>
        {!workspace ? (
          <p className="muted">
            A named profile is optional for library apps and saved arrangements.
            Desktop reality above does not require one.
          </p>
        ) : appsLoading ? (
          <p className="muted">Loading library…</p>
        ) : applications.length === 0 ? (
          <p className="muted">
            {registryEmpty.title}. {registryEmpty.body}
          </p>
        ) : workMode === "flow" ? (
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
        ) : (
          <div className="focus-stage-layout" aria-label="Focus library apps">
            {primary ? (
              <article className="stage-tile stage-tile-primary compact">
                <p className="home-current-label">Library primary</p>
                <span
                  className="application-monogram large"
                  aria-hidden="true"
                >
                  {monogramFromName(primary.name)}
                </span>
                <span className="stage-tile-name">{primary.name}</span>
                <button
                  type="button"
                  className="stage-tile-launch"
                  disabled={busy || !canLaunchApplication(primary)}
                  onClick={() => onLaunchApplication(primary)}
                >
                  Launch
                </button>
              </article>
            ) : null}
            {supporting.length > 0 ? (
              <FocusSupportingAppChips
                apps={supporting}
                onSelect={setPrimaryId}
                selectTitle="Make library primary in Focus"
              />
            ) : null}
          </div>
        )}
      </section>
    </section>
  );
}
