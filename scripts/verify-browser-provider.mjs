#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-browser-provider: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/BROWSER_PROVIDER.md",
  "docs/capability-runtime/research/BROWSER_RESEARCH.md",
  "docs/capability-runtime/product-proof/BROWSER_PROVIDER_PRODUCT_PROOF.md",
  "docs/capability-runtime/product-proof/BROWSER_PROVIDER_COMPOSITION_AUDIT.md",
  "docs/capability-runtime/product-proof/browser-provider.proof.json",
  "packages/windows-integration/src/browser.rs",
  "packages/kernel/src/capability_runtime/browser_provider.rs",
  "packages/kernel/src/commands/browser.rs",
  "tests/browser-provider-product-proof.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const proof = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/capability-runtime/product-proof/browser-provider.proof.json"),
    "utf8",
  ),
);
if (
  proof.provider !== "browser" ||
  (proof.program !== "P14" && proof.program !== "P14.5")
) {
  fail("proof must declare provider=browser program=P14 or P14.5");
}
if (proof.ipc !== "execute_capability_intent") {
  fail("Conversation ipc must be execute_capability_intent");
}

if (!proof.pipeline?.includes("Kernel Operator")) {
  fail("pipeline must include Kernel Operator");
}

const intent = fs.readFileSync(path.join(root, "app/src/lib/intentBridge.ts"), "utf8");
for (const token of [
  "browserStatus",
  "browserOpen",
  "browserOpenBeside",
  "browserExplain",
  "resolveBrowserIntent",
  "isPlausibleWebsite",
]) {
  if (!intent.includes(token)) fail(`intentBridge must include ${token}`);
}
if (/Browser Provider|webbrowser|Capability Runtime/i.test(intent)) {
  fail("intentBridge must not leak browser implementation jargon");
}

const runtime = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/mod.rs"),
  "utf8",
);
if (!runtime.includes("BrowserProvider")) {
  fail("Capability Runtime must register BrowserProvider");
}

const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
if (!plan.includes("browser") || !plan.includes("open_beside")) {
  fail("Kernel Operator plan must allow browser domain and open_beside");
}
if (!plan.includes("CapabilityOperation::Snap")) {
  fail("open_beside must compose Window Snap (Capability Completion Contract)");
}

console.log("verify-browser-provider: ok");
