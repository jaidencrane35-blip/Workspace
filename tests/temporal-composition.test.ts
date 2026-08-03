/**
 * Sprint 48 — temporal continuity unified into CognitiveEngine scoring.
 */
import { describe, expect, it } from "vitest";
import {
  composeSemanticField,
  composeSemanticWindowField,
  decideGuideHint,
  resolveTemporalPhase,
  scoreMomentRelevance,
  scoreTemporalWeight,
  type CognitiveMomentInput,
} from "../app/src/lib/cognitive";
import type { ActionPlanItem } from "../app/src/types/domain";

const hour = 60 * 60 * 1000;
const day = 24 * hour;
const now = Date.parse("2026-08-03T02:00:00.000Z");

const moment = (
  id: string,
  overrides: Partial<CognitiveMomentInput> = {},
): CognitiveMomentInput => ({
  id,
  createdAt: new Date(now - hour).toISOString(),
  capturedAt: new Date(now - hour).toISOString(),
  windowCount: 2,
  handoffLength: 40,
  resumeAffinity: 0,
  reflectionAffinity: 0,
  permanence: 0,
  intentMemory: 0,
  ...overrides,
});

describe("temporal workspace continuity", () => {
  it("unifies temporal and semantic weights into one relevance score", () => {
    const fresh = scoreMomentRelevance(
      moment("fresh", {
        createdAt: new Date(now - 0.5 * hour).toISOString(),
        capturedAt: new Date(now - 0.5 * hour).toISOString(),
        resumeAffinity: 0.2,
      }),
      now,
    );
    const dormant = scoreMomentRelevance(
      moment("dormant", {
        createdAt: new Date(now - 8 * day).toISOString(),
        capturedAt: new Date(now - 7 * day).toISOString(),
        resumeAffinity: 0,
        permanence: 0,
      }),
      now,
    );
    expect(fresh).toBeGreaterThan(dormant);
    expect(scoreTemporalWeight(moment("fresh"), now)).toBeGreaterThan(
      scoreTemporalWeight(
        moment("dormant", {
          createdAt: new Date(now - 8 * day).toISOString(),
          capturedAt: new Date(now - 7 * day).toISOString(),
        }),
        now,
      ),
    );
  });

  it("resolves temporal phases without labels or badges", () => {
    expect(
      resolveTemporalPhase(
        moment("n", {
          createdAt: new Date(now - 0.05 * day).toISOString(),
          permanence: 0,
        }),
        now,
      ),
    ).toBe("nascent");
    expect(
      resolveTemporalPhase(moment("r", { resumeAffinity: 0.8 }), now),
    ).toBe("resumed");
    expect(
      resolveTemporalPhase(
        moment("d", {
          createdAt: new Date(now - 6 * day).toISOString(),
          capturedAt: new Date(now - 5 * day).toISOString(),
          resumeAffinity: 0,
        }),
        now,
      ),
    ).toBe("dormant");
  });

  it("pulls resumed work nearer and quiets dormant work in the field", () => {
    const field = composeSemanticField(
      [
        { id: "hot", score: 0.72, phase: "resumed" },
        { id: "cold", score: 0.48, phase: "dormant" },
      ],
      "presence",
      3,
    );
    const hot = field.find((p) => p.id === "hot")!;
    const cold = field.find((p) => p.id === "cold")!;
    expect(hot.proximity).toBeGreaterThan(cold.proximity);
    expect(hot.opacity).toBeGreaterThan(cold.opacity);
    expect(cold.phase).toBe("dormant");
  });

  it("reconstructs recently resumed work with higher temporal confidence", () => {
    const items: ActionPlanItem[] = [
      {
        item_id: "a",
        action_type: "restore_window",
        target_summary: "Deck",
        proposed_effect: {},
        permission_scope: "desktop",
        projected_disposition: "will_attempt",
      },
    ];
    const resumed = composeSemanticWindowField(items, 5, 0.9);
    const dormant = composeSemanticWindowField(items, 5, 0.15);
    expect(resumed[0]!.placement.opacity).toBeGreaterThan(
      dormant[0]!.placement.opacity,
    );
    expect(resumed[0]!.placement.scale).toBeGreaterThan(
      dormant[0]!.placement.scale,
    );
  });

  it("fades guide hints as temporal certainty grows", () => {
    const uncertain = decideGuideHint({
      hasPrimary: true,
      neighbourCount: 2,
      resumeAffinityMax: 0.05,
      reflectionDepth: 0.05,
      idleVisits: 0,
      temporalCertainty: 0.1,
    });
    const certain = decideGuideHint({
      hasPrimary: true,
      neighbourCount: 2,
      resumeAffinityMax: 0.8,
      reflectionDepth: 0.7,
      idleVisits: 4,
      temporalCertainty: 0.85,
    });
    expect(uncertain.show).toBe(true);
    expect(certain.show).toBe(false);
    expect(certain.confidence).toBeLessThan(uncertain.confidence);
  });

  it("permanence after write raises relevance for the active Moment", () => {
    const before = scoreMomentRelevance(moment("m"), now);
    const after = scoreMomentRelevance(moment("m", { permanence: 0.75 }), now);
    expect(after).toBeGreaterThan(before);
  });
});
