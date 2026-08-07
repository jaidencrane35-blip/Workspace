#!/usr/bin/env node
/**
 * Verifies P13 Notifications Provider wiring + Product Proof artifacts.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-notifications-provider: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/NOTIFICATIONS_PROVIDER.md",
  "docs/capability-runtime/research/NOTIFICATIONS_RESEARCH.md",
  "docs/capability-runtime/product-proof/NOTIFICATIONS_PROVIDER_PRODUCT_PROOF.md",
  "docs/capability-runtime/product-proof/notifications-provider.proof.json",
  "packages/windows-integration/src/notification.rs",
  "packages/kernel/src/capability_runtime/notification_provider.rs",
  "packages/kernel/src/commands/notification.rs",
  "tests/notifications-provider-product-proof.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const proof = JSON.parse(
  fs.readFileSync(
    path.join(
      root,
      "docs/capability-runtime/product-proof/notifications-provider.proof.json",
    ),
    "utf8",
  ),
);
if (proof.provider !== "notifications" || proof.program !== "P13") {
  fail("proof harness must declare provider=notifications program=P13");
}
if (proof.ipc !== "execute_capability_intent") {
  fail("Conversation ipc must be execute_capability_intent");
}
if (!proof.pipeline?.includes("Kernel Operator")) {
  fail("pipeline must include Kernel Operator");
}

const intent = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
for (const token of ["notifyStatus", "notifyShow", "notifyDismiss", "resolveNotificationIntent"]) {
  if (!intent.includes(token)) {
    fail(`intentBridge must include ${token}`);
  }
}
if (/Notification Provider|WinRT|tauri-winrt/i.test(intent)) {
  fail("intentBridge must not leak notification implementation jargon");
}

const runtime = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/mod.rs"),
  "utf8",
);
if (!runtime.includes("NotificationProvider")) {
  fail("Capability Runtime must register NotificationProvider");
}

const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
if (!plan.includes("notifications")) {
  fail("Kernel Operator plan must allow notifications domain");
}

console.log("verify-notifications-provider: ok");
