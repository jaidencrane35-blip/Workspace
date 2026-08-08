/**
 * P23.S4 — Open-window observation answer source (rung 3: capability-observation).
 *
 * The first answer source whose data lives on the desktop rather than in this
 * process. It is deliberately incapable of obtaining that data: it declares the
 * observation it needs, and composes an answer from whatever authorized
 * observation comes back. The Kernel remains the sole observer, the Permission
 * Gateway still decides, and nothing here performs IPC or names a capability.
 *
 * Every word of the answer is derived from the observation. If the observation
 * reports nothing, this source produces nothing — the truthful result of the
 * observation is used instead of an invented inventory.
 */

import type { GoalContract } from "./goalContract";
import type {
  Answer,
  ObservationAnswerSource,
  ObservationResult,
} from "./answerSource";

/** What the Owner is asking about: their windows, apps, or desktop as a whole. */
const INVENTORY_SUBJECT = /\b(windows?|applications?|apps?|programs?|desktop)\b/;

/** The Owner is asking what exists right now, not asking for something to change. */
const OPEN_STATE = /\b(open|running)\b/;

/**
 * A question about *one particular* window is a different request with a
 * different answer, and is already served elsewhere. Excluded so the inventory
 * source never swallows it.
 */
const SINGLE_WINDOW = /\b(active|focused|foreground|in front|current window|using)\b/;

/** How many windows to name before summarising the remainder. */
const NAMED_LIMIT = 6;

function joinNaturally(titles: string[]): string {
  if (titles.length === 1) return titles[0];
  if (titles.length === 2) return `${titles[0]} and ${titles[1]}`;
  return `${titles.slice(0, -1).join(", ")}, and ${titles[titles.length - 1]}`;
}

function sentenceFor(titles: string[]): string {
  if (titles.length === 1) {
    return `You’ve got one window open: ${titles[0]}.`;
  }
  if (titles.length <= NAMED_LIMIT) {
    return `You’ve got ${titles.length} windows open: ${joinNaturally(titles)}.`;
  }
  const named = titles.slice(0, NAMED_LIMIT).join(", ");
  return `You’ve got ${titles.length} windows open: ${named}, and ${titles.length - NAMED_LIMIT} more.`;
}

export const openWindowsAnswerSource: ObservationAnswerSource = {
  id: "open-windows",
  rung: "capability-observation",
  need: "open-windows",

  covers(goal: GoalContract): boolean {
    if (goal.outcome !== "PERCEIVE_MACHINE" || goal.mode !== "observation") {
      return false;
    }
    const text = goal.normalized;
    if (SINGLE_WINDOW.test(text)) return false;
    return INVENTORY_SUBJECT.test(text) && OPEN_STATE.test(text);
  },

  compose(_goal: GoalContract, observation: ObservationResult): Answer | null {
    if (!observation.ok) return null;
    const titles = (observation.items ?? [])
      .map((item) => item.title?.trim())
      .filter((title): title is string => Boolean(title));
    if (titles.length === 0) return null;

    return {
      text: sentenceFor(titles),
      sourceId: "open-windows",
      rung: "capability-observation",
    };
  },
};
