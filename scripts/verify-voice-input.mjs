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
for (const cmd of [
  "voice_status",
  "voice_warm_up",
  "voice_listen_once",
  "voice_cancel",
  "voice_open_settings",
]) {
  if (!libRs.includes(cmd)) fail(`Tauri handler must register ${cmd}`);
}
if (!libRs.includes("warm_voice_engine_async")) {
  fail("app setup must pre-warm the voice engine");
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

const voiceRs = fs.readFileSync(
  path.join(root, "packages/windows-integration/src/voice.rs"),
  "utf8",
);
if (!voiceRs.includes("classify_speech_failure")) {
  fail("voice port must map OS speech failures via classify_speech_failure");
}
if (!voiceRs.includes("0x8004_5509") && !voiceRs.includes("0x80045509")) {
  fail("voice port must recognize Windows speech privacy HRESULT 0x80045509");
}
if (!voiceRs.includes("speech privacy")) {
  fail("voice port must explain speech privacy in desktop language");
}

const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/voice/bridge.ts"),
  "utf8",
);
if (!bridge.includes("desktopVoiceMessage")) {
  fail("voice bridge must sanitize user-facing voice messages");
}
if (!bridge.includes("warmUpVoice") || !bridge.includes("voice-listening")) {
  fail("voice bridge must support warm-up and listening-ready events");
}
if (!bridge.includes("ensureVoiceListeningBridge")) {
  fail("voice bridge must hoist voice-listening subscription off the listen hot path");
}

const micUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/VoiceMicButton.tsx"),
  "utf8",
);
if (!micUi.includes('data-preparing') || !micUi.includes("preparing")) {
  fail("mic UI must distinguish preparing from listening");
}
for (const phase of [
  "idle",
  "preparing",
  "listening",
  "recognizing",
  "processing",
  "finished",
]) {
  if (!micUi.includes(`"${phase}"`) && !micUi.includes(`'${phase}'`) && !micUi.includes(phase)) {
    // phases appear as string literals in phaseLabel / setPhase
  }
}
if (!micUi.includes("recognizing") || !micUi.includes("processing") || !micUi.includes("finished")) {
  fail("mic UI must expose recognizing / processing / finished states");
}
if (!micUi.includes("ready")) {
  fail("mic UI must expose Ready state (Capturing contract)");
}
if (!micUi.includes("op-shell__mic-wave")) {
  fail("mic UI must show a listening activity waveform while capturing");
}
if (!micUi.includes("data-sound") || !micUi.includes("onSoundStarted")) {
  fail("mic UI must react to SoundStarted for live speech activity");
}
if (!micUi.includes("warmed")) {
  fail("mic UI must wait for warm engine before listen when possible");
}
if (!bridge.includes("voice-ready") || !bridge.includes("voice-sound")) {
  fail("voice bridge must subscribe to voice-ready and voice-sound");
}
if (micUi.includes("getVoiceStatus()") && micUi.includes("setPhase(\"preparing\")")) {
  // Hot-path status before listen reintroduces first-word loss.
  const startIdx = micUi.indexOf("const start");
  const startChunk = micUi.slice(startIdx, startIdx + 900);
  if (startChunk.includes("getVoiceStatus")) {
    fail("mic start hot path must not await getVoiceStatus before listen");
  }
}
if (!voiceRs.includes("warm_up") || !voiceRs.includes("listen_once_when_ready")) {
  fail("voice port must expose warm_up and listen_once_when_ready");
}
if (!voiceRs.includes("SpeechRecognizerState::Capturing") || !voiceRs.includes("capturing_contract")) {
  fail("voice port must gate Listening on WinRT Capturing (P16.7 trustworthy contract)");
}
if (
  !voiceRs.includes("ContinuousRecognitionSession") ||
  !voiceRs.includes("winrt_listen_continuous_when_ready") ||
  !voiceRs.includes("SetAutoStopSilenceTimeout")
) {
  fail("voice port must WRAP ContinuousRecognitionSession (P16.8 Conversation Continuity)");
}
if (!voiceRs.includes("StartWithModeAsync") || !voiceRs.includes("ResultGenerated")) {
  fail("continuous listen must start a session and accumulate ResultGenerated fragments");
}
if (!voiceRs.includes("stitch_resume") || !voiceRs.includes("MAX_WALL")) {
  fail("voice port must stitch continuous sessions up to 5 minutes (P16.9)");
}
if (
  voiceRs.includes("open_windows_settings_uri(VoiceSettingsTarget::Microphone") &&
  voiceRs.includes("outcome_from_winrt_error")
) {
  // Auto-open from classify path must remain removed.
}
if (voiceRs.includes("outcome_from_winrt_error") && /fn outcome_from_winrt_error[\s\S]*?open_windows_settings_uri/.test(voiceRs)) {
  fail("outcome_from_winrt_error must not auto-open Settings (Permission Guidance)");
}
if (!micUi.includes("permissionGuidance") || !micUi.includes("speechDetected")) {
  fail("mic UI must implement Permission Guidance + speechDetected Ready contract");
}
const voiceCmd = fs.readFileSync(
  path.join(root, "app/src-tauri/src/commands/voice.rs"),
  "utf8",
);
if (!voiceCmd.includes("spawn_blocking")) {
  fail("voice IPC must run WinRT listen/warm on spawn_blocking (P16.10)");
}
if (!voiceCmd.includes("async fn voice_listen_once")) {
  fail("voice_listen_once must be async so IPC runtime is not blocked");
}
if (bridge.includes("void openVoiceSettings")) {
  fail("voice bridge must not auto-open Settings (Permission Guidance)");
}
if (!voiceRs.includes("peek_microphone_access") || !voiceRs.includes("mic_probe_begin")) {
  fail("mic probe must be warm-only with peek for status (P16.10)");
}
if (!voiceRs.includes("stitch_start_failed_soft") || !voiceRs.includes("280")) {
  fail("stitch path must soft-fail and gap between sessions (crash prevention)");
}
const guidePath = path.join(root, "app/src/lib/voice/permissionGuidance.ts");
if (!fs.existsSync(guidePath)) {
  fail("missing permissionGuidance.ts");
}
const guide = fs.readFileSync(guidePath, "utf8");
if (!guide.includes("rememberVoicePermissionGranted") || !guide.includes("Permission Guidance")) {
  fail("permissionGuidance must remember grants and document the principle");
}
if (!voiceRs.includes("voice.lifecycle:")) {
  fail("voice port must emit lifecycle timing logs for capture evidence");
}
if (!proof.responsiveness?.prewarm || !proof.responsiveness?.listeningIndicatorOnlyWhenCapturing) {
  fail("proof must declare P16.5 responsiveness requirements");
}
if (!proof.responsiveness?.continuousRecognitionSession || !proof.responsiveness?.conversationContinuity) {
  fail("proof must declare P16.8 Conversation Continuity requirements");
}

if (!Array.isArray(proof.failureModes) || proof.failureModes.length < 1) {
  fail("proof must declare Voice failureModes (including speech privacy)");
}
const privacy = proof.failureModes.find((m) => m.id === "windows_speech_privacy");
if (!privacy || privacy.osCode !== "0x80045509") {
  fail("proof must declare windows_speech_privacy failure mode for 0x80045509");
}

console.log("verify-voice-input: ok");
