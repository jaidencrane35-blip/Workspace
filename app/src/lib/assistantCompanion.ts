/**
 * Purpose: Lightweight Assistant companion chat helpers (session recent + answer text).
 * Owner: Frontend product shell
 * Inputs: Compose surface projection + sessionStorage
 * Outputs: Recent turn list + utterance body extraction
 * Dependencies: WorkspaceAssistantSurfaceProjection types only
 * Non-responsibilities: AI engines, durable history IPC, PermissionGateway
 */

import type { WorkspaceAssistantSurfaceProjection } from "../types/domain";

export const ASSISTANT_COMPANION_RECENT_KEY =
  "workspace.assistant.companion.recent.v1";

export const ASSISTANT_COMPANION_RECENT_LIMIT = 8;

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
