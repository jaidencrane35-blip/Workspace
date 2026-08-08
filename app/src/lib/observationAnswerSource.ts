/**
 * P23.S4 / P23.S5 / P23.S6 — Observation answer sources (rung 3:
 * capability-observation).
 *
 * The answer sources whose data lives on the desktop rather than in this
 * process. They are deliberately incapable of obtaining that data: each
 * declares the observation it needs, and composes an answer from whatever
 * authorized observation comes back. The Kernel remains the sole observer, the
 * Permission Gateway still decides, and nothing here performs IPC or names a
 * capability.
 *
 * Every word of the answer is derived from the observation. If the observation
 * reports nothing, this source produces nothing — the truthful result of the
 * observation is used instead of an invented inventory.
 *
 * The two sources here answer two different questions — what is open, and what
 * is in front — and their coverage is mutually exclusive, so a request has one
 * meaning rather than a shortlist to choose from. That exclusivity is what
 * keeps this a set of answers rather than a selection between capabilities.
 *
 * The application behind a window is read from the observation and from nowhere
 * else. There is no application name in this file, and none is derived from a
 * window title: an observation that named no application produces an answer
 * that names none.
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
    const windows = (observation.items ?? [])
      .map((item) => ({
        title: item.title?.trim() ?? "",
        application: item.processName?.trim() ?? "",
      }))
      .filter((window) => window.title.length > 0);
    if (windows.length === 0) return null;

    return {
      text: sentenceFor(labelWindows(windows)),
      sourceId: "open-windows",
      rung: "capability-observation",
    };
  },
};

/**
 * The Owner asked what is open, so titles are the answer — that is how they
 * recognise their own windows. The observed application is added only where a
 * title alone would be ambiguous, because two windows called the same thing are
 * useless to hear listed twice. Naming the application everywhere would make
 * every answer longer for information the Owner did not ask for.
 */
function labelWindows(
  windows: Array<{ title: string; application: string }>,
): string[] {
  const repeated = new Set(
    windows
      .map((window) => window.title)
      .filter((title, index, all) => all.indexOf(title) !== index),
  );
  return windows.map((window) =>
    repeated.has(window.title) && window.application
      ? `${window.title} (${window.application})`
      : window.title,
  );
}

/** The Owner is asking about the one window in front of them. */
const ACTIVE_STATE = /\b(active|focused|foreground|frontmost|current|in front)\b/;

/** The same question asked from the Owner's side rather than the desktop's. */
const IN_USE = /\b(?:am i|are we)\s+(?:using|working in|working on|in|on)\b/;

/**
 * What the Owner called it. `one`/`this`/`that` only reach here once meaning
 * has been grounded, so an ungrounded "which one?" is never treated as a
 * question about the desktop.
 */
const SINGLE_SUBJECT = /\b(windows?|applications?|apps?|programs?|one|this|that)\b/;

/** Asking after the application, which the observation cannot establish. */
const APPLICATION_IDENTITY = /\b(applications?|apps?|programs?)\b/;

/**
 * P23.S5 — Active-window observation answer source.
 *
 * The observation reports window titles and which window is focused. It does
 * not report what program owns that window, so this source answers with the
 * title it was given and says so when the Owner asked after the application.
 * Naming a program from a title would be a guess, and a confident wrong answer
 * about what the Owner is doing is worse than a narrow true one.
 */
export const activeWindowAnswerSource: ObservationAnswerSource = {
  id: "active-window",
  rung: "capability-observation",
  need: "active-window",

  covers(goal: GoalContract): boolean {
    if (goal.outcome !== "PERCEIVE_MACHINE" || goal.mode !== "observation") {
      return false;
    }
    const text = goal.normalized;
    if (!SINGLE_SUBJECT.test(text)) return false;
    return ACTIVE_STATE.test(text) || IN_USE.test(text);
  },

  compose(goal: GoalContract, observation: ObservationResult): Answer | null {
    if (!observation.ok) return null;
    const items = observation.items ?? [];
    const active = items.find((item) => item.focused) ?? items[0];
    const title = active?.title?.trim();
    if (!title) return null;
    const application = active?.processName?.trim();

    return {
      text: APPLICATION_IDENTITY.test(goal.normalized)
        ? applicationSentence(title, application, IN_USE.test(goal.normalized))
        : IN_USE.test(goal.normalized)
          ? `You’re using “${title}” right now.`
          : `The active window is “${title}”.`,
      sourceId: "active-window",
      rung: "capability-observation",
    };
  },
};

/**
 * The Owner asked which application, so the answer stands or falls on whether
 * the observation named one. When it did not — Windows withholds this for
 * protected processes — the window is still reported and the gap is stated,
 * because the only way to produce an application name here would be to read it
 * out of the title, which is a guess dressed up as an answer.
 */
function applicationSentence(
  title: string,
  application: string | undefined,
  inUsePhrasing: boolean,
): string {
  if (!application) {
    return (
      `The active window is “${title}”. I can see the window, but Windows ` +
      `didn’t tell me which application is behind it, so I won’t name one.`
    );
  }
  return inUsePhrasing
    ? `You’re using ${application} — its window is “${title}”.`
    : `The active application is ${application} — its window is “${title}”.`;
}
