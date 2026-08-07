#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-screenshot-provider: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/SCREENSHOT_PROVIDER.md",
  "docs/capability-runtime/research/SCREENSHOT_RESEARCH.md",
  "docs/capability-runtime/product-proof/SCREENSHOT_PROVIDER_PRODUCT_PROOF.md",
  "docs/capability-runtime/product-proof/SCREENSHOT_PROVIDER_COMPOSITION_AUDIT.md",
  "docs/capability-runtime/product-proof/screenshot-provider.proof.json",
  "packages/windows-integration/src/screenshot.rs",
  "packages/kernel/src/capability_runtime/screenshot_provider.rs",
  "packages/kernel/src/commands/screenshot.rs",
  "tests/screenshot-provider-product-proof.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const proof = JSON.parse(
  fs.readFileSync(
    path.join(
      root,
      "docs/capability-runtime/product-proof/screenshot-provider.proof.json",
    ),
    "utf8",
  ),
);
if (proof.provider !== "screenshots" || proof.program !== "P15") {
  fail("proof must declare provider=screenshots program=P15");
}
if (proof.ipc !== "execute_capability_intent") {
  fail("Conversation ipc must be execute_capability_intent");
}
if (!proof.pipeline?.includes("Kernel Operator")) {
  fail("pipeline must include Kernel Operator");
}
if (proof.independenceRule !== true) {
  fail("proof must declare independenceRule=true");
}

const intent = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
for (const token of [
  "screenshotStatus",
  "screenshotDesktop",
  "screenshotWindow",
  "screenshotMonitor",
  "screenshotSave",
  "screenshotCopy",
  "screenshotCaptureAndCopy",
  "resolveScreenshotIntent",
]) {
  if (!intent.includes(token)) fail(`intentBridge must include ${token}`);
}
if (/Screenshot Provider|xcap|DXGI|Capability Runtime/i.test(intent)) {
  fail("intentBridge must not leak screenshot implementation jargon");
}

const runtime = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/mod.rs"),
  "utf8",
);
if (!runtime.includes("ScreenshotProvider")) {
  fail("Capability Runtime must register ScreenshotProvider");
}

const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
if (!plan.includes("screenshots") || !plan.includes("capture_and_copy")) {
  fail("Kernel Operator plan must allow screenshots domain and capture_and_copy");
}

const protocol = fs.readFileSync(
  path.join(root, ".cursor/rules/constitutional-execution-protocol.mdc"),
  "utf8",
);
if (!protocol.includes("Capability Independence Rule")) {
  fail("constitutional protocol must document Capability Independence Rule");
}

const standard = fs.readFileSync(
  path.join(root, "docs/capability-runtime/PROVIDER_ACCEPTANCE_STANDARD.md"),
  "utf8",
);
if (!standard.includes("Capability Independence Rule")) {
  fail("Provider Acceptance Standard must document Capability Independence Rule");
}

console.log("verify-screenshot-provider: ok");
