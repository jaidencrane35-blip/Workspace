/**
 * Purpose: Pure desktop-object model for the Desktop Interaction Layer Stage.
 * Owner: Frontend product shell (Product Contract V8)
 * Inputs: WorkspaceState windows + window_groups from get_workspace_state
 * Outputs: Runtime object tiles, relationship keys from authoritative groups,
 *   Flow/Focus organisation, selection helpers, arrangement overlays
 * Dependencies: None (pure)
 * Non-goals: Fake windows, WindowController ownership, OS geometry apply,
 *   inventing groups (consume WorkspaceState.window_groups)
 */

import type { DesktopArrangementEntry } from "../types/desktopArrangement";
import type {
  DesktopWindowGroup,
  WorkspaceStateMonitor,
  WorkspaceStateWindow,
} from "../types/domain";

/** First-class Stage desktop object projected from observation — not a card. */
export interface StageDesktopWindowTile {
  key: string;
  hwnd: string;
  stableWindowId: string | null;
  title: string;
  processLabel: string;
  processId: number;
  /** Display/process relationship key (name when known, else pid). */
  processKey: string;
  /** 0–5 hue bucket shared by windows of the same process. */
  relationIndex: number;
  visible: boolean;
  focused: boolean;
  minimized: boolean;
  zOrder: number | null;
  monitorIndex: number | null;
  monitorLabel: string | null;
  leftPct: number;
  topPct: number;
  widthPct: number;
  heightPct: number;
}

export type StageDesktopLoadState =
  | "loading"
  | "runtime_unavailable"
  | "error"
  | "ready";

export function stageDesktopWindowKey(window: WorkspaceStateWindow): string {
  const stable = window.stable_window_id?.trim();
  if (stable) {
    return stable;
  }
  return window.hwnd;
}

export function stageDesktopWindowTitle(window: WorkspaceStateWindow): string {
  const title = window.title.trim();
  if (title) {
    return title;
  }
  const process = window.process_name?.trim();
  if (process) {
    return process;
  }
  return `Window ${window.hwnd}`;
}

/** Display name for a spatial app object: prefer process, fall back to title. */
export function stageDesktopAppObjectLabel(
  window: WorkspaceStateWindow,
): string {
  const process = window.process_name?.trim();
  if (process) {
    return process;
  }
  return stageDesktopWindowTitle(window);
}

/** Prefer PID for true process siblings; name only as display fallback key. */
export function stageDesktopProcessKey(window: WorkspaceStateWindow): string {
  return `pid:${window.process_id}`;
}

export function stageDesktopProcessDisplayKey(
  window: WorkspaceStateWindow,
): string {
  const process = window.process_name?.trim().toLowerCase();
  if (process) {
    return process;
  }
  return stageDesktopProcessKey(window);
}

/** Deterministic 0–5 bucket so same process shares an accent without a grouping engine. */
export function stageDesktopRelationIndex(processKey: string): number {
  let hash = 0;
  for (let i = 0; i < processKey.length; i += 1) {
    hash = (hash * 31 + processKey.charCodeAt(i)) >>> 0;
  }
  return hash % 6;
}

/**
 * Map observed windows into a spatial stage using relative bounds.
 * When monitors are present, the plane is the union of observed displays.
 * Rectangles encode identity + geometry only — lightweight representations.
 */
export function layoutStageDesktopWindows(
  windows: WorkspaceStateWindow[],
  monitors: WorkspaceStateMonitor[] = [],
): StageDesktopWindowTile[] {
  if (windows.length === 0) {
    return [];
  }

  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;

  if (monitors.length > 0) {
    for (const monitor of monitors) {
      const w = Math.max(monitor.width, 1);
      const h = Math.max(monitor.height, 1);
      minX = Math.min(minX, monitor.x);
      minY = Math.min(minY, monitor.y);
      maxX = Math.max(maxX, monitor.x + w);
      maxY = Math.max(maxY, monitor.y + h);
    }
  } else {
    for (const window of windows) {
      const w = Math.max(window.width, 1);
      const h = Math.max(window.height, 1);
      minX = Math.min(minX, window.x);
      minY = Math.min(minY, window.y);
      maxX = Math.max(maxX, window.x + w);
      maxY = Math.max(maxY, window.y + h);
    }
  }

  const spanX = Math.max(maxX - minX, 1);
  const spanY = Math.max(maxY - minY, 1);

  return windows.map((window) => {
    const w = Math.max(window.width, 1);
    const h = Math.max(window.height, 1);
    const leftPct = ((window.x - minX) / spanX) * 100;
    const topPct = ((window.y - minY) / spanY) * 100;
    const widthPct = Math.max((w / spanX) * 100, 8);
    const heightPct = Math.max((h / spanY) * 100, 10);
    const monitorLabel =
      window.monitor_name?.trim() ||
      (window.monitor_index === null
        ? null
        : `Monitor ${window.monitor_index + 1}`);

    const appLabel = stageDesktopAppObjectLabel(window);
    const windowTitle = stageDesktopWindowTitle(window);
    const processKey = stageDesktopProcessKey(window);
    const displayKey = stageDesktopProcessDisplayKey(window);
    return {
      key: stageDesktopWindowKey(window),
      hwnd: window.hwnd,
      stableWindowId: window.stable_window_id?.trim() || null,
      title: appLabel,
      processLabel: windowTitle !== appLabel ? windowTitle : "",
      processId: window.process_id,
      processKey,
      relationIndex: stageDesktopRelationIndex(displayKey),
      visible: window.visible,
      focused: window.focused,
      minimized: window.minimized,
      zOrder: window.z_order,
      monitorIndex: window.monitor_index,
      monitorLabel,
      leftPct,
      topPct,
      widthPct: Math.min(widthPct, 100 - leftPct),
      heightPct: Math.min(heightPct, 100 - topPct),
    };
  });
}

export function stageDesktopPlaneMessage(state: StageDesktopLoadState): string {
  if (state === "runtime_unavailable") {
    return "Open the desktop app to see your windows.";
  }
  if (state === "error") {
    return "Could not read the desktop.";
  }
  if (state === "loading") {
    return "Reading desktop…";
  }
  return "No windows open.";
}

export function stageDesktopMetaLine(args: {
  windowCount: number;
  monitorCount: number;
  focusedTitle: string | null;
  selectedCount?: number;
}): string {
  const parts: string[] = [];
  parts.push(
    args.windowCount === 1
      ? "1 window"
      : `${args.windowCount} windows`,
  );
  if (args.monitorCount > 0) {
    parts.push(
      args.monitorCount === 1
        ? "1 monitor"
        : `${args.monitorCount} monitors`,
    );
  }
  if (args.selectedCount && args.selectedCount > 1) {
    parts.push(`${args.selectedCount} selected`);
  }
  if (args.focusedTitle) {
    parts.push(args.focusedTitle);
  }
  return parts.join(" · ");
}

/**
 * Relationships from authoritative WorkspaceState.window_groups.
 * Focus: process_id groups only. Flow: process_id + monitor_index groups.
 * Falls back to empty when groups are absent (no inventing).
 */
export function relatedStageObjectKeys(
  tiles: StageDesktopWindowTile[],
  anchorKey: string | null,
  workMode: "flow" | "focus",
  windowGroups: DesktopWindowGroup[] = [],
): Set<string> {
  const related = new Set<string>();
  if (!anchorKey) {
    return related;
  }
  if (!tiles.some((tile) => tile.key === anchorKey)) {
    return related;
  }
  related.add(anchorKey);
  const criteria =
    workMode === "focus"
      ? new Set(["process_id"])
      : new Set(["process_id", "monitor_index", "arrangement_membership"]);
  for (const group of windowGroups) {
    if (!criteria.has(group.criterion)) {
      continue;
    }
    if (!group.member_ids.includes(anchorKey)) {
      continue;
    }
    for (const memberId of group.member_ids) {
      if (tiles.some((tile) => tile.key === memberId)) {
        related.add(memberId);
      }
    }
  }
  return related;
}

/** Match arrangement entries to Stage object keys via stable id / hwnd. */
export function stageArrangementMemberKeys(
  tiles: StageDesktopWindowTile[],
  entries: DesktopArrangementEntry[] | null | undefined,
): Set<string> {
  const members = new Set<string>();
  if (!entries || entries.length === 0) {
    return members;
  }
  for (const entry of entries) {
    const stable = entry.stable_window_id?.trim();
    const hwnd = entry.hwnd?.trim();
    for (const tile of tiles) {
      if (stable && tile.stableWindowId === stable) {
        members.add(tile.key);
        continue;
      }
      if (hwnd && tile.hwnd === hwnd) {
        members.add(tile.key);
      }
    }
  }
  return members;
}

export function primaryStageSelectionKey(
  selectedKeys: readonly string[],
): string | null {
  if (selectedKeys.length === 0) {
    return null;
  }
  return selectedKeys[selectedKeys.length - 1] ?? null;
}

export function replaceStageSelection(key: string): string[] {
  return [key];
}

export function toggleStageSelection(
  selectedKeys: readonly string[],
  key: string,
): string[] {
  if (selectedKeys.includes(key)) {
    const next = selectedKeys.filter((item) => item !== key);
    return next;
  }
  return [...selectedKeys, key];
}

/** Spatial neighbour for keyboard selection among laid-out tiles. */
export function nextStageSelectionKey(
  tiles: StageDesktopWindowTile[],
  currentKey: string | null,
  direction: "next" | "previous" | "home" | "end",
): string | null {
  if (tiles.length === 0) {
    return null;
  }
  const ordered = [...tiles].sort((a, b) => {
    if (a.topPct !== b.topPct) {
      return a.topPct - b.topPct;
    }
    return a.leftPct - b.leftPct;
  });
  if (direction === "home") {
    return ordered[0]?.key ?? null;
  }
  if (direction === "end") {
    return ordered[ordered.length - 1]?.key ?? null;
  }
  const index = currentKey
    ? ordered.findIndex((tile) => tile.key === currentKey)
    : -1;
  if (direction === "next") {
    if (index < 0) {
      return ordered[0]?.key ?? null;
    }
    return ordered[Math.min(index + 1, ordered.length - 1)]?.key ?? null;
  }
  if (index < 0) {
    return ordered[ordered.length - 1]?.key ?? null;
  }
  return ordered[Math.max(index - 1, 0)]?.key ?? null;
}

/** One dock entry per other process while Focus mode keeps a primary app on the map. */
export interface StageProcessDockEntry {
  processKey: string;
  processId: number;
  label: string;
  hwnd: string;
  tileKey: string;
  windowCount: number;
}

export interface StageWorkModeOrganisation {
  /** Windows drawn on the spatial map (Focus: primary process only). */
  mapWindows: WorkspaceStateWindow[];
  /** Other processes collapsed to dock objects (Focus only). */
  dockEntries: StageProcessDockEntry[];
}

/**
 * Flow keeps every observed window on the map (relationships exposed via accents).
 * Focus keeps one process on the map and docks the rest as process objects.
 * When `windowGroups` is provided, Focus primary process membership comes from
 * authoritative process_id groups (no invented PID buckets).
 */
export function organiseStageForWorkMode(
  windows: WorkspaceStateWindow[],
  workMode: "flow" | "focus",
  selectedKey: string | null,
  windowGroups: DesktopWindowGroup[] = [],
  monitors: WorkspaceStateMonitor[] = [],
): StageWorkModeOrganisation {
  if (workMode !== "focus" || windows.length === 0) {
    return { mapWindows: windows, dockEntries: [] };
  }

  const tiles = layoutStageDesktopWindows(windows, monitors);
  const anchor =
    tiles.find((tile) => tile.key === selectedKey) ??
    tiles.find((tile) => tile.focused) ??
    tiles[0] ??
    null;
  if (!anchor) {
    return { mapWindows: windows, dockEntries: [] };
  }

  const processGroup = windowGroups.find(
    (group) =>
      group.criterion === "process_id" &&
      group.member_ids.includes(anchor.key),
  );
  const primaryKeys = new Set(
    processGroup?.member_ids?.length
      ? processGroup.member_ids
      : tiles
          .filter((tile) => tile.processId === anchor.processId)
          .map((tile) => tile.key),
  );

  const mapWindows = windows.filter((window) =>
    primaryKeys.has(stageDesktopWindowKey(window)),
  );

  const dockByProcess = new Map<number, StageProcessDockEntry>();
  for (const tile of tiles) {
    if (primaryKeys.has(tile.key)) {
      continue;
    }
    const existing = dockByProcess.get(tile.processId);
    if (existing) {
      existing.windowCount += 1;
      continue;
    }
    dockByProcess.set(tile.processId, {
      processKey: tile.processKey,
      processId: tile.processId,
      label: tile.title,
      hwnd: tile.hwnd,
      tileKey: tile.key,
      windowCount: 1,
    });
  }

  return {
    mapWindows,
    dockEntries: [...dockByProcess.values()],
  };
}
