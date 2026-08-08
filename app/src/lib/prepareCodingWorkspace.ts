/**
 * C-PROC-002 — Prepare Coding Workspace (Intent Layer, PCW-001).
 *
 * Resolves prepare/set-up coding-workspace phrasing into an ordered Owner target
 * set. Kernel owns launchability preflight, open composition, wait, and
 * Completion Contract aggregation. No default app set. No private planner.
 */

import {
  encodeCompoundOpenTargets,
  resolveOpenTarget,
  type CompoundOpenTarget,
} from "./compoundOpen";

export type PrepareCodingWorkspaceAction = {
  kind: "prepareCodingWorkspace";
  targets: CompoundOpenTarget[];
  encode: string;
  reply: string;
};

function strip(raw: string): string {
  return raw
    .trim()
    .replace(/[.!?]+$/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

function normalize(raw: string): string {
  return strip(raw)
    .toLowerCase()
    .replace(/^(please|can you|could you|would you)\s+/i, "")
    .replace(/\s+(please|for me|now|thanks|thank you)$/i, "")
    .trim();
}

/**
 * Extract the “with …” target tail from prepare/set-up coding workspace phrasing.
 * Returns:
 * - `{ mode: "with", tail }` when named targets are present
 * - `{ mode: "bare" }` for prepare phrasing without targets
 * - null when this is not a prepare-coding-workspace utterance
 */
export function matchPrepareCodingWorkspacePhrase(
  raw: string,
): { mode: "with"; tail: string } | { mode: "bare" } | null {
  const text = normalize(raw);
  if (!text) {
    return null;
  }

  // Do not steal Situation Goals Continue family (coding environment / set me up / …).
  if (
    /^(i'?m\s+(coding|debugging|researching|reviewing)|set\s+me\s+up|i\s+need\s+my\s+coding\s+environment|coding\s+environment|development\s+setup|dev\s+setup)\b/.test(
      text,
    )
  ) {
    return null;
  }

  const withMatch = text.match(
    /^(?:prepare|set\s*up|setup|get)\s+(?:my\s+|the\s+|a\s+)?(?:coding|development|dev)\s+(?:workspace|environment|setup|space)\s+with\s+(.+)$/i,
  );
  if (withMatch?.[1]) {
    return { mode: "with", tail: strip(withMatch[1]) };
  }

  if (
    /^(?:prepare|set\s*up|setup|get)\s+(?:my\s+|the\s+|a\s+)?(?:coding|development|dev)\s+(?:workspace|environment|setup|space)$/i.test(
      text,
    )
  ) {
    return { mode: "bare" };
  }

  return null;
}

function splitTargetTail(tail: string): string[] {
  const parts = tail
    .split(/\s*(?:,|\band\b)\s*/i)
    .map(strip)
    .filter((p) => p.length > 0);
  return parts;
}

function clarifyTargets(message: string): {
  kind: "unknown";
  reply: string;
  suggestion?: string;
} {
  return {
    kind: "unknown",
    reply: message,
    suggestion: "Name each app or site — for example Cursor and Notepad.",
  };
}

/**
 * PCW-001 Intent resolution for Prepare Coding Workspace.
 * Kernel preflight (launch_alias / browser openability) still runs before Effects.
 */
export function resolvePrepareCodingWorkspace(
  raw: string,
):
  | PrepareCodingWorkspaceAction
  | { kind: "unknown"; reply: string; suggestion?: string }
  | null {
  const matched = matchPrepareCodingWorkspacePhrase(raw);
  if (!matched) {
    return null;
  }

  if (matched.mode === "bare") {
    return clarifyTargets(
      "Which application(s) or site(s) should I prepare for your coding workspace?",
    );
  }

  // Prefer original-casing target tail from the Owner utterance when present.
  const originalWith = raw.match(
    /(?:prepare|set\s*up|setup|get)\s+(?:my\s+|the\s+|a\s+)?(?:coding|development|dev)\s+(?:workspace|environment|setup|space)\s+with\s+(.+?)[.!?]*$/i,
  );
  const parts = splitTargetTail(originalWith?.[1] ?? matched.tail);
  if (parts.length === 0) {
    return clarifyTargets(
      "Which application(s) or site(s) should I prepare for your coding workspace?",
    );
  }

  // Ambiguous pronoun / unbound referent — never invent from installed/running apps.
  if (
    parts.some((p) =>
      /^(that|this|it|those|them)(\s+(app|application|one|window|site|browser))?$/i.test(
        p,
      ),
    )
  ) {
    return clarifyTargets(
      "Which app or site did you mean? Name each target and I’ll prepare only those.",
    );
  }

  const targets: CompoundOpenTarget[] = [];
  for (const part of parts) {
    const target = resolveOpenTarget(part);
    if (!target) {
      return clarifyTargets(
        `I don’t recognize “${part}” as something I can prepare yet. Name apps or sites I already know.`,
      );
    }
    targets.push(target);
  }

  const labels = targets.map((t) => t.label);
  const list =
    labels.length === 1
      ? `“${labels[0]}”`
      : labels.length === 2
        ? `“${labels[0]}” and “${labels[1]}”`
        : `${labels
            .slice(0, -1)
            .map((l) => `“${l}”`)
            .join(", ")}, and “${labels[labels.length - 1]}”`;

  return {
    kind: "prepareCodingWorkspace",
    targets,
    encode: encodeCompoundOpenTargets(targets),
    reply: `Preparing your coding workspace with ${list}.`,
  };
}
