/**
 * Experience explanation resolver (Sprint 130–131).
 *
 * Translates AttentionReason.explanation_key via the canonical catalog
 * (app/src/generated/explanationCatalog.ts, synced from kernel resources).
 * Experience owns human wording; Domain keeps signal / source / weight / key.
 */

import catalog from "../generated/explanationCatalog";
import type { AttentionReason, DecisionReason } from "../types/domain";

export type DisplayImportance = "high" | "medium" | "low";

export interface DisplayReason {
  title: string;
  description: string;
  importance: DisplayImportance;
  explanation_key: string;
  signal: string;
  source: string;
  weight: number;
  known: boolean;
}

type CatalogEntry = { title: string; description: string };

export function displayImportanceFromWeight(weight: number): DisplayImportance {
  const magnitude = Math.abs(weight);
  if (magnitude >= 50) return "high";
  if (magnitude >= 25) return "medium";
  return "low";
}

function lookupKey(key: string): CatalogEntry | null {
  const exact = catalog.exact[key as keyof typeof catalog.exact];
  if (exact) return exact;

  for (const rule of catalog.prefix_rules) {
    if (!key.startsWith(rule.prefix)) continue;
    const rest = key.slice(rule.prefix.length);
    const suffixes = rule.suffixes as Record<string, CatalogEntry>;
    if (suffixes[rest]) return suffixes[rest];
    for (const [suffixKey, entry] of Object.entries(suffixes)) {
      if (suffixKey.endsWith(":*")) {
        const prefix = suffixKey.slice(0, -2);
        if (rest.startsWith(prefix)) return entry;
      }
    }
    if (suffixes["*"]) return suffixes["*"];
  }
  return null;
}

function fallbackTitle(signal: string): string {
  const titles = catalog.signal_titles as Record<string, string>;
  return titles[signal] ?? `Attention signal '${signal}' needs attention`;
}

/** Resolve one Attention reason into display wording. Deterministic; no ranking. */
export function resolveAttentionReason(reason: AttentionReason): DisplayReason {
  const importance = displayImportanceFromWeight(reason.weight);
  const looked = lookupKey(reason.explanation_key);
  if (looked) {
    return {
      title: looked.title,
      description: looked.description,
      importance,
      explanation_key: reason.explanation_key,
      signal: reason.signal,
      source: reason.source,
      weight: reason.weight,
      known: true,
    };
  }
  return {
    title: fallbackTitle(reason.signal),
    description: `No Experience translation for '${reason.explanation_key}' yet (signal ${reason.signal} from ${reason.source}, weight ${reason.weight}).`,
    importance,
    explanation_key: reason.explanation_key,
    signal: reason.signal,
    source: reason.source,
    weight: reason.weight,
    known: false,
  };
}

/** Resolve many reasons in input order — no reordering, no filtering. */
export function resolveAttentionReasons(
  reasons: AttentionReason[],
): DisplayReason[] {
  return reasons.map(resolveAttentionReason);
}

/**
 * Decision Engine reasons: prefer Attention translation when present;
 * otherwise keep Decision's own factual summary as the display title.
 */
export function resolveDecisionReason(reason: DecisionReason): DisplayReason {
  if (reason.attention_reason) {
    return resolveAttentionReason(reason.attention_reason);
  }
  return {
    title: reason.summary,
    description: reason.kind,
    importance: "medium",
    explanation_key: `decision.${reason.kind}`,
    signal: reason.kind,
    source: "decision_engine",
    weight: 0,
    known: true,
  };
}

export function resolveDecisionReasons(
  reasons: DecisionReason[],
): DisplayReason[] {
  return reasons.map(resolveDecisionReason);
}

/** Catalog version for contract diagnostics. */
export function explanationCatalogVersion(): number {
  return catalog.version;
}
