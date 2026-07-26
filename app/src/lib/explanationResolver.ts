/**
 * Experience explanation resolver (Sprint 130–132).
 *
 * Translates AttentionReason.explanation_key via the canonical catalog.
 * Resolution order and templates are catalog-owned — no duplicated rules here.
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

type PrefixPattern = {
  starts_with: string;
  title: string;
  description: string;
};

type PrefixRule = {
  readonly prefix: string;
  readonly suffixes?: Readonly<Record<string, CatalogEntry>>;
  readonly patterns?: ReadonlyArray<PrefixPattern>;
  readonly fallback?: CatalogEntry;
};

function applyTemplate(template: string, replacements: Record<string, string>): string {
  return Object.entries(replacements).reduce(
    (acc, [key, value]) => acc.split(`{${key}}`).join(value),
    template,
  );
}

export function displayImportanceFromWeight(weight: number): DisplayImportance {
  const magnitude = Math.abs(weight);
  if (magnitude >= 50) return "high";
  if (magnitude >= 25) return "medium";
  return "low";
}

/** Catalog-owned resolution: exact → suffix → pattern → prefix fallback. */
export function lookupCatalogEntry(key: string): CatalogEntry | null {
  const exact = catalog.exact[key as keyof typeof catalog.exact];
  if (exact) return exact;

  for (const rawRule of catalog.prefix_rules) {
    const rule = rawRule as PrefixRule;
    if (!key.startsWith(rule.prefix)) continue;
    const rest = key.slice(rule.prefix.length);
    const suffixes = rule.suffixes as Record<string, CatalogEntry> | undefined;
    if (suffixes?.[rest]) return suffixes[rest];
    for (const pattern of rule.patterns ?? []) {
      if (rest.startsWith(pattern.starts_with)) {
        return { title: pattern.title, description: pattern.description };
      }
    }
    if (rule.fallback) return rule.fallback;
  }
  return null;
}

function fallbackTitle(signal: string): string {
  const titles = catalog.signal_titles as Record<string, string>;
  if (titles[signal]) return titles[signal];
  const template =
    catalog.resolution.unknown.signal_title_fallback_template;
  return applyTemplate(template, { signal });
}

function unknownDescription(
  explanationKey: string,
  signal: string,
  source: string,
  weight: number,
): string {
  const template = catalog.resolution.unknown.description_template;
  return applyTemplate(template, {
    explanation_key: explanationKey,
    signal,
    source,
    weight: String(weight),
  });
}

/** Resolve one Attention reason into display wording. Deterministic; no ranking. */
export function resolveAttentionReason(reason: AttentionReason): DisplayReason {
  const importance = displayImportanceFromWeight(reason.weight);
  const looked = lookupCatalogEntry(reason.explanation_key);
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
    description: unknownDescription(
      reason.explanation_key,
      reason.signal,
      reason.source,
      reason.weight,
    ),
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

/** Decision Engine reasons: Attention translation when present; else Decision summary. */
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

export function explanationCatalogVersion(): number {
  return catalog.version;
}
