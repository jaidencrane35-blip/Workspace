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
      "docs/capability-runtime/product-proof/notifications-provider.proof.json",
    ),
    "utf8",
  ),
) as {
  examples: Array<{
    utterance: string;
    expectedKind: string;
    expectedText?: string;
  }>;
  clarifications: Array<{
    utterance: string;
    expectedKind: string;
    mustMention: string;
  }>;
  ipc: string;
  pipeline: string[];
};

describe("Notifications Provider Product Proof harness", () => {
  it("routes Owner-facing conversation examples to notification intents", () => {
    expect(proof.ipc).toBe("execute_capability_intent");
    expect(proof.pipeline).toContain("Kernel Operator");

    for (const example of proof.examples) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);
      if (example.expectedText != null && action.kind === "notifyShow") {
        expect(action.text, example.utterance).toBe(example.expectedText);
      }
      const capability = toCapabilityIntent(action);
      expect(capability?.domain).toBe("notifications");
      expect(JSON.stringify(action).toLowerCase()).not.toMatch(
        /notification provider|capability runtime|provider registry|winrt/,
      );
    }
  });

  it("clarifies deferred watching instead of inventing monitors", () => {
    for (const example of proof.clarifications) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);
      expect(action.reply.toLowerCase()).toContain(
        example.mustMention.toLowerCase(),
      );
    }
  });
});
