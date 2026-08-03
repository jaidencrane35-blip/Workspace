/**
 * Deterministic session replay — development only.
 * Replays navigation / flow markers from a stored trace without touching IPC.
 */

import type {
  ExperienceDestination,
  ExperienceEvent,
  ExperienceSession,
} from "./experienceEvents";
import { computeFrictionScore, type FrictionScore } from "./frictionModel";

export interface ReplayHandlers {
  navigate?: (destination: ExperienceDestination) => void;
  onEvent?: (event: ExperienceEvent, index: number) => void;
}

export interface ReplayResult {
  sessionId: string;
  applied: number;
  destinations: ExperienceDestination[];
  friction: FrictionScore;
  /** Relative timeline of applied events (t from trace). */
  timeline: Array<{ t: number; type: string; destination?: string }>;
}

/**
 * Apply a session trace deterministically.
 * Does not sleep — callers that want wall-clock pacing may schedule by `t`.
 */
export function replaySession(
  session: ExperienceSession,
  handlers: ReplayHandlers = {},
): ReplayResult {
  const destinations: ExperienceDestination[] = [];
  const timeline: ReplayResult["timeline"] = [];
  let applied = 0;

  for (let i = 0; i < session.events.length; i++) {
    const event = session.events[i]!;
    handlers.onEvent?.(event, i);
    timeline.push({
      t: event.t,
      type: event.type,
      destination: event.destination,
    });

    if (event.type === "navigate" && event.destination) {
      destinations.push(event.destination);
      handlers.navigate?.(event.destination);
      applied += 1;
    } else if (
      event.type === "flow_start" ||
      event.type === "save_success" ||
      event.type === "continue_success" ||
      event.type === "flow_abandon" ||
      event.type === "backtrack" ||
      event.type === "command"
    ) {
      applied += 1;
    }
  }

  return {
    sessionId: session.sessionId,
    applied,
    destinations,
    friction: computeFrictionScore(session),
    timeline,
  };
}

/** Two friction scores match when produced from the same trace. */
export function frictionEquals(a: FrictionScore, b: FrictionScore): boolean {
  return (
    a.score === b.score &&
    a.components.timeToConfidenceMs === b.components.timeToConfidenceMs &&
    a.components.unnecessaryNavigation ===
      b.components.unnecessaryNavigation &&
    a.components.interactionRedundancy ===
      b.components.interactionRedundancy &&
    a.components.attentionSwitching === b.components.attentionSwitching &&
    a.components.flowInterruption === b.components.flowInterruption &&
    a.components.successfulRecovery === b.components.successfulRecovery
  );
}
