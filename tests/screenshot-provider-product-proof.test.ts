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
      "docs/capability-runtime/product-proof/screenshot-provider.proof.json",
    ),
    "utf8",
  ),
) as {
  examples: Array<{
    utterance: string;
    expectedKind: string;
    expectedDomain?: string | null;
    expectedOperation?: string;
    expectedQuery?: string;
    expectedMonitorIndex?: number;
  }>;
  clarifications: Array<{
    utterance: string;
    expectedKind: string;
    expectedQuery?: string;
    expectedMonitorIndex?: number;
  }>;
  ipc: string;
  program: string;
  independenceRule: boolean;
};

describe("Screenshot Provider Product Proof harness (P15)", () => {
  it("declares Kernel Operator Conversation IPC and Independence Rule", () => {
    expect(proof.ipc).toBe("execute_capability_intent");
    expect(proof.program).toBe("P15");
    expect(proof.independenceRule).toBe(true);
  });

  it("routes Owner-facing conversation examples with NL robustness", () => {
    for (const example of proof.examples) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);

      if (
        example.expectedQuery != null &&
        "query" in action &&
        typeof action.query === "string"
      ) {
        expect(action.query.toLowerCase(), example.utterance).toBe(
          example.expectedQuery.toLowerCase(),
        );
      }
      if (
        example.expectedMonitorIndex != null &&
        "monitorIndex" in action &&
        typeof action.monitorIndex === "number"
      ) {
        expect(action.monitorIndex, example.utterance).toBe(
          example.expectedMonitorIndex,
        );
      }

      const capability = toCapabilityIntent(action);
      if (example.expectedDomain != null) {
        expect(capability?.domain, example.utterance).toBe(
          example.expectedDomain,
        );
      }
      if (example.expectedOperation != null) {
        expect(capability?.operation, example.utterance).toBe(
          example.expectedOperation,
        );
      }

      expect(JSON.stringify(action).toLowerCase()).not.toMatch(
        /screenshot provider|capability runtime|xcap|dxgi|wgc|provider registry|kernel operator/,
      );
    }
  });

  it("routes failure-oriented utterances without inventing architecture jargon", () => {
    for (const example of proof.clarifications) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);
      if (
        example.expectedQuery != null &&
        "query" in action &&
        typeof action.query === "string"
      ) {
        expect(action.query.toLowerCase()).toBe(
          example.expectedQuery.toLowerCase(),
        );
      }
      if (
        example.expectedMonitorIndex != null &&
        "monitorIndex" in action &&
        typeof action.monitorIndex === "number"
      ) {
        expect(action.monitorIndex).toBe(example.expectedMonitorIndex);
      }
      expect(JSON.stringify(action).toLowerCase()).not.toMatch(
        /screenshot provider|capability runtime|xcap|dxgi/,
      );
    }
  });
});
