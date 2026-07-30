/**
 * Purpose: Programme I IC5 — Arrangement confidence helpers (derived metadata + feedback).
 * Owner: Frontend product shell
 * Inputs: DesktopArrangement, WorkspaceState windows, restore/capture result DTOs
 * Outputs: Product metadata lines, Save/Update/Restore feedback, first-use guidance
 * Dependencies: desktopArrangementUi (bounds helpers), existing types only
 *
 * Product State (user-owned / authoritative):
 *   Profile · Desktop (WorkspaceState) · Arrangement · Restore
 * Interaction State (session-only UI — never persistence):
 *   Editing · Preview · Selection · Pending changes · Guidance dismissed
 *
 * Non-goals: metadata cache, onboarding persistence, notification engine,
 *   operation log, second Arrangement/desktop model
 */

import { layoutEditingUpdatedLabel } from "./desktopLayoutEditing";
import {
  emptyArrangementsCopy,
  entryHasBounds,
  restoreSummaryCopy,
} from "./desktopArrangementUi";
import type {
  DesktopArrangement,
  DesktopArrangementEntry,
  DesktopArrangementRestoreResult,
} from "../types/desktopArrangement";
import type { WorkspaceStateWindow } from "../types/domain";

export type ArrangementCompleteness = "empty" | "partial" | "complete";
export type ArrangementRestoreReadiness =
  | "not_ready"
  | "partial"
  | "ready";

/** Derived Arrangement facts — computed, never cached or persisted. */
export interface ArrangementProductMeta {
  windowCount: number;
  presentCount: number;
  missingCount: number;
  boundsCompleteCount: number;
  completeness: ArrangementCompleteness;
  restoreReadiness: ArrangementRestoreReadiness;
  updatedAtLabel: string;
  /** Compact line for lists and Desktop Arrangement chrome. */
  summaryLine: string;
  /** Longer readiness/overlap line for details. */
  readinessLine: string;
}

function identityKey(
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
  return identityKey(entry.stable_window_id, entry.hwnd);
}

function windowIdentityKey(window: WorkspaceStateWindow): string | null {
  return identityKey(window.stable_window_id, window.hwnd);
}

function windowCountLabel(count: number): string {
  return count === 1 ? "1 window" : `${count} windows`;
}

/**
 * Derive Arrangement product metadata from saved entries + current desktop.
 * Overlap / readiness are observation-time facts only.
 */
export function deriveArrangementProductMeta(
  arrangement: DesktopArrangement,
  observedWindows: readonly WorkspaceStateWindow[] = [],
): ArrangementProductMeta {
  const entries = arrangement.entries;
  const windowCount = entries.length;
  const liveKeys = new Set<string>();
  for (const window of observedWindows) {
    const key = windowIdentityKey(window);
    if (key) {
      liveKeys.add(key);
    }
  }

  let presentCount = 0;
  let boundsCompleteCount = 0;
  for (const entry of entries) {
    if (entryHasBounds(entry)) {
      boundsCompleteCount += 1;
    }
    const key = entryIdentityKey(entry);
    if (key && liveKeys.has(key)) {
      presentCount += 1;
    }
  }
  const missingCount = Math.max(windowCount - presentCount, 0);

  const completeness: ArrangementCompleteness =
    windowCount === 0
      ? "empty"
      : boundsCompleteCount === windowCount
        ? "complete"
        : boundsCompleteCount > 0
          ? "partial"
          : "empty";

  let restoreReadiness: ArrangementRestoreReadiness = "not_ready";
  if (windowCount > 0 && boundsCompleteCount > 0) {
    if (presentCount === windowCount && completeness === "complete") {
      restoreReadiness = "ready";
    } else if (presentCount > 0 || boundsCompleteCount > 0) {
      restoreReadiness = "partial";
    }
  }

  const updatedAtLabel = layoutEditingUpdatedLabel(arrangement.updated_at).replace(
    /^Updated · /,
    "",
  );

  const overlapPart =
    observedWindows.length === 0
      ? null
      : presentCount === windowCount && windowCount > 0
        ? "all open"
        : presentCount === 0
          ? "none open"
          : `${presentCount}/${windowCount} open`;

  const readinessPart =
    restoreReadiness === "ready"
      ? "Restore ready"
      : restoreReadiness === "partial"
        ? "Restore partial"
        : windowCount === 0
          ? "No windows"
          : "Restore not ready";

  const summaryParts = [
    windowCountLabel(windowCount),
    overlapPart,
    `Updated ${updatedAtLabel}`,
  ].filter(Boolean);

  return {
    windowCount,
    presentCount,
    missingCount,
    boundsCompleteCount,
    completeness,
    restoreReadiness,
    updatedAtLabel,
    summaryLine: summaryParts.join(" · "),
    readinessLine: [
      readinessPart,
      completeness === "complete"
        ? "bounds complete"
        : completeness === "partial"
          ? "bounds partial"
          : null,
      missingCount > 0 && observedWindows.length > 0
        ? `${missingCount} missing`
        : null,
    ]
      .filter(Boolean)
      .join(" · "),
  };
}

export function arrangementSavedFeedback(
  arrangement: DesktopArrangement,
): string {
  return `Arrangement saved · “${arrangement.name}” · ${windowCountLabel(
    arrangement.entries.length,
  )} · completed successfully`;
}

export function arrangementUpdatedFeedback(
  arrangement: DesktopArrangement,
  detail?: string | null,
): string {
  const detailPart = detail?.trim()
    ? detail.trim()
    : windowCountLabel(arrangement.entries.length);
  return `Arrangement updated · “${arrangement.name}” · ${detailPart} · completed successfully`;
}

export function arrangementRestoredFeedback(
  arrangementName: string,
  result: DesktopArrangementRestoreResult,
): string {
  const name = arrangementName.trim() || "Arrangement";
  const outcome =
    result.failed_count > 0
      ? "completed with failures"
      : result.gap_count > 0
        ? "completed with gaps"
        : "completed successfully";
  return `Arrangement restored · “${name}” · ${restoreSummaryCopy(
    result,
  )} · ${outcome}`;
}

export interface DesktopFirstUseGuidance {
  title: string;
  body: string;
}

/**
 * First-use guidance from product facts only.
 * Appears when a Profile has zero Arrangements; disappears when any exist.
 * Dismiss is Interaction State (caller) — never persisted.
 */
export function desktopFirstUseGuidance(input: {
  hasProfile: boolean;
  arrangementCount: number;
  desktopReady: boolean;
}): DesktopFirstUseGuidance | null {
  if (!input.hasProfile) {
    const empty = emptyArrangementsCopy(false);
    return { title: empty.title, body: empty.body };
  }
  if (input.arrangementCount > 0) {
    return null;
  }
  if (!input.desktopReady) {
    return {
      title: "No Arrangements yet",
      body: "When your desktop is ready, select windows and Save Arrangement — or Save from Arrangements — then Restore later.",
    };
  }
  return {
    title: "Save your first Arrangement",
    body: "Select windows on Desktop and Save Arrangement, or use Arrangements below. Restore brings that layout back.",
  };
}

/** Shared product workflow language (IC2/IC5). */
export function arrangementProductWorkflowHint(): string {
  return "Desktop · Arrangement · Save / Update / Restore — Edit layout on Desktop for the same Arrangements.";
}
