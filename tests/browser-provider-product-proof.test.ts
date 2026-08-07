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
    expectedDomain?: string | null;
    expectedUrl?: string;
    expectedBeside?: string;
    expectedQuery?: string;
  }>;
  clarifications: Array<{
    utterance: string;
    expectedKind: string;
    mustMention: string;
  }>;
  ipc: string;
  program: string;
};

describe("Browser Provider Product Proof harness (P14.5)", () => {
  it("declares Kernel Operator Conversation IPC", () => {
    expect(proof.ipc).toBe("execute_capability_intent");
    expect(proof.program).toBe("P14.5");
  });

  it("routes Owner-facing conversation examples with NL robustness", () => {
    for (const example of proof.examples) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);

      if (example.expectedUrl != null && "url" in action) {
        expect(action.url, example.utterance).toBe(example.expectedUrl);
      }
      if (
        example.expectedBeside != null &&
        action.kind === "browserOpenBeside"
      ) {
        expect(action.beside.toLowerCase()).toBe(
          example.expectedBeside.toLowerCase(),
        );
      }
      if (
        example.expectedQuery != null &&
        action.kind === "winFocus" &&
        "query" in action
      ) {
        expect(action.query.toLowerCase()).toBe(
          example.expectedQuery.toLowerCase(),
        );
      }

      const capability = toCapabilityIntent(action);
      if (example.expectedDomain === null) {
        expect(capability, example.utterance).toBeNull();
      } else if (example.expectedDomain != null) {
        expect(capability?.domain, example.utterance).toBe(
          example.expectedDomain,
        );
      }

      expect(JSON.stringify(action).toLowerCase()).not.toMatch(
        /browser provider|capability runtime|webbrowser|provider registry|kernel operator/,
      );
    }
  });

  it("clarifies missing and invalid website targets without launching", () => {
    for (const example of proof.clarifications) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);
      expect(action.reply.toLowerCase(), example.utterance).toContain(
        example.mustMention.toLowerCase(),
      );
      expect(toCapabilityIntent(action)).toBeNull();
    }
  });

  it("never opens Guide for browser capability discovery", () => {
    const action = resolveIntent("What can you do with browsers?");
    expect(action.kind).toBe("browserExplain");
    expect(action.kind).not.toBe("navigate");
    if (action.kind === "browserExplain") {
      expect(action.reply.toLowerCase()).toMatch(/open websites|beside/);
      expect(action.reply.toLowerCase()).not.toMatch(
        /provider|runtime|registry|kernel|ipc/,
      );
    }
  });
});
