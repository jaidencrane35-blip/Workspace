#!/usr/bin/env node
/**
 * P16.6 — Conversation quality + User Adaptation Prohibition (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-conversation-quality: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/PRODUCT_PROOF_RULE.md",
  "app/src/lib/conversationGuidance.ts",
  "app/src/lib/intentBridge.ts",
  "tests/conversation-quality.test.ts",
  ".cursor/rules/constitutional-execution-protocol.mdc",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const rule = fs.readFileSync(
  path.join(root, "docs/capability-runtime/PRODUCT_PROOF_RULE.md"),
  "utf8",
);
for (const token of [
  "User Adaptation Prohibition",
  "The software adapts to the user",
  "precise capitalization",
  "rigid wording",
  "Commodity Before Reinvention",
  "WRAP",
  "Conversation Continuity",
  "Semantic Alias Rule",
  "Permission Guidance Principle",
  "Engineering Verification Separation",
  "Owner Directed Product Proof",
  "Production Before Expansion",
  "Evidence Before Modification",
  "Root Cause Before Rewrite",
  "Production Quality Includes Repository Quality",
  "Evidence Before Completion",
  "Repository Health Before Milestone Closure",
]) {
  if (!rule.includes(token)) {
    fail(`PRODUCT_PROOF_RULE.md missing token: ${token}`);
  }
}

const protocol = fs.readFileSync(
  path.join(root, ".cursor/rules/constitutional-execution-protocol.mdc"),
  "utf8",
);
if (!protocol.includes("User Adaptation Prohibition")) {
  fail("protocol must document User Adaptation Prohibition");
}
if (!protocol.includes("Production Before Expansion")) {
  fail("protocol must document Production Before Expansion");
}

const guidance = fs.readFileSync(
  path.join(root, "app/src/lib/conversationGuidance.ts"),
  "utf8",
);
for (const token of [
  "resolveUnknownGuidance",
  "isVoiceCheckUtterance",
  "softenUtterance",
  "GENERIC_REPLIES",
  "companionGreetingReply",
  "maybeRecovery",
]) {
  if (!guidance.includes(token)) {
    fail(`conversationGuidance.ts missing ${token}`);
  }
}
if (
  guidance.includes("won’t invent") ||
  guidance.includes("won't invent") ||
  guidance.includes("won’t pretend") ||
  guidance.includes("won't pretend")
) {
  fail("conversationGuidance must not use defensive invent/pretend chorus");
}
if (guidance.includes("Try “what windows are open?”")) {
  fail("conversationGuidance must not append default command catalogues");
}

const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
if (!bridge.includes("resolveUnknownGuidance")) {
  fail("intentBridge must use resolveUnknownGuidance for unsupported requests");
}
if (!bridge.includes("softenUtterance")) {
  fail("intentBridge must soften ordinary polite phrasing");
}
if (!bridge.includes("canonicalizeOpenTarget")) {
  fail("intentBridge must canonicalize GPT/tab/browser phrasing");
}
if (!bridge.includes("SEMANTIC_ALIASES") || !bridge.includes("expandSemanticAlias")) {
  fail("intentBridge must own Semantic Alias Rule expansions");
}
for (const alias of ["git:", "yt:", "vscode:", "gpt:", "cursor:", "msedge:"]) {
  if (!bridge.includes(alias)) {
    fail(`intentBridge SEMANTIC_ALIASES missing ${alias}`);
  }
}
if (!bridge.includes("browserOpenBeside")) {
  fail("intentBridge must support open-beside Operator composition intents");
}
if (bridge.includes("I don’t have that yet — and I won’t invent it. Closest available:")) {
  fail("static repetitive unknown fallback must not remain in intentBridge");
}

const constitution = fs.readFileSync(
  path.join(root, "docs/00-Constitution/PRODUCT_CONSTITUTION.md"),
  "utf8",
);
if (!constitution.includes("User Adaptation Prohibition")) {
  fail("PRODUCT_CONSTITUTION must record User Adaptation Prohibition");
}
if (!constitution.includes("Production Before Expansion")) {
  fail("PRODUCT_CONSTITUTION must record Production Before Expansion");
}
if (!constitution.includes("Evidence Before Modification")) {
  fail("PRODUCT_CONSTITUTION must record Evidence Before Modification");
}
if (!constitution.includes("Root Cause Before Rewrite")) {
  fail("PRODUCT_CONSTITUTION must record Root Cause Before Rewrite");
}
for (const token of [
  "Production Quality Includes Repository Quality",
  "Evidence Before Completion",
  "Repository Health Before Milestone Closure",
]) {
  if (!constitution.includes(token)) {
    fail(`PRODUCT_CONSTITUTION must record ${token}`);
  }
  if (!protocol.includes(token)) {
    fail(`protocol must document ${token}`);
  }
}
if (!bridge.includes("Open a/another/new browser") && !bridge.includes("another browser")) {
  fail("intentBridge must support open another/new browser phrasing");
}

console.log("verify-conversation-quality: ok");
