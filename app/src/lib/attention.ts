/** Attention Engine — continuous visual priority for Workspace Objects. */

export type AttentionTier = "primary" | "secondary" | "context";

export type AttentionScene =
  | "default"
  | "writing"
  | "restore"
  | "empty"
  | "guide"
  | "checkin";

export interface AttentionVisual {
  weight: number;
  tier: AttentionTier;
  scale: number;
  opacity: number;
  blur: number;
  y: number;
  elevation: 1 | 2 | 3;
  lit: boolean;
  interactionPriority: number;
}

export const ATTENTION_WEIGHT = {
  primary: 1,
  secondary: 0.62,
  context: 0.32,
} as const;

export function tierFromWeight(weight: number): AttentionTier {
  if (weight >= 0.85) {
    return "primary";
  }
  if (weight >= 0.48) {
    return "secondary";
  }
  return "context";
}

/** Map continuous attention weight → visual presentation. */
export function resolveAttentionVisual(weight: number): AttentionVisual {
  const clamped = Math.max(0, Math.min(1, weight));
  const tier = tierFromWeight(clamped);
  return {
    weight: clamped,
    tier,
    scale: 0.94 + clamped * 0.1,
    opacity: 0.38 + clamped * 0.62,
    blur: (1 - clamped) * 3.2,
    y: (1 - clamped) * 6,
    elevation: clamped >= 0.85 ? 3 : clamped >= 0.48 ? 2 : 1,
    lit: clamped >= 0.82,
    interactionPriority: Math.round(clamped * 100),
  };
}

/**
 * Derive per-object weights from a primary id and registered roles.
 * Exactly one object is primary when primaryId is set.
 */
export function computeAttentionWeights(
  objectIds: string[],
  primaryId: string | null,
  secondaryIds: ReadonlySet<string> = new Set(),
): Record<string, number> {
  const weights: Record<string, number> = {};
  if (objectIds.length === 0) {
    return weights;
  }

  const resolvedPrimary =
    primaryId && objectIds.includes(primaryId)
      ? primaryId
      : objectIds[0] ?? null;

  for (const id of objectIds) {
    if (id === resolvedPrimary) {
      weights[id] = ATTENTION_WEIGHT.primary;
    } else if (secondaryIds.has(id)) {
      weights[id] = ATTENTION_WEIGHT.secondary;
    } else {
      weights[id] = ATTENTION_WEIGHT.context;
    }
  }
  return weights;
}
