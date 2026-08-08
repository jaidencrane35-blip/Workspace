/**
 * P16.32 — Hostile natural-language validation for Semantic Intent Engine.
 * Generates hundreds of wording variations; measures deterministic success.
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import {
  isInventedExecutableQuery,
  resolveIntentWithEvidence,
} from "../app/src/lib/intentPipeline";
import { generateCapabilityDiscovery } from "../app/src/lib/capabilityRegistry";

const POLITE = ["", "please ", "can you ", "could you ", "would you "];
const TAILS = ["", " please", " for me", " now"];

function variants(core: string): string[] {
  const out: string[] = [];
  for (const p of POLITE) {
    for (const tail of TAILS) {
      const s = `${p}${core}${tail}`.replace(/\s+/g, " ").trim();
      out.push(s);
      out.push(`${s}.`);
      out.push(
        s.replace(/maximize/gi, "maximise").replace(/minimize/gi, "minimise"),
      );
    }
  }
  return [...new Set(out)];
}

const MUST_RESOLVE: Array<{
  cores: string[];
  accept: (kind: string, action: ReturnType<typeof resolveIntent>) => boolean;
}> = [
  {
    cores: [
      "open GPT",
      "launch GPT",
      "open ChatGPT",
      "bring GPT forward",
      "bring GPT to the front",
      "focus GPT",
      "switch to GPT",
      "take me to GPT",
      "locate ChatGPT",
    ],
    accept: (kind, action) =>
      ["browserOpen", "browserOpenFocus", "winFocus"].includes(kind) &&
      !JSON.stringify(action).toLowerCase().includes(".exe"),
  },
  {
    cores: [
      "open Microsoft Store",
      "launch Store",
      "open Windows Store",
      "open the store",
    ],
    accept: (kind, action) =>
      kind === "appLaunch" &&
      "query" in action &&
      String(action.query).toLowerCase().includes("ms-windows-store"),
  },
  {
    cores: [
      "bring Cursor to the front",
      "restore Cursor",
      "maximise Cursor",
      "maximize Cursor",
      "minimise Cursor",
      "minimize Cursor",
      "focus Chrome",
      "focus Edge",
      "locate browser with YouTube",
      "locate the browser with YouTube open",
      "open Chrome beside Cursor",
      "open File Explorer to Pictures",
      "locate Downloads",
      "show my Desktop",
      "open Settings",
    ],
    accept: (kind) =>
      [
        "winFocus",
        "winRestore",
        "winMaximize",
        "winMinimize",
        "browserOpenBeside",
        "appLaunch",
        "appOpen",
        "appOpenMaximize",
      ].includes(kind),
  },
  {
    cores: [
      "what can you do",
      "show capabilities",
      "list desktop commands",
      "what desktop tasks can you perform",
      "how can you help me",
      "what do you know about windows",
      "what applications can you control",
    ],
    accept: (kind, action) =>
      kind === "capabilityExplain" &&
      "reply" in action &&
      !/provider|registry|kernel/i.test(action.reply),
  },
];

describe("P16.32 hostile NL semantic validation", () => {
  it("resolves hundreds of natural variations without inventing executables", () => {
    let total = 0;
    let passed = 0;
    const failures: string[] = [];

    for (const group of MUST_RESOLVE) {
      for (const core of group.cores) {
        for (const utterance of variants(core)) {
          total += 1;
          const action = resolveIntent(utterance);
          const blob = JSON.stringify(action);
          if (/"[^"]*\.exe"/i.test(blob) && isInventedExecutableQuery(
            "query" in action && typeof action.query === "string"
              ? action.query
              : "",
          )) {
            failures.push(`${utterance} → invented exe in ${blob}`);
            continue;
          }
          if (group.accept(action.kind, action)) {
            passed += 1;
          } else {
            failures.push(`${utterance} → ${action.kind}`);
          }
        }
      }
    }

    expect(total).toBeGreaterThanOrEqual(200);
    // Hostile suite must stay overwhelmingly green; residual misses are listed.
    expect(passed / total).toBeGreaterThanOrEqual(0.92);
    if (failures.length > 0 && passed / total < 0.92) {
      expect(failures.slice(0, 20)).toEqual([]);
    }
  });

  it("refuses unknown open/launch without inventing .exe names", () => {
    for (const utterance of [
      "Open foobarbaz",
      "Launch sentence with many words here",
      "Start totallyunknownapp",
      "Open please do the thing",
    ]) {
      const evidence = resolveIntentWithEvidence(utterance);
      expect(evidence.action.kind).toBe("unknown");
      expect(evidence.action.reply.toLowerCase()).toMatch(
        /don.?t recognize|don.?t know how to launch|can.?t open|not something i can/,
      );
      expect(evidence.action.reply.toLowerCase()).not.toMatch(/\.exe/);
      if ("query" in evidence.action) {
        expect(isInventedExecutableQuery(String(evidence.action.query))).toBe(
          false,
        );
      }
    }
  });

  it("traces the complete Intent pipeline for spoken-style requests", () => {
    for (const utterance of [
      "Open Microsoft Store",
      "Bring GPT to the front",
      "What can you do?",
      "Locate Downloads",
    ]) {
      const evidence = resolveIntentWithEvidence(utterance);
      expect(evidence.stages.some((s) => s.stage === "semantic_engine")).toBe(
        true,
      );
      expect(evidence.stages.some((s) => s.stage === "resolve_intent" && s.hit))
        .toBe(true);
      expect(evidence.action.kind).not.toBe("unknown");
      expect(evidence.semanticOwned).toBe(true);
    }
  });

  it("generates capability discovery only from the live registry", () => {
    const discovery = generateCapabilityDiscovery("all");
    expect(discovery.reply).toMatch(/Applications|Windows|Browser/);
    expect(discovery.reply).toMatch(/won’t overclaim|won't overclaim|Why:/i);
    expect(discovery.reply).not.toMatch(/Provider|Registry|Kernel|WinRT/);
    const scoped = generateCapabilityDiscovery("windows");
    expect(scoped.reply.toLowerCase()).toMatch(/window/);
    expect(scoped.reply).toMatch(/Example/);
  });
});
