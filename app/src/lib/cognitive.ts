/**
 * Cognitive scoring pipeline — single place for attention / relevance / yield.
 * Unifies semantic relationship and temporal continuity into one weight.
 */

import type { ActionPlanItem } from "../types/domain";
import {
  ATTENTION_WEIGHT,
  type AttentionScene,
  resolveAttentionVisual,
  type AttentionVisual,
} from "./attention";

const DAY_MS = 1000 * 60 * 60 * 24;

export interface CognitiveMomentInput {
  id: string;
  createdAt: string;
  /** Last observed activity; falls back to createdAt. */
  capturedAt?: string;
  windowCount: number;
  handoffLength: number;
  /** Quiet boost after a recent Continue/resume of this Moment. */
  resumeAffinity: number;
  /** Quiet affinity from Check-in reflection (0–1). */
  reflectionAffinity: number;
  /** Strengthened permanence after a completed write (0–1). */
  permanence?: number;
  /** Slow intent memory from repeated reflection (0–1). */
  intentMemory?: number;
}

/** Temporal life of a Moment — expressed spatially, never as a label. */
export type TemporalPhase =
  | "nascent"
  | "evolving"
  | "resumed"
  | "waiting"
  | "dormant";

export interface GuideSignal {
  hasPrimary: boolean;
  neighbourCount: number;
  resumeAffinityMax: number;
  reflectionDepth: number;
  idleVisits: number;
  /** Accumulated temporal familiarity (0–1). */
  temporalCertainty?: number;
}

export type GuideHintId = "save" | "continue" | "checkin";

export interface GuideDecision {
  show: boolean;
  hintId: GuideHintId;
  confidence: number;
}

function clamp01(value: number): number {
  return Math.max(0, Math.min(1, value));
}

/** Age in days from an ISO timestamp. */
export function ageInDays(iso: string, nowMs: number = Date.now()): number {
  const t = Date.parse(iso);
  if (Number.isNaN(t)) {
    return 14;
  }
  return Math.max(0, (nowMs - t) / DAY_MS);
}

function activeIso(moment: CognitiveMomentInput): string {
  return moment.capturedAt ?? moment.createdAt;
}

/**
 * Infer temporal phase from unified signals — no badges, counters, or labels.
 */
export function resolveTemporalPhase(
  moment: CognitiveMomentInput,
  nowMs: number = Date.now(),
): TemporalPhase {
  const ageCreated = ageInDays(moment.createdAt, nowMs);
  const ageActive = ageInDays(activeIso(moment), nowMs);
  const permanence = moment.permanence ?? 0;
  const resume = moment.resumeAffinity;

  if (resume >= 0.55) {
    return "resumed";
  }
  if (ageCreated < 0.2 && permanence < 0.28) {
    return "nascent";
  }
  if (permanence >= 0.32 || (ageActive < 1.1 && moment.windowCount > 0 && resume > 0.12)) {
    return "evolving";
  }
  if (ageActive >= 3.5 || (ageCreated >= 4 && resume < 0.12 && permanence < 0.2)) {
    return "dormant";
  }
  return "waiting";
}

/**
 * Temporal continuity weight (0–1) — freshness, resume, permanence, intent.
 * Single decay curve; no parallel recency heuristics elsewhere.
 */
export function scoreTemporalWeight(
  moment: CognitiveMomentInput,
  nowMs: number = Date.now(),
): number {
  const ageActive = ageInDays(activeIso(moment), nowMs);
  const ageCreated = ageInDays(moment.createdAt, nowMs);
  const freshness = Math.max(0.1, Math.exp(-ageActive / 11));
  const nascence = ageCreated < 0.35 ? 0.12 : 0;
  const resume = clamp01(moment.resumeAffinity);
  const permanence = clamp01(moment.permanence ?? 0);
  const intent = clamp01(
    Math.max(moment.intentMemory ?? 0, moment.reflectionAffinity * 0.65),
  );
  const dormancy =
    ageActive > 4 ? Math.max(0.22, 1 - (ageActive - 4) / 18) : 1;

  return clamp01(
    (freshness * 0.36 +
      resume * 0.28 +
      permanence * 0.18 +
      intent * 0.14 +
      nascence) *
      dormancy,
  );
}

/**
 * Semantic substance weight (0–1) — windows, handoff, reflection affinity.
 */
export function scoreSemanticWeight(moment: CognitiveMomentInput): number {
  const substance = Math.min(
    1,
    0.32 +
      moment.windowCount * 0.09 +
      Math.min(0.28, moment.handoffLength / 150),
  );
  const reflect = clamp01(moment.reflectionAffinity);
  return clamp01(substance * 0.72 + reflect * 0.28);
}

/**
 * Unified Moment relevance — temporal × semantic in one weighting function.
 * Higher → closer to the active cluster / more optical energy.
 */
export function scoreMomentRelevance(
  moment: CognitiveMomentInput,
  nowMs: number = Date.now(),
): number {
  const temporal = scoreTemporalWeight(moment, nowMs);
  const semantic = scoreSemanticWeight(moment);
  return Math.max(0.12, Math.min(1, temporal * 0.54 + semantic * 0.46));
}

/** Rank Moments for neighbour placement — primary excluded by caller. */
export function rankMomentsByRelevance(
  moments: CognitiveMomentInput[],
  nowMs: number = Date.now(),
): Array<{ id: string; score: number; phase: TemporalPhase }> {
  return [...moments]
    .map((m) => ({
      id: m.id,
      score: scoreMomentRelevance(m, nowMs),
      phase: resolveTemporalPhase(m, nowMs),
    }))
    .sort((a, b) => b.score - a.score);
}

/**
 * Window reconstruction priority — disposition × temporal confidence.
 */
export function scoreWindowImportance(
  item: ActionPlanItem,
  temporalConfidence = 0.5,
): number {
  let score = 0.38;
  if (item.projected_disposition === "will_attempt") {
    score += 0.32 + temporalConfidence * 0.12;
  } else if (item.projected_disposition === "will_skip_unresolvable") {
    score += 0.06;
  }
  if (item.target_summary.length < 48) {
    score += 0.07;
  } else if (item.target_summary.length > 72) {
    score -= 0.04;
  }
  return score * (0.78 + temporalConfidence * 0.28);
}

export function sortWindowsByImportance(
  items: ActionPlanItem[],
  temporalConfidence = 0.5,
): ActionPlanItem[] {
  return [...items].sort(
    (a, b) =>
      scoreWindowImportance(b, temporalConfidence) -
      scoreWindowImportance(a, temporalConfidence),
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
  temporalConfidence = 0.5,
): Array<{ item: ActionPlanItem; placement: SemanticWindowPlacement }> {
  const ordered = sortWindowsByImportance(items, temporalConfidence).slice(
    0,
    limit,
  );
  if (ordered.length === 0) {
    return [];
  }
  const max = Math.max(
    ...ordered.map((item) => scoreWindowImportance(item, temporalConfidence)),
    0.01,
  );
  const slots = [
    { angle: -0.7, baseY: 12 },
    { angle: 0.55, baseY: 8 },
    { angle: 0.15, baseY: 72 },
    { angle: -1.15, baseY: 88 },
    { angle: 1.05, baseY: 96 },
  ] as const;

  return ordered.map((item, index) => {
    const importance =
      scoreWindowImportance(item, temporalConfidence) / max;
    const skip = item.projected_disposition !== "will_attempt";
    const slot = slots[index] ?? slots[slots.length - 1]!;
    const radius = 36 + (1 - importance) * (120 - temporalConfidence * 24);
    const x = Math.round(Math.sin(slot.angle) * radius * 1.55);
    const y = Math.round(slot.baseY + (1 - importance) * 28);
    const confidenceScale = 0.8 + temporalConfidence * 0.18;
    return {
      item,
      placement: {
        itemId: item.item_id,
        importance,
        x,
        y,
        scale: (0.8 + importance * 0.22) * confidenceScale,
        opacity: skip
          ? 0.28
          : 0.48 + importance * 0.42 * (0.85 + temporalConfidence * 0.2),
        zIndex: Math.round(1 + importance * 4),
      },
    };
  });
}

/**
 * Guide is observational — fades as temporal certainty grows;
 * surfaces only when evidence suggests genuine uncertainty.
 */
export function decideGuideHint(signal: GuideSignal): GuideDecision {
  const certainty = clamp01(
    signal.temporalCertainty ??
      signal.resumeAffinityMax * 0.45 +
        signal.reflectionDepth * 0.35 +
        Math.min(1, signal.idleVisits / 5) * 0.2,
  );

  if (!signal.hasPrimary) {
    return { show: true, hintId: "save", confidence: 0.84 };
  }
  if (signal.neighbourCount === 0) {
    return {
      show: certainty < 0.72,
      hintId: "save",
      confidence: Math.max(0.2, 0.8 - certainty * 0.45),
    };
  }
  if (certainty >= 0.62) {
    // Familiar over time — stay silent.
    return {
      show: false,
      hintId: "continue",
      confidence: Math.max(0.12, 0.4 - certainty * 0.25),
    };
  }
  if (signal.resumeAffinityMax < 0.22 && signal.idleVisits < 3) {
    return {
      show: true,
      hintId: "continue",
      confidence: Math.max(0.45, 0.78 - certainty * 0.35),
    };
  }
  if (signal.reflectionDepth < 0.18 && signal.idleVisits >= 1) {
    return {
      show: true,
      hintId: "checkin",
      confidence: Math.max(0.4, 0.72 - certainty * 0.3),
    };
  }
  return {
    show: false,
    hintId: "continue",
    confidence: Math.max(0.1, 0.32 - certainty * 0.2),
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

/** Decay affinity / permanence over time (organisation tick). */
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

/** Spatial placement derived from unified cognitive relevance. */
export interface SemanticPlacement {
  id: string;
  score: number;
  proximity: number;
  x: number;
  y: number;
  scale: number;
  opacity: number;
  band: SemanticBand;
  phase: TemporalPhase;
}

export interface RankedMoment {
  id: string;
  score: number;
  phase?: TemporalPhase;
}

function phaseSpatialBias(phase: TemporalPhase): {
  proximity: number;
  scale: number;
  opacity: number;
} {
  switch (phase) {
    case "nascent":
      return { proximity: 0.08, scale: 0.04, opacity: 0.08 };
    case "evolving":
      return { proximity: 0.1, scale: 0.06, opacity: 0.06 };
    case "resumed":
      return { proximity: 0.14, scale: 0.08, opacity: 0.1 };
    case "waiting":
      return { proximity: -0.02, scale: -0.02, opacity: -0.04 };
    case "dormant":
      return { proximity: -0.16, scale: -0.08, opacity: -0.18 };
  }
}

/**
 * Compose neighbour positions from unified cognitive scores.
 * Recent work gravitates inward; dormant work quiets without vanishing.
 */
export function composeSemanticField(
  ranked: RankedMoment[],
  presence: SemanticPresence,
  limit = 3,
): SemanticPlacement[] {
  const slice = ranked.slice(0, limit);
  if (slice.length === 0) {
    return [];
  }

  return slice.map((entry, index) => {
    const phase = entry.phase ?? "waiting";
    const bias = phaseSpatialBias(phase);
    let proximity = entry.score;

    if (presence === "writing") {
      proximity =
        entry.score >= 0.5
          ? Math.min(1, entry.score * 1.18)
          : entry.score * 0.48;
    } else if (presence === "restoring") {
      proximity = Math.min(1, entry.score * 1.08 + (phase === "resumed" ? 0.06 : 0));
    } else if (presence === "reflecting") {
      proximity = Math.min(1, entry.score * 1.12);
    } else if (presence === "guided") {
      proximity = Math.min(1, 0.55 + entry.score * 0.4);
    }

    proximity = clamp01(proximity + bias.proximity);

    const rankAccent = 1 - index / Math.max(1, slice.length);
    proximity = Math.max(
      0.08,
      Math.min(1, proximity * (0.68 + rankAccent * 0.34)),
    );
    if (presence === "writing") {
      proximity = Math.max(0.08, proximity - index * 0.14);
    }
    if (phase === "dormant") {
      proximity = Math.max(0.08, proximity * 0.72);
    }

    const band: SemanticBand =
      proximity >= 0.62 ? "near" : proximity >= 0.38 ? "mid" : "far";

    const slots = [
      { angle: -0.95, baseY: 8 },
      { angle: 0.95, baseY: 18 },
      { angle: 0.08, baseY: 54 },
    ] as const;
    const slot = slots[index] ?? slots[2]!;
    const radius = 72 + (1 - proximity) * 150;
    const x = Math.round(Math.sin(slot.angle) * radius * 1.35);
    const y = Math.round(slot.baseY + (1 - proximity) * 42);

    let scale = 0.78 + proximity * 0.22 + bias.scale;
    let opacity =
      presence === "writing"
        ? 0.16 + proximity * 0.7
        : presence === "restoring"
          ? 0.28 + proximity * 0.45
          : 0.34 + proximity * 0.55;
    opacity = clamp01(opacity + bias.opacity);

    return {
      id: entry.id,
      score: entry.score,
      proximity,
      x,
      y,
      scale: Math.max(0.68, Math.min(1.05, scale)),
      opacity: Math.max(0.14, Math.min(0.94, opacity)),
      band,
      phase,
    };
  });
}
