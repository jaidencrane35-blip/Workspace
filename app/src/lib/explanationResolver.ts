/**
 * Experience explanation resolver (Sprint 130–135).
 *
 * Low-level resolver — UI components should import from `experienceTranslation.ts`.
 * Traced APIs are developer diagnostics only — never wire into Work/Assistant render paths.
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

export type ExperienceResolverPathKind =
  | "exact"
  | "prefix_suffix"
  | "prefix_pattern"
  | "prefix_fallback"
  | "unknown"
  | "decision_native";

export interface ExperienceResolverPath {
  kind: ExperienceResolverPathKind;
  /** e.g. exact:decision.base.outstanding or prefix_pattern:purpose.obstacle.composition:* */
  match_key: string;
}

/** Developer-only translation trace — not for normal UI rendering. */
export interface ExperienceTranslationTrace {
  source_reasoning_type: "attention_reason" | "decision_reason";
  source_identifier: string;
  explanation_key: string;
  resolver_path: ExperienceResolverPath;
  display: DisplayReason;
  rendering_surface: string | null;
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

type CatalogLookup = {
  entry: CatalogEntry;
  kind: Exclude<ExperienceResolverPathKind, "unknown" | "decision_native">;
  match_key: string;
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

/** Catalog-owned resolution with path identity for traces. */
export function lookupCatalogEntryWithPath(key: string): CatalogLookup | null {
  const exact = catalog.exact[key as keyof typeof catalog.exact];
  if (exact) {
    return {
      entry: exact,
      kind: "exact",
      match_key: `exact:${key}`,
    };
  }

  for (const rawRule of catalog.prefix_rules) {
    const rule = rawRule as PrefixRule;
    if (!key.startsWith(rule.prefix)) continue;
    const rest = key.slice(rule.prefix.length);
    const suffixes = rule.suffixes as Record<string, CatalogEntry> | undefined;
    if (suffixes?.[rest]) {
      return {
        entry: suffixes[rest],
        kind: "prefix_suffix",
        match_key: `prefix_suffix:${key}`,
      };
    }
    for (const pattern of rule.patterns ?? []) {
      if (rest.startsWith(pattern.starts_with)) {
        return {
          entry: { title: pattern.title, description: pattern.description },
          kind: "prefix_pattern",
          match_key: `prefix_pattern:${rule.prefix}${pattern.starts_with}*`,
        };
      }
    }
    if (rule.fallback) {
      return {
        entry: rule.fallback,
        kind: "prefix_fallback",
        match_key: `prefix_fallback:${rule.prefix}*`,
      };
    }
  }
  return null;
}

/** Catalog-owned resolution: exact → suffix → pattern → prefix fallback. */
export function lookupCatalogEntry(key: string): CatalogEntry | null {
  return lookupCatalogEntryWithPath(key)?.entry ?? null;
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
  return resolveAttentionReasonTraced(reason).display;
}

/** Developer diagnostic: resolve + translation trace. */
export function resolveAttentionReasonTraced(
  reason: AttentionReason,
  renderingSurface: string | null = null,
): ExperienceTranslationTrace {
  const importance = displayImportanceFromWeight(reason.weight);
  const looked = lookupCatalogEntryWithPath(reason.explanation_key);
  if (looked) {
    const display: DisplayReason = {
      title: looked.entry.title,
      description: looked.entry.description,
      importance,
      explanation_key: reason.explanation_key,
      signal: reason.signal,
      source: reason.source,
      weight: reason.weight,
      known: true,
    };
    return {
      source_reasoning_type: "attention_reason",
      source_identifier: reason.explanation_key,
      explanation_key: reason.explanation_key,
      resolver_path: { kind: looked.kind, match_key: looked.match_key },
      display,
      rendering_surface: renderingSurface,
    };
  }
  const display: DisplayReason = {
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
  return {
    source_reasoning_type: "attention_reason",
    source_identifier: reason.explanation_key,
    explanation_key: reason.explanation_key,
    resolver_path: {
      kind: "unknown",
      match_key: `unknown:${reason.explanation_key}`,
    },
    display,
    rendering_surface: renderingSurface,
  };
}

/** Resolve many reasons in input order — no reordering, no filtering. */
export function resolveAttentionReasons(
  reasons: AttentionReason[],
): DisplayReason[] {
  return reasons.map(resolveAttentionReason);
}

export function resolveAttentionReasonsTraced(
  reasons: AttentionReason[],
  renderingSurface: string | null = null,
): ExperienceTranslationTrace[] {
  return reasons.map((r) => resolveAttentionReasonTraced(r, renderingSurface));
}

/** Decision Engine reasons: Attention translation when present; else Decision summary. */
export function resolveDecisionReason(reason: DecisionReason): DisplayReason {
  return resolveDecisionReasonTraced(reason).display;
}

export function resolveDecisionReasonTraced(
  reason: DecisionReason,
  renderingSurface: string | null = null,
): ExperienceTranslationTrace {
  if (reason.attention_reason) {
    const trace = resolveAttentionReasonTraced(
      reason.attention_reason,
      renderingSurface,
    );
    return {
      ...trace,
      source_reasoning_type: "decision_reason",
      source_identifier: `decision.${reason.kind}→${reason.attention_reason.explanation_key}`,
    };
  }
  const explanationKey = `decision.${reason.kind}`;
  const display: DisplayReason = {
    title: reason.summary,
    description: reason.kind,
    importance: "medium",
    explanation_key: explanationKey,
    signal: reason.kind,
    source: "decision_engine",
    weight: 0,
    known: true,
  };
  return {
    source_reasoning_type: "decision_reason",
    source_identifier: reason.kind,
    explanation_key: explanationKey,
    resolver_path: {
      kind: "decision_native",
      match_key: `decision_native:${explanationKey}`,
    },
    display,
    rendering_surface: renderingSurface,
  };
}

export function resolveDecisionReasons(
  reasons: DecisionReason[],
): DisplayReason[] {
  return reasons.map(resolveDecisionReason);
}

export function resolveDecisionReasonsTraced(
  reasons: DecisionReason[],
  renderingSurface: string | null = null,
): ExperienceTranslationTrace[] {
  return reasons.map((r) => resolveDecisionReasonTraced(r, renderingSurface));
}

export function explanationCatalogVersion(): number {
  return catalog.version;
}

/** Compact developer label matching Rust `ExperienceResolverPath::label`. */
export function formatResolverPathLabel(path: ExperienceResolverPath): string {
  return `${path.kind}:\n${path.match_key}`;
}
