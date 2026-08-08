/**
 * P21.S2 — Compound Goal Decomposition (Intent Layer).
 *
 * When an Owner names multiple independently resolvable open targets in one
 * utterance, emit a single IntentAction. Kernel owns sequential execution.
 * No autonomy: every target must already resolve; never invent.
 */

import {
  resolveDesktopEntity,
  type DesktopEntity,
} from "./semanticIntentEngine";

export type CompoundOpenTarget =
  | { kind: "app"; query: string; label: string }
  | { kind: "browser"; url: string; label: string };

export type CompoundOpenAction = {
  kind: "compoundOpen";
  targets: CompoundOpenTarget[];
  /** Encoded payload for Kernel (`app:Cursor|browser:https://…`). */
  encode: string;
  reply: string;
};

function stripPart(raw: string): string {
  return raw
    .trim()
    .replace(/^[,\s]+|[,\s]+$/g, "")
    .replace(/[.!?]+$/g, "")
    .trim();
}

/** Resolve one Owner-named open part into an existing compound target. */
export function resolveOpenTarget(raw: string): CompoundOpenTarget | null {
  const part = stripPart(raw);
  if (!part) {
    return null;
  }
  // Refuse layout / fullscreen modifiers inside a compound part.
  if (
    /\bbeside\b/i.test(part) ||
    /\b(full\s*size|fullscreen|full\s*screen|maximized|maximised)\b/i.test(part)
  ) {
    return null;
  }

  const entity: DesktopEntity | null = resolveDesktopEntity(part);
  if (!entity) {
    return null;
  }

  if (entity.kind === "site") {
    return { kind: "browser", url: entity.value, label: entity.label };
  }

  if (
    entity.kind === "application" ||
    entity.kind === "browser" ||
    entity.kind === "protocol" ||
    entity.kind === "shell"
  ) {
    return {
      kind: "app",
      query: entity.openQuery ?? entity.value,
      label: entity.label,
    };
  }

  return null;
}

/** Encode targets for CapabilityIntent.text — Kernel plan parses this. */
export function encodeCompoundOpenTargets(targets: CompoundOpenTarget[]): string {
  return targets
    .map((t) =>
      t.kind === "browser"
        ? `browser:${t.url}`
        : `app:${t.query}`,
    )
    .join("|");
}

/**
 * Resolve “open A and B …” when every part is a known desktop target.
 * Returns null when any part is unresolved (caller keeps clarify / unknown).
 */
export function resolveCompoundOpen(rawOpenTail: string): CompoundOpenAction | null {
  const tail = stripPart(rawOpenTail);
  if (!/\band\b/i.test(tail)) {
    return null;
  }

  const parts = tail
    .split(/\s+and\s+/i)
    .map(stripPart)
    .filter((p) => p.length > 0);

  if (parts.length < 2) {
    return null;
  }

  const targets: CompoundOpenTarget[] = [];
  for (const part of parts) {
    const target = resolveOpenTarget(part);
    if (!target) {
      return null;
    }
    targets.push(target);
  }

  const labels = targets.map((t) => t.label);
  const list =
    labels.length === 2
      ? `“${labels[0]}” and “${labels[1]}”`
      : labels.map((l) => `“${l}”`).join(", ");

  return {
    kind: "compoundOpen",
    targets,
    encode: encodeCompoundOpenTargets(targets),
    reply: `Opening ${list}.`,
  };
}
