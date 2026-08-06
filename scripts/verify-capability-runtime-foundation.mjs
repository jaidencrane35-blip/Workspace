#!/usr/bin/env node
/**
 * Verifies P10 Capability Runtime Foundation artifacts and pipeline laws.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-capability-runtime-foundation: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/CAPABILITY_RUNTIME_FOUNDATION.md",
  "docs/capability-runtime/INTENT_LAYER_SPECIFICATION.md",
  "docs/capability-runtime/CAPABILITY_ROUTER_SPECIFICATION.md",
  "docs/capability-runtime/PROVIDER_REGISTRY.md",
  "docs/capability-runtime/CLIPBOARD_PROVIDER.md",
  "docs/capability-runtime/FIVE_PROGRAM_ROADMAP.md",
  "docs/capability-runtime/CAPABILITY_CONTRACTS.md",
  "docs/ui/UI_ARCHITECTURE_SPECIFICATION.md",
  "packages/kernel/src/capability_runtime/mod.rs",
  "packages/kernel/src/capability_runtime/router.rs",
  "packages/kernel/src/capability_runtime/registry.rs",
  "packages/kernel/src/capability_runtime/clipboard_provider.rs",
  "packages/windows-integration/src/clipboard.rs",
  "packages/kernel/src/commands/clipboard.rs",
  "app/src-tauri/src/commands/clipboard.rs",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const router = fs.readFileSync(
  path.join(root, "docs/capability-runtime/CAPABILITY_ROUTER_SPECIFICATION.md"),
  "utf8",
);
for (const token of [
  "Conversation",
  "Intent Layer",
  "Capability Router",
  "Provider Registry",
  "Capability Provider",
  "Desktop Service",
  "Conversation Response",
  "No capability may bypass",
]) {
  if (!router.includes(token)) {
    fail(`router spec missing law token: ${token}`);
  }
}

const clipboard = fs.readFileSync(
  path.join(root, "docs/capability-runtime/CLIPBOARD_PROVIDER.md"),
  "utf8",
);
for (const token of ["WRAP", "arboard", "ClipboardPort", "clipboard.read", "clipboard.write"]) {
  if (!clipboard.includes(token)) {
    fail(`clipboard provider doc missing: ${token}`);
  }
}

const roadmap = fs.readFileSync(
  path.join(root, "docs/capability-runtime/FIVE_PROGRAM_ROADMAP.md"),
  "utf8",
);
for (const token of ["P10", "P11", "P12", "P13", "P14", "Notifications Provider"]) {
  if (!roadmap.includes(token)) {
    fail(`roadmap missing: ${token}`);
  }
}

const libRs = fs.readFileSync(
  path.join(root, "app/src-tauri/src/lib.rs"),
  "utf8",
);
if (!libRs.includes("read_clipboard") || !libRs.includes("write_clipboard")) {
  fail("Tauri generate_handler must register read_clipboard / write_clipboard");
}

const intent = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
if (!intent.includes("clipboardRead") || !intent.includes("clipboardWrite")) {
  fail("intentBridge must define clipboardRead / clipboardWrite");
}

const cargo = fs.readFileSync(
  path.join(root, "packages/windows-integration/Cargo.toml"),
  "utf8",
);
if (!cargo.includes("arboard")) {
  fail("windows-integration must WRAP arboard");
}

console.log("verify-capability-runtime-foundation: ok");
