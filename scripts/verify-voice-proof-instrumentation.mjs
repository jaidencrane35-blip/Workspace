/**
 * P16.23 — Voice Product Proof instrumentation must exist, be env-gated,
 * and remain removable after P16 closure (no ordinary-user surface).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-voice-proof-instrumentation: ${msg}`);
  process.exit(1);
}

const proofRs = path.join(root, "packages/windows-integration/src/voice_proof.rs");
if (!fs.existsSync(proofRs)) {
  fail("missing voice_proof.rs (P16.23)");
}
const proofSrc = fs.readFileSync(proofRs, "utf8");
if (!proofSrc.includes("WORKSPACE_VOICE_PRODUCT_PROOF")) {
  fail("instrumentation must be gated by WORKSPACE_VOICE_PRODUCT_PROOF");
}
if (!proofSrc.includes("voice.proof.report")) {
  fail("instrumentation must emit voice.proof.report summary logs");
}
if (!proofSrc.includes("click_to_capturing_ms") || !proofSrc.includes("click_to_ready_ms")) {
  fail("instrumentation must measure Click→Capturing and Click→Ready");
}
if (!proofSrc.includes("Remove after P16")) {
  fail("instrumentation must document removability after P16");
}

const voiceRs = fs.readFileSync(
  path.join(root, "packages/windows-integration/src/voice.rs"),
  "utf8",
);
if (!voiceRs.includes("proof_begin_listen") || !voiceRs.includes("proof_mark(\"ready_emitted\")")) {
  fail("System/Memory Voice listen path must record proof marks");
}
if (!voiceRs.includes("product_proof_instrumentation_measures_memory_listen_intervals")) {
  fail("harness test must measure MemoryVoicePort proof intervals");
}

const doc = path.join(
  root,
  "docs/capability-runtime/product-proof/VOICE_LIVE_INSTRUMENTATION.md",
);
if (!fs.existsSync(doc)) {
  fail("missing VOICE_LIVE_INSTRUMENTATION.md");
}
const docText = fs.readFileSync(doc, "utf8");
if (!docText.includes("Owner finding evidence table")) {
  fail("live instrumentation doc must include Owner finding evidence table");
}
if (!docText.includes("Pending live Owner session")) {
  fail("live instrumentation doc must not invent WinRT timings without Owner launch");
}

const proofJson = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/capability-runtime/product-proof/voice-input.proof.json"),
    "utf8",
  ),
);
if (!proofJson.responsiveness?.liveProductProofInstrumentation) {
  fail("voice-input.proof.json must declare liveProductProofInstrumentation");
}

// Must not surface Product Proof env / WinRT terms in mic UI.
const micUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/VoiceMicButton.tsx"),
  "utf8",
);
if (micUi.includes("WORKSPACE_VOICE_PRODUCT_PROOF") || micUi.includes("voice.proof")) {
  fail("mic UI must not expose Product Proof instrumentation to users");
}

console.log("verify-voice-proof-instrumentation: ok");
