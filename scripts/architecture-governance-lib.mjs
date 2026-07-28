/**
 * Architecture governance enforcement — fail-closed detector (not authority).
 *
 * Authority path (allowed):
 *   React → IPC → Kernel Command → Permission Gateway → Service → Repository → Domain
 *
 * Does not redesign ownership; audits that growth stays within sealed rules.
 * Empty/missing inventory is a failure — never "nothing found ⇒ clean".
 */
import fs from "node:fs";
import path from "node:path";

const read = (file) => fs.readFileSync(file, "utf8");

const sorted = (values) => [...new Set(values)].sort();

/** Minimum MutationCommand inventory — dropping below this fails closed. */
export const MUTATION_COMMAND_BASELINE = 68;

/** Capability id → authority owner (permission-token scope, not lifecycle owner). */
export const CAPABILITY_AUTHORITY_OWNERS = {
  "workspace.read": "Workspace",
  "workspace.write": "Workspace",
  "settings.read": "Settings",
  "settings.write": "Settings",
  "audit.read": "Audit",
  "audit.write": "Audit",
  "system.startup": "System",
  "system.shutdown": "System",
  "zone.read": "Zone",
  "zone.write": "Zone",
  "application.read": "Application",
  "application.write": "Application",
  "application.launch": "Application",
  "widget.read": "Widget",
  "widget.write": "Widget",
  "layout.read": "Layout",
  "layout.write": "Layout",
  "memory.read": "Memory",
  "memory.write": "Memory",
  "personalization.read": "Personalization",
  "personalization.write": "Personalization",
  "work_context.read": "WorkContext",
  "work_context.write": "WorkContext",
  "desktop.read": "System",
};

/**
 * Broad capability tokens that intentionally gate multiple domains.
 * These are permission tokens — not lifecycle ownership transfers.
 */
export const INTENTIONAL_BROAD_CAPABILITIES = {
  "work_context.write":
    "Shared write token for RE / DQ / DE / TaskGraph / adaptation gated mutations — lifecycle owners remain the domain services",
  "audit.write":
    "Used for suggestion accept/reject and some execution bookkeeping commands — audit capability, not lifecycle ownership",
};

/** Fields that imply executable / mutation authority on history or projection DTOs. */
export const FORBIDDEN_AUTHORITY_FIELDS = [
  "execute",
  "execute_command",
  "handoff_command",
  "dispatch",
  "dispatch_allowed",
  "cancellation_allowed",
  "recommended_action",
  "select_command",
  "dismiss_command",
  "approve",
  "approve_command",
  "mutate",
  "transition",
  "transition_command",
  "command_envelope",
  "command_handler",
  "mutation_allowed",
  "mutation_payload",
  "spawn_process",
  "launch_application",
  "permission_bypass",
  "capability_grant",
  "capability_grants",
];

/** @deprecated use FORBIDDEN_AUTHORITY_FIELDS */
export const FORBIDDEN_HISTORY_FIELDS = FORBIDDEN_AUTHORITY_FIELDS;

/** @deprecated use FORBIDDEN_AUTHORITY_FIELDS — kept so no exported denylist is unenforced */
export const FORBIDDEN_PROJECTION_COMMAND_FIELDS = FORBIDDEN_AUTHORITY_FIELDS;

export const HISTORY_STRUCTS = [
  "RecommendationHistoryEntry",
  "DecisionOverlayHistoryEntry",
  "DecisionArtifactHistoryEntry",
  "TaskHistoryEntry",
  "ExecutionLifecycleHistoryEntry",
  "PlanningHistoryEntry",
  "ReasoningHistoryEntry",
  "CognitiveGraphHistoryEntry",
  "OrchestrationHistoryEntry",
  "LearningHistoryEntry",
  "CognitiveAgentCastHistoryEntry",
  "CognitiveAutonomyHistoryEntry",
  "WorkspaceStateHistoryEntry",
  "PolicyGovernanceHistoryEntry",
  "HistoricalReconstructionHistoryEntry",
  "TemporalIntelligenceHistoryEntry",
  "WorkspaceExplanationHistoryEntry",
  "ContextualUnderstandingHistoryEntry",
  "KnowledgeSynthesisHistoryEntry",
];

export const PROJECTION_SUMMARY_STRUCTS = [
  "WorkspaceRecommendationEngineSummary",
  "DecisionQueueSummary",
  "DecisionEngineSummary",
  "TaskGraphSummary",
  "ExecutionLifecycleProjection",
  "PlanningSummary",
  "ReasoningSummary",
  "CognitiveGraphSummary",
  "WorkspaceOrchestrationSummary",
  "LearningSummary",
  "CognitiveAgentCastSummary",
  "CognitiveAutonomySummary",
  "WorkspaceStateSummary",
  "PolicyGovernanceSummary",
  "HistoricalReconstructionSummary",
  "TemporalIntelligenceSummary",
  "WorkspaceExplanationSummary",
  "ContextualUnderstandingSummary",
  "KnowledgeSynthesisSummary",
];

/** Append-only recovery diagnostic event types — evidence only, never commands. */
export const RECOVERY_DIAGNOSTIC_EVENT_TYPES = [
  "system.recovery.startup.attempted",
  "system.recovery.startup.completed",
  "system.recovery.startup.failed",
];

const LIFECYCLE_MUTATION_SERVICE_FNS = [
  "accept_recommendation",
  "reject_recommendation",
  "present_recommendation",
  "confirm_recommendation_decision",
  "decline_recommendation_decision",
  "mark_viewed",
  "dismiss",
  "defer",
  "select",
  "postpone",
  "claim",
  "complete",
  "mark_failed",
];

const LIFECYCLE_SERVICE_FILES = new Set([
  "workspace_recommendation.rs",
  "decision_engine.rs",
  "decision_queue.rs",
  "task_graph.rs",
  "execution_lifecycle.rs",
  "application_launch.rs",
  "suggestion.rs",
  "suggestion_lifecycle.rs",
  "workspace_planning.rs",
  "workspace_reasoning_memory.rs",
  "workspace_cognitive_graph.rs",
  "workspace_cognitive_orchestration.rs",
  "workspace_learning_adaptation.rs",
  "workspace_cognitive_agent_cast.rs",
  "workspace_cognitive_autonomy.rs",
  "workspace_state_composition.rs",
  "policy_governance.rs",
  "workspace_historical_reconstruction.rs",
  "workspace_temporal_intelligence.rs",
  "workspace_explanation.rs",
  "workspace_contextual_understanding.rs",
  "workspace_knowledge_synthesis.rs",
]);

/**
 * Handler methods that mutate in-memory AI plan/workflow bookkeeping without
 * granting desktop authority. Step execution still routes through CommandPipeline.
 */
const HANDLER_PIPELINE_ALLOWLIST = new Set([
  "get_settings",
  "initialize_workspace",
  "initialize_workspace_in_memory",
  "shutdown",
  "compare_workspace_intelligence_states",
  "compare_assistant_plan_revisions",
  "compare_personalized_vs_neutral_plan",
  // AI plan / workflow bookkeeping — not PermissionGateway desktop mutations.
  "create_orchestrated_ai_plan",
  "cancel_orchestrated_ai_plan",
  "submit_ai_plan",
  "submit_assistant_goal",
  "revise_assistant_goal",
  "regenerate_assistant_plan",
  "record_assistant_explanation_viewed",
  "cancel_assistant_workflow",
]);

export const REQUIRED_SOURCE_TREES = [
  { id: "react", rel: "app/src", minFiles: 1 },
  { id: "ipc", rel: "app/src-tauri", minFiles: 1 },
  { id: "kernel_commands", rel: "packages/kernel/src/commands", minFiles: 5 },
  { id: "permission_gateway", rel: "packages/kernel/src/security", minFiles: 1 },
  { id: "services", rel: "packages/kernel/src/services", minFiles: 5 },
  { id: "domain", rel: "packages/domain/src", minFiles: 5 },
  { id: "repositories", rel: "packages/database/src", minFiles: 1 },
  { id: "execution", rel: "packages/windows-integration/src", minFiles: 1 },
];

export const REQUIRED_FILES = [
  "packages/domain/src/capability/mod.rs",
  "packages/domain/src/recovery_contract.rs",
  "packages/kernel/src/commands/handler.rs",
  "packages/kernel/src/commands/pipeline.rs",
  "packages/kernel/src/security/gateway.rs",
  "docs/03-Engineering/OPERATIONAL-RECOVERY.md",
  "scripts/generated/architecture-map.json",
];

function walkFiles(directory, predicate, acc = []) {
  if (!fs.existsSync(directory)) return acc;
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const full = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      if (
        entry.name === "node_modules" ||
        entry.name === "target" ||
        entry.name === "dist"
      ) {
        continue;
      }
      walkFiles(full, predicate, acc);
    } else if (predicate(entry.name, full)) {
      acc.push(full);
    }
  }
  return acc;
}

function rustSources(dir) {
  return walkFiles(dir, (name) => name.endsWith(".rs"));
}

function tsSources(dir) {
  return walkFiles(dir, (name) => /\.(?:ts|tsx)$/.test(name));
}

function cargoDependsOn(cargoToml, crateName) {
  if (!fs.existsSync(cargoToml)) return false;
  return new RegExp(`^\\s*${crateName}\\s*=`, "m").test(read(cargoToml));
}

function extractStructFields(source, structName) {
  const block = source.match(
    new RegExp(`pub struct ${structName} \\{([\\s\\S]*?)\\n\\}`),
  );
  if (!block) return null;
  return [
    ...block[1].matchAll(/^\s*pub(?:\([^)]*\))?\s+([a-z][a-z0-9_]*):/gm),
  ].map((m) => m[1]);
}

function findStructInSources(sources, structName) {
  for (const file of sources) {
    const source = read(file);
    const fields = extractStructFields(source, structName);
    if (fields) {
      return { file, fields, source };
    }
  }
  return null;
}

/**
 * Recovery diagnostics must remain evidence-only detectors — never mutation
 * commands, retry authority, or lifecycle owners.
 * @param {string} rootDir
 * @param {{ command?: string, capabilityMethod?: string, capabilityId?: string }[]} mutations
 */
export function auditRecoveryDiagnosticsEvidenceOnly(rootDir, mutations = []) {
  const violations = [];
  const assertions = [];
  const contractPath = path.join(rootDir, "packages/domain/src/recovery_contract.rs");
  if (!fs.existsSync(contractPath)) {
    violations.push(
      "packages/domain/src/recovery_contract.rs missing — recovery evidence contract cannot be verified",
    );
    return { violations, assertions };
  }
  const contract = read(contractPath);
  for (const eventType of RECOVERY_DIAGNOSTIC_EVENT_TYPES) {
    if (!contract.includes(`"${eventType}"`)) {
      violations.push(
        `recovery_contract.rs missing diagnostic event type constant for ${eventType}`,
      );
    }
    if (
      /execute|retry|dispatch|mutate/.test(eventType) &&
      !eventType.startsWith("system.recovery.")
    ) {
      violations.push(
        `recovery diagnostic event type looks commandable: ${eventType}`,
      );
    }
  }
  if (
    /\b(Database|mark_failed|execute_mutation|PermissionGateway|CommandPipeline)\b/.test(
      contract,
    )
  ) {
    violations.push(
      "recovery_contract.rs must remain detector-only (no Database/lifecycle/pipeline symbols)",
    );
  }
  if (!/detector predicates only/i.test(contract)) {
    violations.push(
      "recovery_contract.rs must declare detector-only ownership in module docs",
    );
  }

  for (const entry of mutations) {
    const name = entry.command ?? "";
    if (
      /Recover|ReconcileStale|RetryStale|FabricateRecovery/i.test(name) ||
      RECOVERY_DIAGNOSTIC_EVENT_TYPES.some((t) => name.includes(t))
    ) {
      violations.push(
        `MutationCommand ${name} must not expose recovery diagnostic / retry authority`,
      );
    }
  }

  const auditPath = path.join(rootDir, "packages/kernel/src/services/audit.rs");
  if (fs.existsSync(auditPath)) {
    const auditSrc = read(auditPath);
    if (!/fn record_recovery_diagnostic\b/.test(auditSrc)) {
      violations.push(
        "AuditService::record_recovery_diagnostic missing — recovery diagnostics have no append path",
      );
    }
  }

  if (violations.length === 0) {
    assertions.push(
      `Verified ${RECOVERY_DIAGNOSTIC_EVENT_TYPES.length} recovery diagnostic event types remain evidence-only.`,
    );
  }
  return { violations, assertions };
}

function listCapabilityConstructors(capabilitySource) {
  const ids = [];
  const pattern = /CapabilityId::new\(\s*"([a-z0-9_.]+)"\s*\)/g;
  for (const match of capabilitySource.matchAll(pattern)) {
    ids.push(match[1]);
  }
  return sorted(ids);
}

function capabilityMethodToCatalogId(method) {
  const overrides = {
    workspace_read: "workspace.read",
    workspace_write: "workspace.write",
    settings_read: "settings.read",
    settings_write: "settings.write",
    audit_read: "audit.read",
    audit_write: "audit.write",
    system_startup: "system.startup",
    system_shutdown: "system.shutdown",
    zone_read: "zone.read",
    zone_write: "zone.write",
    application_read: "application.read",
    application_write: "application.write",
    application_launch: "application.launch",
    widget_read: "widget.read",
    widget_write: "widget.write",
    layout_read: "layout.read",
    layout_write: "layout.write",
    memory_read: "memory.read",
    memory_write: "memory.write",
    personalization_read: "personalization.read",
    personalization_write: "personalization.write",
    work_context_read: "work_context.read",
    work_context_write: "work_context.write",
    desktop_read: "desktop.read",
  };
  return overrides[method] ?? null;
}

function listCommandCapabilities(commandsDir, traitName) {
  const entries = [];
  for (const file of rustSources(commandsDir)) {
    const source = read(file);
    if (!source.includes(`impl ${traitName} for`)) continue;
    const implBlocks = [
      ...source.matchAll(
        new RegExp(
          `impl ${traitName} for ([A-Za-z0-9_]+) \\{([\\s\\S]*?)\\n\\}`,
          "g",
        ),
      ),
    ];
    for (const block of implBlocks) {
      const name = block[1];
      const body = block[2];
      const cap = body.match(
        /fn required_capability\(&[^\)]*\)\s*->\s*Capability\s*\{([\s\S]*?)\}/,
      );
      if (!cap) {
        entries.push({ command: name, capability: null, file, trait: traitName });
        continue;
      }
      const idMatch = cap[1].match(/Capability::([a-z0-9_]+)\(\)/);
      const literal = cap[1].match(/CapabilityId::new\("([a-z0-9_.]+)"\)/);
      entries.push({
        command: name,
        capabilityMethod: idMatch?.[1] ?? null,
        capabilityId: literal?.[1] ?? null,
        file,
        trait: traitName,
      });
    }
  }
  return entries;
}

function extractBalancedFnBodies(source, fnPattern) {
  const methods = [];
  const re = new RegExp(fnPattern, "g");
  let match;
  while ((match = re.exec(source)) !== null) {
    const name = match[1];
    let i = match.index + match[0].length;
    while (i < source.length && source[i] !== "{") i += 1;
    if (i >= source.length) continue;
    let depth = 0;
    const start = i;
    for (; i < source.length; i += 1) {
      if (source[i] === "{") depth += 1;
      else if (source[i] === "}") {
        depth -= 1;
        if (depth === 0) {
          methods.push({ name, body: source.slice(start, i + 1) });
          break;
        }
      }
    }
  }
  return methods;
}

function functionBodyContainsPipeline(body) {
  return (
    /CommandPipeline::new/.test(body) ||
    /PermissionGateway::require/.test(body) ||
    /execute_mutation|execute_query/.test(body)
  );
}

/**
 * True when body (or a chain of Self::*_inner helpers) reaches CommandPipeline.
 * Does not accept arbitrary Self::foo() — only *_inner helpers, depth-bounded.
 */
function bodyRoutesThroughPipeline(body, byName, depth = 0) {
  if (!body || depth > 6) return false;
  if (functionBodyContainsPipeline(body)) return true;
  const innerCalls = [
    ...body.matchAll(/Self::([a-z0-9_]+_inner)\s*\(/g),
  ].map((m) => m[1]);
  return innerCalls.some((innerName) =>
    bodyRoutesThroughPipeline(byName.get(innerName), byName, depth + 1),
  );
}

/**
 * Stronger handler mutation-path check:
 * - CommandPipeline / require in the method body, OR
 * - delegates through Self::<name>_inner chain that reaches the pipeline.
 * Blanket Self::foo() is not accepted.
 */
export function handlerMethodsMissingPipeline(handlerSource) {
  const violations = [];
  const allFns = extractBalancedFnBodies(
    handlerSource,
    /(?:pub(?:\(crate\))? )?fn ([a-z0-9_]+)\s*\(/,
  );
  const byName = new Map();
  for (const fn of allFns) {
    byName.set(fn.name, fn.body);
  }

  const mutationName =
    /^(accept_|reject_|present_|create_|update_|delete_|dismiss_|select_|defer_|postpone_|claim_|complete_|mark_|confirm_|decline_|launch_|execute_|record_|set_|reset_|submit_|advance_|resume_|cancel_|decide_)/;

  const autoAllow = (name) =>
    HANDLER_PIPELINE_ALLOWLIST.has(name) ||
    /_attempt_execute$/.test(name) ||
    /^compare_/.test(name);

  const publicEntries = extractBalancedFnBodies(
    handlerSource,
    /pub fn ([a-z0-9_]+)\s*\(/,
  );

  for (const method of publicEntries) {
    const { name, body } = method;
    if (autoAllow(name)) continue;
    if (!mutationName.test(name)) continue;

    if (bodyRoutesThroughPipeline(body, byName)) continue;

    const innerCalls = [
      ...body.matchAll(/Self::([a-z0-9_]+_inner)\s*\(/g),
    ].map((m) => m[1]);
    if (innerCalls.length > 0) {
      violations.push(
        `${name}: delegates to ${innerCalls.join(", ")} without reaching CommandPipeline`,
      );
      continue;
    }

    violations.push(name);
  }
  return sorted(violations);
}

function permissionGatewayRequireSites(rootDir) {
  const sites = [];
  for (const file of rustSources(path.join(rootDir, "packages/kernel/src"))) {
    if (/_tests\.rs$/.test(file) || /\/tests\.rs$/.test(file)) continue;
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    for (const match of source.matchAll(/PermissionGateway::require\s*\(/g)) {
      const line = source.slice(0, match.index).split("\n").length;
      sites.push({ file: rel, line, kind: "require" });
    }
  }
  return sites;
}

/** Services may call evaluate() for discovery probes; require() is forbidden. */
function servicesCallingGatewayRequire(servicesDir, rootDir) {
  const violations = [];
  for (const file of rustSources(servicesDir)) {
    const source = read(file);
    if (/PermissionGateway::require\s*\(/.test(source)) {
      violations.push(
        `${path.relative(rootDir, file)}: services must not call PermissionGateway::require (use CommandPipeline)`,
      );
    }
  }
  return violations;
}

function lifecycleMutationVisibility(servicesDir) {
  const violations = [];
  for (const file of rustSources(servicesDir)) {
    const base = path.basename(file);
    if (!LIFECYCLE_SERVICE_FILES.has(base)) continue;
    const source = read(file);
    for (const fnName of LIFECYCLE_MUTATION_SERVICE_FNS) {
      const matches = [
        ...source.matchAll(
          new RegExp(`^\\s*(pub(?:\\(crate\\))? )fn ${fnName}\\s*\\(`, "gm"),
        ),
      ];
      for (const m of matches) {
        if (m[1] === "pub ") {
          violations.push(
            `${base}: lifecycle mutator '${fnName}' is pub — must be pub(crate)`,
          );
        }
      }
    }
  }
  return violations;
}

function reactLifecycleMutationImports(appSrcDir) {
  const violations = [];
  for (const file of tsSources(appSrcDir)) {
    const source = read(file);
    const rel = path.relative(appSrcDir, file).replace(/\\/g, "/");
    if (
      /from\s+["'][^"']*(?:workspace_database|workspace_kernel|rusqlite|packages\/database|packages\/kernel)["']/.test(
        source,
      )
    ) {
      violations.push(`${rel}: React must not import kernel/database crates`);
    }
    if (/Projection\.ts$/.test(rel) || /projection\.ts$/.test(rel)) {
      for (const fn of LIFECYCLE_MUTATION_SERVICE_FNS) {
        const camel = fn.replace(/_([a-z])/g, (_, c) => c.toUpperCase());
        if (
          new RegExp(`export\\s+(?:async\\s+)?function\\s+${fn}\\b`).test(source) ||
          new RegExp(`export\\s+(?:async\\s+)?function\\s+${camel}\\b`).test(source)
        ) {
          violations.push(
            `${rel}: projection module exports lifecycle mutator '${fn}'`,
          );
        }
      }
    }
  }
  return violations;
}

function repositoryServiceImports(databaseSrcDir) {
  const violations = [];
  for (const file of rustSources(databaseSrcDir)) {
    const source = read(file);
    const rel = path.relative(databaseSrcDir, file).replace(/\\/g, "/");
    if (
      /use\s+workspace_kernel::/.test(source) ||
      /workspace_kernel::services/.test(source)
    ) {
      violations.push(`${rel}: repository/database must not import kernel services`);
    }
  }
  return violations;
}

function recommendationEngineProcessSpawn(rootDir) {
  const violations = [];
  const targets = [
    path.join(rootDir, "packages/kernel/src/services/workspace_recommendation.rs"),
    path.join(rootDir, "packages/domain/src/workspace_recommendation"),
  ];
  for (const target of targets) {
    const files =
      fs.existsSync(target) && fs.statSync(target).isDirectory()
        ? rustSources(target)
        : fs.existsSync(target)
          ? [target]
          : [];
    for (const file of files) {
      const source = read(file);
      if (
        /std::process::Command/.test(source) ||
        /workspace_windows_integration::/.test(source) ||
        /ApplicationLaunchService/.test(source)
      ) {
        violations.push(
          `${path.relative(rootDir, file)}: Recommendation Engine must not spawn processes or call launch integration`,
        );
      }
    }
  }
  return violations;
}

/**
 * Programme II Batch 5 — Cognitive Orchestration governance guards.
 * Orchestration coordinates; it must never execute or import launch/execution services.
 */
function cognitiveOrchestrationGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_cognitive_orchestration.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_cognitive_orchestration",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/cognitive_orchestration.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /ApplicationLaunchService/.test(source) ||
      /ExecutionLifecycleService/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Cognitive Orchestration must not import or invoke execution/launch services`,
      );
    }
    // Orchestration must not call generate on foreign engines as plan execution.
    if (
      /WorkspacePlanningService::generate\b/.test(source) ||
      /WorkspaceReasoningMemoryService::generate\b/.test(source) ||
      /WorkspaceCognitiveGraphService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Cognitive Orchestration must not execute foreign artefact regeneration`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceCognitiveOrchestrationService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/cognitive_orchestration.rs: repository must not call orchestration service`,
      );
    }
  }

  // Orchestration DTOs must exist for authority-field scanning (covered by HISTORY/PROJECTION lists).
  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_cognitive_orchestration missing; governance cannot verify orchestration DTOs",
    );
  }

  return violations;
}


/**
 * Programme II Batch 6 — Learning & Adaptation governance guards.
 * Learning observes outcomes; it must never execute or mutate foreign domains.
 */
function learningAdaptationGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_learning_adaptation.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_learning_adaptation",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/learning_adaptation.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Learning Adaptation must not import or invoke execution/launch services`,
      );
    }
    if (
      /WorkspacePlanningService::generate\b/.test(source) ||
      /WorkspaceReasoningMemoryService::generate\b/.test(source) ||
      /WorkspaceCognitiveGraphService::generate\b/.test(source) ||
      /WorkspaceCognitiveOrchestrationService::generate\b/.test(source) ||
      /TaskGraphService::generate\b/.test(source) ||
      /DecisionEngineService::generate\b/.test(source) ||
      /WorkspaceRecommendationEngineService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Learning Adaptation must not mutate planning/task/decision domains via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceLearningAdaptationService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/learning_adaptation.rs: repository must not call learning service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_learning_adaptation missing; governance cannot verify learning DTOs",
    );
  }

  return violations;
}


/**
 * Programme II Batch 7 — Cognitive Agent Cast governance guards.
 * Agents are role representations; never actors, executors, or permission owners.
 */
function cognitiveAgentCastGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_cognitive_agent_cast.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_cognitive_agent_cast",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/cognitive_agent_cast.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Cognitive Agent Cast must not import or invoke execution/launch services`,
      );
    }
    if (
      /WorkspacePlanningService::generate\b/.test(source) ||
      /WorkspaceReasoningMemoryService::generate\b/.test(source) ||
      /WorkspaceCognitiveGraphService::generate\b/.test(source) ||
      /WorkspaceCognitiveOrchestrationService::generate\b/.test(source) ||
      /WorkspaceLearningAdaptationService::generate\b/.test(source) ||
      /TaskGraphService::generate\b/.test(source) ||
      /DecisionEngineService::generate\b/.test(source) ||
      /PermissionGateway::/.test(source)
    ) {
      violations.push(
        `${rel}: Cognitive Agent Cast must not mutate lifecycle domains or own permissions`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceCognitiveAgentCastService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/cognitive_agent_cast.rs: repository must not call agent cast service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_cognitive_agent_cast missing; governance cannot verify agent cast DTOs",
    );
  }

  return violations;
}

/**
 * Programme II Batch 8 — Governed Cognitive Autonomy governance guards.
 * Autonomy may suggest. Authority must still approve. Never executes.
 */
function cognitiveAutonomyGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_cognitive_autonomy.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_cognitive_autonomy",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/cognitive_autonomy.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Cognitive Autonomy must not import or invoke execution/launch services`,
      );
    }
    if (
      /WorkspacePlanningService::generate\b/.test(source) ||
      /WorkspaceReasoningMemoryService::generate\b/.test(source) ||
      /WorkspaceCognitiveGraphService::generate\b/.test(source) ||
      /WorkspaceCognitiveOrchestrationService::generate\b/.test(source) ||
      /WorkspaceLearningAdaptationService::generate\b/.test(source) ||
      /WorkspaceCognitiveAgentCastService::generate\b/.test(source) ||
      /TaskGraphService::generate\b/.test(source) ||
      /DecisionEngineService::generate\b/.test(source) ||
      /PermissionGateway::/.test(source)
    ) {
      violations.push(
        `${rel}: Cognitive Autonomy must not mutate lifecycle domains or own permissions`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceCognitiveAutonomyService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/cognitive_autonomy.rs: repository must not call autonomy service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_cognitive_autonomy missing; governance cannot verify autonomy DTOs",
    );
  }

  return violations;
}

/**
 * Programme III Batch 1 — Unified Workspace State Envelope governance guards.
 * Envelope composes; never owns, mutates, repairs, or silently refreshes sources.
 */
function workspaceStateEnvelopeGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_state_composition.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_state_envelope",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/workspace_state_envelope.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Unified Workspace State must not import or invoke execution/launch services`,
      );
    }
    if (
      /WorkspacePlanningService::generate\b/.test(source) ||
      /WorkspaceReasoningMemoryService::generate\b/.test(source) ||
      /WorkspaceCognitiveGraphService::generate\b/.test(source) ||
      /WorkspaceCognitiveOrchestrationService::generate\b/.test(source) ||
      /WorkspaceLearningAdaptationService::generate\b/.test(source) ||
      /WorkspaceCognitiveAgentCastService::generate\b/.test(source) ||
      /WorkspaceCognitiveAutonomyService::generate\b/.test(source) ||
      /TaskGraphService::generate\b/.test(source) ||
      /DecisionEngineService::generate\b/.test(source) ||
      /PermissionGateway::/.test(source)
    ) {
      violations.push(
        `${rel}: Unified Workspace State must not mutate lifecycle domains, own permissions, or silently refresh via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceStateCompositionService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/workspace_state_envelope.rs: repository must not call composition service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_state_envelope missing; governance cannot verify state envelope DTOs",
    );
  }

  return violations;
}

/**
 * Programme III Batch 2 — Policy & Governance Engine guards.
 * Policy explains authority; Gateway remains final. No second permission system.
 */
function policyGovernanceGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/policy_governance.rs",
  );
  const domainPath = path.join(rootDir, "packages/domain/src/policy_governance");
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/policy_governance.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /\bTaskGraphService\b/.test(source) ||
      /\bDecisionEngineService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Policy governance must not import or invoke lifecycle/execution services`,
      );
    }
    if (
      /PermissionGateway::/.test(source) ||
      /\bCapabilityGrant\b/.test(source) ||
      /CommandPipeline::/.test(source)
    ) {
      violations.push(
        `${rel}: Policy governance must not own permissions, grant capabilities, or execute via Pipeline`,
      );
    }
    if (
      /WorkspaceStateCompositionService::generate\b/.test(source) ||
      /WorkspacePlanningService::generate\b/.test(source) ||
      /WorkspaceCognitiveAutonomyService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Policy governance must not silently refresh foreign sources via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /PolicyGovernanceService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/policy_governance.rs: repository must not call policy service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/policy_governance missing; governance cannot verify policy DTOs",
    );
  }

  return violations;
}

/**
 * Programme III Batch 3 — Historical Workspace Reconstruction guards.
 * Reconstruction explains change; never becomes source of truth / replay / lifecycle.
 */
function historicalReconstructionGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_historical_reconstruction.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_historical_reconstruction",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/historical_reconstruction.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /\bTaskGraphService\b/.test(source) ||
      /\bDecisionEngineService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Historical reconstruction must not import or invoke lifecycle/execution services`,
      );
    }
    if (
      /PermissionGateway::/.test(source) ||
      /\bCapabilityGrant\b/.test(source) ||
      /CommandPipeline::/.test(source)
    ) {
      violations.push(
        `${rel}: Historical reconstruction must not own permissions, grant capabilities, or execute via Pipeline`,
      );
    }
    if (
      /WorkspaceStateCompositionService::generate\b/.test(source) ||
      /PolicyGovernanceService::generate\b/.test(source) ||
      /WorkspacePlanningService::generate\b/.test(source) ||
      /WorkspaceCognitiveAutonomyService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Historical reconstruction must not silently refresh foreign sources via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceHistoricalReconstructionService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/historical_reconstruction.rs: repository must not call reconstruction service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_historical_reconstruction missing; governance cannot verify reconstruction DTOs",
    );
  }

  return violations;
}

/**
 * Programme III Batch 4 — Temporal Intelligence guards.
 * Organises evidence over time; never predicts, simulates, corrects, or becomes truth.
 */
function temporalIntelligenceGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_temporal_intelligence.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_temporal_intelligence",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/temporal_intelligence.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /\bTaskGraphService\b/.test(source) ||
      /\bDecisionEngineService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Temporal intelligence must not import or invoke lifecycle/execution services`,
      );
    }
    if (
      /PermissionGateway::/.test(source) ||
      /\bCapabilityGrant\b/.test(source) ||
      /CommandPipeline::/.test(source)
    ) {
      violations.push(
        `${rel}: Temporal intelligence must not own permissions, grant capabilities, or execute via Pipeline`,
      );
    }
    if (
      /WorkspaceStateCompositionService::generate\b/.test(source) ||
      /WorkspaceHistoricalReconstructionService::generate\b/.test(source) ||
      /PolicyGovernanceService::generate\b/.test(source) ||
      /WorkspacePlanningService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Temporal intelligence must not silently refresh foreign sources via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceTemporalIntelligenceService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/temporal_intelligence.rs: repository must not call temporal intelligence service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_temporal_intelligence missing; governance cannot verify temporal DTOs",
    );
  }

  return violations;
}

/**
 * Programme III Batch 5 — Workspace Explanation Layer guards.
 * Explain evidence; never become authority that changes reality.
 */
function workspaceExplanationGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_explanation.rs",
  );
  const domainPath = path.join(rootDir, "packages/domain/src/workspace_explanation");
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/workspace_explanation.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /\bTaskGraphService\b/.test(source) ||
      /\bDecisionEngineService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Explanation layer must not import or invoke lifecycle/execution services`,
      );
    }
    if (
      /PermissionGateway::/.test(source) ||
      /\bCapabilityGrant\b/.test(source) ||
      /CommandPipeline::/.test(source)
    ) {
      violations.push(
        `${rel}: Explanation layer must not own permissions, grant capabilities, or execute via Pipeline`,
      );
    }
    if (
      /WorkspaceStateCompositionService::generate\b/.test(source) ||
      /WorkspaceHistoricalReconstructionService::generate\b/.test(source) ||
      /WorkspaceTemporalIntelligenceService::generate\b/.test(source) ||
      /PolicyGovernanceService::generate\b/.test(source) ||
      /WorkspacePlanningService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Explanation layer must not silently refresh foreign sources via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceExplanationService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/workspace_explanation.rs: repository must not call explanation service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_explanation missing; governance cannot verify explanation DTOs",
    );
  }

  return violations;
}

/**
 * Programme III Batch 6 — Contextual Workspace Understanding guards.
 * Organise situational meaning; never decide, predict, simulate, or change reality.
 */
function contextualUnderstandingGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_contextual_understanding.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_contextual_understanding",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/workspace_contextual_understanding.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /\bTaskGraphService\b/.test(source) ||
      /\bDecisionEngineService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Contextual understanding must not import or invoke lifecycle/execution services`,
      );
    }
    if (
      /PermissionGateway::/.test(source) ||
      /\bCapabilityGrant\b/.test(source) ||
      /CommandPipeline::/.test(source)
    ) {
      violations.push(
        `${rel}: Contextual understanding must not own permissions, grant capabilities, or execute via Pipeline`,
      );
    }
    if (
      /WorkspaceStateCompositionService::generate\b/.test(source) ||
      /WorkspaceHistoricalReconstructionService::generate\b/.test(source) ||
      /WorkspaceTemporalIntelligenceService::generate\b/.test(source) ||
      /PolicyGovernanceService::generate\b/.test(source) ||
      /WorkspaceExplanationService::generate\b/.test(source) ||
      /WorkspacePlanningService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Contextual understanding must not silently refresh foreign sources via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceContextualUnderstandingService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/workspace_contextual_understanding.rs: repository must not call contextual understanding service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_contextual_understanding missing; governance cannot verify contextual DTOs",
    );
  }

  return violations;
}

/**
 * Programme III Batch 7 — Workspace Knowledge Synthesis guards.
 * Synthesize understanding from evidence; never create reality / Memory SoT / decisions.
 */
function knowledgeSynthesisGuards(rootDir) {
  const violations = [];
  const servicePath = path.join(
    rootDir,
    "packages/kernel/src/services/workspace_knowledge_synthesis.rs",
  );
  const domainPath = path.join(
    rootDir,
    "packages/domain/src/workspace_knowledge_synthesis",
  );
  const repoPath = path.join(
    rootDir,
    "packages/database/src/repositories/workspace_knowledge_synthesis.rs",
  );

  const serviceFiles = fs.existsSync(servicePath) ? [servicePath] : [];
  const domainFiles = fs.existsSync(domainPath) ? rustSources(domainPath) : [];
  for (const file of [...serviceFiles, ...domainFiles]) {
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    if (
      /\bApplicationLaunchService\b/.test(source) ||
      /\bExecutionLifecycleService\b/.test(source) ||
      /\bTaskGraphService\b/.test(source) ||
      /\bDecisionEngineService\b/.test(source) ||
      /std::process::Command/.test(source) ||
      /workspace_windows_integration::/.test(source)
    ) {
      violations.push(
        `${rel}: Knowledge synthesis must not import or invoke lifecycle/execution services`,
      );
    }
    if (
      /PermissionGateway::/.test(source) ||
      /\bCapabilityGrant\b/.test(source) ||
      /CommandPipeline::/.test(source)
    ) {
      violations.push(
        `${rel}: Knowledge synthesis must not own permissions, grant capabilities, or execute via Pipeline`,
      );
    }
    if (
      /WorkspaceStateCompositionService::generate\b/.test(source) ||
      /WorkspaceHistoricalReconstructionService::generate\b/.test(source) ||
      /WorkspaceTemporalIntelligenceService::generate\b/.test(source) ||
      /PolicyGovernanceService::generate\b/.test(source) ||
      /WorkspaceExplanationService::generate\b/.test(source) ||
      /WorkspaceContextualUnderstandingService::generate\b/.test(source) ||
      /WorkspacePlanningService::generate\b/.test(source)
    ) {
      violations.push(
        `${rel}: Knowledge synthesis must not silently refresh foreign sources via generate`,
      );
    }
  }

  if (fs.existsSync(repoPath)) {
    const source = read(repoPath);
    if (
      /WorkspaceKnowledgeSynthesisService/.test(source) ||
      /use\s+workspace_kernel::/.test(source)
    ) {
      violations.push(
        `packages/database/src/repositories/workspace_knowledge_synthesis.rs: repository must not call knowledge synthesis service`,
      );
    }
  }

  if (!fs.existsSync(domainPath) && !fs.existsSync(`${domainPath}.rs`)) {
    violations.push(
      "packages/domain/src/workspace_knowledge_synthesis missing; governance cannot verify knowledge synthesis DTOs",
    );
  }

  return violations;
}

function checkDtoAuthorityFields(domainSources, structNames, label) {
  const violations = [];
  const found = [];
  for (const structName of structNames) {
    const hit = findStructInSources(domainSources, structName);
    if (!hit) {
      violations.push(
        `No ${label} structure '${structName}' found; governance audit cannot establish compliance`,
      );
      continue;
    }
    found.push(structName);
    for (const field of hit.fields) {
      if (FORBIDDEN_AUTHORITY_FIELDS.includes(field)) {
        violations.push(
          `${structName}.${field}: ${label} DTO must not gain authority/command fields`,
        );
      }
    }
    if (label === "history" && !hit.fields.includes("actionable")) {
      violations.push(
        `${structName}: history DTO missing required 'actionable' evidence flag`,
      );
    }
  }
  return { violations, found };
}

function assertRequiredTrees(rootDir) {
  const violations = [];
  const evidence = [];
  for (const tree of REQUIRED_SOURCE_TREES) {
    const abs = path.join(rootDir, tree.rel);
    if (!fs.existsSync(abs)) {
      violations.push(
        `Required source tree missing: ${tree.rel} (${tree.id}) — governance audit cannot establish compliance`,
      );
      continue;
    }
    const files = walkFiles(abs, () => true);
    if (files.length < tree.minFiles) {
      violations.push(
        `Required source tree empty or too sparse: ${tree.rel} (found ${files.length}, need ≥ ${tree.minFiles})`,
      );
      continue;
    }
    evidence.push(`${tree.id}:${files.length}`);
  }
  return { violations, evidence };
}

function assertRequiredFiles(rootDir) {
  const violations = [];
  for (const rel of REQUIRED_FILES) {
    if (!fs.existsSync(path.join(rootDir, rel))) {
      violations.push(
        `Required architecture input missing: ${rel} — governance audit cannot establish compliance`,
      );
    }
  }
  return violations;
}

function buildArchitectureMap(rootDir) {
  const layers = REQUIRED_SOURCE_TREES.map((t) => ({
    id: t.id,
    path: t.rel,
    role: {
      react: "Projection-only UI; IPC invoke only",
      ipc: "Tauri command edge; forwards to CommandHandler",
      kernel_commands: "CommandPipeline entry; declares required_capability",
      permission_gateway: "Sole Allow/Deny/ApprovalRequired via require()",
      services: "Lifecycle owners; mutators pub(crate) after gateway Allow",
      domain: "Canonical models; no persistence or process spawn",
      repositories: "Persistence guards; no service imports",
      execution:
        "OS spawn / desktop capture — only via ApplicationLaunchService after gateway",
    }[t.id],
  }));

  const allowedEdges = [
    ["react", "ipc"],
    ["ipc", "kernel_commands"],
    ["kernel_commands", "permission_gateway"],
    ["permission_gateway", "services"],
    ["services", "domain"],
    ["services", "repositories"],
    ["repositories", "domain"],
    ["services", "execution"],
  ];

  // Narrowed: services may call PermissionGateway::evaluate for discovery probes.
  // PermissionGateway::require remains pipeline/shutdown-only (enforced separately).
  const forbiddenEdges = [
    {
      from: "react",
      to: "repositories",
      reason: "React must never open the database",
      enforced: "react_import_scan",
    },
    {
      from: "react",
      to: "services",
      reason: "React must not call lifecycle services directly",
      enforced: "react_import_scan",
    },
    {
      from: "react",
      to: "domain",
      reason: "React consumes IPC DTOs, not Rust domain crates",
      enforced: "react_import_scan",
    },
    {
      from: "react",
      to: "execution",
      reason: "React must not spawn processes",
      enforced: "react_import_scan",
    },
    {
      from: "domain",
      to: "execution",
      reason: "Domain models must not spawn processes",
      enforced: "cargo_edge",
    },
    {
      from: "domain",
      to: "repositories",
      reason: "Domain must not depend on persistence",
      enforced: "cargo_edge",
    },
    {
      from: "repositories",
      to: "services",
      reason: "Repositories must not call services",
      enforced: "cargo_edge+import_scan",
    },
    {
      from: "repositories",
      to: "kernel_commands",
      reason: "Repositories must not import kernel",
      enforced: "cargo_edge+import_scan",
    },
    {
      from: "services",
      to: "permission_gateway_require",
      reason:
        "Services must not call PermissionGateway::require; evaluate() discovery probes remain allowed",
      enforced: "services_require_scan+call_site_allowlist",
    },
  ];

  const crateEdges = {
    "workspace-domain→workspace-database": cargoDependsOn(
      path.join(rootDir, "packages/domain/Cargo.toml"),
      "workspace-database",
    ),
    "workspace-domain→workspace-kernel": cargoDependsOn(
      path.join(rootDir, "packages/domain/Cargo.toml"),
      "workspace-kernel",
    ),
    "workspace-domain→workspace-windows-integration": cargoDependsOn(
      path.join(rootDir, "packages/domain/Cargo.toml"),
      "workspace-windows-integration",
    ),
    "workspace-database→workspace-kernel": cargoDependsOn(
      path.join(rootDir, "packages/database/Cargo.toml"),
      "workspace-kernel",
    ),
    "workspace-database→workspace-windows-integration": cargoDependsOn(
      path.join(rootDir, "packages/database/Cargo.toml"),
      "workspace-windows-integration",
    ),
  };

  return {
    generated_at_note:
      "Detector inventory derived from repository layout + Cargo edges. Not an authority — live verify:architecture-governance enforces rules and map drift.",
    layers,
    allowed_edges: allowedEdges,
    forbidden_edges: forbiddenEdges,
    crate_edges: crateEdges,
    capability_authority_owners: CAPABILITY_AUTHORITY_OWNERS,
    intentional_broad_capabilities: INTENTIONAL_BROAD_CAPABILITIES,
    mutation_command_baseline: MUTATION_COMMAND_BASELINE,
  };
}

/**
 * Canonical map payload for drift comparison (stable key order via construction).
 */
export function buildCanonicalMapPayload(audit) {
  return {
    generated_at_note: audit.map.generated_at_note,
    layers: audit.map.layers,
    allowed_edges: audit.map.allowed_edges,
    forbidden_edges: audit.map.forbidden_edges,
    crate_edges: audit.map.crate_edges,
    capability_authority_owners: audit.map.capability_authority_owners,
    intentional_broad_capabilities: audit.map.intentional_broad_capabilities,
    mutation_command_baseline: audit.map.mutation_command_baseline,
    mutation_capability_inventory: audit.mutationInventory,
    unused_capabilities: audit.unusedCapabilities,
    permission_gateway_require_sites: audit.map.permission_gateway_require_sites,
    evidence: audit.evidence,
    stats: audit.map.stats,
  };
}

export function canonicalMapJson(audit) {
  return `${JSON.stringify(buildCanonicalMapPayload(audit), null, 2)}\n`;
}

/**
 * Full architecture governance audit (fail-closed).
 * @param {string} rootDir
 * @param {{ skipMapFileRequirement?: boolean }} [options]
 */
export function auditArchitectureGovernance(rootDir, options = {}) {
  const violations = [];
  const evidence = {
    source_trees: [],
    history_dtos_found: [],
    projection_dtos_found: [],
    mutation_commands: 0,
    query_commands: 0,
    assertions: [],
  };

  const requiredFiles = options.skipMapFileRequirement
    ? REQUIRED_FILES.filter((f) => f !== "scripts/generated/architecture-map.json")
    : REQUIRED_FILES;
  for (const rel of requiredFiles) {
    if (!fs.existsSync(path.join(rootDir, rel))) {
      violations.push(
        `Required architecture input missing: ${rel} — governance audit cannot establish compliance`,
      );
    }
  }

  const trees = assertRequiredTrees(rootDir);
  violations.push(...trees.violations);
  evidence.source_trees = trees.evidence;

  const map = buildArchitectureMap(rootDir);

  if (map.crate_edges["workspace-domain→workspace-database"]) {
    violations.push("Forbidden crate edge: domain → database");
  }
  if (map.crate_edges["workspace-domain→workspace-kernel"]) {
    violations.push("Forbidden crate edge: domain → kernel");
  }
  if (map.crate_edges["workspace-domain→workspace-windows-integration"]) {
    violations.push(
      "Forbidden crate edge: domain → windows-integration (process spawn)",
    );
  }
  if (map.crate_edges["workspace-database→workspace-kernel"]) {
    violations.push(
      "Forbidden crate edge: database → kernel (repositories calling services)",
    );
  }
  if (map.crate_edges["workspace-database→workspace-windows-integration"]) {
    violations.push("Forbidden crate edge: database → windows-integration");
  }

  const appSrc = path.join(rootDir, "app/src");
  if (fs.existsSync(appSrc)) {
    violations.push(...reactLifecycleMutationImports(appSrc));
  }

  const databaseSrc = path.join(rootDir, "packages/database/src");
  if (fs.existsSync(databaseSrc)) {
    violations.push(...repositoryServiceImports(databaseSrc));
  }

  violations.push(...recommendationEngineProcessSpawn(rootDir));
  violations.push(...cognitiveOrchestrationGuards(rootDir));
  violations.push(...learningAdaptationGuards(rootDir));
  violations.push(...cognitiveAgentCastGuards(rootDir));
  violations.push(...cognitiveAutonomyGuards(rootDir));
  violations.push(...workspaceStateEnvelopeGuards(rootDir));
  violations.push(...policyGovernanceGuards(rootDir));
  violations.push(...historicalReconstructionGuards(rootDir));
  violations.push(...temporalIntelligenceGuards(rootDir));
  violations.push(...workspaceExplanationGuards(rootDir));
  violations.push(...contextualUnderstandingGuards(rootDir));
  violations.push(...knowledgeSynthesisGuards(rootDir));

  const domainSrc = path.join(rootDir, "packages/domain/src");
  const domainSources = fs.existsSync(domainSrc) ? rustSources(domainSrc) : [];
  if (domainSources.length === 0 && fs.existsSync(domainSrc)) {
    violations.push(
      "Domain source tree has no .rs files; governance audit cannot establish compliance",
    );
  }

  const historyCheck = checkDtoAuthorityFields(
    domainSources,
    HISTORY_STRUCTS,
    "history",
  );
  violations.push(...historyCheck.violations);
  evidence.history_dtos_found = historyCheck.found;
  if (historyCheck.found.length === HISTORY_STRUCTS.length) {
    evidence.assertions.push(
      `Found ${historyCheck.found.length} history DTOs and verified command-field restrictions.`,
    );
  } else if (historyCheck.found.length === 0) {
    evidence.assertions.push(
      "No history DTO structures found; governance audit cannot establish compliance.",
    );
  }

  const projectionCheck = checkDtoAuthorityFields(
    domainSources,
    PROJECTION_SUMMARY_STRUCTS,
    "projection",
  );
  violations.push(...projectionCheck.violations);
  evidence.projection_dtos_found = projectionCheck.found;
  if (projectionCheck.found.length === PROJECTION_SUMMARY_STRUCTS.length) {
    evidence.assertions.push(
      `Found ${projectionCheck.found.length} projection summary DTOs and verified authority-field restrictions.`,
    );
  }

  const servicesDir = path.join(rootDir, "packages/kernel/src/services");
  if (fs.existsSync(servicesDir)) {
    violations.push(...lifecycleMutationVisibility(servicesDir));
    violations.push(...servicesCallingGatewayRequire(servicesDir, rootDir));
  }

  const gatewaySites = permissionGatewayRequireSites(rootDir);
  const gatewayAllow = new Set([
    "packages/kernel/src/commands/pipeline.rs",
    "packages/kernel/src/commands/handler.rs",
  ]);
  for (const site of gatewaySites) {
    if (!gatewayAllow.has(site.file)) {
      violations.push(
        `${site.file}:${site.line}: PermissionGateway::require outside CommandPipeline/shutdown`,
      );
    }
  }

  const handlerPath = path.join(
    rootDir,
    "packages/kernel/src/commands/handler.rs",
  );
  if (fs.existsSync(handlerPath)) {
    const handlerSource = read(handlerPath);
    for (const match of handlerSource.matchAll(/PermissionGateway::require\s*\(/g)) {
      const before = handlerSource.slice(
        Math.max(0, match.index - 400),
        match.index,
      );
      if (!/fn shutdown\b|ShutdownWorkspace/.test(before)) {
        const line = handlerSource.slice(0, match.index).split("\n").length;
        violations.push(
          `packages/kernel/src/commands/handler.rs:${line}: PermissionGateway::require outside shutdown — use CommandPipeline`,
        );
      }
    }
  }

  const capabilityPath = path.join(
    rootDir,
    "packages/domain/src/capability/mod.rs",
  );
  let catalogIds = new Set();
  if (fs.existsSync(capabilityPath)) {
    catalogIds = new Set(listCapabilityConstructors(read(capabilityPath)));
    for (const id of catalogIds) {
      if (!CAPABILITY_AUTHORITY_OWNERS[id]) {
        violations.push(
          `Capability '${id}' has no authority owner mapping in architecture governance`,
        );
      }
    }
    // Duplicate ownership values are allowed (many caps share an owner).
    // Detect duplicate owner *assignments for the same id* — N/A for map.
    // Detect conflicting owners if CAPABILITY_AUTHORITY_OWNERS had dup keys — JS object can't.
  }

  const commandsDir = path.join(rootDir, "packages/kernel/src/commands");
  const mutations = fs.existsSync(commandsDir)
    ? listCommandCapabilities(commandsDir, "MutationCommand")
    : [];
  const queries = fs.existsSync(commandsDir)
    ? listCommandCapabilities(commandsDir, "QueryCommand")
    : [];

  const mutationInventory = [];
  const usedCapabilityIds = new Set();

  for (const entry of mutations) {
    if (!entry.capabilityMethod && !entry.capabilityId) {
      violations.push(
        `MutationCommand ${entry.command} missing required_capability()`,
      );
      continue;
    }
    const capabilityId =
      entry.capabilityId ??
      capabilityMethodToCatalogId(entry.capabilityMethod);
    if (!capabilityId || !catalogIds.has(capabilityId)) {
      violations.push(
        `MutationCommand ${entry.command} capability does not map to catalog id (got ${entry.capabilityMethod ?? entry.capabilityId})`,
      );
      continue;
    }
    const owner = CAPABILITY_AUTHORITY_OWNERS[capabilityId];
    if (!owner) {
      violations.push(
        `MutationCommand ${entry.command} capability '${capabilityId}' has no authority owner`,
      );
      continue;
    }
    usedCapabilityIds.add(capabilityId);
    mutationInventory.push({
      command: entry.command,
      capability: capabilityId,
      authority_owner: owner,
    });
  }

  for (const entry of queries) {
    const capabilityId =
      entry.capabilityId ??
      capabilityMethodToCatalogId(entry.capabilityMethod);
    if (capabilityId) usedCapabilityIds.add(capabilityId);
  }

  evidence.mutation_commands = mutationInventory.length;
  evidence.query_commands = queries.length;

  const recoveryDiag = auditRecoveryDiagnosticsEvidenceOnly(
    rootDir,
    mutationInventory,
  );
  violations.push(...recoveryDiag.violations);
  evidence.assertions.push(...recoveryDiag.assertions);

  if (mutationInventory.length < MUTATION_COMMAND_BASELINE) {
    violations.push(
      `Mutation command inventory dropped below baseline: found ${mutationInventory.length}, need ≥ ${MUTATION_COMMAND_BASELINE}`,
    );
  } else {
    evidence.assertions.push(
      `Found ${mutationInventory.length} mutation commands (≥ baseline ${MUTATION_COMMAND_BASELINE}) with capability paths.`,
    );
  }

  const unusedCapabilities = sorted(
    [...catalogIds].filter((id) => !usedCapabilityIds.has(id)),
  );
  // Unused is informational debt, not a hard fail — but must be reported in map.
  // Owner mismatch: inventory owner must equal CAPABILITY_AUTHORITY_OWNERS[capability]
  for (const entry of mutationInventory) {
    if (CAPABILITY_AUTHORITY_OWNERS[entry.capability] !== entry.authority_owner) {
      violations.push(
        `Owner mismatch for ${entry.command}: ${entry.capability} → ${entry.authority_owner}`,
      );
    }
  }

  if (fs.existsSync(handlerPath)) {
    const missing = handlerMethodsMissingPipeline(read(handlerPath));
    for (const name of missing) {
      violations.push(
        `CommandHandler::${name} appears to mutate without CommandPipeline`,
      );
    }
  }

  map.mutation_capability_inventory = mutationInventory;
  map.permission_gateway_require_sites = gatewaySites;
  map.unused_capabilities = unusedCapabilities;
  map.stats = {
    mutation_commands: mutationInventory.length,
    query_commands: queries.length,
    capability_catalog: catalogIds.size,
    unused_capabilities: unusedCapabilities.length,
    gateway_require_sites: gatewaySites.length,
    history_dtos_found: evidence.history_dtos_found.length,
    projection_dtos_found: evidence.projection_dtos_found.length,
    violation_count: violations.length,
  };

  return {
    map,
    violations: sorted(violations),
    mutationInventory,
    unusedCapabilities,
    evidence,
  };
}

/**
 * Compare live canonical map to committed JSON. Returns drift violations.
 */
export function detectArchitectureMapDrift(rootDir, mapPath) {
  const audit = auditArchitectureGovernance(rootDir, {
    skipMapFileRequirement: false,
  });
  const generated = canonicalMapJson(audit);
  if (!fs.existsSync(mapPath)) {
    return {
      audit,
      driftViolations: [
        `Committed architecture map missing at ${path.relative(rootDir, mapPath)}`,
      ],
      generated,
    };
  }
  const committed = read(mapPath);
  if (committed !== generated) {
    return {
      audit,
      driftViolations: [
        "Architecture map drift detected: scripts/generated/architecture-map.json differs from live inventory. Run `pnpm verify:architecture-governance -- --write` to refresh after intentional changes.",
      ],
      generated,
    };
  }
  return { audit, driftViolations: [], generated };
}

export function writeArchitectureMap(rootDir, outPath) {
  const audit = auditArchitectureGovernance(rootDir, {
    skipMapFileRequirement: true,
  });
  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  fs.writeFileSync(outPath, canonicalMapJson(audit));
  return audit;
}

/**
 * Test helper: run fail-closed assertions against a synthetic root.
 * @param {string} rootDir
 */
export function auditFailClosedExpectations(rootDir) {
  return auditArchitectureGovernance(rootDir, { skipMapFileRequirement: true });
}
