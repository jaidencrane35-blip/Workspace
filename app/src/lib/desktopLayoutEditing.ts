/**
 * Purpose: Desktop layout editing helpers (Programme I IC3 foundation + IC4 refinement).
 * Owner: Frontend product shell
 * Inputs: Arrangement entries, WorkspaceState windows/monitors, edit UI flags
 * Outputs: Editing lifecycle phase, change awareness, preview ghosts, status copy
 * Dependencies: Existing DesktopArrangement + WorkspaceState types only
 * Non-goals: New persistence, canvas Layout HWND store, set_bounds IPC,
 *   parallel arrangement models, second comparison engines, Intelligence coupling
 */

import type { DesktopArrangementEntry } from "../types/desktopArrangement";
import type {
  WorkspaceStateMonitor,
  WorkspaceStateWindow,
} from "../types/domain";

/** Explicit editing session phase — derived, not a second runtime authority. */
export type LayoutEditingPhase = "idle" | "editing" | "changes_pending";

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
 * Meaningful edit delta between a saved Arrangement and the proposed
 * capture set (selected windows, or all observed when selection is empty).
 * Pure field comparison — not a second layout engine.
 */
export interface LayoutEditingChangeDiff {
  added: number;
  removed: number;
  boundsChanged: number;
  zOrderChanged: number;
  unchanged: number;
  totalProposed: number;
  totalStored: number;
  hasChanges: boolean;
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

function identityKeyFromParts(
  stableWindowId: string | null | undefined,
  hwnd: string | null | undefined,
): string | null {
  const stable = stableWindowId?.trim();
  if (stable) {
    return `stable:${stable}`;
  }
  const handle = hwnd?.trim();
  if (handle) {
    return `hwnd:${handle}`;
  }
  return null;
}

function entryIdentityKey(entry: DesktopArrangementEntry): string | null {
  return identityKeyFromParts(entry.stable_window_id, entry.hwnd);
}

function windowIdentityKey(window: WorkspaceStateWindow): string | null {
  return identityKeyFromParts(window.stable_window_id, window.hwnd);
}

function boundsEqual(
  entry: DesktopArrangementEntry,
  window: WorkspaceStateWindow,
): boolean {
  if (!entryHasBounds(entry)) {
    return false;
  }
  return (
    entry.x === window.x &&
    entry.y === window.y &&
    entry.width === window.width &&
    entry.height === window.height
  );
}

/**
 * Windows that Update Arrangement would capture given current selection.
 * Empty selection → all observed windows (matches capture IPC behaviour).
 */
export function layoutEditingProposedWindows(
  observedWindows: readonly WorkspaceStateWindow[],
  selectedKeys: readonly string[],
  windowKey: (window: WorkspaceStateWindow) => string,
): WorkspaceStateWindow[] {
  if (selectedKeys.length === 0) {
    return [...observedWindows];
  }
  const selected = new Set(selectedKeys);
  return observedWindows.filter((window) => selected.has(windowKey(window)));
}

/**
 * Diff saved Arrangement membership/geometry against the proposed capture set.
 */
export function diffLayoutEditingChanges(
  entries: readonly DesktopArrangementEntry[] | null | undefined,
  proposedWindows: readonly WorkspaceStateWindow[],
): LayoutEditingChangeDiff {
  const stored = entries ?? [];
  const storedByKey = new Map<string, DesktopArrangementEntry>();
  for (const entry of stored) {
    const key = entryIdentityKey(entry);
    if (key) {
      storedByKey.set(key, entry);
    }
  }

  const proposedByKey = new Map<string, WorkspaceStateWindow>();
  for (const window of proposedWindows) {
    const key = windowIdentityKey(window);
    if (key) {
      proposedByKey.set(key, window);
    }
  }

  let added = 0;
  let removed = 0;
  let boundsChanged = 0;
  let zOrderChanged = 0;
  let unchanged = 0;

  for (const [key, window] of proposedByKey) {
    const entry = storedByKey.get(key);
    if (!entry) {
      added += 1;
      continue;
    }
    // Arrangement entries do not persist z-order today — zOrderChanged remains 0
    // until that field exists on the saved Arrangement (no parallel store).
    if (boundsEqual(entry, window)) {
      unchanged += 1;
    } else {
      boundsChanged += 1;
    }
  }

  for (const key of storedByKey.keys()) {
    if (!proposedByKey.has(key)) {
      removed += 1;
    }
  }

  const hasChanges =
    added > 0 || removed > 0 || boundsChanged > 0 || zOrderChanged > 0;

  return {
    added,
    removed,
    boundsChanged,
    zOrderChanged,
    unchanged,
    totalProposed: proposedByKey.size,
    totalStored: storedByKey.size,
    hasChanges,
  };
}

export function layoutEditingPhase(
  editing: boolean,
  hasPendingChanges: boolean,
): LayoutEditingPhase {
  if (!editing) {
    return "idle";
  }
  return hasPendingChanges ? "changes_pending" : "editing";
}

export function layoutEditingPhaseLabel(phase: LayoutEditingPhase): string {
  if (phase === "changes_pending") {
    return "Changes pending";
  }
  if (phase === "editing") {
    return "No changes";
  }
  return "Idle";
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

export function layoutEditingHint(phase: LayoutEditingPhase): string {
  if (phase === "changes_pending") {
    return "Changes pending — Update records the current selection and window positions into this Arrangement. Restore applies the saved layout to the desktop.";
  }
  return "Select windows to set membership · Preview shows saved positions · Organise windows on the desktop, then Update when something changes · Restore applies the saved layout";
}

export function layoutEditingWorkflowLine(phase: LayoutEditingPhase): string {
  if (phase === "changes_pending") {
    return "Editing · Changes pending · Update · Restore · Done";
  }
  if (phase === "editing") {
    return "Editing · No changes · Preview · Restore · Done";
  }
  return "Edit layout · Preview · Update · Restore · Done";
}

export function layoutEditingPreviewStateLabel(previewEnabled: boolean): string {
  return previewEnabled ? "Preview on" : "Preview off";
}

export function layoutEditingUpdatedLabel(
  updatedAt: string | null | undefined,
): string {
  const trimmed = updatedAt?.trim() ?? "";
  if (!trimmed) {
    return "Updated · unknown";
  }
  const ms = Date.parse(trimmed);
  if (Number.isNaN(ms)) {
    return `Updated · ${trimmed}`;
  }
  return `Updated · ${new Date(ms).toLocaleString()}`;
}

export function layoutEditingChangeSummary(
  diff: LayoutEditingChangeDiff,
): string {
  if (!diff.hasChanges) {
    return "No effective changes";
  }
  const parts: string[] = [];
  if (diff.added > 0) {
    parts.push(
      `${diff.added} added`,
    );
  }
  if (diff.removed > 0) {
    parts.push(
      `${diff.removed} removed`,
    );
  }
  if (diff.boundsChanged > 0) {
    parts.push(
      `${diff.boundsChanged} moved`,
    );
  }
  if (diff.zOrderChanged > 0) {
    parts.push(
      `${diff.zOrderChanged} reordered`,
    );
  }
  return parts.join(" · ");
}

/** Confirmation line before/after Update — derived from Arrangement + observation only. */
export function layoutEditingUpdateConfirmation(
  diff: LayoutEditingChangeDiff,
): string {
  const total = diff.totalProposed;
  const totalLabel = `${total} window${total === 1 ? "" : "s"}`;
  if (!diff.hasChanges) {
    return `Update · ${totalLabel} · no effective changes`;
  }
  const affected = diff.added + diff.removed + diff.boundsChanged + diff.zOrderChanged;
  return `Update · ${totalLabel} · ${affected} affected · ${diff.unchanged} unchanged`;
}

export function layoutEditingStatusMeta(input: {
  phase: LayoutEditingPhase;
  previewEnabled: boolean;
  updatedAt: string | null | undefined;
  diff: LayoutEditingChangeDiff;
}): string {
  return [
    layoutEditingPhaseLabel(input.phase),
    layoutEditingPreviewStateLabel(input.previewEnabled),
    layoutEditingUpdatedLabel(input.updatedAt),
    input.phase === "changes_pending"
      ? layoutEditingChangeSummary(input.diff)
      : null,
  ]
    .filter(Boolean)
    .join(" · ");
}
