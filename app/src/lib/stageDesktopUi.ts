/**
 * Purpose: Pure view-model helpers for Desktop Reality Stage.
 * Owner: Frontend product shell (Product Contract V4)
 * Inputs: WorkspaceStateWindow rows from get_workspace_state
 * Outputs: Spatial tiles with app-object identity + process relationship accents
 * Dependencies: None (pure)
 * Non-goals: Fake windows, WindowController, grouping engines, OS geometry apply
 */

import type { WorkspaceStateWindow } from "../types/domain";

/** Stage tile bound to a real observed window identity (not a registry invent). */
export interface StageDesktopWindowTile {
  key: string;
  title: string;
  processLabel: string;
  /** Stable process key for relationship accents (empty when unknown). */
  processKey: string;
  /** 0–5 hue bucket shared by windows of the same process. */
  relationIndex: number;
  focused: boolean;
  minimized: boolean;
  leftPct: number;
  topPct: number;
  widthPct: number;
  heightPct: number;
  monitorLabel: string | null;
  boundsLabel: string;
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

export function stageDesktopProcessKey(window: WorkspaceStateWindow): string {
  const process = window.process_name?.trim().toLowerCase();
  if (process) {
    return process;
  }
  return `pid:${window.process_id}`;
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
 * No fake pixels — rectangles encode identity + geometry only.
 */
export function layoutStageDesktopWindows(
  windows: WorkspaceStateWindow[],
): StageDesktopWindowTile[] {
  if (windows.length === 0) {
    return [];
  }

  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;

  for (const window of windows) {
    const w = Math.max(window.width, 1);
    const h = Math.max(window.height, 1);
    minX = Math.min(minX, window.x);
    minY = Math.min(minY, window.y);
    maxX = Math.max(maxX, window.x + w);
    maxY = Math.max(maxY, window.y + h);
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
    return {
      key: stageDesktopWindowKey(window),
      title: appLabel,
      processLabel: windowTitle !== appLabel ? windowTitle : "",
      processKey,
      relationIndex: stageDesktopRelationIndex(processKey),
      focused: window.focused,
      minimized: window.minimized,
      leftPct,
      topPct,
      widthPct: Math.min(widthPct, 100 - leftPct),
      heightPct: Math.min(heightPct, 100 - topPct),
      monitorLabel,
      boundsLabel: `${window.width}×${window.height} @ ${window.x},${window.y}`,
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

export function stageDesktopEmptyCopy(state: StageDesktopLoadState): {
  title: string;
  body: string;
} {
  const line = stageDesktopPlaneMessage(state);
  return { title: line, body: "" };
}

export function stageDesktopMetaLine(args: {
  windowCount: number;
  monitorCount: number;
  observationPassId: string | null;
  focusedTitle: string | null;
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
  if (args.focusedTitle) {
    parts.push(args.focusedTitle);
  }
  return parts.join(" · ");
}
