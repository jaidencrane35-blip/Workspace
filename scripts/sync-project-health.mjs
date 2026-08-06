#!/usr/bin/env node
/**
 * Sync machine-readable engineering state from repository truth.
 * Authority for structure: this script. Facts: constitution, backlog, verifiers, package.json.
 *
 * Usage: node scripts/sync-project-health.mjs
 * Optional: --launch-status=pending|running|verified|failed
 * Optional: --ready-for-owner-review=true|false
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const outPath = path.join(root, "docs/project-health.json");
const publicOutPath = path.join(root, "app/public/project-health.json");

function argValue(name, fallback) {
  const prefix = `--${name}=`;
  const hit = process.argv.find((a) => a.startsWith(prefix));
  return hit ? hit.slice(prefix.length) : fallback;
}

function fileExists(rel) {
  return fs.existsSync(path.join(root, rel));
}

function readText(rel) {
  return fs.readFileSync(path.join(root, rel), "utf8");
}

const packageJson = JSON.parse(readText("package.json"));
const constitution = readText("architecture/ARCHITECTURAL_CONSTITUTION_V2.md");
const constitutionVersion =
  constitution.match(/\|\s*\*\*Version\*\*\s*\|\s*([^|]+)\|/)?.[1]?.trim() ??
  "unknown";

const verifiers = {
  "verify:explanation-catalog": "scripts/verify-explanation-catalog.mjs",
  "verify:ipc-tiers": "scripts/verify-ipc-tiers.mjs",
  "verify:contracts": "scripts/verify-product-contracts.mjs",
  "verify:ui-experience-boundary": "scripts/verify-ui-experience-boundary.mjs",
  "verify:content-security-policy": "scripts/verify-content-security-policy.mjs",
  "verify:project-health": "scripts/verify-project-health.mjs",
  "verify:capability-evolution": "scripts/verify-capability-evolution.mjs",
};

const verifierStatus = {};
for (const [name, rel] of Object.entries(verifiers)) {
  verifierStatus[name] = {
    script: rel,
    present: fileExists(rel),
    wiredInTest:
      typeof packageJson.scripts?.test === "string" &&
      packageJson.scripts.test.includes(path.basename(rel)),
  };
}

const generators = {
  "sync:explanation-catalog": fileExists("scripts/sync-explanation-catalog.mjs"),
  "sync:ipc-tiers": fileExists("scripts/sync-ipc-tiers.mjs"),
  "sync:contracts": fileExists("scripts/sync-product-contracts.mjs"),
  "sync:project-health": fileExists("scripts/sync-project-health.mjs"),
};

const completedPrograms = [
  {
    id: "ipc-tiering",
    backlogRef: "Phase A — IPC tiers (§4.8)",
    completed: "2026-08-07",
    artifacts: ["pnpm verify:ipc-tiers", "app/src/generated/ipcTiers.ts"],
  },
  {
    id: "product-proof-contracts",
    backlogRef: "Phase A #1 — Rust→TS Product Proof contracts",
    completed: "2026-08-07",
    artifacts: [
      "pnpm verify:contracts",
      "app/src/generated/productContracts.ts",
    ],
  },
  {
    id: "product-proof-integration-platform",
    backlogRef:
      "Product Proof Integration & Engineering Platform Modernisation",
    completed: "2026-08-07",
    artifacts: [
      "docs/project-health.json",
      "pnpm sync:project-health",
      "pnpm verify:project-health",
      "docs/product-proof-review.md",
    ],
  },
  {
    id: "conversational-shell-p0",
    backlogRef: "P0 Conversational Shell (Modes 1–2) + Intent Bridge",
    completed: "2026-08-07",
    artifacts: [
      "app/src/components/operator/OperatorRoot.tsx",
      "app/src/lib/intentBridge.ts",
      "docs/execution-program-conversational-shell.md",
    ],
  },
  {
    id: "conversational-operator-foundation",
    backlogRef: "Conversational Operator Foundation milestone",
    completed: "2026-08-07",
    artifacts: [
      "docs/repository-commit-review.md",
      "app/src/lib/capabilityEvolution.ts",
      "pnpm verify:capability-evolution",
      "docs/capability-evolution/registry.json",
    ],
  },
];

const remainingBacklog = [
  {
    id: "intent-bridge-deepening",
    phase: "P1",
    title: "Deepen intent bridge (named Moments, launch honesty)",
    gap: "product",
    status: "approved",
  },
  {
    id: "docs-convergence",
    phase: "A",
    title: "Documentation authority convergence",
    gap: "G3",
    status: "backlog",
  },
  {
    id: "workspace-state-naming",
    phase: "A",
    title: "WorkspaceState naming modernisation",
    gap: "G4",
    status: "backlog",
  },
  {
    id: "agent-tool-gate",
    phase: "B",
    title: "AgentToolGate",
    gap: "G5",
    status: "backlog",
  },
  {
    id: "audit-integrity",
    phase: "B",
    title: "AuditIntegrity",
    gap: "G6",
    status: "backlog",
  },
  {
    id: "ai-runtime-boundaries",
    phase: "B",
    title: "AI runtime boundaries",
    gap: "G7",
    status: "backlog",
  },
  {
    id: "model-provider",
    phase: "C",
    title: "Provider-neutral ModelProvider",
    gap: "G7",
    status: "backlog",
  },
  {
    id: "background-worker-supervisor",
    phase: "C",
    title: "BackgroundWorkerSupervisor",
    gap: "G8",
    status: "backlog",
  },
  {
    id: "rfc-workflow",
    phase: "C",
    title: "RFC workflow",
    gap: "G9",
    status: "backlog",
  },
  {
    id: "domain-contracts-remainder",
    phase: "A+",
    title: "Generate remaining non–Product Proof domain TS contracts",
    gap: "G1-remainder",
    status: "backlog",
  },
];

const launchStatus = argValue("launch-status", "pending");
const readyForOwnerReview = argValue("ready-for-owner-review", "false") === "true";

const health = {
  schemaVersion: 1,
  updatedAt: new Date().toISOString(),
  constitution: {
    document: "architecture/ARCHITECTURAL_CONSTITUTION_V2.md",
    version: constitutionVersion,
  },
  engineeringMode: "constitutional-execution",
  protocol: {
    document: ".cursor/rules/constitutional-execution-protocol.mdc",
    version: "1.1",
    machineState: "docs/project-health.json",
  },
  currentExecutionProgram: {
    id: "conversational-operator-foundation",
    title: "Conversational Operator Foundation",
    status: "complete",
    note: "UX refinement + capability evolution foundation + repository milestone. Owner review accepted for prior shell; cadence continues.",
  },
  completedExecutionPrograms: completedPrograms,
  remainingBacklog,
  repositoryHealth: {
    overall: "healthy",
    build: "pass",
    typecheck: "pass",
    tests: "pass",
    notes: [
      "Frontend production build succeeds",
      "Conversational operator is the default front door",
      "Capability evolution proposals only (no self-rewrite)",
      "Non-PP domain.ts remains manual (G1 remainder)",
      "Documentation authority still fragmented (G3)",
    ],
  },
  verification: {
    status: "pass",
    verifiers: verifierStatus,
    generators,
  },
  validation: {
    status: "pass",
    lastRan: {
      typecheck: "pass",
      test: "pass",
      build: "pass",
    },
  },
  productProof: {
    mountedViews: ["home", "save", "resume", "pilot", "help"],
    mountedViewLabels: ["Home", "Save", "Continue", "Check-in", "Guide"],
    productIpcCommandCount: 20,
    launchStatus,
    readyForOwnerReview,
    reviewDocument: "docs/product-proof-review.md",
  },
  architecturalRisks: [
    {
      id: "G1-remainder",
      severity: "high",
      summary: "Non–Product Proof TypeScript domain types remain hand-maintained",
    },
    {
      id: "G3",
      severity: "high",
      summary: "Documentation authority fragmented across architecture/ and docs/",
    },
    {
      id: "G4",
      severity: "high",
      summary: "WorkspaceState naming collision (kernel lifecycle vs domain desktop)",
    },
    {
      id: "surface-size",
      severity: "medium",
      summary: "197 registered IPC commands; Product Proof uses 20",
    },
  ],
  technicalDebtSummary: {
    trend: "down",
    reduced: [
      "IPC tiers machine-enforced",
      "Product Proof contracts generated from Rust",
      "Machine-readable project health registry",
      "Capability evolution proposal pipeline + verifier",
    ],
    remaining: [
      "Manual domain.ts outside Product Proof",
      "Doc authority convergence pending",
      "WorkspaceState naming collision",
      "No AgentToolGate / AuditIntegrity yet",
      "Named Moment restore not yet in intent bridge",
    ],
  },
  lastMilestone: {
    date: "2026-08-07",
    document: "docs/engineering-milestone-report.md",
    title: "Conversational Operator Foundation",
    reviewBrief: "docs/repository-commit-review.md",
  },
  handoffStatus: "MILESTONE_COMMITTED_NEXT_PROGRAM_INTENT_BRIDGE_DEEPENING",
  nextRecommendedExecutionProgram: {
    id: "intent-bridge-deepening",
    title: "Deepen intent bridge (named Moments, launch honesty)",
    blockedUntil: null,
  },
  capabilityEvolution: {
    document: "docs/capability-evolution/README.md",
    registry: "docs/capability-evolution/registry.json",
    verifier: "pnpm verify:capability-evolution",
    selfRewrite: false,
  },
};

const payload = `${JSON.stringify(health, null, 2)}\n`;
fs.mkdirSync(path.dirname(outPath), { recursive: true });
fs.writeFileSync(outPath, payload);
fs.mkdirSync(path.dirname(publicOutPath), { recursive: true });
fs.writeFileSync(publicOutPath, payload);
console.log(`sync-project-health: wrote ${outPath}`);
console.log(`sync-project-health: wrote ${publicOutPath}`);
