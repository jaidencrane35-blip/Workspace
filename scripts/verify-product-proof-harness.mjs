#!/usr/bin/env node
/**
 * Verifies the permanent Product Proof Rule + provider harness artifacts.
 * Routing correctness is enforced by Vitest against *.proof.json examples.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-product-proof-harness: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/PRODUCT_PROOF_RULE.md",
  "docs/capability-runtime/product-proof/WINDOW_PROVIDER_PRODUCT_PROOF.md",
  "docs/capability-runtime/product-proof/window-provider.proof.json",
  ".cursor/rules/constitutional-execution-protocol.mdc",
  "tests/window-provider-product-proof.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const rule = fs.readFileSync(
  path.join(root, "docs/capability-runtime/PRODUCT_PROOF_RULE.md"),
  "utf8",
);
for (const token of [
  "Product Complete",
  "connected to Conversation",
  "truthful",
  "Engineering Completion",
  "Product Proof",
]) {
  if (!rule.includes(token)) {
    fail(`PRODUCT_PROOF_RULE.md missing token: ${token}`);
  }
}

const protocol = fs.readFileSync(
  path.join(root, ".cursor/rules/constitutional-execution-protocol.mdc"),
  "utf8",
);
if (!protocol.includes("Product Proof Rule")) {
  fail("constitutional-execution-protocol.mdc must document Product Proof Rule");
}
if (!protocol.includes("Engineering Completion")) {
  fail("protocol must require Engineering Completion + Product Proof");
}

const proof = JSON.parse(
  fs.readFileSync(
    path.join(
      root,
      "docs/capability-runtime/product-proof/window-provider.proof.json",
    ),
    "utf8",
  ),
);

if (proof.provider !== "window" || proof.program !== "P12.5") {
  fail("window-provider.proof.json must declare provider=window program=P12.5");
}
if (!Array.isArray(proof.examples) || proof.examples.length < 8) {
  fail("window-provider.proof.json must include substantial conversation examples");
}
for (const key of [
  "expectedKind",
  "utterance",
  "category",
  "expectedOperation",
]) {
  for (const example of proof.examples) {
    if (!(key in example)) {
      fail(`proof example missing ${key}: ${JSON.stringify(example)}`);
    }
  }
}
if (!Array.isArray(proof.ownerReviewChecklist) || proof.ownerReviewChecklist.length < 4) {
  fail("proof harness must include ownerReviewChecklist");
}
if (!Array.isArray(proof.clarifications) || proof.clarifications.length < 1) {
  fail("proof harness must include clarification examples");
}

const intent = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
for (const kind of [
  "winEnumerate",
  "winActive",
  "winSnap",
  "winCenter",
  "winMaximize",
  "winRestore",
  "winFocus",
  "winMoveMonitor",
  "winResize",
  "resolveWindowIntent",
]) {
  if (!intent.includes(kind)) {
    fail(`intentBridge must include ${kind} for Product Proof`);
  }
}

if (/through Window Provider/i.test(intent)) {
  fail("Conversation replies must not expose Window Provider terminology");
}

console.log("verify-product-proof-harness: ok");
