#!/usr/bin/env node
/**
 * Verify P16.PI2 Gate C — single-instance process integrity is wired.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-single-instance: ${message}`);
  process.exit(1);
}

const cargoPath = path.join(root, "app/src-tauri/Cargo.toml");
const libPath = path.join(root, "app/src-tauri/src/lib.rs");
const reportPath = path.join(
  root,
  "docs/production/P16_PI2_GATE_C_SINGLE_INSTANCE.md",
);
const gatesPath = path.join(root, "docs/production/PRODUCTION_GATES.md");
const matrixPath = path.join(root, "docs/production/production-readiness.json");

for (const p of [cargoPath, libPath, reportPath, gatesPath, matrixPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const cargo = fs.readFileSync(cargoPath, "utf8");
if (!/tauri-plugin-single-instance\s*=/.test(cargo)) {
  fail("Cargo.toml missing tauri-plugin-single-instance dependency");
}

const lib = fs.readFileSync(libPath, "utf8");
if (!lib.includes("tauri_plugin_single_instance::init")) {
  fail("lib.rs must register tauri_plugin_single_instance::init");
}
if (!lib.includes("focus_primary_instance")) {
  fail("lib.rs must focus primary instance on secondary launch");
}

// Plugin must appear before invoke_handler / setup kernel path.
const pluginIdx = lib.indexOf("tauri_plugin_single_instance::init");
const invokeIdx = lib.indexOf("invoke_handler");
const kernelIdx = lib.indexOf("WorkspaceKernel::initialize");
if (pluginIdx < 0 || invokeIdx < 0 || pluginIdx > invokeIdx) {
  fail("single-instance plugin must be registered before invoke_handler");
}
if (kernelIdx > 0 && pluginIdx > kernelIdx) {
  fail("single-instance plugin must be registered before kernel initialization");
}

const matrix = JSON.parse(fs.readFileSync(matrixPath, "utf8"));
if (!matrix.completedGates?.includes("C-single-instance")) {
  fail("production-readiness.json completedGates must include C-single-instance");
}

const startup = matrix.areas?.find((a) => a.id === "startupBehaviour");
if (!startup?.evidence?.toLowerCase().includes("single-instance")) {
  fail("startupBehaviour evidence must mention single-instance");
}

console.log("verify-single-instance: ok (plugin first + focus + matrix gate)");
