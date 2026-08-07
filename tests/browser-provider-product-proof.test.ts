import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const proof = JSON.parse(
  readFileSync(
    path.join(
      root,
      "docs/capability-runtime/product-proof/browser-provider.proof.json",
    ),
    "utf8",
  ),
) as {
  examples: Array<{
    utterance: string;
    expectedKind: string;
    expectedUrl?: string;
    expectedBeside?: string;
  }>;
  clarifications: Array<{
    utterance: string;
    expectedKind: string;
    mustMention: string;
  }>;
  ipc: string;
};

describe("Browser Provider Product Proof harness", () => {
  it("routes Owner-facing conversation examples to browser intents", () => {
    expect(proof.ipc).toBe("execute_capability_intent");
    for (const example of proof.examples) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);
      if (example.expectedUrl != null && "url" in action) {
        expect(action.url, example.utterance).toBe(example.expectedUrl);
      }
      if (example.expectedBeside != null && action.kind === "browserOpenBeside") {
        expect(action.beside).toBe(example.expectedBeside);
      }
      const capability = toCapabilityIntent(action);
      expect(capability?.domain).toBe("browser");
      expect(JSON.stringify(action).toLowerCase()).not.toMatch(
        /browser provider|capability runtime|webbrowser/,
      );
    }
  });

  it("clarifies missing website targets", () => {
    for (const example of proof.clarifications) {
      const action = resolveIntent(example.utterance);
      expect(action.kind).toBe(example.expectedKind);
      expect(action.reply.toLowerCase()).toContain(
        example.mustMention.toLowerCase(),
      );
    }
  });
});
