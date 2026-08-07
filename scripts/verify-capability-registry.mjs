/**
 * P16.32 — Capability Registry is the single discovery authority.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-capability-registry: ${msg}`);
  process.exit(1);
}

const registryPath = path.join(root, "app/src/lib/capabilityRegistry.ts");
if (!fs.existsSync(registryPath)) {
  fail("missing capabilityRegistry.ts");
}
const registry = fs.readFileSync(registryPath, "utf8");

for (const token of [
  "CAPABILITY_GRAPH",
  "limitations",
  "documentation",
  "failureRecovery",
  "arguments",
  "related",
  "similar",
  "alternatives",
  "discoverability",
  "ownership",
  "purpose",
  "permissions",
  "dependencies",
  "benchmark",
  "architecturalJustification",
  "validateCapabilityGraphGovernance",
  "Related / similar / alternatives",
  "describeCapability",
  "generateCapabilityDiscovery",
  "generateRecoveryGuidance",
  "isCapabilityDiscoveryUtterance",
  "suggestNearbyCapabilities",
  "resolveDiscoveryScope",
]) {
  if (!registry.includes(token)) {
    fail(`capability registry must define ${token}`);
  }
}

// Discovery generation must pull examples from nodes — not a separate hard-coded catalogue.
if (!registry.includes("node.examples")) {
  fail("discovery must generate examples from CAPABILITY_GRAPH nodes");
}
if (!registry.includes("node.limitations") || !registry.includes("node.requirements")) {
  fail("discovery must self-describe limitations and requirements from the graph");
}
if (!registry.includes("won’t overclaim") && !registry.includes("won't overclaim")) {
  fail("discovery must include can/cannot self-description");
}

const engine = fs.readFileSync(
  path.join(root, "app/src/lib/semanticIntentEngine.ts"),
  "utf8",
);
if (!engine.includes("generateCapabilityDiscovery(resolveDiscoveryScope")) {
  fail("semantic engine must scope discovery from the live registry");
}
const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
if (!bridge.includes("suggestNearbyCapabilities")) {
  fail("unknown opens must suggest nearby capabilities from the registry");
}

const appProvider = fs.readFileSync(
  path.join(
    root,
    "packages/kernel/src/capability_runtime/application_provider.rs",
  ),
  "utf8",
);
if (appProvider.includes('format!("{other}.exe")') || appProvider.includes("format!(\"{other}.exe\")")) {
  fail("Kernel must not invent {other}.exe for unknown launch targets");
}
if (!appProvider.includes("won’t invent a program name") && !appProvider.includes("won't invent a program name")) {
  fail("Kernel launch_alias must refuse unknown targets with truthful recovery");
}

const pipeline = path.join(root, "app/src/lib/intentPipeline.ts");
if (!fs.existsSync(pipeline)) {
  fail("missing intentPipeline.ts (pipeline evidence)");
}

const hostile = path.join(root, "tests/semantic-hostile-nl.test.ts");
if (!fs.existsSync(hostile)) {
  fail("missing semantic-hostile-nl.test.ts");
}

console.log("verify-capability-registry: ok");
