#!/usr/bin/env node
/**
 * P22.S1 — Intelligence Kind Routing & Reasoning Provider (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-intelligence-routing: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/intelligenceRouting.ts",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/conversationGuidance.ts",
  "app/src/lib/capabilityRegistry.ts",
  "tests/intelligence-routing.test.ts",
  "docs/capability-runtime/product-proof/P22_S1_INTELLIGENCE_KIND_ROUTING.md",
  "docs/capability-runtime/product-proof/P22_A1_INTELLIGENCE_ROUTING_AUDIT.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const routing = fs.readFileSync(
  path.join(root, "app/src/lib/intelligenceRouting.ts"),
  "utf8",
);
const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
const guidance = fs.readFileSync(
  path.join(root, "app/src/lib/conversationGuidance.ts"),
  "utf8",
);
const report = fs.readFileSync(
  path.join(
    root,
    "docs/capability-runtime/product-proof/P22_S1_INTELLIGENCE_KIND_ROUTING.md",
  ),
  "utf8",
);

for (const token of [
  "REASONING_LOCAL",
  "REASONING_PROVIDER",
  "HYBRID",
  "CLARIFICATION",
  "CAPABILITY_LIMIT",
  "classifyIntelligenceKind",
  "resolveIntelligenceRoute",
  "chatgptReasoningUrl",
  "chatgpt.com",
]) {
  if (!routing.includes(token)) fail(`intelligenceRouting.ts missing ${token}`);
}

if (!bridge.includes("resolveIntelligenceRoute")) {
  fail("intentBridge must call resolveIntelligenceRoute");
}

if (guidance.includes("not general chat or look-ups")) {
  fail("conversationGuidance must not refuse knowledge as anti-chat near-miss");
}
if (guidance.includes("outside what I can do on the desktop right now")) {
  fail("generic desktop-outside refusal must be retired from GENERIC_REPLIES");
}

if (!report.includes("P22.S1")) fail("report must title P22.S1");
if (!report.includes("Intelligence Kind")) {
  fail("report must describe Intelligence Kind routing");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-intelligence-routing.mjs")) {
  fail("package.json must wire verify-intelligence-routing.mjs");
}

console.log("verify-intelligence-routing: ok");
