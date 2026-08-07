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
  fail("voice port must soft-probe mic (never sticky MediaCapture Denied)");
}
if (!voiceRs.includes("ConfirmedDenied")) {
  fail("voice port must distinguish ConfirmedDenied from MediaCapture Denied");
}
if (!voiceRs.includes("mic_probe_denied_soft")) {
  fail("voice port must log soft deny without caching");
}
if (!voiceRs.includes("listen_fail_recover") || !voiceRs.includes("engine_reset")) {
  fail("voice port must reset engine after listen failure (idle recovery)");
}
if (!voiceRs.includes("warm_lock")) {
  fail("voice port must serialize warm_up (startup + UI contention)");
}
// Hard-block on plain MediaCapture Denied must stay gone.
if (/Some\(MicAccess::Denied\)\s*\n\s*\)\s*\{[\s\S]{0,200}permission_denied/.test(voiceRs)) {
  fail("listen must not hard-block on MicAccess::Denied from MediaCapture cache");
}
if (!research.includes("P16.13") || !research.includes("false-deny")) {
  fail("VOICE_RESEARCH must document P16.13 false-deny regression root cause");
}
if (!proof.responsiveness?.noStickyMediaCaptureDeny) {
  fail("proof must declare noStickyMediaCaptureDeny");
}
if (!proof.responsiveness?.engineResetOnListenFailure) {
  fail("proof must declare engineResetOnListenFailure");
}
if (!protocol.includes("Engineering Completion Gate")) {
  fail("protocol must adopt Engineering Completion Gate permanently");
}
if (!protocol.includes("Capability Regression Prevention")) {
  fail("protocol must adopt Capability Regression Prevention permanently");
}

console.log("verify-voice-regression: ok");
