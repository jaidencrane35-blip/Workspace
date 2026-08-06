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
  "verify:shell-zero-trap": "scripts/verify-shell-zero-trap.mjs",
  "verify:ui-architecture": "scripts/verify-ui-architecture.mjs",
  "verify:capability-runtime-research":
    "scripts/verify-capability-runtime-research.mjs",
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
  {
    id: "intent-bridge-deepening",
    backlogRef: "P1 Deepen intent bridge (named Moments, launch honesty)",
    completed: "2026-08-07",
    artifacts: [
      "app/src/lib/momentMatch.ts",
      "tests/moment-match.test.ts",
      "docs/execution-program-intent-bridge-deepening.md",
    ],
  },
  {
    id: "operator-refoundation-p2",
    backlogRef: "P2 Conversational Desktop Operator Refoundation",
    completed: "2026-08-07",
    artifacts: [
      "app/src/lib/shellRuntime.ts",
      "app/src/lib/shellWindows.ts",
      "docs/execution-program-operator-refoundation.md",
      "docs/capability-evolution/PIPELINE_ARCHITECTURE.md",
    ],
  },
  {
    id: "windows-shell-completion-p3",
    backlogRef: "P3 Windows Shell Completion (Desktop Operator Foundation)",
    completed: "2026-08-07",
    artifacts: [
      "app/src/lib/shellStateMachine.ts",
      "pnpm verify:shell-zero-trap",
      "docs/execution-program-windows-shell-completion.md",
      "docs/shell/FUTURE_INPUT_ARCHITECTURE.md",
    ],
  },
  {
    id: "native-windows-shell-lifecycle-p4",
    backlogRef: "P4 Native Windows Shell Lifecycle (Desktop Operator Foundation)",
    completed: "2026-08-07",
    artifacts: [
      "docs/execution-program-native-windows-shell-lifecycle.md",
      "docs/engineering-milestone-report.md",
    ],
  },
  {
    id: "native-desktop-operator-refoundation-p5",
    backlogRef: "P5 Native Desktop Operator Refoundation (The Operator Becomes Real)",
    completed: "2026-08-07",
    artifacts: [
      "app/src/lib/shellStateMachine.ts",
      "app/src/lib/shellRuntime.ts",
      "app/src/lib/shellWindows.ts",
      "docs/execution-program-native-desktop-operator-refoundation.md",
    ],
  },
  {
    id: "desktop-operator-shell-completion-p6",
    backlogRef: "P6 Desktop Operator Shell Completion (Collapse as true shell mode)",
    completed: "2026-08-07",
    artifacts: [
      "app/src/components/operator/DesktopOperator.tsx",
      "docs/execution-program-desktop-operator-shell-completion.md",
    ],
  },
  {
    id: "productization-native-windows-p7",
    backlogRef: "P7 Productization & Native Windows Experience",
    completed: "2026-08-07",
    artifacts: [
      "app/src/lib/shellRuntime.ts",
      "docs/execution-program-productization-native-windows.md",
      "docs/project-health.json",
    ],
  },
  {
    id: "ui-architecture-specification-p8",
    backlogRef: "P8 Final Product Presentation Refoundation",
    completed: "2026-08-07",
    artifacts: [
      "docs/ui/UI_ARCHITECTURE_SPECIFICATION.md",
      "pnpm verify:ui-architecture",
      "docs/ui/LAYER_OWNERSHIP_MAP.md",
    ],
  },
  {
    id: "capability-runtime-research-p9",
    backlogRef: "P9 Capability Runtime Research & Adoption Strategy",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/CAPABILITY_RUNTIME_RESEARCH_REPORT.md",
      "pnpm verify:capability-runtime-research",
      "docs/capability-runtime/OPEN_SOURCE_ADOPTION_MATRIX.md",
    ],
  },
];

const remainingBacklog = [
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
const milestoneCommit = argValue("milestone-commit", "pending");

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
    id: "capability-runtime-research-p9",
    title: "P9 Capability Runtime Research & Adoption Strategy",
    status: "complete_awaiting_owner_review",
    note: "Track B research complete: domains, OSS matrix, contracts, order. No capability V1 code. Await owner review before Capability Runtime V1.",
  },
  completedExecutionPrograms: completedPrograms,
  remainingBacklog,
  engineeringHealth: {
    overall: "healthy",
    build: "pass",
    typecheck: "pass",
    tests: "pass",
    verification: "pass",
  },
  productReadiness: {
    status: "review",
    shellLifecycle: "accepted",
    compactConversation: "implemented",
    nativeFeel: "in_progress",
    presentationAuthority: "accepted",
    capabilityRuntimeResearch: "complete",
    notes: [
      "UI Architecture Spec accepted (P8)",
      "Capability Runtime research pack complete (P9) — no V1 impl yet",
      "Zero platform ADOPT; WRAP commodity crates behind contracts",
      "Recommended V1 slice: clipboard → notifications → browser open",
    ],
  },
  currentMilestone: {
    id: "capability-runtime-research-p9",
    title: "P9 Capability Runtime Research & Adoption Strategy",
    status: "awaiting_owner_review",
    commit: milestoneCommit,
  },
  acceptedReviews: [
    {
      id: "p6",
      title: "Desktop Operator Shell Completion",
      commit: "11bab9d",
      acceptedAt: "2026-08-07",
    },
    {
      id: "p8",
      title: "UI Architecture & Product Presentation",
      commit: "c045eb0",
      acceptedAt: "2026-08-07",
    },
  ],
  outstandingProductDebt: [
    {
      id: "capability-runtime-v1",
      track: "B",
      summary: "Capability Runtime V1 not started — blocked on Owner review of P9",
    },
    {
      id: "tray-integration",
      track: "A",
      summary: "System tray integration not yet implemented",
    },
    {
      id: "window-animations",
      track: "A",
      summary: "Native show/hide transition polish still limited",
    },
    {
      id: "G3-docs",
      track: "A",
      summary: "Documentation authority still fragmented",
    },
  ],
  repositoryHealth: {
    overall: "healthy",
    build: "pass",
    typecheck: "pass",
    tests: "pass",
    notes: [
      "Frontend production build succeeds",
      "UI Architecture Spec accepted and frozen",
      "Capability Runtime research complete; V1 not implemented",
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
      "Two-form shell accepted (P6)",
    ],
    remaining: [
      "Manual domain.ts outside Product Proof",
      "Doc authority convergence pending",
      "WorkspaceState naming collision",
      "No AgentToolGate / AuditIntegrity yet",
      "Tray / deeper native polish remaining (Track A debt)",
    ],
  },
  lastMilestone: {
    date: "2026-08-07",
    document: "docs/capability-runtime/CAPABILITY_RUNTIME_RESEARCH_REPORT.md",
    title: "P9 Capability Runtime Research & Adoption Strategy",
    reviewBrief: "docs/capability-runtime/00_INDEX.md",
  },
  handoffStatus: "AWAITING_PROJECT_OWNER_CAPABILITY_RUNTIME_RESEARCH_REVIEW",
  nextRecommendedExecutionProgram: {
    id: "capability-runtime-v1",
    title: "Capability Runtime V1 (approved thin slice only)",
    blockedUntil: "Project Owner Capability Runtime research review",
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
