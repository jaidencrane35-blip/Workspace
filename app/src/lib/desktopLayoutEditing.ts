/**
 * Purpose: Desktop layout editing foundation helpers (Programme I IC3).
 * Owner: Frontend product shell
 * Inputs: Arrangement entries, monitors / observed windows for plane bounds
 * Outputs: Editing copy, preview ghost geometry on the Desktop map
 * Dependencies: Existing DesktopArrangement + WorkspaceState types only
 * Non-goals: New persistence, canvas Layout HWND store, set_bounds IPC,
 *   parallel arrangement models
 */

import type { DesktopArrangementEntry } from "../types/desktopArrangement";
import type {
  WorkspaceStateMonitor,
  WorkspaceStateWindow,
} from "../types/domain";

/** Ghost rectangle projected from stored Arrangement entry bounds. */
export interface LayoutPreviewGhost {
  key: string;
  label: string;
  leftPct: number;
  topPct: number;
  widthPct: number;
  heightPct: number;
}

export interface StagePlaneBounds {
  minX: number;
  minY: number;
  spanX: number;
  spanY: number;
}

/**
 * Same spatial plane as live Stage tiles: monitor union when known,
 * otherwise union of fallback rects (observed windows or entry bounds).
 */
export function stagePlaneBoundsFromRects(
  monitors: readonly WorkspaceStateMonitor[],
  fallbackRects: readonly {
    x: number;
    y: number;
    width: number;
    height: number;
  }[],
): StagePlaneBounds | null {
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
  } else if (fallbackRects.length > 0) {
    for (const rect of fallbackRects) {
      const w = Math.max(rect.width, 1);
      const h = Math.max(rect.height, 1);
      minX = Math.min(minX, rect.x);
      minY = Math.min(minY, rect.y);
      maxX = Math.max(maxX, rect.x + w);
      maxY = Math.max(maxY, rect.y + h);
    }
  } else {
    return null;
  }

  return {
    minX,
    minY,
    spanX: Math.max(maxX - minX, 1),
    spanY: Math.max(maxY - minY, 1),
  };
}

function entryHasBounds(
  entry: DesktopArrangementEntry,
): entry is DesktopArrangementEntry & {
  x: number;
  y: number;
  width: number;
  height: number;
} {
  return (
    entry.x !== null &&
    entry.y !== null &&
    entry.width !== null &&
    entry.height !== null &&
    entry.width > 0 &&
    entry.height > 0
  );
}

/**
 * Project stored Arrangement geometry onto the Desktop map as preview ghosts.
 * Does not apply OS bounds — Restore remains the apply pathway.
 */
export function layoutArrangementPreviewGhosts(
  entries: readonly DesktopArrangementEntry[] | null | undefined,
  monitors: readonly WorkspaceStateMonitor[] = [],
  observedWindows: readonly WorkspaceStateWindow[] = [],
): LayoutPreviewGhost[] {
  if (!entries || entries.length === 0) {
    return [];
  }

  const bounded = entries.filter(entryHasBounds);
  if (bounded.length === 0) {
    return [];
  }

  const plane = stagePlaneBoundsFromRects(
    monitors,
    observedWindows.length > 0
      ? observedWindows.map((window) => ({
          x: window.x,
          y: window.y,
          width: window.width,
          height: window.height,
        }))
      : bounded.map((entry) => ({
          x: entry.x,
          y: entry.y,
          width: entry.width,
          height: entry.height,
        })),
  );
  if (!plane) {
    return [];
  }

  const { minX, minY, spanX, spanY } = plane;

  return bounded.map((entry, index) => {
    const leftPct = ((entry.x - minX) / spanX) * 100;
    const topPct = ((entry.y - minY) / spanY) * 100;
    const widthPct = Math.max((entry.width / spanX) * 100, 8);
    const heightPct = Math.max((entry.height / spanY) * 100, 10);
    const label = entry.label.trim() || `Window ${index + 1}`;
    return {
      key: entry.id || `${entry.hwnd ?? "entry"}-${index}`,
      label,
      leftPct,
      topPct,
      widthPct: Math.min(widthPct, 100 - leftPct),
      heightPct: Math.min(heightPct, 100 - topPct),
    };
  });
}

export function layoutEditingBanner(arrangementName: string): string {
  const name = arrangementName.trim() || "Arrangement";
  return `Editing · ${name}`;
}

export function layoutEditingHint(): string {
  return "Select windows to set membership · Preview shows saved positions · Update records current window positions · Apply Restores to the desktop";
}

export function layoutEditingWorkflowLine(): string {
  return "Edit layout · Preview · Update · Apply (Restore)";
}
