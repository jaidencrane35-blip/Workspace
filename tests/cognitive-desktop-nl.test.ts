/**
 * P16.33 — Cognitive desktop language (reasoning, not alias memorization).
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import {
  generateCapabilityDiscovery,
  generateRecoveryGuidance,
} from "../app/src/lib/capabilityRegistry";
import { resolveIntentWithEvidence } from "../app/src/lib/intentPipeline";

const COGNITIVE: Array<{
  utterance: string;
  accept: (action: ReturnType<typeof resolveIntent>) => boolean;
}> = [
  {
    utterance: "I've got ChatGPT somewhere",
    accept: (a) => a.kind === "winFocus",
  },
  {
    utterance: "Where is Cursor?",
    accept: (a) => a.kind === "winFocus" && "query" in a && /cursor/i.test(a.query),
  },
  {
    utterance: "Where did my browser go?",
    accept: (a) => a.kind === "winFocus",
  },
  {
    utterance: "Bring back my browser",
    accept: (a) => a.kind === "winFocus",
  },
  {
    utterance: "I was just using Chrome",
    accept: (a) => a.kind === "winFocus" && "query" in a && /chrome/i.test(a.query),
  },
  {
    utterance: "Take me to YouTube",
    accept: (a) =>
      (a.kind === "browserOpen" || a.kind === "browserOpenFocus") &&
      "url" in a &&
      /youtube/i.test(a.url),
  },
  {
    utterance: "Find my Downloads",
    accept: (a) =>
      a.kind === "appLaunch" &&
      "query" in a &&
      /downloads/i.test(String(a.query)),
  },
  {
    utterance: "Open the pictures from yesterday",
    accept: (a) =>
      a.kind === "appLaunch" &&
      "query" in a &&
      /pictures/i.test(String(a.query)) &&
      /yesterday|date|filter/i.test(a.reply),
  },
  {
    utterance: "Show me the folder with screenshots",
    accept: (a) =>
      a.kind === "appLaunch" &&
      "query" in a &&
      /pictures/i.test(String(a.query)),
  },
  {
    utterance: "I want ChatGPT next to Cursor",
    accept: (a) =>
      a.kind === "browserOpenBeside" &&
      "beside" in a &&
      /cursor/i.test(a.beside),
  },
  {
    utterance: "Put Chrome on the other monitor",
    accept: (a) =>
      a.kind === "winMoveMonitor" &&
      "monitorIndex" in a &&
      a.monitorIndex === 2,
  },
  {
    utterance: "What windows are open?",
    accept: (a) => a.kind === "winEnumerate",
  },
  {
    utterance: "I'm trying to find Explorer",
    accept: (a) => a.kind === "winFocus" || a.kind === "appOpen",
  },
  {
    utterance: "Show me everything you can control",
    accept: (a) =>
      a.kind === "capabilityExplain" &&
      /won’t overclaim|won't overclaim|Why:/i.test(a.reply) &&
      !/Provider|Registry|Kernel|WinRT/i.test(a.reply),
  },
];

describe("P16.33 cognitive desktop NL", () => {
  it("reasons over Owner cognitive phrases without inventing executables", () => {
    const failures: string[] = [];
    for (const row of COGNITIVE) {
      const action = resolveIntent(row.utterance);
      if (
        "query" in action &&
        typeof action.query === "string" &&
        /\.exe$/i.test(action.query) &&
        !/^https?:/i.test(action.query) &&
        !/^shell:/i.test(action.query) &&
        !/^ms-/i.test(action.query)
      ) {
        failures.push(`${row.utterance} → invented query ${action.query}`);
        continue;
      }
      if (/Provider|Registry|Kernel|WinRT/i.test(action.reply ?? "")) {
        failures.push(`${row.utterance} → exposed internals`);
        continue;
      }
      if (!row.accept(action)) {
        failures.push(`${row.utterance} → ${action.kind}: ${action.reply}`);
      }
    }
    expect(failures).toEqual([]);
  });

  it("generates self-describing discovery from the Capability Registry only", () => {
    const discovery = generateCapabilityDiscovery("all");
    expect(discovery.reply).toMatch(/can control|Here’s what I can/i);
    expect(discovery.reply).toMatch(/won’t overclaim|Why:/i);
    expect(discovery.reply).toMatch(/Needs —|Example —/);
    expect(discovery.reply).not.toMatch(/Provider|Registry|Kernel|WinRT/);
  });

  it("recovers conversationally for unknown entities via registry guidance", () => {
    const recovery = generateRecoveryGuidance("totallyunknownthing");
    expect(recovery.reply).toMatch(/won’t invent|can't do that/i);
    expect(recovery.suggestion.toLowerCase()).toMatch(/try|what can you do/);
    const action = resolveIntent("Where is TotallyUnknownApp?");
    expect(action.kind).toBe("unknown");
    expect(action.reply).not.toMatch(/Provider|Registry|Kernel|WinRT/i);
    expect(action.reply.toLowerCase()).toMatch(/won’t invent|can't do that|cannot/);
  });

  it("keeps the full Intent pipeline evidence for cognitive phrasing", () => {
    for (const utterance of [
      "I've got ChatGPT somewhere",
      "Show me everything you can control",
      "Put Chrome on the other monitor",
    ]) {
      const evidence = resolveIntentWithEvidence(utterance);
      expect(evidence.stages.some((s) => s.stage === "semantic_engine")).toBe(
        true,
      );
      expect(evidence.action.kind).not.toBe("unknown");
    }
  });
});
