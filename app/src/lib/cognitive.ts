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
      recency * 0.42 + substance * 0.28 + resume * 0.2 + reflect * 0.1,
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
