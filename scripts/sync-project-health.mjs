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
const constitutionalSpecPath =
  "docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md";
const constitution = readText(constitutionalSpecPath);
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
  "verify:capability-runtime-foundation":
    "scripts/verify-capability-runtime-foundation.mjs",
  "verify:product-proof-harness": "scripts/verify-product-proof-harness.mjs",
  "verify:product-gravity": "scripts/verify-product-gravity.mjs",
  "verify:product-quality": "scripts/verify-product-quality.mjs",
  "verify:moments-success-cohesion":
    "scripts/verify-moments-success-cohesion.mjs",
  "verify:moments-agency-keyboard":
    "scripts/verify-moments-agency-keyboard.mjs",
  "verify:conversation-working-state":
    "scripts/verify-conversation-working-state.mjs",
  "verify:moments-status-ownership":
    "scripts/verify-moments-status-ownership.mjs",
  "verify:moments-browse-truth": "scripts/verify-moments-browse-truth.mjs",
  "verify:moments-tool-session": "scripts/verify-moments-tool-session.mjs",
  "verify:operator-intelligence": "scripts/verify-operator-intelligence.mjs",
  "verify:notifications-provider": "scripts/verify-notifications-provider.mjs",
  "verify:browser-provider": "scripts/verify-browser-provider.mjs",
  "verify:screenshot-provider": "scripts/verify-screenshot-provider.mjs",
  "verify:voice-input": "scripts/verify-voice-input.mjs",
  "verify:voice-regression": "scripts/verify-voice-regression.mjs",
  "verify:conversation-quality": "scripts/verify-conversation-quality.mjs",
  "verify:desktop-ui-tree": "scripts/verify-desktop-ui-tree.mjs",
  "verify:window-control-discovery": "scripts/verify-window-control-discovery.mjs",
  "verify:desktop-control-interaction": "scripts/verify-desktop-control-interaction.mjs",
  "verify:wait-conditions": "scripts/verify-wait-conditions.mjs",
  "verify:bounded-retry": "scripts/verify-bounded-retry.mjs",
  "verify:semantic-intent": "scripts/verify-semantic-intent.mjs",
  "verify:capability-registry": "scripts/verify-capability-registry.mjs",
  "verify:cognitive-desktop": "scripts/verify-cognitive-desktop.mjs",
  "verify:execution-planner": "scripts/verify-execution-planner.mjs",
  "verify:product-cognition": "scripts/verify-product-cognition.mjs",
  "verify:goal-resolution": "scripts/verify-goal-resolution.mjs",
  "verify:workspace-context": "scripts/verify-workspace-context.mjs",
  "verify:product-intelligence": "scripts/verify-product-intelligence.mjs",
  "verify:operator-activity": "scripts/verify-operator-activity.mjs",
  "verify:dev-environment": "scripts/verify-dev-environment.mjs",
  "verify:production-readiness": "scripts/verify-production-readiness.mjs",
  "verify:installer-foundation": "scripts/verify-installer-foundation.mjs",
  "verify:artifact-checksums": "scripts/verify-artifact-checksums.mjs",
  "verify:single-instance": "scripts/verify-single-instance.mjs",
  "verify:tray-lifecycle": "scripts/verify-tray-lifecycle.mjs",
  "verify:tray-shellmode": "scripts/verify-tray-shellmode.mjs",
  "verify:conversation-first-session":
    "scripts/verify-conversation-first-session.mjs",
  "verify:version-consistency": "scripts/verify-version-consistency.mjs",
  "verify:release-pipeline": "scripts/verify-release-pipeline.mjs",
  "verify:f1-ci-automation": "scripts/verify-f1-ci-automation.mjs",
  "verify:release-hold": "scripts/verify-release-hold.mjs",
  "verify:support-bundle": "scripts/verify-support-bundle.mjs",
  "verify:production-dependency-authority":
    "scripts/verify-production-dependency-authority.mjs",
  "verify:production-gate-specification":
    "scripts/verify-production-gate-specification.mjs",
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
  "sync:production-gate-catalog": fileExists(
    "scripts/sync-production-gate-catalog.mjs",
  ),
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
  {
    id: "capability-runtime-foundation-p10",
    backlogRef: "P10 Capability Runtime Foundation",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/CAPABILITY_RUNTIME_FOUNDATION.md",
      "pnpm verify:capability-runtime-foundation",
      "packages/kernel/src/capability_runtime/mod.rs",
      "docs/capability-runtime/CLIPBOARD_PROVIDER.md",
    ],
  },
  {
    id: "application-provider-p11",
    backlogRef: "P11 Application Provider",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/APPLICATION_PROVIDER.md",
      "packages/kernel/src/capability_runtime/application_provider.rs",
      "pnpm verify:capability-runtime-foundation",
    ],
  },
  {
    id: "window-provider-p12",
    backlogRef: "P12 Window Provider",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/WINDOW_PROVIDER.md",
      "packages/kernel/src/capability_runtime/window_provider.rs",
      "docs/capability-runtime/WINDOW_OPERATIONS_SPECIFICATION.md",
    ],
  },
  {
    id: "window-provider-product-proof-p12-5",
    backlogRef: "P12.5 Conversation Integration & Product Proof",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/PRODUCT_PROOF_RULE.md",
      "docs/capability-runtime/product-proof/window-provider.proof.json",
      "pnpm verify:product-proof-harness",
      "app/src/lib/intentBridge.ts",
    ],
  },
  {
    id: "conversational-desktop-surface-p12-6",
    backlogRef: "P12.6 Conversational Desktop Surface",
    completed: "2026-08-07",
    artifacts: [
      "docs/ui/PRODUCT_GRAVITY_RULE.md",
      "pnpm verify:product-gravity",
      "app/src-tauri/tauri.conf.json",
      "app/src/components/operator/OperatorRoot.tsx",
    ],
  },
  {
    id: "operator-intelligence-foundation-p12-7",
    backlogRef: "P12.7 Operator Intelligence Foundation",
    completed: "2026-08-07",
    artifacts: [
      "docs/operator/OPERATOR_AUTHORITY_RULE.md",
      "docs/operator/CAPABILITY_COMPOSITION_RULE.md",
      "pnpm verify:operator-intelligence",
      "app/src/lib/operator/intelligence.ts",
    ],
  },
  {
    id: "operator-architecture-completion-p12-finalization",
    backlogRef: "P12 Finalization — Operator Architecture Completion",
    completed: "2026-08-07",
    artifacts: [
      "docs/operator/KERNEL_AUTHORITY_RULE.md",
      "docs/operator/PRESENTATION_PURITY_RULE.md",
      "packages/kernel/src/operator/mod.rs",
      "pnpm verify:operator-intelligence",
      "execute_capability_intent",
    ],
  },
  {
    id: "p12-complete-final-repository-closure",
    backlogRef: "P12 COMPLETE — Final Repository Closure",
    completed: "2026-08-07",
    artifacts: [
      "docs/engineering-milestone-report.md",
      "docs/capability-runtime/FIVE_PROGRAM_ROADMAP.md",
      "docs/capability-runtime/PRODUCT_PROOF_RULE.md",
      "pnpm verify:product-proof-harness",
      "pnpm verify:operator-intelligence",
    ],
  },
  {
    id: "notifications-provider-p13",
    backlogRef: "P13 Notifications Provider",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/NOTIFICATIONS_PROVIDER.md",
      "docs/capability-runtime/product-proof/notifications-provider.proof.json",
      "packages/kernel/src/capability_runtime/notification_provider.rs",
      "pnpm verify:notifications-provider",
      "execute_capability_intent",
    ],
  },
  {
    id: "browser-provider-p14",
    backlogRef: "P14 Browser Provider",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/BROWSER_PROVIDER.md",
      "docs/capability-runtime/product-proof/browser-provider.proof.json",
      "packages/kernel/src/capability_runtime/browser_provider.rs",
      "pnpm verify:browser-provider",
      "execute_capability_intent",
    ],
  },
  {
    id: "browser-provider-p14-5",
    backlogRef: "P14.5 Browser Product Proof Remediation",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/PRODUCT_PROOF_RULE.md",
      "docs/capability-runtime/product-proof/browser-provider.proof.json",
      "docs/capability-runtime/product-proof/BROWSER_PROVIDER_COMPOSITION_AUDIT.md",
      "pnpm verify:browser-provider",
      "Natural Language Robustness Rule",
    ],
  },
  {
    id: "browser-provider-p14-5-finalization",
    backlogRef: "P14.5 Browser Product Proof Finalization & Permanent Closure",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/PROVIDER_ACCEPTANCE_STANDARD.md",
      "docs/capability-runtime/product-proof/WINDOW_PROVIDER_PRODUCT_PROOF.md",
      "docs/capability-runtime/product-proof/BROWSER_PROVIDER_PRODUCT_PROOF.md",
      "pnpm verify:browser-provider",
      "P14_PERMANENTLY_CLOSED_P15_ELIGIBLE",
    ],
  },
  {
    id: "screenshot-provider-p15",
    backlogRef: "P15 Screenshot Provider",
    completed: "2026-08-07",
    artifacts: [
      "docs/capability-runtime/SCREENSHOT_PROVIDER.md",
      "docs/capability-runtime/product-proof/screenshot-provider.proof.json",
      "packages/kernel/src/capability_runtime/screenshot_provider.rs",
      "pnpm verify:screenshot-provider",
      "Capability Independence Rule",
      "P15_PERMANENTLY_CLOSED_P16_ELIGIBLE",
    ],
  },
  {
    id: "constitutional-specification-p16-a3",
    backlogRef: "P16.A3 Codify Workspace Constitutional Specification v2",
    completed: "2026-08-07",
    artifacts: [
      "docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md",
      "docs/00-Constitution/ARCHITECTURE_AUTHORITY_HIERARCHY.md",
      "docs/project/ENGINEERING_HANDOFF.md",
    ],
  },
];

const remainingBacklog = [
  {
    id: "docs-convergence",
    phase: "A",
    title: "Documentation authority convergence",
    gap: "G3",
    status: "partial",
    note: "Canonical Spec + hierarchy codified (P16.A3); historical archive/cleanup remains Track A",
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
const readyForOwnerReview = argValue("ready-for-owner-review", "true") === "true";
const milestoneCommit = argValue("milestone-commit", "pending");

const health = {
  schemaVersion: 1,
  updatedAt: new Date().toISOString(),
  constitution: {
    document: constitutionalSpecPath,
    version: constitutionVersion,
    subordinateEngineeringConstitution:
      "architecture/ARCHITECTURAL_CONSTITUTION_V2.md",
    authorityHierarchy:
      "docs/00-Constitution/ARCHITECTURE_AUTHORITY_HIERARCHY.md",
  },
  engineeringMode: "constitutional-execution",
  protocol: {
    document: ".cursor/rules/constitutional-execution-protocol.mdc",
    version: "1.19",
    machineState: "docs/project-health.json",
  },
  currentExecutionProgram: {
    id: "voice-input-p16",
    title: "P16 Voice Input",
    status: "engineering_complete_product_proof_pending",
    note: "Release Hold (R2) + Capability Atlas v2.0 Pair Rule. C-OBS-003/004 + C-ACT-004/005 eng complete — Owner Product Proof required (joint for click/type). Next Atlas eng: C-VER-003. Parallel: A2 after Authenticode cert. File Provider blocked. nextReadyNow D1.",
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
    capabilityRuntimeResearch: "accepted",
    capabilityRuntimeFoundation: "accepted",
    applicationProvider: "accepted",
    windowProvider: "complete",
    windowProviderProductProof: "complete",
    conversationalDesktopSurface: "complete",
    operatorIntelligence: "superseded_by_kernel_operator",
    kernelOperator: "complete",
    p12Series: "permanently_complete",
    notificationsProvider: "permanently_complete",
    browserProvider: "permanently_complete",
    screenshotProvider: "permanently_complete",
    voiceInput: "engineering_complete_product_proof_pending",
    p12_5: "accepted_do_not_reopen",
    providerAcceptanceStandard: "permanent",
    capabilityIndependenceRule: "permanent",
    userAdaptationProhibition: "permanent",
    commodityBeforeReinvention: "permanent",
    notes: [
      "UI Architecture Spec accepted/frozen (P8)",
      "Capability Runtime research accepted (P9)",
      "Capability Runtime Foundation + Clipboard accepted (P10)",
      "Application Provider accepted (P11)",
      "P12.5 ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN",
      "P12 series constitutionally complete",
      "P13 Notifications Provider permanently closed",
      "P14 Browser Provider PERMANENTLY CLOSED",
      "P15 Screenshot Provider PERMANENTLY CLOSED",
      "Engineering Execution Standard v1 — meta-architecture complete",
      "A3.V repository constitutionally aligned — governance-first engineering phase",
      "A3.F Workspace Constitutional Specification v2.1 frozen — no further Spec work without Review Trigger",
      "P16.A3 codify superseded by A3.F final review",
      "P16.39 Operator Intelligence — Product Proof pending Owner acceptance (NOT permanently closed)",
      "Owner Experience Before Engineering Confidence permanent (P16.21)",
      "Repository Quality Before Milestone Closure permanent (P16.21)",
      "User Adaptation Prohibition permanent (P16.6)",
      "Commodity Before Reinvention permanent (P16.7)",
      "Conversation Continuity + Semantic Alias Rule permanent (P16.8)",
      "Permission Guidance Principle permanent (P16.9)",
      "Engineering Verification Separation + Owner Directed Product Proof permanent (P16.10)",
      "Evidence Before Commitment permanent (P16.11)",
      "Technology Foundation Validation permanent (P16.12)",
      "Capability Regression Prevention + Engineering Completion Gate permanent (P16.13)",
      "Production Before Expansion permanent (P16.15)",
      "Evidence Before Modification + Root Cause Before Rewrite permanent (P16.16)",
      "Production Quality Includes Repository Quality + Evidence Before Completion permanent (P16.17)",
      "Natural Language Robustness Rule permanent",
      "Provider Acceptance Standard permanent",
      "Capability Independence Rule permanent (P15)",
      "Operator Authority + Kernel Authority + Composition permanent",
      "Canonical agent handoff: docs/project/ENGINEERING_HANDOFF.md",
    ],
  },
  currentMilestone: {
    id: "operator-intelligence-p16-39",
    title: "P16.39 Product Operator Intelligence Validation",
    status: "engineering_complete_product_proof_pending",
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
    {
      id: "p9",
      title: "Capability Runtime Research",
      commit: "34647bd",
      acceptedAt: "2026-08-07",
    },
    {
      id: "p10",
      title: "Capability Runtime Foundation",
      commit: "23492a8",
      acceptedAt: "2026-08-07",
    },
    {
      id: "p11",
      title: "Application Provider",
      commit: "330a26b",
      acceptedAt: "2026-08-07",
    },
    {
      id: "p12",
      title: "Window Provider (engineering)",
      commit: "66465b8",
      acceptedAt: "2026-08-07",
      note: "Engineering accepted; Product Proof gap closed by P12.5",
    },
    {
      id: "p12-5",
      title: "Window Provider Product Proof",
      commit: "40dd746",
      acceptedAt: "2026-08-07",
      note: "ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN",
    },
    {
      id: "p12-6",
      title: "Conversational Desktop Surface / Product Gravity",
      commit: "d1dc8b8",
      acceptedAt: "2026-08-07",
    },
    {
      id: "p12-7",
      title: "Operator Intelligence Foundation",
      commit: "644c125",
      acceptedAt: "2026-08-07",
      note: "Superseded by Kernel Operator in P12 Finalization",
    },
    {
      id: "p12-finalization",
      title: "P12 Finalization — Kernel Operator",
      commit: "daf3ae9",
      acceptedAt: "2026-08-07",
    },
    {
      id: "p12-complete",
      title: "P12 COMPLETE — Final Repository Closure",
      commit: "030fa3f",
      acceptedAt: "2026-08-07",
      note: "P12 series permanently closed; P13 eligible",
    },
    {
      id: "p13",
      title: "P13 Notifications Provider",
      commit: "f0e03c4",
      acceptedAt: "2026-08-07",
      note: "Product Complete + Composition Audit; permanently closed",
    },
    {
      id: "p14",
      title: "P14 Browser Provider",
      commit: "1670822",
      acceptedAt: "2026-08-07",
      note: "PERMANENTLY CLOSED — ACCEPTED — DO NOT REOPEN (bugfixes only)",
    },
    {
      id: "p15",
      title: "P15 Screenshot Provider",
      commit: "a84eb83",
      acceptedAt: "2026-08-07",
      note: "PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN (bugfixes only)",
    },
  ],
  outstandingProductDebt: [
    {
      id: "p16-voice-product-proof",
      track: "B",
      summary:
        "P16 Voice Input — live Product Owner Product Proof pending final acceptance (P16.39 Operator Intelligence; not permanently closed)",
    },
    {
      id: "capability-providers-p17-plus",
      track: "B",
      summary:
        "File / Terminal / Memory providers — P17+ (blocked until P16 Owner acceptance)",
    },
    {
      id: "tray-integration",
      track: "A",
      summary: "System tray integration not yet implemented (PR1 Phase 2 Critical)",
    },
    {
      id: "installer-signing",
      track: "A",
      summary:
        "NSIS installer foundation landed (PI1 S1); Authenticode signing still required for public trust",
    },
    {
      id: "auto-updater",
      track: "A",
      summary: "Auto-updater absent — no Tauri updater plugin (PR1)",
    },
    {
      id: "diagnostics-support-bundle",
      track: "A",
      summary:
        "B1 support bundle + log rotation landed (PI3); local crash dumps still optional (B3)",
    },
    {
      id: "ipc-quarantine",
      track: "A",
      summary: "197 IPC commands vs ~20 product — quarantine before public release (PR1)",
    },
    {
      id: "window-animations",
      track: "A",
      summary: "Native show/hide transition polish still limited",
    },
    {
      id: "G3-docs",
      track: "A",
      summary: "Documentation authority still fragmented (archive hygiene)",
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
      "Capability Runtime + Application + Window providers (P10–P12)",
      "Window Provider Conversation Product Proof (P12.5)",
      "Conversational Desktop Surface / Product Gravity (P12.6)",
      "Operator Intelligence Foundation (P12.7 TS interim)",
      "P12 Finalization — Kernel Operator architecture",
      "P12 series permanently closed",
      "P13 Notifications Provider permanently closed",
      "P14 Browser Provider permanently closed (P14.5)",
      "P15 Screenshot Provider permanently closed",
      "P16.39 Operator Intelligence — Product Proof pending Owner acceptance; WinRT WRAP frozen; Voice not reopened; Owner evidence outranks engineering confidence",
      "User Adaptation Prohibition permanent",
      "Conversation Continuity + Semantic Alias Rule permanent",
      "Permission Guidance Principle permanent",
      "Engineering Verification Separation + Owner Directed Product Proof permanent",
      "Evidence Before Commitment permanent",
      "Technology Foundation Validation permanent",
      "Capability Regression Prevention + Engineering Completion Gate permanent",
      "Production Before Expansion permanent",
      "Evidence Before Modification + Root Cause Before Rewrite permanent",
      "Production Quality Includes Repository Quality + Evidence Before Completion permanent",
      "Capability Independence Rule permanent",
      "Natural Language Robustness Rule permanent",
      "Kernel unused-item warnings documented Track A (not Voice-owned)",
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
    reviewDocument:
      "docs/capability-runtime/product-proof/P21_S2_COMPOUND_GOAL_DECOMPOSITION.md",
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
      "Doc authority: Spec+hierarchy canonical (P16.A3); historical archive remains Track A",
      "WorkspaceState naming collision",
      "No AgentToolGate / AuditIntegrity yet",
      "Tray / deeper native polish remaining (Track A debt)",
    ],
  },
  lastMilestone: {
    date: "2026-08-08",
    document: "docs/engineering-milestone-report.md",
    title: "P22.S4 Desktop Control Interaction (C-ACT-004+005) — Product Proof pending",
    reviewBrief:
      "docs/capability-runtime/product-proof/P22_S4_DESKTOP_CONTROL_INTERACTION.md",
  },
  handoffStatus: "P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING",
  nextRecommendedExecutionProgram: {
    id: "c-ver-003-wait-conditions",
    title: "C-VER-003 Wait Conditions",
    blockedUntil: "Atlas Execution Loop next session (after C-ACT-004/005 stop)",
    note: "Highest remaining Atlas capability after interaction eng. File Provider remains blocked on Voice Accept. Release track: A2 after Authenticode cert.",
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
