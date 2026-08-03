/**
 * Sprint 47 — semantic workspace composition from CognitiveEngine scores.
 */
import { describe, expect, it } from "vitest";
import {
  composeSemanticField,
  composeSemanticWindowField,
  scoreMomentRelevance,
  type CognitiveMomentInput,
} from "../app/src/lib/cognitive";
import type { ActionPlanItem } from "../app/src/types/domain";

const moment = (
  id: string,
  overrides: Partial<CognitiveMomentInput> = {},
): CognitiveMomentInput => ({
  id,
  createdAt: new Date().toISOString(),
  windowCount: 2,
  handoffLength: 40,
  resumeAffinity: 0,
  reflectionAffinity: 0,
  ...overrides,
});

describe("semantic workspace composition", () => {
  it("places higher relevance Moments nearer the anchor", () => {
    const ranked = [
      { id: "near", score: 0.85 },
      { id: "far", score: 0.25 },
    ];
    const field = composeSemanticField(ranked, "presence", 3);
    const near = field.find((p) => p.id === "near")!;
    const far = field.find((p) => p.id === "far")!;
    expect(near.proximity).toBeGreaterThan(far.proximity);
    expect(near.band).toBe("near");
    expect(far.band).toBe("far");
    const nearDist = Math.hypot(near.x, near.y);
    const farDist = Math.hypot(far.x, far.y);
    expect(nearDist).toBeLessThan(farDist);
  });

  it("pulls relevant Moments closer while writing and recedes unrelated ones", () => {
    const ranked = [
      { id: "related", score: 0.7 },
      { id: "unrelated", score: 0.3 },
    ];
    const idle = composeSemanticField(ranked, "presence", 3);
    const writing = composeSemanticField(ranked, "writing", 3);
    const relatedIdle = idle.find((p) => p.id === "related")!;
    const relatedWrite = writing.find((p) => p.id === "related")!;
    const unrelatedIdle = idle.find((p) => p.id === "unrelated")!;
    const unrelatedWrite = writing.find((p) => p.id === "unrelated")!;
    expect(relatedWrite.proximity).toBeGreaterThanOrEqual(relatedIdle.proximity);
    expect(unrelatedWrite.proximity).toBeLessThan(unrelatedIdle.proximity);
    expect(unrelatedWrite.opacity).toBeLessThan(relatedWrite.opacity);
  });

  it("reflection affinity reshapes relative Moment scores", () => {
    const base = scoreMomentRelevance(
      moment("a", { resumeAffinity: 0.8, reflectionAffinity: 0 }),
    );
    const reflected = scoreMomentRelevance(
      moment("a", { resumeAffinity: 0.8, reflectionAffinity: 0.7 }),
    );
    const cold = scoreMomentRelevance(
      moment("b", { resumeAffinity: 0, reflectionAffinity: 0.7 }),
    );
    expect(reflected).toBeGreaterThan(base);
    expect(reflected).toBeGreaterThan(cold);
  });

  it("orders restore windows by semantic importance, not input order", () => {
    const base = {
      action_type: "restore_window",
      proposed_effect: {},
      permission_scope: "desktop",
    };
    const items: ActionPlanItem[] = [
      {
        ...base,
        item_id: "skip",
        target_summary: "Very long chrome title that should not lead restore",
        projected_disposition: "will_skip_unresolvable",
      },
      {
        ...base,
        item_id: "focus",
        target_summary: "Deck",
        projected_disposition: "will_attempt",
      },
    ];
    const field = composeSemanticWindowField(items);
    expect(field[0]?.item.item_id).toBe("focus");
    expect(field[0]!.placement.importance).toBeGreaterThan(
      field[1]!.placement.importance,
    );
    expect(field[0]!.placement.zIndex).toBeGreaterThan(
      field[1]!.placement.zIndex,
    );
  });
});
