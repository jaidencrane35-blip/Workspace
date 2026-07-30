/**
 * Purpose: Pure view-model helpers for Desktop Reality Stage.
 * Owner: Frontend product shell (IM-1 — Stage empty spatial calm)
 * Inputs: WorkspaceStateWindow rows from get_workspace_state
 * Outputs: Spatial tile layout percentages, short status lines
 * Dependencies: None (pure)
 * Non-goals: Observation capture, WindowController, fake thumbnails, AI,
 *   duplicate window models, arrangement editing, IM-2+ slices
 *
 * Why here: Stage presentation mapping — not domain ownership.
 * Why not elsewhere: Must not invent a parallel observation store or engine.
 */

import type { WorkspaceStateWindow } from "../types/domain";

/** Stage tile bound to a real observed window identity (not a registry invent). */
export interface StageDesktopWindowTile {
  key: string;
  title: string;
  processLabel: string;
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

export function stageDesktopProcessLabel(window: WorkspaceStateWindow): string {
  const process = window.process_name?.trim();
  if (process) {
    return `${process} · PID ${window.process_id}`;
  }
  return `PID ${window.process_id}`;
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

    return {
      key: stageDesktopWindowKey(window),
      title: stageDesktopWindowTitle(window),
      processLabel: stageDesktopProcessLabel(window),
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

/**
 * One short line for the empty desktop plane (layout carries meaning; text supports).
 * IM-1: no paragraphs, no setup language.
 */
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

/** @deprecated Prefer stageDesktopPlaneMessage — kept for tests/callers expecting title/body. */
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
