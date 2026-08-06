#!/usr/bin/env node
/**
 * Verifies authoritative UI Architecture Specification artifacts + key laws.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-ui-architecture: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/ui/UI_ARCHITECTURE_SPECIFICATION.md",
  "docs/ui/PRODUCT_PRESENTATION_SPECIFICATION.md",
  "docs/ui/COMPONENT_OWNERSHIP_MAP.md",
  "docs/ui/WINDOW_LIFECYCLE_SPECIFICATION.md",
  "docs/ui/LAYER_OWNERSHIP_MAP.md",
  "docs/ui/PRODUCT_READINESS_ASSESSMENT.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const spec = fs.readFileSync(
  path.join(root, "docs/ui/UI_ARCHITECTURE_SPECIFICATION.md"),
  "utf8",
);

for (const token of [
  "Authoritative",
  "Layer 1",
  "Layer 2",
  "Layer 3",
  "Layer 4",
  "Desktop Operator",
  "Conversation",
  "Capability Runtime",
  "Workspace Intelligence",
  "Expanded Workspace",
  "Never appears",
  "Track A",
  "Track B",
  "Track C",
  "conversation-defining",
]) {
  if (!spec.includes(token)) {
    fail(`UI Architecture Spec missing required token: ${token}`);
  }
}

// Expanded Workspace must be presentation, not a third shell form.
if (!spec.includes("Not** a third shell form") && !spec.includes("Not a third shell form")) {
  fail("Spec must state Expanded Workspace is not a third shell form");
}

const rootUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
  "utf8",
);
if (rootUi.includes("op-shell__btn--primary") && rootUi.includes("Expand")) {
  fail("OperatorRoot must not expose Expand chrome (rejected)");
}
if (/Ask for Save,\s*Continue/.test(rootUi)) {
  fail("OperatorRoot must not teach capabilities via dock copy");
}

console.log("verify-ui-architecture: ok");
