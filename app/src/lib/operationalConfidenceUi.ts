/**
 * Purpose: Programme I IC6 — Operational confidence explanations (derived only).
 * Owner: Frontend product shell
 * Inputs: WorkspaceState load facts, Arrangement, observed windows, restore DTOs,
 *   Interaction State flags (editing / preview / in-flight op)
 * Outputs: Activity, currency, pre-Restore, change-since-capture, post-op explanations
 * Dependencies: arrangementProductUi, desktopLayoutEditing (pure comparison)
 *
 * Explanation is a projection of existing truth — not an activity engine,
 * planner, cache, log, or second model.
 *
 * Product State: Profile · Desktop · Arrangement · Restore
 * Interaction State: Editing · Preview · Selection · In-flight op · Last result
 *
 * Non-goals: event history, telemetry, explanation cache, confidence scoring,
 *   background analysis, persistent diagnostics, reopen/planning engines
 */

import { deriveArrangementProductMeta } from "./arrangementProductUi";
import {
  diffLayoutEditingChanges,
  layoutEditingChangeSummary,
  type LayoutEditingChangeDiff,
  type LayoutEditingPhase,
} from "./desktopLayoutEditing";
import type { StageDesktopLoadState } from "./stageDesktopUi";
import type {
  DesktopArrangement,
  DesktopArrangementRestoreResult,
} from "../types/desktopArrangement";
import type { WorkspaceStateWindow } from "../types/domain";

/** Formal Arrangement currency — derived, never persisted. */
export type ArrangementCurrency =
  | "current"
  | "out_of_date"
  | "partial"
  | "unavailable";

export type OperationalOwner =
  | "Observation"
  | "Desktop"
  | "Arrangement"
  | "Interaction"
  | "Restore";

/** In-flight product operation — Interaction State only. */
export type OperationalInFlight =
  | null
  | "observe"
  | "save"
  | "update"
  | "restore";

export interface OperationalActivityExplanation {
  activity: string;
  why: string;
  owner: OperationalOwner;
  /** Single line: activity because why · owner */
  line: string;
}

export interface ArrangementCurrencyExplanation {
  currency: ArrangementCurrency;
  label: string;
  why: string;
  line: string;
  changeDiff: LayoutEditingChangeDiff;
  changeLine: string;
}

export interface PreRestoreExplanation {
  available: boolean;
  title: string;
  bullets: string[];
  why: string;
  summaryLine: string;
  line: string;
}

export interface PostRestoreExplanation {
  title: string;
  bullets: string[];
  why: string;
  line: string;
}

function countLabel(count: number, singular: string, plural: string): string {
  return `${count} ${count === 1 ? singular : plural}`;
}

function composeWhyLine(
  statement: string,
  why: string,
  owner: OperationalOwner,
): string {
  return `${statement} because ${why} · ${owner}`;
}

/**
 * Current activity surface — what Workspace is doing, why, and who owns it.
 */
export function explainOperationalActivity(input: {
  loadState: StageDesktopLoadState;
  hasProfile: boolean;
  arrangementSelected: boolean;
  arrangementName?: string | null;
  layoutEditing: boolean;
  layoutPreview: boolean;
  editingPhase?: LayoutEditingPhase;
  inFlight: OperationalInFlight;
  hasLastRestoreResult: boolean;
}): OperationalActivityExplanation {
  if (input.inFlight === "restore") {
    return {
      activity: "Restore executing",
      why: "Restore was started and Permission Gateway / desktop integration are applying saved bounds",
      owner: "Restore",
      line: composeWhyLine(
        "Restore executing",
        "Restore was started and desktop integration is applying saved bounds",
        "Restore",
      ),
    };
  }
  if (input.inFlight === "update") {
    return {
      activity: "Updating Arrangement",
      why: "Capture is recording current desktop observation into the selected Arrangement",
      owner: "Arrangement",
      line: composeWhyLine(
        "Updating Arrangement",
        "Capture is recording current desktop observation into the selected Arrangement",
        "Arrangement",
      ),
    };
  }
  if (input.inFlight === "save") {
    return {
      activity: "Saving Arrangement",
      why: "Capture is creating an Arrangement from the current desktop observation",
      owner: "Arrangement",
      line: composeWhyLine(
        "Saving Arrangement",
        "Capture is creating an Arrangement from the current desktop observation",
        "Arrangement",
      ),
    };
  }
  if (input.inFlight === "observe" || input.loadState === "loading") {
    return {
      activity: "Observing desktop",
      why: "WorkspaceState is refreshing from the observation pipeline",
      owner: "Observation",
      line: composeWhyLine(
        "Observing desktop",
        "WorkspaceState is refreshing from the observation pipeline",
        "Observation",
      ),
    };
  }
  if (input.loadState === "runtime_unavailable") {
    return {
      activity: "Desktop runtime unavailable",
      why: "the desktop app runtime is required to observe windows",
      owner: "Observation",
      line: composeWhyLine(
        "Desktop runtime unavailable",
        "the desktop app runtime is required to observe windows",
        "Observation",
      ),
    };
  }
  if (input.loadState === "error") {
    return {
      activity: "Desktop observation failed",
      why: "the latest WorkspaceState refresh did not succeed",
      owner: "Observation",
      line: composeWhyLine(
        "Desktop observation failed",
        "the latest WorkspaceState refresh did not succeed",
        "Observation",
      ),
    };
  }
  if (input.layoutEditing && input.layoutPreview) {
    return {
      activity: "Preview active",
      why: "Edit layout is showing saved Arrangement bounds as visual ghosts only",
      owner: "Interaction",
      line: composeWhyLine(
        "Preview active",
        "Edit layout is showing saved Arrangement bounds as visual ghosts only",
        "Interaction",
      ),
    };
  }
  if (input.layoutEditing) {
    const pending = input.editingPhase === "changes_pending";
    return {
      activity: pending ? "Editing · changes pending" : "Editing Arrangement",
      why: pending
        ? "the live desktop differs from the saved Arrangement membership or bounds"
        : "Edit layout is open and waiting for membership or desktop changes",
      owner: "Interaction",
      line: composeWhyLine(
        pending ? "Editing · changes pending" : "Editing Arrangement",
        pending
          ? "the live desktop differs from the saved Arrangement membership or bounds"
          : "Edit layout is open and waiting for membership or desktop changes",
        "Interaction",
      ),
    };
  }
  if (input.hasLastRestoreResult) {
    return {
      activity: "Restore complete",
      why: "the latest Restore result is still shown from this session",
      owner: "Restore",
      line: composeWhyLine(
        "Restore complete",
        "the latest Restore result is still shown from this session",
        "Restore",
      ),
    };
  }
  if (input.arrangementSelected) {
    const name = input.arrangementName?.trim() || "Arrangement";
    return {
      activity: "Arrangement selected",
      why: `“${name}” is selected and Restore can use saved Arrangement data`,
      owner: "Arrangement",
      line: composeWhyLine(
        "Arrangement selected",
        `“${name}” is selected and Restore can use saved Arrangement data`,
        "Arrangement",
      ),
    };
  }
  if (!input.hasProfile) {
    return {
      activity: "Waiting for Profile",
      why: "a Profile is required before Arrangements can be saved",
      owner: "Desktop",
      line: composeWhyLine(
        "Waiting for Profile",
        "a Profile is required before Arrangements can be saved",
        "Desktop",
      ),
    };
  }
  if (input.loadState === "ready") {
    return {
      activity: "Waiting for desktop changes",
      why: "WorkspaceState is ready and Observation owns live window facts",
      owner: "Observation",
      line: composeWhyLine(
        "Waiting for desktop changes",
        "WorkspaceState is ready and Observation owns live window facts",
        "Observation",
      ),
    };
  }
  return {
    activity: "Desktop idle",
    why: "no Arrangement is selected and Observation is not actively refreshing",
    owner: "Desktop",
    line: composeWhyLine(
      "Desktop idle",
      "no Arrangement is selected and Observation is not actively refreshing",
      "Desktop",
    ),
  };
}

/**
 * Arrangement currency + what changed since capture.
 * Derived from Arrangement entries vs live WorkspaceState windows only.
 */
export function explainArrangementCurrency(
  arrangement: DesktopArrangement,
  observedWindows: readonly WorkspaceStateWindow[],
): ArrangementCurrencyExplanation {
  const meta = deriveArrangementProductMeta(arrangement, observedWindows);
  const changeDiff = diffLayoutEditingChanges(
    arrangement.entries,
    observedWindows,
  );
  const changeLine = changeDiff.hasChanges
    ? formatChangeSinceCapture(changeDiff)
    : "No effective changes since capture";

  if (meta.windowCount === 0) {
    const why = "this Arrangement has no tracked windows";
    return {
      currency: "unavailable",
      label: "Unavailable",
      why,
      line: composeWhyLine("Arrangement is unavailable", why, "Arrangement"),
      changeDiff,
      changeLine,
    };
  }

  if (observedWindows.length === 0) {
    const why =
      "Workspace has not observed any desktop windows to compare against";
    return {
      currency: "unavailable",
      label: "Unavailable",
      why,
      line: composeWhyLine("Arrangement is unavailable", why, "Observation"),
      changeDiff,
      changeLine,
    };
  }

  if (meta.presentCount === 0) {
    const why =
      "none of the tracked windows are open on the current desktop";
    return {
      currency: "unavailable",
      label: "Unavailable",
      why,
      line: composeWhyLine("Arrangement is unavailable", why, "Desktop"),
      changeDiff,
      changeLine,
    };
  }

  if (meta.presentCount < meta.windowCount || meta.completeness !== "complete") {
    const why =
      meta.presentCount < meta.windowCount
        ? `${meta.missingCount} tracked ${meta.missingCount === 1 ? "window is" : "windows are"} missing from the current desktop`
        : "some tracked windows are missing stored bounds";
    return {
      currency: "partial",
      label: "Partial",
      why,
      line: composeWhyLine("Arrangement is partial", why, "Desktop"),
      changeDiff,
      changeLine,
    };
  }

  if (changeDiff.boundsChanged > 0 || changeDiff.added > 0) {
    // All arrangement members present; desktop differs in bounds or extras.
    if (changeDiff.boundsChanged > 0) {
      const why = `${countLabel(changeDiff.boundsChanged, "tracked window has", "tracked windows have")} different bounds than when captured`;
      return {
        currency: "out_of_date",
        label: "Out of date",
        why,
        line: composeWhyLine("Arrangement is out of date", why, "Desktop"),
        changeDiff,
        changeLine,
      };
    }
  }

  // Members present with matching bounds — treat as current even if desktop
  // has additional windows (added), unless members themselves moved.
  if (
    changeDiff.removed === 0 &&
    changeDiff.boundsChanged === 0 &&
    meta.presentCount === meta.windowCount
  ) {
    const why =
      "every tracked window matches the current desktop membership and bounds";
    return {
      currency: "current",
      label: "Current",
      why,
      line: composeWhyLine("Arrangement is current", why, "Desktop"),
      changeDiff,
      changeLine,
    };
  }

  const why =
    "the current desktop no longer fully matches this Arrangement’s tracked windows";
  return {
    currency: "out_of_date",
    label: "Out of date",
    why,
    line: composeWhyLine("Arrangement is out of date", why, "Desktop"),
    changeDiff,
    changeLine,
  };
}

export function formatChangeSinceCapture(
  diff: LayoutEditingChangeDiff,
): string {
  const parts: string[] = [];
  if (diff.added > 0) {
    parts.push(`+ ${diff.added} ${diff.added === 1 ? "window" : "windows"}`);
  }
  if (diff.removed > 0) {
    parts.push(`− ${diff.removed} ${diff.removed === 1 ? "window" : "windows"}`);
  }
  if (diff.boundsChanged > 0) {
    parts.push(`${diff.boundsChanged} moved`);
  }
  if (diff.unchanged > 0) {
    parts.push(`${diff.unchanged} unchanged`);
  }
  if (parts.length === 0) {
    return layoutEditingChangeSummary(diff);
  }
  return parts.join(" · ");
}

/**
 * Pre-Restore explanation — what Restore will do, from comparison only.
 * Does not apply OS bounds; Restore remains the sole apply path.
 */
export function explainPreRestore(
  arrangement: DesktopArrangement,
  observedWindows: readonly WorkspaceStateWindow[],
): PreRestoreExplanation {
  const meta = deriveArrangementProductMeta(arrangement, observedWindows);
  const changeDiff = diffLayoutEditingChanges(
    arrangement.entries,
    observedWindows,
  );
  const currency = explainArrangementCurrency(arrangement, observedWindows);

  const moveCount = changeDiff.boundsChanged;
  const alreadyCorrect = changeDiff.unchanged;
  const unavailable = meta.missingCount;
  const withoutBounds = Math.max(
    meta.windowCount - meta.boundsCompleteCount,
    0,
  );

  const bullets: string[] = [];
  if (moveCount > 0) {
    bullets.push(
      `move ${countLabel(moveCount, "window", "windows")} to saved positions`,
    );
  }
  if (alreadyCorrect > 0) {
    bullets.push(
      `leave ${countLabel(alreadyCorrect, "window", "windows")} unchanged (already matching)`,
    );
  }
  if (unavailable > 0) {
    bullets.push(
      `ignore ${countLabel(unavailable, "unavailable window", "unavailable windows")}`,
    );
  }
  if (withoutBounds > 0) {
    bullets.push(
      `skip ${countLabel(withoutBounds, "window without stored bounds", "windows without stored bounds")}`,
    );
  }
  if (bullets.length === 0 && meta.windowCount > 0) {
    bullets.push("apply saved bounds where matching windows are available");
  }
  if (meta.windowCount === 0) {
    bullets.push("do nothing — this Arrangement has no tracked windows");
  }

  const available =
    meta.windowCount > 0 &&
    meta.boundsCompleteCount > 0 &&
    (meta.presentCount > 0 || moveCount > 0 || alreadyCorrect > 0);

  const why = available
    ? `Workspace has Arrangement data with ${countLabel(
        meta.boundsCompleteCount,
        "stored bounds entry",
        "stored bounds entries",
      )} and Restore is the only product path that moves windows`
    : currency.currency === "unavailable"
      ? currency.why
      : "Restore needs stored bounds and matching desktop windows";

  const statement = available
    ? "Restore is available"
    : "Restore is not ready";

  const summaryLine = `Restore will: ${bullets.join("; ")}`;

  return {
    available,
    title: "Restore will",
    bullets,
    why,
    summaryLine,
    line: composeWhyLine(statement, why, "Restore"),
  };
}

/**
 * Post-Restore explanation — projection of existing restore result DTO.
 */
export function explainPostRestore(
  arrangementName: string,
  result: DesktopArrangementRestoreResult,
): PostRestoreExplanation {
  const name = arrangementName.trim() || "Arrangement";
  const skipped = result.outcomes.filter(
    (outcome) => outcome.status === "skipped",
  ).length;
  const applied = result.applied_count;
  const failed = result.failed_count;
  const gaps = result.gap_count;
  const simulated = result.outcomes.some((outcome) => outcome.simulated);

  const bullets: string[] = [
    `${countLabel(applied, "window restored", "windows restored")}`,
  ];
  if (skipped > 0) {
    bullets.push(
      `${countLabel(skipped, "window skipped", "windows skipped")}`,
    );
  }
  if (gaps > 0) {
    bullets.push(
      `${countLabel(gaps, "window unavailable", "windows unavailable")}`,
    );
  }
  if (failed > 0) {
    bullets.push(`${countLabel(failed, "window failed", "windows failed")}`);
  }
  if (simulated) {
    bullets.push("results were simulated (desktop integration stub)");
  }

  const why =
    failed > 0
      ? "Restore finished through Gateway / desktop integration with one or more apply failures"
      : gaps > 0
        ? "Restore finished and reported gaps where tracked windows were not available"
        : "Restore finished and applied saved bounds through the existing Restore pathway";

  const title =
    failed > 0
      ? `Restore completed with failures · “${name}”`
      : gaps > 0
        ? `Restore completed with gaps · “${name}”`
        : `Restore completed · “${name}”`;

  return {
    title,
    bullets,
    why,
    line: `${title} because ${why} · Restore`,
  };
}

/** Banner / message string from post-restore explanation. */
export function arrangementRestoredExplanationMessage(
  arrangementName: string,
  result: DesktopArrangementRestoreResult,
): string {
  const explained = explainPostRestore(arrangementName, result);
  return `${explained.title} · ${explained.bullets.join(" · ")}`;
}
