import type {
  DesktopArrangement,
  DesktopArrangementApplyOutcome,
  DesktopArrangementEntry,
  DesktopArrangementEntryDiagnostic,
  DesktopArrangementRestoreResult,
  DesktopArrangementStatus,
  ObservedWindowAvailability,
} from "../types/desktopArrangement";

/** Human-readable arrangement lifecycle label. */
export function arrangementStatusLabel(
  status: DesktopArrangementStatus,
): string {
  switch (status) {
    case "draft":
      return "Draft";
    case "active":
      return "Active";
    case "archived":
      return "Archived";
    default:
      return status;
  }
}

export function availabilityLabel(
  availability: ObservedWindowAvailability,
): string {
  switch (availability) {
    case "available":
      return "Available";
    case "identity_known_window_missing":
      return "Known, not open";
    case "unavailable":
      return "Unavailable";
    default:
      return availability;
  }
}

export function applyStatusLabel(
  status: DesktopArrangementApplyOutcome["status"],
): string {
  switch (status) {
    case "applied":
      return "Applied";
    case "failed":
      return "Failed";
    case "skipped":
      return "Skipped";
    default:
      return status;
  }
}

export function entryIdentitySummary(entry: DesktopArrangementEntry): string {
  const parts: string[] = [];
  if (entry.process_name?.trim()) {
    parts.push(entry.process_name.trim());
  }
  if (entry.stable_window_id?.trim()) {
    parts.push(`id ${entry.stable_window_id.trim()}`);
  } else if (entry.hwnd?.trim()) {
    parts.push(`hwnd ${entry.hwnd.trim()}`);
  }
  if (entryHasBounds(entry)) {
    parts.push(
      `${entry.width}×${entry.height} @ ${entry.x},${entry.y}`,
    );
  } else {
    parts.push("no stored bounds");
  }
  return parts.join(" · ");
}

export function entryHasBounds(entry: DesktopArrangementEntry): boolean {
  return (
    entry.x != null &&
    entry.y != null &&
    entry.width != null &&
    entry.height != null &&
    entry.width > 0 &&
    entry.height > 0
  );
}

export function arrangementSummaryLine(
  arrangement: DesktopArrangement,
): string {
  const count = arrangement.entries.length;
  const windows = count === 1 ? "1 window" : `${count} windows`;
  return `${arrangementStatusLabel(arrangement.status)} · ${windows}`;
}

/** Prefer derived product meta when live windows are available (IC5). */
export function arrangementListMetaLine(
  arrangement: DesktopArrangement,
  productSummaryLine?: string | null,
): string {
  const base = arrangementSummaryLine(arrangement);
  const derived = productSummaryLine?.trim();
  if (!derived) {
    return base;
  }
  return `${arrangementStatusLabel(arrangement.status)} · ${derived}`;
}

export function emptyArrangementsCopy(hasWorkspace: boolean): {
  title: string;
  body: string;
} {
  if (!hasWorkspace) {
    return {
      title: "Saving needs a Profile",
      body: "Choose a Profile to save Arrangements you can Restore later.",
    };
  }
  return {
    title: "No Arrangements yet",
    body: "Save open windows as an Arrangement when you want to Restore this layout later.",
  };
}

export function restoreSummaryCopy(
  result: DesktopArrangementRestoreResult,
): string {
  const parts = [
    `${result.applied_count} applied`,
    `${result.gap_count} gap${result.gap_count === 1 ? "" : "s"}`,
  ];
  if (result.failed_count > 0) {
    parts.push(`${result.failed_count} failed`);
  }
  if (result.outcomes.some((outcome) => outcome.simulated)) {
    parts.push("simulated");
  }
  return parts.join(" · ");
}

export function gapDiagnostics(
  diagnostics: DesktopArrangementEntryDiagnostic[],
): DesktopArrangementEntryDiagnostic[] {
  return diagnostics.filter((item) => item.restore_gap);
}

export function permissionHintForError(message: string): string | null {
  const lower = message.toLowerCase();
  if (lower.includes("permission") || lower.includes("denied")) {
    return "Restore requires the desktop.restore permission. Approve the request or sign in as the local user.";
  }
  if (lower.includes("approval")) {
    return "This action needs explicit approval before windows can move.";
  }
  return null;
}
