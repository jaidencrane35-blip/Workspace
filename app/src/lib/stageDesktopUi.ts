/**
 * Purpose: Pure view-model helpers for Desktop Reality Stage (Milestone R).
 * Owner: Frontend product shell
 * Inputs: WorkspaceStateWindow rows from get_workspace_state
 * Outputs: Spatial tile layout percentages, labels, empty/runtime copy
 * Dependencies: None (pure)
 * Non-goals: Observation capture, WindowController, fake thumbnails, AI,
 *   duplicate window models, arrangement editing
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

export function stageDesktopEmptyCopy(state: StageDesktopLoadState): {
  title: string;
  body: string;
} {
  if (state === "runtime_unavailable") {
    return {
      title: "Desktop runtime required",
      body: "Open the Workspace desktop app to observe your current environment. Browser preview cannot read live windows — nothing here is invented.",
    };
  }
  if (state === "error") {
    return {
      title: "Desktop observation unavailable",
      body: "Workspace could not read the latest desktop state. Retry from the Stage, or confirm observation is running in the desktop app.",
    };
  }
  if (state === "loading") {
    return {
      title: "Reading your desktop…",
      body: "Loading the latest observed windows.",
    };
  }
  return {
    title: "No windows observed yet",
    body: "Your desktop is the product — when observation captures open windows, they appear here spatially. No named profile or app registration is required first.",
  };
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
      ? "1 observed window"
      : `${args.windowCount} observed windows`,
  );
  if (args.monitorCount > 0) {
    parts.push(
      args.monitorCount === 1
        ? "1 monitor"
        : `${args.monitorCount} monitors`,
    );
  }
  if (args.focusedTitle) {
    parts.push(`Focused: ${args.focusedTitle}`);
  }
  if (!args.observationPassId) {
    parts.push("No observation pass yet");
  }
  return parts.join(" · ");
}
