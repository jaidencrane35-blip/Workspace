#!/usr/bin/env node
/**
 * Verifies Product Gravity Rule + Conversational Desktop Surface (P12.6).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-product-gravity: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/ui/PRODUCT_GRAVITY_RULE.md",
  "docs/ui/WORKSPACE_INTERACTION_LANGUAGE.md",
  "docs/ui/UI_ARCHITECTURE_SPECIFICATION.md",
  "docs/ui/PRODUCT_PRESENTATION_SPECIFICATION.md",
  "docs/ui/WINDOW_LIFECYCLE_SPECIFICATION.md",
  ".cursor/rules/constitutional-execution-protocol.mdc",
  "app/src-tauri/tauri.conf.json",
  "app/src/lib/shellRuntime.ts",
  "app/src/lib/shellWindows.ts",
  "app/src/components/operator/OperatorRoot.tsx",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const gravity = fs.readFileSync(
  path.join(root, "docs/ui/PRODUCT_GRAVITY_RULE.md"),
  "utf8",
);
for (const token of [
  "Product Gravity",
  "Conversation",
  "Desktop operation",
  "Trust",
  "Conversation Complete ≠ Product Complete",
  "desktop operation",
]) {
  if (!gravity.includes(token)) {
    fail(`PRODUCT_GRAVITY_RULE.md missing: ${token}`);
  }
}

const protocol = fs.readFileSync(
  path.join(root, ".cursor/rules/constitutional-execution-protocol.mdc"),
  "utf8",
);
if (!protocol.includes("Product Gravity Rule")) {
  fail("protocol must document Product Gravity Rule");
}
if (!protocol.includes("Conversation Complete ≠ Product Complete")) {
  fail("protocol must distinguish Conversation Complete vs Product Complete");
}

const conf = JSON.parse(
  fs.readFileSync(path.join(root, "app/src-tauri/tauri.conf.json"), "utf8"),
);
const main = conf.app?.windows?.find((w) => w.label === "main");
if (!main) {
  fail("tauri.conf missing main window");
}
if (main.decorations !== false) {
  fail("main window must be undecorated (Product Gravity)");
}
if (main.transparent !== true) {
  fail("main window must be transparent (conversation on the desktop)");
}

const runtime = fs.readFileSync(
  path.join(root, "app/src/lib/shellRuntime.ts"),
  "utf8",
);
if (!/loadShellMode\(fallback: ShellMode = 1\)/.test(runtime)) {
  fail("loadShellMode default fallback must be Conversation (1)");
}

const windows = fs.readFileSync(
  path.join(root, "app/src/lib/shellWindows.ts"),
  "utf8",
);
if (windows.includes("setDecorations(true)")) {
  fail("shellWindows must not re-enable OS decorations on Conversation");
}
if (!windows.includes("loadShellMode(1)")) {
  fail("bootstrap must default to Conversation gravity");
}

const rootUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
  "utf8",
);
if (!rootUi.includes('data-gravity="conversation"')) {
  fail("OperatorRoot must mark conversation gravity surface");
}
if (!rootUi.includes("loadShellMode(1)")) {
  fail("OperatorRoot must default loadShellMode to Conversation");
}
// P16.PX3: after voice review, Send is the one obvious next action.
if (!rootUi.includes("voiceReviewPending") || !rootUi.includes("data-voice-ready")) {
  fail("OperatorRoot must soft-emphasize Send after voice transcript (P16.PX3)");
}

const css = fs.readFileSync(path.join(root, "app/src/App.css"), "utf8");
if (!css.includes("backdrop-filter")) {
  fail("conversation surface must use translucent host treatment");
}
if (!css.includes('.op-shell__send[data-voice-ready="true"]')) {
  fail("App.css must style voice-ready Send cue (P16.PX3)");
}

const interaction = fs.readFileSync(
  path.join(root, "docs/ui/WORKSPACE_INTERACTION_LANGUAGE.md"),
  "utf8",
);
for (const token of [
  "One obvious action",
  "Calm over clever",
  "Trust over spectacle",
  "Conversation",
  "prefers-reduced-motion",
]) {
  if (!interaction.includes(token)) {
    fail(`WORKSPACE_INTERACTION_LANGUAGE.md missing: ${token}`);
  }
}

const lifecycle = fs.readFileSync(
  path.join(root, "docs/ui/WINDOW_LIFECYCLE_SPECIFICATION.md"),
  "utf8",
);
if (!lifecycle.includes("Product Gravity default")) {
  fail("window lifecycle must document Conversation-first launch");
}

console.log("verify-product-gravity: ok");
