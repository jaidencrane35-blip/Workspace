/**
 * P16.13 — Voice regression guards (Capability Regression Prevention).
 * Ensures the MediaCapture false-deny cache cannot hard-block listen again.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-voice-regression: ${msg}`);
  process.exit(1);
}

const voiceRs = fs.readFileSync(
  path.join(root, "packages/windows-integration/src/voice.rs"),
  "utf8",
);
const research = fs.readFileSync(
  path.join(root, "docs/capability-runtime/research/VOICE_RESEARCH.md"),
  "utf8",
);
const proof = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/capability-runtime/product-proof/voice-input.proof.json"),
    "utf8",
  ),
);
const protocol = fs.readFileSync(
  path.join(root, ".cursor/rules/constitutional-execution-protocol.mdc"),
  "utf8",
);

if (!voiceRs.includes("probe_microphone_access_soft")) {
  fail("voice port must soft-probe mic on recheck (never sticky MediaCapture Denied)");
}
if (!voiceRs.includes("ConfirmedDenied")) {
  fail("voice port must distinguish ConfirmedDenied from MediaCapture Denied");
}
if (!voiceRs.includes("mic_probe_denied_soft")) {
  fail("voice port must log soft deny without caching");
}
if (!voiceRs.includes("listen_fail_recover") || !voiceRs.includes("engine_reset")) {
  fail("voice port must reset engine after poison listen failures");
}
if (!voiceRs.includes("listen_idle_keep_engine")) {
  fail("voice port must keep engine on no_speech/cancelled (P16.14)");
}
if (!voiceRs.includes("warm_lock")) {
  fail("voice port must serialize warm_up (startup + UI contention)");
}
// warm_up must not MediaCapture-probe (races SpeechRecognizer for the mic).
const warmFn = voiceRs.match(/fn warm_up\(&self\)[\s\S]*?\n    fn listen_once_when_ready/);
if (!warmFn) {
  fail("could not locate warm_up implementation bounds");
}
if (warmFn[0].includes("probe_microphone_access")) {
  fail("warm_up must not MediaCapture-probe (P16.14 mic race)");
}
// Hard-block on plain MediaCapture Denied must stay gone.
if (/Some\(MicAccess::Denied\)\s*\n\s*\)\s*\{[\s\S]{0,200}permission_denied/.test(voiceRs)) {
  fail("listen must not hard-block on MicAccess::Denied from MediaCapture cache");
}
if (!research.includes("P16.13") || !research.includes("false-deny")) {
  fail("VOICE_RESEARCH must document P16.13 false-deny regression root cause");
}
if (!research.includes("P16.14") || !research.includes("MediaCapture warm race")) {
  fail("VOICE_RESEARCH must document P16.14 MediaCapture warm race root cause");
}
if (!research.includes("P16.15") || !research.includes("Production Before Expansion")) {
  fail("VOICE_RESEARCH must document P16.15 Product Completion + Production Before Expansion");
}
if (!proof.responsiveness?.noStickyMediaCaptureDeny) {
  fail("proof must declare noStickyMediaCaptureDeny");
}
if (!proof.responsiveness?.engineResetOnListenFailure) {
  fail("proof must declare engineResetOnListenFailure");
}
if (proof.responsiveness?.settleBeforeReadyMs !== 20) {
  fail("proof must declare settleBeforeReadyMs: 20 (P16.15)");
}
if (!proof.responsiveness?.noListenPathWarmGate) {
  fail("proof must declare noListenPathWarmGate (P16.15)");
}
if (!proof.responsiveness?.productionBeforeExpansion) {
  fail("proof must declare productionBeforeExpansion (P16.15)");
}
if (!protocol.includes("Engineering Completion Gate")) {
  fail("protocol must adopt Engineering Completion Gate permanently");
}
if (!protocol.includes("Capability Regression Prevention")) {
  fail("protocol must adopt Capability Regression Prevention permanently");
}
if (!protocol.includes("Production Before Expansion")) {
  fail("protocol must adopt Production Before Expansion permanently");
}
if (!voiceRs.includes("from_millis(20)")) {
  fail("voice port Ready settle must be 20ms after Capturing (P16.15)");
}
const micUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/VoiceMicButton.tsx"),
  "utf8",
);
if (micUi.includes("if (!warmed)") && micUi.includes("warmUpVoice()")) {
  fail("mic UI must not gate listen on frontend warmed + warmUpVoice (P16.15)");
}
if (!micUi.includes("setWarmed(false)")) {
  fail("mic UI must clear warmed after poison listen failures (P16.15)");
}
const matrixPath = path.join(
  root,
  "docs/capability-runtime/product-proof/VOICE_REGRESSION_MATRIX.md",
);
if (!fs.existsSync(matrixPath)) {
  fail("missing VOICE_REGRESSION_MATRIX.md (P16.15 artifact)");
}
const matrix = fs.readFileSync(matrixPath, "utf8");
if (!matrix.includes("R8") || !matrix.includes("noListenPathWarmGate")) {
  fail("regression matrix must cover P16.15 listen-path warm gate");
}

console.log("verify-voice-regression: ok");
