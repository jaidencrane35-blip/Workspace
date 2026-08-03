/**
 * Cognitive scoring pipeline — single place for attention / relevance / yield.
 * Consolidates former attention-weight + intent scene modulation heuristics.
 */

import type { ActionPlanItem } from "../types/domain";
import {
  ATTENTION_WEIGHT,
  type AttentionScene,
  resolveAttentionVisual,
  type AttentionVisual,
} from "./attention";

export interface CognitiveMomentInput {
  id: string;
  createdAt: string;
  windowCount: number;
  handoffLength: number;
  /** Quiet boost after a recent Continue/resume of this Moment. */
  resumeAffinity: number;
  /** Quiet affinity from Check-in reflection (0–1). */
  reflectionAffinity: number;
}

export interface GuideSignal {
  hasPrimary: boolean;
  neighbourCount: number;
  resumeAffinityMax: number;
  reflectionDepth: number;
  idleVisits: number;
}

export type GuideHintId = "save" | "continue" | "checkin";

export interface GuideDecision {
  show: boolean;
  hintId: GuideHintId;
  confidence: number;
}

/** Recency decay in days — calm, not urgent. */
function recencyScore(iso: string, nowMs: number): number {
  const t = Date.parse(iso);
  if (Number.isNaN(t)) {
    return 0.35;
  }
  const days = Math.max(0, (nowMs - t) / (1000 * 60 * 60 * 24));
  return Math.max(0.15, Math.exp(-days / 12));
}

/**
 * Inferred Moment relevance for self-organisation (0–1).
 * Higher → closer to centre / more visual energy.
 */
export function scoreMomentRelevance(
  moment: CognitiveMomentInput,
  nowMs: number = Date.now(),
): number {
  const recency = recencyScore(moment.createdAt, nowMs);
  const substance = Math.min(
    1,
    0.35 + moment.windowCount * 0.08 + Math.min(0.25, moment.handoffLength / 160),
  );
  const resume = Math.min(1, moment.resumeAffinity);
  const reflect = Math.min(1, moment.reflectionAffinity);
  return Math.max(
    0.12,
    Math.min(
      1,
      recency * 0.38 + substance * 0.26 + resume * 0.2 + reflect * 0.16,
    ),
  );
}

/** Rank Moments for neighbour placement — primary excluded by caller. */
export function rankMomentsByRelevance(
  moments: CognitiveMomentInput[],
  nowMs: number = Date.now(),
): Array<{ id: string; score: number }> {
  return [...moments]
    .map((m) => ({ id: m.id, score: scoreMomentRelevance(m, nowMs) }))
    .sort((a, b) => b.score - a.score);
}

/**
 * Window reconstruction priority — focused / will_attempt first,
 * not plan enumeration order.
 */
export function scoreWindowImportance(item: ActionPlanItem): number {
  let score = 0.4;
  if (item.projected_disposition === "will_attempt") {
    score += 0.35;
  } else if (item.projected_disposition === "will_skip_unresolvable") {
    score += 0.08;
  }
  // Shorter titles tend to be the active document; longer often chrome.
  if (item.target_summary.length < 48) {
    score += 0.08;
  } else if (item.target_summary.length > 72) {
    score -= 0.04;
  }
  return score;
}

export function sortWindowsByImportance(
  items: ActionPlanItem[],
): ActionPlanItem[] {
  return [...items].sort(
    (a, b) => scoreWindowImportance(b) - scoreWindowImportance(a),
  );
}

/** Window placement in the restore neighbourhood — importance, not symmetry. */
export interface SemanticWindowPlacement {
  itemId: string;
  importance: number;
  x: number;
  y: number;
  scale: number;
  opacity: number;
  zIndex: number;
}

export function composeSemanticWindowField(
  items: ActionPlanItem[],
  limit = 5,
): Array<{ item: ActionPlanItem; placement: SemanticWindowPlacement }> {
  const ordered = sortWindowsByImportance(items).slice(0, limit);
  if (ordered.length === 0) {
    return [];
  }
  const max = Math.max(...ordered.map(scoreWindowImportance), 0.01);
  const slots = [
    { angle: -0.7, baseY: 12 },
    { angle: 0.55, baseY: 8 },
    { angle: 0.15, baseY: 72 },
    { angle: -1.15, baseY: 88 },
    { angle: 1.05, baseY: 96 },
  ] as const;

  return ordered.map((item, index) => {
    const importance = scoreWindowImportance(item) / max;
    const skip = item.projected_disposition !== "will_attempt";
    const slot = slots[index] ?? slots[slots.length - 1]!;
    const radius = 40 + (1 - importance) * 110;
    const x = Math.round(Math.sin(slot.angle) * radius * 1.55);
    const y = Math.round(slot.baseY + (1 - importance) * 28);
    return {
      item,
      placement: {
        itemId: item.item_id,
        importance,
        x,
        y,
        scale: 0.82 + importance * 0.2,
        opacity: skip ? 0.32 : 0.55 + importance * 0.4,
        zIndex: Math.round(1 + importance * 4),
      },
    };
  });
}

/**
 * Guide is observational — only surface a hint when confidence is high.
 */
export function decideGuideHint(signal: GuideSignal): GuideDecision {
  if (!signal.hasPrimary) {
    return { show: true, hintId: "save", confidence: 0.82 };
  }
  if (signal.neighbourCount === 0) {
    return { show: true, hintId: "save", confidence: 0.78 };
  }
  if (signal.resumeAffinityMax < 0.2 && signal.idleVisits < 2) {
    return { show: true, hintId: "continue", confidence: 0.74 };
  }
  if (signal.reflectionDepth < 0.15 && signal.idleVisits >= 1) {
    return { show: true, hintId: "checkin", confidence: 0.7 };
  }
  // High familiarity — stay silent (observational).
  return {
    show: false,
    hintId: "continue",
    confidence: 0.35,
  };
}

/**
 * Single weight pipeline: role base × relevance × scene yield.
 */
export function computeCognitiveWeights(args: {
  objectIds: string[];
  primaryId: string | null;
  secondaryIds: ReadonlySet<string>;
  relevanceById: Record<string, number>;
  scene: AttentionScene;
}): Record<string, number> {
  const { objectIds, primaryId, secondaryIds, relevanceById, scene } = args;
  const weights: Record<string, number> = {};
  if (objectIds.length === 0) {
    return weights;
  }

  const resolvedPrimary =
    primaryId && objectIds.includes(primaryId)
      ? primaryId
      : (objectIds[0] ?? null);

  for (const id of objectIds) {
    let base =
      id === resolvedPrimary
        ? ATTENTION_WEIGHT.primary
        : secondaryIds.has(id)
          ? ATTENTION_WEIGHT.secondary
          : ATTENTION_WEIGHT.context;

    const relevance = relevanceById[id];
    if (relevance != null && id !== resolvedPrimary) {
      // Calm rebalance — relevance steers context/secondary, not the anchor.
      base = base * (0.55 + relevance * 0.55);
    }

    weights[id] = modulateByScene(base, scene, id === resolvedPrimary);
  }
  return weights;
}

export function modulateByScene(
  weight: number,
  scene: AttentionScene,
  isPrimary: boolean,
): number {
  let adjusted = weight;
  if (scene === "writing") {
    adjusted = isPrimary ? Math.min(1, adjusted * 1.06) : adjusted * 0.42;
  } else if (scene === "restore") {
    adjusted = isPrimary ? adjusted : adjusted * 0.55;
  } else if (scene === "checkin") {
    adjusted = isPrimary ? adjusted : adjusted * 0.5;
  } else if (scene === "guide") {
    adjusted = isPrimary ? adjusted : adjusted * 0.48;
  } else if (scene === "empty" && !isPrimary) {
    adjusted = Math.min(adjusted, 0.4);
  }
  return Math.max(0.08, Math.min(1, adjusted));
}

/** Intent-kind soft yield (moved out of WorkspaceObject duplication). */
export function modulateByIntentKind(
  weight: number,
  intent: string,
  kind: string,
  objectId: string,
  influenceObjectId: string | null,
  slot: string,
): number {
  let next = weight;
  if (influenceObjectId && influenceObjectId !== objectId) {
    next *= slot === "orbit" || slot === "utility" ? 0.82 : 0.9;
  } else if (influenceObjectId === objectId) {
    next = Math.min(1, next + 0.08);
  }
  if (intent === "capture" && objectId !== "write-surface") {
    next *= 0.7;
  }
  if (intent === "restore" && kind !== "moment" && kind !== "continue-preview") {
    next *= 0.75;
  }
  if (intent === "learn" && kind !== "guide-step") {
    next *= 0.72;
  }
  if (intent === "reflect" && kind !== "checkin-summary") {
    next *= 0.8;
  }
  return Math.max(0.08, Math.min(1, next));
}

export function visualFromCognitiveWeight(weight: number): AttentionVisual {
  return resolveAttentionVisual(weight);
}

/** Decay resume affinity over time (called each organisation tick). */
export function decayAffinity(value: number, factor = 0.92): number {
  const next = value * factor;
  return next < 0.04 ? 0 : next;
}

/** Presence modes that reshape the semantic field. */
export type SemanticPresence =
  | "presence"
  | "writing"
  | "restoring"
  | "reflecting"
  | "guided";

export type SemanticBand = "near" | "mid" | "far";

/** Spatial placement derived from cognitive relevance — not a layout grid. */
export interface SemanticPlacement {
  id: string;
  score: number;
  /** 1 = beside the active work; 0 = archival distance. */
  proximity: number;
  /** Horizontal offset from field centre (px). */
  x: number;
  /** Vertical offset within the field (px). */
  y: number;
  scale: number;
  opacity: number;
  band: SemanticBand;
}

/**
 * Compose neighbour positions from cognitive scores.
 * Nearby ⇒ relevance; distant ⇒ archival context. No categories.
 */
export function composeSemanticField(
  ranked: Array<{ id: string; score: number }>,
  presence: SemanticPresence,
  limit = 3,
): SemanticPlacement[] {
  const slice = ranked.slice(0, limit);
  if (slice.length === 0) {
    return [];
  }

  return slice.map((entry, index) => {
    let proximity = entry.score;

    if (presence === "writing") {
      // Relevant Moments drift closer; unrelated quietly recede.
      proximity =
        entry.score >= 0.5
          ? Math.min(1, entry.score * 1.18)
          : entry.score * 0.48;
    } else if (presence === "restoring") {
      proximity = Math.min(1, entry.score * 1.08);
    } else if (presence === "reflecting") {
      proximity = Math.min(1, entry.score * 1.12);
    } else if (presence === "guided") {
      proximity = Math.min(1, 0.55 + entry.score * 0.4);
    }

    // Rank accent keeps neighbourhoods legible when scores cluster.
    const rankAccent = 1 - index / Math.max(1, slice.length);
    proximity = Math.max(
      0.08,
      Math.min(1, proximity * (0.68 + rankAccent * 0.34)),
    );
    if (presence === "writing") {
      proximity = Math.max(0.08, proximity - index * 0.14);
    }

    const band: SemanticBand =
      proximity >= 0.62 ? "near" : proximity >= 0.38 ? "mid" : "far";

    // Angular slots around the anchor — relationship, not row/column.
    const slots = [
      { angle: -0.95, baseY: 8 },
      { angle: 0.95, baseY: 18 },
      { angle: 0.08, baseY: 54 },
    ] as const;
    const slot = slots[index] ?? slots[2]!;
    const radius = 72 + (1 - proximity) * 150;
    const x = Math.round(Math.sin(slot.angle) * radius * 1.35);
    const y = Math.round(slot.baseY + (1 - proximity) * 42);

    const scale = 0.78 + proximity * 0.22;
    const opacity =
      presence === "writing"
        ? 0.16 + proximity * 0.7
        : presence === "restoring"
          ? 0.28 + proximity * 0.45
          : 0.34 + proximity * 0.55;

    return {
      id: entry.id,
      score: entry.score,
      proximity,
      x,
      y,
      scale: Math.max(0.72, Math.min(1, scale)),
      opacity: Math.max(0.16, Math.min(0.92, opacity)),
      band,
    };
  });
}
