#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-voice-input: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/VOICE_INPUT.md",
  "docs/capability-runtime/research/VOICE_RESEARCH.md",
  "docs/capability-runtime/product-proof/VOICE_INPUT_PRODUCT_PROOF.md",
  "docs/capability-runtime/product-proof/VOICE_INPUT_COMPOSITION_AUDIT.md",
  "docs/capability-runtime/product-proof/voice-input.proof.json",
  "packages/windows-integration/src/voice.rs",
  "app/src-tauri/src/commands/voice.rs",
  "app/src/lib/voice/bridge.ts",
  "app/src/components/operator/VoiceMicButton.tsx",
  "tests/voice-input-product-proof.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const proof = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/capability-runtime/product-proof/voice-input.proof.json"),
    "utf8",
  ),
);
if (proof.provider !== "voice_input" || proof.program !== "P16") {
  fail("proof must declare provider=voice_input program=P16");
}
if (proof.kind !== "conversation_input_device") {
  fail("Voice must be declared as conversation_input_device");
}
if (proof.ipc?.desktop !== "execute_capability_intent") {
  fail("desktop ipc after transcript must be execute_capability_intent");
}
if (proof.independenceRule !== true) {
  fail("proof must declare independenceRule=true");
}

const rootUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
  "utf8",
);
if (!rootUi.includes("VoiceMicButton")) {
  fail("Conversation composer must include VoiceMicButton");
}

const libRs = fs.readFileSync(
  path.join(root, "app/src-tauri/src/lib.rs"),
  "utf8",
);
for (const cmd of ["voice_status", "voice_listen_once", "voice_cancel"]) {
  if (!libRs.includes(cmd)) fail(`Tauri handler must register ${cmd}`);
}

const runtime = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/mod.rs"),
  "utf8",
);
if (runtime.includes("VoiceProvider")) {
  fail("Voice must not register a Capability Runtime desktop provider");
}

const research = fs.readFileSync(
  path.join(root, "docs/capability-runtime/research/VOICE_RESEARCH.md"),
  "utf8",
);
if (!research.includes("WRAP") || !research.includes("WinRT")) {
  fail("research must document WRAP WinRT decision");
}

console.log("verify-voice-input: ok");
