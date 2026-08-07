import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator";
import { desktopVoiceMessage } from "../app/src/lib/voice";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const proof = JSON.parse(
  readFileSync(
    path.join(
      root,
      "docs/capability-runtime/product-proof/voice-input.proof.json",
    ),
    "utf8",
  ),
) as {
  examples: Array<{
    utterance: string;
    expectedKind: string;
    expectedDomain?: string;
  }>;
  uiRequirements: string[];
  program: string;
  kind: string;
  independenceRule: boolean;
  ipc: { desktop: string; voice: string[] };
  failureModes: Array<{
    id: string;
    osCode?: string;
    status?: string;
    mustMention?: string[];
    mustNotMention?: string[];
  }>;
};

describe("Voice Input Product Proof harness (P16)", () => {
  it("declares input-device role and Independence Rule", () => {
    expect(proof.program).toBe("P16");
    expect(proof.kind).toBe("conversation_input_device");
    expect(proof.independenceRule).toBe(true);
    expect(proof.ipc.desktop).toBe("execute_capability_intent");
    expect(proof.ipc.voice).toContain("voice_listen_once");
  });

  it("routes voice help and typed-equivalence utterances", () => {
    for (const example of proof.examples) {
      const action = resolveIntent(example.utterance);
      expect(action.kind, example.utterance).toBe(example.expectedKind);

      const capability = toCapabilityIntent(action);
      if (example.expectedDomain != null) {
        expect(capability?.domain, example.utterance).toBe(
          example.expectedDomain,
        );
      }

      expect(JSON.stringify(action).toLowerCase()).not.toMatch(
        /voice provider|capability runtime|winrt|speechrecognizer|provider registry|kernel operator/,
      );
    }
  });

  it("ships Conversation mic chrome", () => {
    const ui = readFileSync(
      path.join(root, "app/src/components/operator/VoiceMicButton.tsx"),
      "utf8",
    );
    const rootTsx = readFileSync(
      path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
      "utf8",
    );
    const css = readFileSync(path.join(root, "app/src/App.css"), "utf8");
    const bridge = readFileSync(
      path.join(root, "app/src/lib/voice/bridge.ts"),
      "utf8",
    );
    expect(ui).toContain("op-shell__mic");
    expect(rootTsx).toContain("VoiceMicButton");
    expect(css).toContain("op-mic-pulse");
    for (const token of proof.uiRequirements) {
      if (token === "listening indicator") {
        expect(css).toContain("op-shell__mic-pulse");
      } else if (token === "composer integration") {
        expect(rootTsx).toContain("op-shell__composer-actions");
      } else if (token === "warmUpVoice") {
        expect(bridge).toContain("warmUpVoice");
        expect(ui).toContain("warmUpVoice");
      } else {
        expect(`${ui}\n${rootTsx}`).toContain(token);
      }
    }
    // Ready contract: Listening UI only after speech detected — not Preparing.
    expect(ui).toContain('data-ready={phase === "ready" ? "true" : "false"}');
    expect(ui).toMatch(
      /const listening =\s*phase === "speechDetected" \|\| phase === "listening"/,
    );
    expect(ui).toContain("permissionGuidance");
    expect(ui).not.toMatch(/await openVoiceSettings[\s\S]{0,80}setPhase\("idle"\)/);
    const startIdx = ui.indexOf("const start");
    expect(ui.slice(startIdx, startIdx + 400)).not.toContain("getVoiceStatus");
  });

  it("maps Windows speech privacy failure to desktop language", () => {
    const privacy = proof.failureModes.find(
      (mode) => mode.id === "windows_speech_privacy",
    );
    expect(privacy?.osCode).toBe("0x80045509");
    const raw =
      "Voice input failed: recognize: The speech privacy policy was not accepted prior to attempting a speech recognition. (0x80045509)";
    const message = desktopVoiceMessage(raw);
    for (const token of privacy?.mustMention ?? []) {
      expect(message.toLowerCase()).toContain(token.toLowerCase());
    }
    for (const token of privacy?.mustNotMention ?? []) {
      expect(message.toLowerCase()).not.toContain(token.toLowerCase());
    }
  });
});
