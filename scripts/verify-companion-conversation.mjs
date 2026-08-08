#!/usr/bin/env node
/**
 * T1 — Companion Conversation Behavior (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-companion-conversation: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/conversationGuidance.ts",
  "app/src/lib/workspaceContext.ts",
  "app/src/lib/intentBridge.ts",
  "tests/companion-conversation.test.ts",
  "docs/capability-runtime/product-proof/T1_COMPANION_CONVERSATION_BEHAVIOR.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const guidance = fs.readFileSync(
  path.join(root, "app/src/lib/conversationGuidance.ts"),
  "utf8",
);
const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
const ctx = fs.readFileSync(
  path.join(root, "app/src/lib/workspaceContext.ts"),
  "utf8",
);
const tests = fs.readFileSync(
  path.join(root, "tests/companion-conversation.test.ts"),
  "utf8",
);

for (const token of [
  "companionGreetingReply",
  "maybeRecovery",
  "GENERIC_REPLIES",
  "resolveUnknownGuidance",
]) {
  if (!guidance.includes(token)) {
    fail(`conversationGuidance missing ${token}`);
  }
}

if (
  /won.?t invent|won.?t pretend|won.?t fake/i.test(guidance) ||
  guidance.includes("won’t invent") ||
  guidance.includes("won't invent")
) {
  fail("conversationGuidance must retire invent/pretend chorus");
}

if (!bridge.includes("companionGreetingReply")) {
  fail("intentBridge must use companionGreetingReply for greetings");
}
if (bridge.includes('Try “open ChatGPT”, “take a screenshot”')) {
  fail("greeting must not advertise the default command catalogue");
}

for (const token of [
  "open_pronoun→",
  "winMinimize",
  "previous one",
  "Doing that again from what we just did",
]) {
  if (!ctx.includes(token)) {
    fail(`workspaceContext continuity missing ${token}`);
  }
}

if (!tests.includes("greets calmly") || !tests.includes("continues pronouns")) {
  fail("companion-conversation tests must cover greeting + continuity");
}

const report = fs.readFileSync(
  path.join(
    root,
    "docs/capability-runtime/product-proof/T1_COMPANION_CONVERSATION_BEHAVIOR.md",
  ),
  "utf8",
);
if (!report.includes("Companion Conversation") || !report.includes("Before / after")) {
  fail("T1 report must document companion behaviour and before/after examples");
}

console.log("verify-companion-conversation: ok");
