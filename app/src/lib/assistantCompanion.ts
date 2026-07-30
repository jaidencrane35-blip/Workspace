/**
 * Purpose: Lightweight Assistant companion — history + desktop-fact enrichment.
 * Owner: Frontend product shell (Product Contract V6)
 * Inputs: Compose surface + sessionStorage + WorkspaceState / delta / arrangements
 * Outputs: Chat history, answer extraction, desktop-aware ask enrichment,
 *   deterministic local answers for common desktop questions
 * Dependencies: domain projection + arrangement types only
 * Non-responsibilities: AI engines, durable history IPC, PermissionGateway, restore/focus
 */

import type { DesktopArrangement } from "../types/desktopArrangement";
import type {
  WorkspaceAssistantSurfaceProjection,
  WorkspaceObservationDelta,
  WorkspaceState,
} from "../types/domain";

export const ASSISTANT_COMPANION_RECENT_KEY =
  "workspace.assistant.companion.recent.v1";

/** Chat-style history bound for the companion rail session. */
export const ASSISTANT_COMPANION_RECENT_LIMIT = 24;

export interface AssistantCompanionTurn {
  id: string;
  ask: string;
  answer: string;
  at: string;
}

export interface AssistantDesktopFacts {
  state: WorkspaceState | null;
  delta: WorkspaceObservationDelta | null;
  arrangements: DesktopArrangement[];
}

export function companionAnswerFromSurface(
  projection: WorkspaceAssistantSurfaceProjection | null | undefined,
): string {
  const body = projection?.current?.utterance?.body?.trim();
  if (body) {
    return body;
  }
  const summary = projection?.current?.narrative_summary?.trim();
  if (summary) {
    return summary;
  }
  return "No answer text was available from the existing assistant surface.";
}

export function loadCompanionRecentTurns(): AssistantCompanionTurn[] {
  try {
    const raw = sessionStorage.getItem(ASSISTANT_COMPANION_RECENT_KEY);
    if (!raw) {
      return [];
    }
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) {
      return [];
    }
    return parsed
      .filter((item): item is AssistantCompanionTurn => {
        if (!item || typeof item !== "object") {
          return false;
        }
        const turn = item as AssistantCompanionTurn;
        return (
          typeof turn.id === "string" &&
          typeof turn.ask === "string" &&
          typeof turn.answer === "string" &&
          typeof turn.at === "string"
        );
      })
      .slice(0, ASSISTANT_COMPANION_RECENT_LIMIT);
  } catch {
    return [];
  }
}

export function appendCompanionRecentTurn(
  turn: AssistantCompanionTurn,
): AssistantCompanionTurn[] {
  const next = [
    turn,
    ...loadCompanionRecentTurns().filter((item) => item.id !== turn.id),
  ].slice(0, ASSISTANT_COMPANION_RECENT_LIMIT);
  try {
    sessionStorage.setItem(ASSISTANT_COMPANION_RECENT_KEY, JSON.stringify(next));
  } catch {
    // sessionStorage may be unavailable; keep in-memory return value only
  }
  return next;
}

/** Newest-first storage → oldest-first thread for ChatGPT-like reading. */
export function companionThreadTurns(
  recent: AssistantCompanionTurn[],
): AssistantCompanionTurn[] {
  return [...recent].reverse();
}

function summariseOpen(state: WorkspaceState): string {
  const apps = state.active_applications
    .map((app) => {
      const name = app.process_name?.trim() || `pid ${app.process_id}`;
      return `${name} (${app.window_count})`;
    })
    .slice(0, 12);
  const focused = state.focused_window?.title?.trim();
  const parts = [
    `${state.windows.length} window${state.windows.length === 1 ? "" : "s"} open`,
  ];
  if (focused) {
    parts.push(`focused on ${focused}`);
  }
  if (apps.length > 0) {
    parts.push(`apps: ${apps.join(", ")}`);
  }
  return parts.join(". ") + ".";
}

function summariseWorkingOn(state: WorkspaceState): string {
  const focused = state.focused_window;
  if (!focused) {
    return "No focused window is recorded in the latest observation.";
  }
  const focusedMemberId =
    focused.stable_window_id?.trim() || focused.hwnd;
  const processGroup = (state.window_groups ?? []).find(
    (group) =>
      group.criterion === "process_id" &&
      group.member_ids.includes(focusedMemberId),
  );
  const siblingCount = processGroup?.member_ids.length
    ?? state.windows.filter((window) => window.process_id === focused.process_id)
        .length;
  const name =
    processGroup?.label?.trim() ||
    state.windows.find((window) => window.hwnd === focused.hwnd)?.process_name?.trim() ||
    focused.title.trim() ||
    `hwnd ${focused.hwnd}`;
  return `You appear to be working in ${name} (${focused.title || "untitled"}${
    siblingCount > 1 ? `; ${siblingCount} windows in that process` : ""
  }).`;
}

function summariseBelongsTogether(
  state: WorkspaceState,
  arrangements: DesktopArrangement[],
): string {
  const processGroups = (state.window_groups ?? []).filter(
    (group) => group.criterion === "process_id" && group.member_ids.length > 1,
  );
  const monitorGroups = (state.window_groups ?? []).filter(
    (group) =>
      group.criterion === "monitor_index" && group.member_ids.length > 1,
  );
  const arrangementGroups = (state.window_groups ?? []).filter(
    (group) =>
      group.criterion === "arrangement_membership" &&
      group.member_ids.length > 0,
  );
  const parts: string[] = [];
  if (processGroups.length > 0) {
    parts.push(
      `process groups: ${processGroups
        .slice(0, 5)
        .map((group) => `${group.label} (${group.member_ids.length})`)
        .join(", ")}`,
    );
  }
  if (monitorGroups.length > 0) {
    parts.push(
      `monitor co-location: ${monitorGroups
        .slice(0, 3)
        .map((group) => `${group.label} (${group.member_ids.length})`)
        .join(", ")}`,
    );
  }
  if (arrangementGroups.length > 0) {
    parts.push(
      `arrangement membership: ${arrangementGroups
        .slice(0, 5)
        .map((group) => `${group.label} (${group.member_ids.length})`)
        .join(", ")}`,
    );
  } else if (arrangements.length > 0) {
    parts.push(
      `saved arrangements: ${arrangements
        .slice(0, 5)
        .map((item) => `${item.name} (${item.entries.length})`)
        .join(", ")}`,
    );
  }
  if (parts.length === 0) {
    return "No multi-window process or monitor groups, and no saved arrangements yet.";
  }
  return `${parts.join(". ")}.`;
}

function summariseChanged(delta: WorkspaceObservationDelta | null): string {
  if (!delta || !delta.has_changes) {
    return "No desktop changes between the latest observation passes.";
  }
  const parts: string[] = [];
  if (delta.opened_windows.length > 0) {
    parts.push(
      `opened ${delta.opened_windows
        .map((window) => window.title || window.hwnd)
        .slice(0, 5)
        .join(", ")}`,
    );
  }
  if (delta.closed_windows.length > 0) {
    parts.push(
      `closed ${delta.closed_windows
        .map((window) => window.title || window.hwnd)
        .slice(0, 5)
        .join(", ")}`,
    );
  }
  if (delta.focused_window_changed?.current) {
    parts.push(
      `focus → ${delta.focused_window_changed.current.title || delta.focused_window_changed.current.hwnd}`,
    );
  }
  if (delta.minimized_changes.length > 0) {
    parts.push(`${delta.minimized_changes.length} minimize changes`);
  }
  return parts.length > 0
    ? `Since last observation: ${parts.join("; ")}.`
    : "Observation reports changes, but no window open/close/focus details.";
}

function summariseReopen(
  delta: WorkspaceObservationDelta | null,
  arrangements: DesktopArrangement[],
): string {
  const closed = delta?.closed_windows ?? [];
  if (closed.length > 0) {
    return `Recently closed: ${closed
      .map((window) => window.title || window.hwnd)
      .slice(0, 6)
      .join(", ")}. Restore a saved arrangement under Stage if you want those layouts back — Assistant cannot move windows.`;
  }
  if (arrangements.length > 0) {
    return `Nothing closed in the latest delta. Saved arrangements you can restore from Stage: ${arrangements
      .map((item) => item.name)
      .slice(0, 5)
      .join(", ")}.`;
  }
  return "Nothing closed in the latest delta, and there are no saved arrangements to reopen yet.";
}

function summariseContinuity(state: WorkspaceState): string {
  const withIdentity = state.windows.filter((window) => window.first_seen_at);
  if (withIdentity.length === 0) {
    return "No identity continuity facts are available for the current windows yet.";
  }
  const focused =
    state.windows.find((window) => window.focused) ??
    state.windows.find((window) => window.hwnd === state.focused_window?.hwnd);
  const parts: string[] = [];
  if (focused?.first_seen_at) {
    parts.push(
      `focused ${focused.title || focused.hwnd} first seen ${focused.first_seen_at}${
        focused.identity_confidence
          ? ` (confidence ${focused.identity_confidence})`
          : ""
      }`,
    );
  }
  const oldest = [...withIdentity].sort((a, b) =>
    (a.first_seen_at ?? "").localeCompare(b.first_seen_at ?? ""),
  )[0];
  const newest = [...withIdentity].sort((a, b) =>
    (b.first_seen_at ?? "").localeCompare(a.first_seen_at ?? ""),
  )[0];
  if (oldest?.first_seen_at) {
    parts.push(
      `longest-running: ${oldest.title || oldest.hwnd} since ${oldest.first_seen_at}`,
    );
  }
  if (
    newest?.first_seen_at &&
    newest.stable_window_id !== oldest?.stable_window_id
  ) {
    parts.push(
      `most recently identified: ${newest.title || newest.hwnd} since ${newest.first_seen_at}`,
    );
  }
  return `${parts.join(". ")}.`;
}

/**
 * Answer common desktop questions from observed facts without calling compose.
 * Returns null when the ask needs the broader assistant surface.
 */
export function answerDesktopQuestionLocally(
  ask: string,
  facts: AssistantDesktopFacts,
): string | null {
  const trimmed = ask.trim().toLowerCase();
  if (!trimmed) {
    return null;
  }
  const state = facts.state;
  if (!state) {
    return null;
  }
  const delta = facts.delta ?? state.latest_delta ?? null;

  if (
    /what('s| is) open|what windows|what apps|what applications/.test(trimmed)
  ) {
    return summariseOpen(state);
  }
  if (
    /what am i working on|what('s| is) focused|current (focus|work)/.test(
      trimmed,
    )
  ) {
    return summariseWorkingOn(state);
  }
  if (/belong|related|together|group/.test(trimmed)) {
    return summariseBelongsTogether(state, facts.arrangements);
  }
  if (
    /how long|been open|first seen|continuity|longest.?running/.test(trimmed)
  ) {
    return summariseContinuity(state);
  }
  if (/what changed|what('s| is) new|delta|recent change/.test(trimmed)) {
    return summariseChanged(delta);
  }
  if (/reopen|restore|bring back|closed/.test(trimmed)) {
    return summariseReopen(delta, facts.arrangements);
  }
  return null;
}

/**
 * Prefix the compose ask with observed desktop facts when available.
 * Displayed user ask stays unprefixed; enrichment is compose-only.
 */
export function enrichAskWithDesktopObservation(
  ask: string,
  facts: AssistantDesktopFacts | WorkspaceState | null | undefined,
): string {
  const trimmed = ask.trim();
  if (!trimmed) {
    return trimmed;
  }

  const normalized: AssistantDesktopFacts =
    facts && "state" in (facts as AssistantDesktopFacts)
      ? (facts as AssistantDesktopFacts)
      : {
          state: (facts as WorkspaceState | null | undefined) ?? null,
          delta: null,
          arrangements: [],
        };

  const state = normalized.state;
  if (!state || state.windows.length === 0) {
    return trimmed;
  }
  const delta = normalized.delta ?? state.latest_delta ?? null;

  const focused = state.focused_window?.title?.trim() || null;
  const processes: string[] = [];
  const seen = new Set<string>();
  for (const window of state.windows) {
    const label =
      window.process_name?.trim() || window.title.trim() || `hwnd ${window.hwnd}`;
    const key = `${window.process_id}:${label.toLowerCase()}`;
    if (seen.has(key)) {
      continue;
    }
    seen.add(key);
    processes.push(label);
    if (processes.length >= 8) {
      break;
    }
  }

  const parts = [
    `${state.windows.length} window${state.windows.length === 1 ? "" : "s"}`,
  ];
  if (focused) {
    parts.push(`focused: ${focused}`);
  }
  if (processes.length > 0) {
    parts.push(`apps: ${processes.join(", ")}`);
  }
  if (delta?.has_changes) {
    parts.push(
      `changed: +${delta.opened_windows.length}/-${delta.closed_windows.length}`,
    );
  }
  if (normalized.arrangements.length > 0) {
    parts.push(
      `arrangements: ${normalized.arrangements
        .map((item) => `${item.name}(${item.entries.length})`)
        .slice(0, 4)
        .join(", ")}`,
    );
  }
  const groups = state.window_groups ?? [];
  if (groups.length > 0) {
    parts.push(
      `groups: ${groups
        .slice(0, 6)
        .map((group) => `${group.criterion}:${group.label}(${group.member_ids.length})`)
        .join(", ")}`,
    );
  }

  return `Observed desktop (${parts.join("; ")}).\n\n${trimmed}`;
}
