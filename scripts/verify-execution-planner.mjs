/**
 * P16.34 — Execution planner + Capability Registry intelligence guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-execution-planner: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/executionPlanner.ts",
  "app/src/lib/capabilityRegistry.ts",
  "app/src/lib/intentPipeline.ts",
  "tests/execution-planner.test.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_34_DESKTOP_COGNITION.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const planner = fs.readFileSync(
  path.join(root, "app/src/lib/executionPlanner.ts"),
  "utf8",
);
const registry = fs.readFileSync(
  path.join(root, "app/src/lib/capabilityRegistry.ts"),
  "utf8",
);
const pipeline = fs.readFileSync(
  path.join(root, "app/src/lib/intentPipeline.ts"),
  "utf8",
);

if (!planner.includes("buildExecutionPlan")) {
  fail("execution planner must export buildExecutionPlan");
}
if (!planner.includes("delegated_to_kernel")) {
  fail("plans must mark Kernel-delegated steps explicitly");
}
if (!registry.includes("failureRecovery") || !registry.includes("arguments:")) {
  fail("CapabilityNode must declare arguments + failureRecovery");
}
if (!registry.includes("related:") || !registry.includes("describeCapability")) {
  fail("registry must support related capabilities + describeCapability");
}
if (!pipeline.includes("execution_plan") || !pipeline.includes("buildExecutionPlan")) {
  fail("intent pipeline must record execution_plan evidence");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-execution-planner.mjs")) {
  fail("package.json must wire verify-execution-planner into pnpm test");
}

console.log("verify-execution-planner: ok");
