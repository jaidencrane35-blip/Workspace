/**
 * Purpose: Lightweight Assistant companion chat helpers (history + answer text).
 * Owner: Frontend product shell (Product Contract V5)
 * Inputs: Compose surface projection + sessionStorage + optional WorkspaceState
 * Outputs: Chat history turns, answer extraction, desktop-aware ask enrichment
 * Dependencies: domain projection types only
 * Non-responsibilities: AI engines, durable history IPC, PermissionGateway, restore/focus
 */

import type {
  WorkspaceAssistantSurfaceProjection,
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

/**
 * Prefix the compose ask with observed desktop facts when available.
 * Displayed user ask stays unprefixed; enrichment is compose-only.
 */
export function enrichAskWithDesktopObservation(
  ask: string,
  state: WorkspaceState | null | undefined,
): string {
  const trimmed = ask.trim();
  if (!trimmed || !state || state.windows.length === 0) {
    return trimmed;
  }

  const focused = state.focused_window?.title?.trim() || null;
  const processes: string[] = [];
  const seen = new Set<string>();
  for (const window of state.windows) {
    const label =
      window.process_name?.trim() || window.title.trim() || `hwnd ${window.hwnd}`;
    const key = label.toLowerCase();
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

  return `Observed desktop (${parts.join("; ")}).\n\n${trimmed}`;
}
