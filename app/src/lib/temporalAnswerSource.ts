/**
 * P23.S3 — Local temporal answer source (rung 1: deterministic local).
 *
 * The single authority for "what time / date is it, and where?". It owns both
 * halves of that question:
 *
 *  - **Region resolution** — a deterministic phrase → IANA zone table. Purely
 *    semantic: it maps what the Owner said to a zone name, and nothing else.
 *  - **The clock** — `Intl.DateTimeFormat` with an IANA `timeZone`, which is the
 *    runtime's authoritative time-zone database. Offsets and daylight saving
 *    are its job, never ours: this module contains no offset arithmetic, so a
 *    zone that changes its rules stays correct without a code change.
 *
 * Deliberately not a geopolitical database. A region is listed only when it
 * resolves to exactly one zone; genuinely ambiguous names are reported as
 * ambiguous so Conversation can ask, and unlisted places return nothing so the
 * request falls through to the existing external-information path.
 *
 * Performs no IPC, selects no capability, and causes no desktop effect.
 */

import type { GoalContract } from "./goalContract";
import type { Answer, AnswerSource } from "./answerSource";
import { getWorkspaceContext } from "./workspaceContext";

interface RegionEntry {
  match: RegExp;
  zone: string;
  /** How the answer names the place — the Owner's framing, not the zone id. */
  label: string;
}

/**
 * Phrase → zone. Longer, more specific names are listed before short codes so
 * "Western Australia" is never reached by a looser rule.
 */
const REGION_ZONES: readonly RegionEntry[] = [
  { match: /\b(queensland|qld)\b/, zone: "Australia/Brisbane", label: "Queensland" },
  { match: /\bbrisbane\b/, zone: "Australia/Brisbane", label: "Brisbane" },
  {
    match: /\b(western australia|wa time)\b/,
    zone: "Australia/Perth",
    label: "Western Australia",
  },
  { match: /\bperth\b/, zone: "Australia/Perth", label: "Perth" },
  {
    match: /\b(new south wales|nsw)\b/,
    zone: "Australia/Sydney",
    label: "New South Wales",
  },
  { match: /\b(sydney|canberra)\b/, zone: "Australia/Sydney", label: "Sydney" },
  { match: /\bsouth australia\b/, zone: "Australia/Adelaide", label: "South Australia" },
  { match: /\b(adelaide|sa)\b/, zone: "Australia/Adelaide", label: "Adelaide" },
  { match: /\b(victoria|melbourne)\b/, zone: "Australia/Melbourne", label: "Melbourne" },
  { match: /\b(tasmania|hobart)\b/, zone: "Australia/Hobart", label: "Tasmania" },
  {
    match: /\b(northern territory|darwin)\b/,
    zone: "Australia/Darwin",
    label: "Northern Territory",
  },
  { match: /\b(utc|gmt)\b/, zone: "UTC", label: "UTC" },
  { match: /\b(tokyo|japan)\b/, zone: "Asia/Tokyo", label: "Tokyo" },
  { match: /\b(london|uk)\b/, zone: "Europe/London", label: "London" },
  { match: /\b(new york|nyc|et)\b/, zone: "America/New_York", label: "New York" },
  {
    match: /\b(los angeles|la|pt|pacific)\b/,
    zone: "America/Los_Angeles",
    label: "Los Angeles",
  },
];

/**
 * Names that identify more than one real place. Never resolved by guessing.
 * `whenAustralian` is used only when the conversation is already about
 * Australian regions, which is evidence rather than assumption.
 */
const AMBIGUOUS_REGIONS: readonly {
  match: RegExp;
  options: [string, string];
  whenAustralian: RegionEntry | null;
}[] = [
  {
    match: /\bwa\b/,
    options: ["Western Australia", "Washington state"],
    whenAustralian: {
      match: /\bwa\b/,
      zone: "Australia/Perth",
      label: "Western Australia",
    },
  },
  {
    match: /\bgeorgia\b/,
    options: ["the US state of Georgia", "the country of Georgia"],
    whenAustralian: null,
  },
];

/** An IANA identifier the Owner typed directly, e.g. "Australia/Brisbane". */
const IANA_SHAPE = /\b([A-Za-z]+(?:\/[A-Za-z_+-]+)+)\b/;

export type RegionResolution =
  | { kind: "zone"; zone: string; label: string }
  | { kind: "ambiguous"; options: [string, string] }
  | { kind: "none" };

function isKnownZone(zone: string): boolean {
  try {
    new Intl.DateTimeFormat("en-US", { timeZone: zone });
    return true;
  } catch {
    return false;
  }
}

/** Region resolution with no conversational context — used to read the context itself. */
function resolveRegionDirect(phrase: string): RegionResolution {
  const text = phrase.toLowerCase();
  for (const entry of REGION_ZONES) {
    if (entry.match.test(text)) {
      return { kind: "zone", zone: entry.zone, label: entry.label };
    }
  }
  const iana = IANA_SHAPE.exec(phrase);
  if (iana && isKnownZone(iana[1])) {
    const zone = iana[1];
    const place = zone.split("/").pop()!.replace(/_/g, " ");
    return {
      kind: "zone",
      zone,
      label: place.replace(/\b[a-z]/g, (c) => c.toUpperCase()),
    };
  }
  return { kind: "none" };
}

/** True when the previous turn was already about an Australian region. */
function priorTurnWasAustralian(): boolean {
  const previous = getWorkspaceContext().currentGoal?.subject;
  if (!previous) return false;
  const resolved = resolveRegionDirect(previous);
  return resolved.kind === "zone" && resolved.zone.startsWith("Australia/");
}

/**
 * Resolve a phrase to a single time zone.
 *
 * The phrase may be a bare place ("queensland") or a whole utterance — matching
 * is by word boundary either way, so both callers behave identically.
 */
export function resolveRegionZone(phrase: string | null): RegionResolution {
  if (!phrase) return { kind: "none" };
  const text = phrase.toLowerCase();

  for (const ambiguous of AMBIGUOUS_REGIONS) {
    if (!ambiguous.match.test(text)) continue;
    if (ambiguous.whenAustralian && priorTurnWasAustralian()) {
      const { zone, label } = ambiguous.whenAustralian;
      return { kind: "zone", zone, label };
    }
    return { kind: "ambiguous", options: ambiguous.options };
  }

  return resolveRegionDirect(phrase);
}

/** The zone this machine is set to, per the runtime. */
export function systemZone(): string {
  return Intl.DateTimeFormat().resolvedOptions().timeZone;
}

/** Current wall-clock time in a zone, e.g. "7:41 PM". DST handled by the runtime. */
export function timeIn(zone: string, now: Date = new Date()): string {
  return new Intl.DateTimeFormat("en-US", {
    timeZone: zone,
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  }).format(now);
}

/** Current calendar date in a zone, e.g. "Saturday, 8 August 2026". */
export function dateIn(zone: string, now: Date = new Date()): string {
  return new Intl.DateTimeFormat("en-AU", {
    timeZone: zone,
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
  }).format(now);
}

export const temporalAnswerSource: AnswerSource = {
  id: "local-clock",
  rung: "deterministic-local",

  covers(goal: GoalContract): boolean {
    return (
      goal.requestedResult === "current time" ||
      goal.requestedResult === "current date"
    );
  },

  answer(goal: GoalContract): Answer | null {
    if (!temporalAnswerSource.covers(goal)) return null;

    const region = resolveRegionZone(goal.subject);
    // Ambiguous or unrecognised places are left alone: Conversation asks, or
    // the request continues to the existing external-information path.
    if (region.kind === "ambiguous") return null;
    if (region.kind === "none" && goal.subject) return null;

    const zone = region.kind === "zone" ? region.zone : systemZone();
    const place = region.kind === "zone" ? ` in ${region.label}` : "";
    const value =
      goal.requestedResult === "current date" ? dateIn(zone) : timeIn(zone);

    return {
      text: `It’s ${value}${place}.`,
      sourceId: "local-clock",
      rung: "deterministic-local",
    };
  },
};
