/**
 * Purpose: Desktop surface — runtime objects, Arrangement Restore + layout editing.
 * Owner: Frontend product shell (Product Contract V6 / Programme I IC4)
 * Inputs: optional profile, registry apps, work mode, launch + navigate;
 *   WorkspaceState via refreshObservedWorkspaceState; focus_desktop_window;
 *   list/capture/restore_desktop_arrangement
 * Outputs: Spatial desktop objects; Flow/Focus; Arrangement select + Restore;
 *   layout edit session (lifecycle, change awareness, preview ghosts, update);
 *   subtle runtime awareness (attention primary, semantic roles)
 * Dependencies: stageDesktopUi, layoutsStageUi, desktopLayoutEditing, ipc
 * Non-goals: Fake windows, Assistant-owned control, minimize APIs, new set_bounds
 *   product IPC, canvas Layout HWND store, new persistence, parallel authorities
 */

import { useCallback, useEffect, useMemo, useState, type KeyboardEvent } from "react";
import {
  applicationIdentityLine,
  applicationStatusLabel,
  canLaunchApplication,
} from "../lib/applicationsUi";
import {
  diffLayoutEditingChanges,
  layoutArrangementPreviewGhosts,
  layoutEditingBanner,
  layoutEditingHint,
  layoutEditingPhase,
  layoutEditingProposedWindows,
  layoutEditingStatusMeta,
  layoutEditingUpdateConfirmation,
  layoutEditingWorkflowLine,
} from "../lib/desktopLayoutEditing";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  layoutsStageEmptyAppsCopy,
  layoutsStageRegistryHeading,
  layoutsStageTitle,
  workspaceDesktopWorkflowLine,
} from "../lib/layoutsStageUi";
import { monogramFromName } from "../lib/productShellUi";
import {
  layoutStageDesktopWindows,
  nextStageSelectionKey,
  organiseStageForWorkMode,
  primaryStageSelectionKey,
  relatedStageObjectKeys,
  replaceStageSelection,
  sortStageTilesByZOrder,
  stageArrangementMemberKeys,
  stageAttentionAwarenessLine,
  stageAttentionPrimaryKeys,
  stageContinuityAwarenessLine,
  stageContinuityByKey,
  stageDeltaOpenedKeys,
  stageDesktopMetaLine,
  stageDesktopPlaneMessage,
  stageDesktopWindowKey,
  stageFocusPreferredKeys,
  stageSemanticRoleByKey,
  toggleStageSelection,
  type StageDesktopLoadState,
  type StageDesktopWindowTile,
} from "../lib/stageDesktopUi";
import { useObservedWorkspaceState } from "../lib/useObservedWorkspaceState";
import type { WorkMode } from "../lib/workMode";
import type {
  DesktopArrangement,
  DesktopArrangementRestoreResult,
  DesktopWindowFocusResult,
} from "../types/desktopArrangement";
import type {
  ApplicationReference,
  Workspace,
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
  /** Notify shell after Restore mutates the desktop (same path as arrangements rail). */
  onDesktopChanged?: () => void;
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
  onDesktopChanged,
}: WorkspaceApplicationStageProps) {
  const runtime = isIpcRuntimeAvailable();
  const registryEmpty = layoutsStageEmptyAppsCopy();
  const [loadState, setLoadState] = useState<StageDesktopLoadState>(
    runtime ? "loading" : "runtime_unavailable",
  );
  const { workspaceState, refreshWorkspaceState } = useObservedWorkspaceState();
  const [selectedKeys, setSelectedKeys] = useState<string[]>([]);
  const [arrangements, setArrangements] = useState<DesktopArrangement[]>([]);
  const [workingSetId, setWorkingSetId] = useState<string>("");
  const [layoutEditing, setLayoutEditing] = useState(false);
  const [layoutPreview, setLayoutPreview] = useState(false);
  /** Ephemeral confirmation after Update — cleared on exit; not persistence. */
  const [updateConfirmation, setUpdateConfirmation] = useState<string | null>(
    null,
  );

  const windows = workspaceState?.windows ?? [];
  const windowGroups = workspaceState?.window_groups ?? [];
  const monitors = workspaceState?.monitors ?? [];
  const monitorCount =
    monitors.length || workspaceState?.metadata.monitor_count || 0;
  const focusedTitle = (() => {
    const focused = workspaceState?.focused_window;
    if (!focused) {
      return null;
    }
    const title = focused.title.trim();
    return title || `Window ${focused.hwnd}`;
  })();
  const attentionPrimaryKeys = useMemo(
    () => stageAttentionPrimaryKeys(workspaceState?.attention),
    [workspaceState?.attention],
  );
  const semanticRoles = useMemo(
    () => stageSemanticRoleByKey(workspaceState?.semantics),
    [workspaceState?.semantics],
  );
  const focusPreferredKeys = useMemo(
    () =>
      stageFocusPreferredKeys(
        workspaceState?.attention,
        workspaceState?.semantics,
      ),
    [workspaceState?.attention, workspaceState?.semantics],
  );
  const awarenessLine = (() => {
    const attentionLine = stageAttentionAwarenessLine(
      workspaceState?.attention,
    );
    const continuityLine = stageContinuityAwarenessLine(
      workspaceState?.runtime_memory,
      workspaceState?.latest_delta,
    );
    if (attentionLine && continuityLine) {
      return `${attentionLine} · ${continuityLine}`;
    }
    return attentionLine || continuityLine;
  })();
  const continuityByKey = useMemo(
    () => stageContinuityByKey(workspaceState?.runtime_memory),
    [workspaceState?.runtime_memory],
  );
  const deltaOpenedKeys = useMemo(
    () => stageDeltaOpenedKeys(workspaceState?.latest_delta),
    [workspaceState?.latest_delta],
  );

  const selectedKey = primaryStageSelectionKey(selectedKeys);

  const refreshDesktop = useCallback(async () => {
    if (!isIpcRuntimeAvailable()) {
      setLoadState("runtime_unavailable");
      return;
    }
    setLoadState("loading");
    try {
      await refreshWorkspaceState("workspace_stage");
      setLoadState("ready");
    } catch {
      setLoadState("error");
    }
  }, [refreshWorkspaceState]);

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
    () => layoutStageDesktopWindows(windows, monitors),
    [windows, monitors],
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
    () =>
      relatedStageObjectKeys(
        tiles,
        selectedKey,
        workMode,
        windowGroups,
        workspaceState?.semantics,
      ),
    [tiles, selectedKey, workMode, windowGroups, workspaceState?.semantics],
  );

  const workingSet = arrangements.find((item) => item.id === workingSetId) ?? null;
  const workingSetKeys = useMemo(
    () => stageArrangementMemberKeys(tiles, workingSet?.entries),
    [tiles, workingSet],
  );

  useEffect(() => {
    if (!workingSetId) {
      setLayoutEditing(false);
      setLayoutPreview(false);
      setUpdateConfirmation(null);
    }
  }, [workingSetId]);

  const editingChangeDiff = useMemo(() => {
    if (!layoutEditing || !workingSet) {
      return null;
    }
    const proposed = layoutEditingProposedWindows(
      windows,
      selectedKeys,
      stageDesktopWindowKey,
    );
    return diffLayoutEditingChanges(workingSet.entries, proposed);
  }, [layoutEditing, workingSet, windows, selectedKeys]);

  const editingPhase = layoutEditingPhase(
    layoutEditing,
    Boolean(editingChangeDiff?.hasChanges),
  );

  const previewGhosts = useMemo(() => {
    if (!layoutEditing || !layoutPreview || !workingSet) {
      return [];
    }
    return layoutArrangementPreviewGhosts(
      workingSet.entries,
      monitors,
      windows,
    );
  }, [layoutEditing, layoutPreview, workingSet, monitors, windows]);

  const organisation = useMemo(
    () =>
      organiseStageForWorkMode(
        windows,
        workMode,
        selectedKey,
        windowGroups,
        monitors,
        focusPreferredKeys,
      ),
    [windows, workMode, selectedKey, windowGroups, monitors, focusPreferredKeys],
  );

  const mapTiles = useMemo(() => {
    const base =
      workMode === "focus"
        ? layoutStageDesktopWindows(organisation.mapWindows, monitors)
        : tiles;
    return sortStageTilesByZOrder(base);
  }, [workMode, organisation.mapWindows, tiles, monitors]);

  const showDesktopMap = loadState === "ready" && tiles.length > 0;
  const planeCalm = !showDesktopMap;
  const planeMessage = stageDesktopPlaneMessage(loadState);
  const matchedLibrary = selectedTile
    ? matchLibraryApp(selectedTile, applications)
    : null;

  const saveSelectionAsWorkingSet = () => {
    if (!workspace) {
      onError("Choose a Profile to save an Arrangement.");
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
            description: "Arrangement from Desktop selection",
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
          `Arrangement saved “${saved.name}” · ${saved.entries.length} window${
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

  const restoreSelectedArrangement = () => {
    if (!workingSet) {
      onError("Choose an Arrangement to Restore.");
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const result = await invokeIpc<DesktopArrangementRestoreResult>(
          "restore_desktop_arrangement",
          {
            arrangementId: workingSet.id,
            focusFirst: true,
          },
        );
        onDesktopChanged?.();
        await refreshDesktop();
        onMessage(
          result.outcomes.some((outcome) => outcome.simulated)
            ? `Restored ${workingSet.name} (simulated)`
            : `Restored ${workingSet.name}`,
        );
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  const enterLayoutEditing = () => {
    if (!workingSet) {
      onError("Choose an Arrangement to edit.");
      return;
    }
    const memberKeys = [...workingSetKeys];
    if (memberKeys.length > 0) {
      setSelectedKeys(memberKeys);
    }
    setLayoutEditing(true);
    setLayoutPreview(true);
    setUpdateConfirmation(null);
    onMessage(null);
    onError(null);
  };

  const exitLayoutEditing = () => {
    setLayoutEditing(false);
    setLayoutPreview(false);
    setUpdateConfirmation(null);
  };

  const updateEditingArrangement = () => {
    if (!workspace || !workingSet) {
      onError("Choose an Arrangement to update.");
      return;
    }
    if (!editingChangeDiff?.hasChanges) {
      onMessage("No effective changes to update.");
      return;
    }
    const hwnds = selectedKeys
      .map((key) => tiles.find((tile) => tile.key === key)?.hwnd)
      .filter((hwnd): hwnd is string => Boolean(hwnd));
    const confirmation = layoutEditingUpdateConfirmation(editingChangeDiff);
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const saved = await invokeIpc<DesktopArrangement>(
          "capture_desktop_arrangement",
          {
            workspaceId: workspace.id,
            name: workingSet.name,
            description: workingSet.description || null,
            arrangementId: workingSet.id,
            refreshObservation: true,
            ...(hwnds.length > 0 ? { memberHwnds: hwnds } : {}),
          },
        );
        setArrangements((prev) => {
          const without = prev.filter((item) => item.id !== saved.id);
          return [saved, ...without];
        });
        setWorkingSetId(saved.id);
        setUpdateConfirmation(confirmation);
        await refreshDesktop();
        onMessage(`Updated “${saved.name}” · ${confirmation}`);
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
    const attentionPrimary = attentionPrimaryKeys.has(tile.key);
    const semanticRole = semanticRoles.get(tile.key) ?? null;
    const continuity = continuityByKey.get(tile.key);
    const justOpened = deltaOpenedKeys.has(tile.key);
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
      attentionPrimary ? "attention-primary" : null,
      semanticRole ? `role-${semanticRole}` : null,
      continuity?.presence === "returning" ? "presence-returning" : null,
      continuity?.knowledge === "interrupted" ||
      continuity?.knowledge === "fading"
        ? `knowledge-${continuity.knowledge}`
        : null,
      justOpened ? "just-opened" : null,
      layoutEditing && layoutPreview ? "live-over-preview" : null,
    ]
      .filter(Boolean)
      .join(" ");
    const roleHint =
      semanticRole === "working"
        ? "Working"
        : semanticRole === "returning"
          ? "Returning"
          : semanticRole === "interrupted"
            ? "Interrupted"
            : semanticRole === "companion"
              ? "Companion"
              : null;
    const continuityHint =
      continuity?.presence === "returning"
        ? "Returning"
        : continuity?.knowledge === "interrupted"
          ? "Interrupted"
          : continuity?.knowledge === "fading"
            ? "Fading"
            : null;
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
            roleHint,
            continuityHint && continuityHint !== roleHint
              ? continuityHint
              : null,
            justOpened ? "Just opened" : null,
            attentionPrimary ? "Noticed" : null,
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
    layoutEditing ? "layout-editing" : null,
    editingPhase === "changes_pending" ? "layout-changes-pending" : null,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <section
      className={stageClass}
      aria-label="Desktop"
      data-work-mode={workMode}
      data-stage-plane={planeCalm ? "calm" : "live"}
      data-layout-editing={layoutEditing ? "true" : "false"}
      data-layout-editing-phase={editingPhase}
    >
      <header className="stage-hero stage-hero-minimal">
        <h2>{layoutsStageTitle(workspace?.name)}</h2>
        <p className="stage-workflow-line muted">
          {layoutEditing
            ? layoutEditingWorkflowLine(editingPhase)
            : workspaceDesktopWorkflowLine(workspace?.name)}
        </p>
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

      {layoutEditing && workingSet && editingChangeDiff ? (
        <div
          className={
            editingPhase === "changes_pending"
              ? "stage-layout-edit-banner pending"
              : "stage-layout-edit-banner"
          }
          role="status"
          aria-live="polite"
        >
          <p className="stage-layout-edit-title">
            {layoutEditingBanner(workingSet.name)}
          </p>
          <p className="stage-layout-edit-status muted">
            {layoutEditingStatusMeta({
              phase: editingPhase,
              previewEnabled: layoutPreview,
              updatedAt: workingSet.updated_at,
              diff: editingChangeDiff,
            })}
          </p>
          {updateConfirmation ? (
            <p className="stage-layout-edit-confirmation muted">
              Last update · {updateConfirmation}
            </p>
          ) : null}
          {editingPhase === "changes_pending" ? (
            <p className="stage-layout-edit-confirmation">
              {layoutEditingUpdateConfirmation(editingChangeDiff)}
            </p>
          ) : null}
          <p className="stage-layout-edit-hint muted">
            {layoutEditingHint(editingPhase)}
          </p>
        </div>
      ) : null}

      {showDesktopMap ? (
        <p className="stage-meta stage-meta-live muted">
          {stageDesktopMetaLine({
            windowCount: windows.length,
            monitorCount,
            focusedTitle,
            selectedCount: selectedKeys.length,
            awarenessLine,
          })}
        </p>
      ) : null}

      {showDesktopMap && arrangements.length > 0 ? (
        <div className="stage-working-set-row stage-arrangement-row">
          <label htmlFor="stage-working-set">
            Arrangement
            <select
              id="stage-working-set"
              value={workingSetId}
              disabled={busy || layoutEditing}
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
          {!layoutEditing ? (
            <>
              <button
                type="button"
                disabled={busy || !runtime || !workingSetId}
                onClick={restoreSelectedArrangement}
              >
                Restore
              </button>
              <button
                type="button"
                className="ghost"
                disabled={busy || !runtime || !workingSetId}
                onClick={enterLayoutEditing}
              >
                Edit layout
              </button>
            </>
          ) : (
            <>
              <button
                type="button"
                className={layoutPreview ? undefined : "ghost"}
                disabled={busy}
                aria-pressed={layoutPreview}
                title={
                  layoutPreview
                    ? "Hide saved-layout preview"
                    : "Show saved-layout preview"
                }
                onClick={() => {
                  setLayoutPreview((prev) => !prev);
                }}
              >
                {layoutPreview ? "Preview on" : "Preview"}
              </button>
              <button
                type="button"
                disabled={
                  busy ||
                  !runtime ||
                  !editingChangeDiff?.hasChanges
                }
                title={
                  editingChangeDiff?.hasChanges
                    ? layoutEditingUpdateConfirmation(editingChangeDiff)
                    : "No effective changes to update"
                }
                onClick={updateEditingArrangement}
              >
                Update
              </button>
              <button
                type="button"
                className="ghost"
                disabled={busy || !runtime}
                title="Restore saved Arrangement to the desktop"
                onClick={restoreSelectedArrangement}
              >
                Restore
              </button>
              <button
                type="button"
                className="ghost"
                disabled={busy}
                onClick={exitLayoutEditing}
              >
                Done
              </button>
            </>
          )}
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
          <>
            {previewGhosts.map((ghost) => (
              <div
                key={`preview-${ghost.key}`}
                className="stage-layout-preview-ghost"
                style={{
                  left: `${ghost.leftPct}%`,
                  top: `${ghost.topPct}%`,
                  width: `${ghost.widthPct}%`,
                  height: `${ghost.heightPct}%`,
                }}
                title={`Saved preview · ${ghost.label}`}
                aria-hidden="true"
              >
                <span className="stage-layout-preview-label">
                  Saved · {ghost.label}
                </span>
              </div>
            ))}
            {mapTiles.map(renderTile)}
          </>
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
              disabled={busy || !runtime || !workspace || layoutEditing}
              onClick={saveSelectionAsWorkingSet}
            >
              Save Arrangement
            </button>
            {layoutEditing ? (
              <button
                type="button"
                className="ghost"
                disabled={
                  busy || !runtime || !editingChangeDiff?.hasChanges
                }
                title={
                  editingChangeDiff?.hasChanges
                    ? layoutEditingUpdateConfirmation(editingChangeDiff)
                    : "No effective changes to update"
                }
                onClick={updateEditingArrangement}
              >
                Update
              </button>
            ) : null}
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
