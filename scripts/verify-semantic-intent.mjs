/**
 * P16.31 — Semantic Intent Engine + Capability Registry guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-semantic-intent: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/semanticIntentEngine.ts",
  "app/src/lib/capabilityRegistry.ts",
  "app/src/lib/intentGrammar.ts",
  "app/src/lib/intentPipeline.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_31_SEMANTIC_INTENT.md",
  "docs/capability-runtime/product-proof/VOICE_P16_32_TRUST_VALIDATION.md",
  "docs/capability-runtime/product-proof/VOICE_P16_33_COGNITIVE_DESKTOP.md",
  "docs/capability-runtime/product-proof/VOICE_P16_34_DESKTOP_COGNITION.md",
  "docs/capability-runtime/product-proof/VOICE_P16_35_PRODUCT_COGNITION.md",
  "tests/semantic-hostile-nl.test.ts",
  "tests/cognitive-desktop-nl.test.ts",
  "tests/execution-planner.test.ts",
  "tests/product-cognition-battery.test.ts",
  "app/src/lib/executionPlanner.ts",
  "app/src/lib/situationGoals.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const engine = fs.readFileSync(
  path.join(root, "app/src/lib/semanticIntentEngine.ts"),
  "utf8",
);
const registry = fs.readFileSync(
  path.join(root, "app/src/lib/capabilityRegistry.ts"),
  "utf8",
);
const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
const appProvider = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/application_provider.rs"),
  "utf8",
);
const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);

if (!engine.includes("resolveSemanticIntent")) {
  fail("semantic engine must export resolveSemanticIntent");
}
if (!engine.includes("reasonCognitiveDesktop")) {
  fail("semantic engine must include cognitive desktop reasoning (P16.33)");
}
if (!engine.includes("resolveDesktopEntity")) {
  fail("semantic engine must resolve desktop entities");
}
if (!engine.includes("ms-windows-store:")) {
  fail("semantic engine must resolve Microsoft Store to protocol (not .exe)");
}
if (!registry.includes("CAPABILITY_GRAPH")) {
  fail("capability registry must define CAPABILITY_GRAPH");
}
if (!registry.includes("generateCapabilityDiscovery")) {
  fail("capability discovery must be generated from the registry");
}
if (!bridge.includes("resolveSemanticIntent")) {
  fail("intentBridge must call Semantic Intent Engine");
}
if (!bridge.includes("capabilityExplain")) {
  fail("intentBridge must support capabilityExplain from the live graph");
}
if (!appProvider.includes("url.dll,FileProtocolHandler")) {
  fail("application launch must materialize ms-* protocols (never spaced .exe)");
}
if (
  !appProvider.includes("won’t invent a program name") &&
  !appProvider.includes("won't invent a program name")
) {
  fail("launch_alias must refuse unknown targets without inventing .exe names");
}
if (appProvider.includes('format!("{other}.exe")') || appProvider.includes('format!("{_unknown}.exe")')) {
  fail("launch_alias must not invent {other}.exe");
}
if (!plan.includes("focus_minimize")) {
  fail("Kernel must compose window.focus_minimize for locate+minimise");
}

const tests = fs.readFileSync(
  path.join(root, "tests/intent-bridge.test.ts"),
  "utf8",
);
for (const phrase of [
  "Open Microsoft Store",
  "Bring GPT to the front",
  "Locate the browser with YouTube",
  "Restore Cursor",
  "Maximise Cursor",
  "Focus Chrome",
  "What can you do",
  "capabilityExplain",
]) {
  if (!tests.includes(phrase)) {
    fail(`intent-bridge tests must cover “${phrase}”`);
  }
}

console.log("verify-semantic-intent: ok");
