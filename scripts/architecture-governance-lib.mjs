/**
 * Architecture governance enforcement — machine-checkable inventory + forbidden edges.
 *
 * Authority path (allowed):
 *   React → IPC → Kernel Command → Permission Gateway → Service → Repository → Domain
 *
 * Does not redesign ownership; only audits that growth stays within sealed rules.
 */
import fs from "node:fs";
import path from "node:path";

const read = (file) => fs.readFileSync(file, "utf8");

const sorted = (values) => [...new Set(values)].sort();

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

/** Capability id → authority owner (scope owner from domain Capability catalog). */
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

/** History / projection DTO field names that imply executable capability. */
export const FORBIDDEN_HISTORY_FIELDS = [
  "execute",
  "execute_command",
  "handoff_command",
  "dispatch_allowed",
  "cancellation_allowed",
  "recommended_action",
  "select_command",
  "dismiss_command",
  "mutate",
  "transition_command",
  "command_envelope",
  "mutation_allowed",
  "spawn_process",
  "launch_application",
];

/** Projection summary fields that must not become command inputs. */
export const FORBIDDEN_PROJECTION_COMMAND_FIELDS = [
  "command_envelope",
  "execute_command",
  "mutation_payload",
  "spawn_process",
  "permission_bypass",
];

const HISTORY_STRUCTS = [
  "RecommendationHistoryEntry",
  "DecisionOverlayHistoryEntry",
  "DecisionArtifactHistoryEntry",
  "TaskHistoryEntry",
  "ExecutionLifecycleHistoryEntry",
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

/** Handler methods allowed to skip CommandPipeline (documented exceptions). */
const HANDLER_PIPELINE_ALLOWLIST = new Set([
  "get_settings",
  "initialize_workspace",
  "initialize_workspace_in_memory",
  "shutdown",
  // Architecture-guard stubs that only return Err — never authorize/execute.
  "workspace_observation_attempt_execute",
  "workspace_recommendation_engine_attempt_execute",
  "decision_queue_attempt_execute",
  "decision_engine_attempt_execute",
  "workspace_attention_attempt_execute",
  "workspace_activity_attempt_execute",
  "workspace_intelligence_attempt_execute",
  "workspace_experience_attempt_execute",
  "workspace_work_context_attempt_execute",
  "workspace_navigation_attempt_execute",
  "workspace_milestone_attempt_execute",
  "workspace_working_style_attempt_execute",
  "workspace_transition_attempt_execute",
  "workspace_interaction_attempt_execute",
  "workspace_profile_attempt_execute",
  // Pure composition of already-gated queries / diagnostics.
  "compare_workspace_intelligence_states",
  "compare_assistant_plan_revisions",
  "compare_personalized_vs_neutral_plan",
]);

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

function listCapabilityConstructors(capabilitySource) {
  const ids = [];
  // Catalog ids appear as CapabilityId::new("…") — may span lines before .expect.
  const pattern = /CapabilityId::new\(\s*"([a-z0-9_.]+)"\s*\)/g;
  for (const match of capabilitySource.matchAll(pattern)) {
    ids.push(match[1]);
  }
  return sorted(ids);
}

function listMutationRequiredCapabilities(commandsDir) {
  const entries = [];
  for (const file of rustSources(commandsDir)) {
    const source = read(file);
    if (!source.includes("impl MutationCommand for")) continue;
    const implBlocks = [
      ...source.matchAll(
        /impl MutationCommand for ([A-Za-z0-9_]+) \{([\s\S]*?)\n\}/g,
      ),
    ];
    for (const block of implBlocks) {
      const name = block[1];
      const body = block[2];
      const cap = body.match(
        /fn required_capability\(&[^\)]*\)\s*->\s*Capability\s*\{([\s\S]*?)\}/,
      );
      if (!cap) {
        entries.push({ command: name, capability: null, file });
        continue;
      }
      const idMatch = cap[1].match(/Capability::([a-z0-9_]+)\(\)/);
      const literal = cap[1].match(/CapabilityId::new\("([a-z0-9_.]+)"\)/);
      entries.push({
        command: name,
        capabilityMethod: idMatch?.[1] ?? null,
        capabilityId: literal?.[1] ?? null,
        file,
      });
    }
  }
  return entries;
}

/** Normalize Capability::foo_bar() to catalog id foo.bar with known overrides. */
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

function permissionGatewayCallSites(rootDir) {
  const sites = [];
  for (const file of rustSources(path.join(rootDir, "packages/kernel/src"))) {
    // Test modules may call require() to assert fail-closed behaviour.
    if (/_tests\.rs$/.test(file) || /\/tests\.rs$/.test(file)) continue;
    const source = read(file);
    const rel = path.relative(rootDir, file).replace(/\\/g, "/");
    for (const match of source.matchAll(/PermissionGateway::require\s*\(/g)) {
      const before = source.slice(0, match.index);
      const line = before.split("\n").length;
      sites.push({ file: rel, line });
    }
  }
  return sites;
}

function splitHandlerMethods(handlerSource) {
  const methods = [];
  const parts = handlerSource.split(/\n    pub fn /);
  for (let i = 1; i < parts.length; i += 1) {
    const part = parts[i];
    const nameMatch = part.match(/^([a-z0-9_]+)\s*\(/);
    if (!nameMatch) continue;
    const name = nameMatch[1];
    const brace = part.indexOf("{");
    if (brace < 0) continue;
    methods.push({ name, body: part.slice(brace) });
  }
  return methods;
}

function handlerMethodsMissingPipeline(handlerSource) {
  const violations = [];
  const methods = splitHandlerMethods(handlerSource);
  const mutationName =
    /^(accept_|reject_|present_|create_|update_|delete_|dismiss_|select_|defer_|postpone_|claim_|complete_|mark_|confirm_|decline_|launch_|execute_|record_|set_|reset_|submit_|advance_|resume_|cancel_|decide_)/;

  // Architecture-guard stubs and pure compare/composition helpers.
  const autoAllow = (name) =>
    HANDLER_PIPELINE_ALLOWLIST.has(name) ||
    /_attempt_execute$/.test(name) ||
    /^compare_/.test(name);

  for (const method of methods) {
    const { name, body } = method;
    if (autoAllow(name)) continue;
    if (!mutationName.test(name)) continue;

    const usesPipeline =
      /CommandPipeline::new/.test(body) ||
      /PermissionGateway::require/.test(body) ||
      /execute_mutation|execute_query/.test(body);

    // Delegates to an *_inner helper that owns the pipeline call.
    const delegatesInner =
      /Self::[a-z0-9_]+_inner\s*\(/.test(body) ||
      /Self::[a-z0-9_]+\(/.test(body);

    if (!usesPipeline && !delegatesInner) {
      violations.push(name);
    }
  }
  return sorted(violations);
}

/** Services whose mutators must remain crate-private behind CommandPipeline. */
const LIFECYCLE_SERVICE_FILES = new Set([
  "workspace_recommendation.rs",
  "decision_engine.rs",
  "decision_queue.rs",
  "task_graph.rs",
  "execution_lifecycle.rs",
  "application_launch.rs",
  "suggestion.rs",
  "suggestion_lifecycle.rs",
]);

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
            `${base}: lifecycle mutator '${fnName}' is pub — must be pub(crate) behind CommandPipeline`,
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
    // No Rust crate / database imports from React.
    if (
      /from\s+["'][^"']*(?:workspace_database|workspace_kernel|rusqlite|packages\/database|packages\/kernel)["']/.test(
        source,
      )
    ) {
      violations.push(`${rel}: React must not import kernel/database crates`);
    }
    // Projection helpers must not export lifecycle mutators.
    if (/Projection\.ts$/.test(rel) || /projection\.ts$/.test(rel)) {
      for (const fn of LIFECYCLE_MUTATION_SERVICE_FNS) {
        const camel = fn.replace(/_([a-z])/g, (_, c) => c.toUpperCase());
        if (
          new RegExp(`export\\s+(?:async\\s+)?function\\s+${fn}\\b`).test(source) ||
          new RegExp(`export\\s+(?:async\\s+)?function\\s+${camel}\\b`).test(source)
        ) {
          violations.push(
            `${rel}: projection module exports lifecycle mutator '${fn}' — React is projection-only`,
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
    const files = fs.existsSync(target) && fs.statSync(target).isDirectory()
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

function historyDtoCommandFields(domainSrcDir) {
  const violations = [];
  for (const file of rustSources(domainSrcDir)) {
    const source = read(file);
    for (const structName of HISTORY_STRUCTS) {
      const fields = extractStructFields(source, structName);
      if (!fields) continue;
      for (const field of fields) {
        if (FORBIDDEN_HISTORY_FIELDS.includes(field)) {
          violations.push(
            `${structName}.${field}: history DTO must not gain executable capability fields`,
          );
        }
      }
      if (fields.includes("actionable") === false) {
        // actionable flag is required for fail-closed projection contract
        violations.push(
          `${structName}: history DTO missing required 'actionable' evidence flag`,
        );
      }
    }
  }
  return violations;
}

function buildArchitectureMap(rootDir) {
  const layers = [
    {
      id: "react",
      path: "app/src",
      role: "Projection-only UI; IPC invoke only",
    },
    {
      id: "ipc",
      path: "app/src-tauri",
      role: "Tauri command edge; forwards to CommandHandler",
    },
    {
      id: "kernel_commands",
      path: "packages/kernel/src/commands",
      role: "CommandPipeline entry; declares required_capability",
    },
    {
      id: "permission_gateway",
      path: "packages/kernel/src/security",
      role: "Sole Allow/Deny/ApprovalRequired authority decision",
    },
    {
      id: "services",
      path: "packages/kernel/src/services",
      role: "Lifecycle owners; mutators pub(crate) after gateway Allow",
    },
    {
      id: "domain",
      path: "packages/domain/src",
      role: "Canonical models; no persistence or process spawn",
    },
    {
      id: "repositories",
      path: "packages/database/src",
      role: "Persistence guards; no service imports",
    },
    {
      id: "execution",
      path: "packages/windows-integration/src",
      role: "OS spawn / desktop capture — only via ApplicationLaunchService after gateway",
    },
  ];

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

  const forbiddenEdges = [
    {
      from: "react",
      to: "repositories",
      reason: "React must never open the database",
    },
    {
      from: "react",
      to: "services",
      reason: "React must not call lifecycle services directly",
    },
    {
      from: "react",
      to: "domain",
      reason: "React consumes IPC DTOs, not Rust domain crates",
    },
    {
      from: "react",
      to: "execution",
      reason: "React must not spawn processes",
    },
    {
      from: "domain",
      to: "execution",
      reason: "Domain models must not spawn processes",
    },
    {
      from: "domain",
      to: "repositories",
      reason: "Domain must not depend on persistence",
    },
    {
      from: "repositories",
      to: "services",
      reason: "Repositories must not call services",
    },
    {
      from: "repositories",
      to: "kernel_commands",
      reason: "Repositories must not import kernel",
    },
    {
      from: "services",
      to: "permission_gateway",
      reason: "Services must not self-authorize; gateway runs before service entry",
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
      "Inventory is derived from repository layout + Cargo edges; forbidden edges are enforced by verify:architecture-governance",
    layers,
    allowed_edges: allowedEdges,
    forbidden_edges: forbiddenEdges,
    crate_edges: crateEdges,
    capability_authority_owners: CAPABILITY_AUTHORITY_OWNERS,
  };
}

/**
 * Full architecture governance audit.
 * @param {string} rootDir repository root
 */
export function auditArchitectureGovernance(rootDir) {
  const violations = [];
  const map = buildArchitectureMap(rootDir);

  // --- Cargo / crate forbidden edges ---
  if (map.crate_edges["workspace-domain→workspace-database"]) {
    violations.push("Forbidden crate edge: domain → database");
  }
  if (map.crate_edges["workspace-domain→workspace-kernel"]) {
    violations.push("Forbidden crate edge: domain → kernel");
  }
  if (map.crate_edges["workspace-domain→workspace-windows-integration"]) {
    violations.push("Forbidden crate edge: domain → windows-integration (process spawn)");
  }
  if (map.crate_edges["workspace-database→workspace-kernel"]) {
    violations.push("Forbidden crate edge: database → kernel (repositories calling services)");
  }
  if (map.crate_edges["workspace-database→workspace-windows-integration"]) {
    violations.push("Forbidden crate edge: database → windows-integration");
  }

  // --- React ---
  violations.push(
    ...reactLifecycleMutationImports(path.join(rootDir, "app/src")),
  );

  // --- Repositories must not import services ---
  violations.push(
    ...repositoryServiceImports(path.join(rootDir, "packages/database/src")),
  );

  // --- Recommendation Engine must not spawn ---
  violations.push(...recommendationEngineProcessSpawn(rootDir));

  // --- History / projection DTO fields ---
  violations.push(
    ...historyDtoCommandFields(path.join(rootDir, "packages/domain/src")),
  );

  // --- Lifecycle mutators must stay pub(crate) ---
  violations.push(
    ...lifecycleMutationVisibility(
      path.join(rootDir, "packages/kernel/src/services"),
    ),
  );

  // --- PermissionGateway.require call-site allowlist ---
  const gatewaySites = permissionGatewayCallSites(rootDir);
  const gatewayAllow = new Set([
    "packages/kernel/src/commands/pipeline.rs",
    "packages/kernel/src/commands/handler.rs", // shutdown only
  ]);
  for (const site of gatewaySites) {
    if (!gatewayAllow.has(site.file)) {
      violations.push(
        `${site.file}:${site.line}: PermissionGateway::require outside CommandPipeline/shutdown — convenience bypass`,
      );
    }
  }
  // handler.rs may only call require inside shutdown
  const handlerPath = path.join(
    rootDir,
    "packages/kernel/src/commands/handler.rs",
  );
  if (fs.existsSync(handlerPath)) {
    const handlerSource = read(handlerPath);
    const requireMatches = [
      ...handlerSource.matchAll(/PermissionGateway::require\s*\(/g),
    ];
    for (const match of requireMatches) {
      const before = handlerSource.slice(Math.max(0, match.index - 400), match.index);
      if (!/fn shutdown\b|ShutdownWorkspace/.test(before)) {
        const line = handlerSource.slice(0, match.index).split("\n").length;
        violations.push(
          `packages/kernel/src/commands/handler.rs:${line}: PermissionGateway::require outside shutdown — use CommandPipeline`,
        );
      }
    }
  }

  // --- Mutation commands must declare capability with authority owner ---
  const capabilitySource = read(
    path.join(rootDir, "packages/domain/src/capability/mod.rs"),
  );
  const catalogIds = new Set(listCapabilityConstructors(capabilitySource));
  for (const id of catalogIds) {
    if (!CAPABILITY_AUTHORITY_OWNERS[id]) {
      violations.push(
        `Capability '${id}' has no authority owner mapping in architecture governance`,
      );
    }
  }

  const mutations = listMutationRequiredCapabilities(
    path.join(rootDir, "packages/kernel/src/commands"),
  );
  const mutationInventory = [];
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
    if (!CAPABILITY_AUTHORITY_OWNERS[capabilityId]) {
      violations.push(
        `MutationCommand ${entry.command} capability '${capabilityId}' has no authority owner`,
      );
      continue;
    }
    mutationInventory.push({
      command: entry.command,
      capability: capabilityId,
      authority_owner: CAPABILITY_AUTHORITY_OWNERS[capabilityId],
    });
  }

  // --- Handler mutation path uses CommandPipeline ---
  if (fs.existsSync(handlerPath)) {
    const missing = handlerMethodsMissingPipeline(read(handlerPath));
    for (const name of missing) {
      violations.push(
        `CommandHandler::${name} appears to mutate without CommandPipeline`,
      );
    }
  }

  // --- Kernel services must not import React / app paths (sanity) ---
  // covered by crate graph

  map.mutation_capability_inventory = mutationInventory;
  map.permission_gateway_call_sites = gatewaySites;
  map.stats = {
    mutation_commands: mutationInventory.length,
    capability_catalog: catalogIds.size,
    gateway_require_sites: gatewaySites.length,
    violation_count: violations.length,
  };

  return {
    map,
    violations: sorted(violations),
    mutationInventory,
  };
}

/**
 * Write machine-checkable architecture map JSON.
 */
export function writeArchitectureMap(rootDir, outPath) {
  const audit = auditArchitectureGovernance(rootDir);
  const payload = {
    ...audit.map,
    verified_clean: audit.violations.length === 0,
    violations: audit.violations,
  };
  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  fs.writeFileSync(outPath, `${JSON.stringify(payload, null, 2)}\n`);
  return audit;
}
