import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { isEvolutionRequest } from "../app/src/lib/capabilityEvolution";
import { resolveIntent } from "../app/src/lib/intentBridge";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const proof = JSON.parse(
  readFileSync(
    path.join(
      root,
      "docs/capability-runtime/product-proof/window-provider.proof.json",
    ),
    "utf8",
  ),
) as {
  examples: Array<{
    utterance: string;
    expectedKind: string;
    expectedQuery?: string;
    expectedSnap?: string;
    expectedMonitorIndex?: number;
    expectedWidth?: number;
    expectedHeight?: number;
  }>;
  clarifications: Array<{
    utterance: string;
    expectedKind: string;
    mustMention: string;
  }>;
};

describe("Window Provider Product Proof harness", () => {
  it("routes Owner-facing conversation examples to Window intents", () => {
    for (const example of proof.examples) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);
      if (example.expectedQuery != null && "query" in action) {
        expect(action.query, example.utterance).toBe(example.expectedQuery);
      }
      if (example.expectedSnap != null && action.kind === "winSnap") {
        expect(action.snap).toBe(example.expectedSnap);
      }
      if (
        example.expectedMonitorIndex != null &&
        action.kind === "winMoveMonitor"
      ) {
        expect(action.monitorIndex).toBe(example.expectedMonitorIndex);
      }
      if (example.expectedWidth != null && action.kind === "winResize") {
        expect(action.width).toBe(example.expectedWidth);
        expect(action.height).toBe(example.expectedHeight);
      }
      expect(JSON.stringify(action).toLowerCase()).not.toMatch(
        /window provider|capability runtime|provider registry/,
      );
      expect(isEvolutionRequest(example.utterance)).toBe(false);
    }
  });

  it("asks for clarification instead of inventing placement", () => {
    for (const example of proof.clarifications) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);
      expect(action.reply.toLowerCase()).toContain(
        example.mustMention.toLowerCase(),
      );
    }
  });

  it("does not send Restore Chrome to Moment restore", () => {
    expect(resolveIntent("Restore Chrome.")).toMatchObject({
      kind: "winRestore",
      query: "Chrome",
    });
  });
});
