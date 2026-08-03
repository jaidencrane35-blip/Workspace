/**
 * Sprint 73 — Continuous Engineering Certification.
 * Canonical invariant registry + deterministic certification runner.
 * Engineering confidence only — no Runtime / Experience / persistence behaviour changes.
 */

import {
  ARCHITECTURE_AUTHORITY_DOCS,
  ENGINEERING_STORAGE_KEY,
  type ArchitectureAuthorityDoc,
} from "./engineeringGovernance";
import {
  ARCHITECTURE_INTEGRITY_STORAGE_KEY,
  AUTHORITY_CHAIN,
  AUTHORITY_ROOT,
  buildArchitectureGraph,
  validateArchitectureIntegrity,
} from "./architecturalIntegrity";
import {
  contentAddressedId,
  storeArchitectureIntegrityValid,
} from "./governancePrimitives";
import type { ExperienceStoreAdapter } from "./experienceStore";
import { EVIDENCE_STORAGE_KEY } from "./experienceEvidence";
import { GOVERNANCE_STORAGE_KEY } from "./experienceGovernance";
import {
  COMPLEXITY_AFTER,
  buildArchitecturalComplexityReport,
} from "../experience/architecturalComplexity";
import { validateComposition } from "../experience/adaptationComposition";
import { resolvePresentationWithAnticipation } from "../experience/workspaceAnticipation";
import {
  CERTIFICATION_STORAGE_KEY,
  listCertifications,
} from "../experience/adaptationCertification";
import { PACK_STORAGE_KEY } from "../experience/adaptationPacks";
import { ADAPTATION_STORAGE_KEY } from "../experience/workspaceAdaptation";
import * as experienceBarrel from "../experience/index";

export type EngineeringSubsystem =
  | "authority"
  | "governance"
  | "evidence"
  | "adaptation"
  | "presentation"
  | "certification"
  | "complexity"
  | "integrity";

export type InvariantScope =
  | "resolver"
  | "lineage"
  | "replay"
  | "storage"
  | "api"
  | "schema"
  | "authority";

export type InvariantOutcome = "passed" | "failed" | "skipped";

export type InvariantFailureCause =
  | "expected_mismatch"
  | "missing_store"
  | "lineage_incomplete"
  | "integrity_invalid"
  | "composition_failed"
  | "schema_mismatch"
  | "api_missing"
  | "authority_gap"
  | "nondeterministic";

/**
 * Canonical engineering invariant — derived metadata only.
 * Avoids forbidden content keys (no note/summary/message/label/text).
 */
export interface EngineeringInvariant {
  schemaVersion: 1;
  invariantId: string;
  authorityDocument: ArchitectureAuthorityDoc | string;
  scope: InvariantScope;
  validationMethod: string;
  expectedResult: "pass";
  owningSubsystem: EngineeringSubsystem;
}

export interface InvariantExecutionResult {
  invariantId: string;
  outcome: InvariantOutcome;
  cause: InvariantFailureCause | null;
  owningSubsystem: EngineeringSubsystem;
  authorityDocument: string;
  durationMs: number;
}

export interface EngineeringCertificationReport {
  schemaVersion: 1;
  certificationId: string;
  certifiedAt: number;
  durationMs: number;
  passed: string[];
  failed: string[];
  skipped: string[];
  results: InvariantExecutionResult[];
  authorityReferences: string[];
  subsystems: EngineeringSubsystem[];
  /** True only when failed.length === 0 (skipped allowed). */
  complete: boolean;
}

export interface EngineeringCertificationComparison {
  schemaVersion: 1;
  newlyFailed: string[];
  newlyResolved: string[];
  invariantsAdded: string[];
  invariantsRemoved: string[];
}

const MAX_HISTORY = 20;

let certificationHistory: EngineeringCertificationReport[] = [];

const REQUIRED_PUBLIC_EXPORTS = [
  "resolvePresentationConfiguration",
  "resolvePresentationWithAnticipation",
  "validateComposition",
  "certifyAdaptationSet",
  "listAdaptations",
  "buildArchitecturalComplexityReport",
] as const;

/** Existing key constants referenced for compatibility checks only — no new keys. */
const EXISTING_STORAGE_KEY_CONSTANTS = [
  EVIDENCE_STORAGE_KEY,
  GOVERNANCE_STORAGE_KEY,
  ENGINEERING_STORAGE_KEY,
  ARCHITECTURE_INTEGRITY_STORAGE_KEY,
  ADAPTATION_STORAGE_KEY,
  CERTIFICATION_STORAGE_KEY,
  PACK_STORAGE_KEY,
] as const;

/** Canonical invariant registry — single source of truth. */
export const ENGINEERING_INVARIANTS: readonly EngineeringInvariant[] = [
  {
    schemaVersion: 1,
    invariantId: "inv-resolver-stage-count",
    authorityDocument: "63_Architectural_Simplification.md",
    scope: "resolver",
    validationMethod: "complexity_resolver_stages",
    expectedResult: "pass",
    owningSubsystem: "presentation",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-authority-chain-linkage",
    authorityDocument: "47_Architectural_Integrity.md",
    scope: "authority",
    validationMethod: "authority_chain_pairs",
    expectedResult: "pass",
    owningSubsystem: "authority",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-authority-docs-tip",
    authorityDocument: "65_Architectural_Maintainability.md",
    scope: "authority",
    validationMethod: "authority_docs_include_tip",
    expectedResult: "pass",
    owningSubsystem: "authority",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-complexity-zero-duplication",
    authorityDocument: "63_Architectural_Simplification.md",
    scope: "schema",
    validationMethod: "complexity_zero_dup",
    expectedResult: "pass",
    owningSubsystem: "complexity",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-storage-keys-present",
    authorityDocument: "56_Governance_Consolidation.md",
    scope: "storage",
    validationMethod: "storage_key_constants",
    expectedResult: "pass",
    owningSubsystem: "governance",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-public-api-exports",
    authorityDocument: "63_Architectural_Simplification.md",
    scope: "api",
    validationMethod: "experience_barrel_exports",
    expectedResult: "pass",
    owningSubsystem: "presentation",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-replay-presentation-determinism",
    authorityDocument: "60_Workspace_Anticipation.md",
    scope: "replay",
    validationMethod: "presentation_double_resolve",
    expectedResult: "pass",
    owningSubsystem: "presentation",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-architecture-graph-integrity",
    authorityDocument: "47_Architectural_Integrity.md",
    scope: "lineage",
    validationMethod: "empty_graph_integrity",
    expectedResult: "pass",
    owningSubsystem: "integrity",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-composition-on-store",
    authorityDocument: "53_Adaptation_Composition.md",
    scope: "lineage",
    validationMethod: "validate_composition_store",
    expectedResult: "pass",
    owningSubsystem: "adaptation",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-store-architecture-integrity",
    authorityDocument: "47_Architectural_Integrity.md",
    scope: "lineage",
    validationMethod: "store_architecture_integrity",
    expectedResult: "pass",
    owningSubsystem: "integrity",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-certification-immutability-shape",
    authorityDocument: "54_Adaptation_Certification.md",
    scope: "schema",
    validationMethod: "certification_list_shape",
    expectedResult: "pass",
    owningSubsystem: "certification",
  },
  {
    schemaVersion: 1,
    invariantId: "inv-evidence-schema-key",
    authorityDocument: "43_Experience_Evidence_Model.md",
    scope: "schema",
    validationMethod: "evidence_storage_key",
    expectedResult: "pass",
    owningSubsystem: "evidence",
  },
] as const;

export function listEngineeringInvariants(): EngineeringInvariant[] {
  return ENGINEERING_INVARIANTS.map((inv) => ({ ...inv }));
}

function pass(): { outcome: InvariantOutcome; cause: InvariantFailureCause | null } {
  return { outcome: "passed", cause: null };
}

function fail(
  cause: InvariantFailureCause,
): { outcome: InvariantOutcome; cause: InvariantFailureCause | null } {
  return { outcome: "failed", cause };
}

function skip(
  cause: InvariantFailureCause,
): { outcome: InvariantOutcome; cause: InvariantFailureCause | null } {
  return { outcome: "skipped", cause };
}

function executeInvariant(
  invariant: EngineeringInvariant,
  store: ExperienceStoreAdapter | null,
): { outcome: InvariantOutcome; cause: InvariantFailureCause | null } {
  switch (invariant.validationMethod) {
    case "complexity_resolver_stages": {
      const report = buildArchitecturalComplexityReport();
      return report.after.resolverStages === COMPLEXITY_AFTER.resolverStages &&
        report.after.resolverStages === 5
        ? pass()
        : fail("expected_mismatch");
    }
    case "authority_chain_pairs": {
      if (AUTHORITY_CHAIN.length !== ARCHITECTURE_AUTHORITY_DOCS.length - 1) {
        return fail("authority_gap");
      }
      if (AUTHORITY_CHAIN[0]?.[0] !== AUTHORITY_ROOT) {
        return fail("authority_gap");
      }
      for (let i = 0; i < AUTHORITY_CHAIN.length; i++) {
        const [from, to] = AUTHORITY_CHAIN[i]!;
        if (from !== ARCHITECTURE_AUTHORITY_DOCS[i]) {
          return fail("authority_gap");
        }
        if (to !== ARCHITECTURE_AUTHORITY_DOCS[i + 1]) {
          return fail("authority_gap");
        }
      }
      return pass();
    }
    case "authority_docs_include_tip": {
      const tip = ARCHITECTURE_AUTHORITY_DOCS[ARCHITECTURE_AUTHORITY_DOCS.length - 1];
      return tip === "65_Architectural_Maintainability.md"
        ? pass()
        : fail("authority_gap");
    }
    case "complexity_zero_dup": {
      const report = buildArchitecturalComplexityReport();
      return report.after.duplicatedClampRoundSites === 0 &&
        report.after.duplicatedVarianceSites === 0 &&
        report.after.deprecatedPresentationAliases === 0
        ? pass()
        : fail("expected_mismatch");
    }
    case "storage_key_constants": {
      const ok = EXISTING_STORAGE_KEY_CONSTANTS.every(
        (key) => typeof key === "string" && key.startsWith("ws."),
      );
      return ok ? pass() : fail("schema_mismatch");
    }
    case "experience_barrel_exports": {
      for (const name of REQUIRED_PUBLIC_EXPORTS) {
        const value = (experienceBarrel as Record<string, unknown>)[name];
        if (typeof value !== "function") {
          return fail("api_missing");
        }
      }
      return pass();
    }
    case "presentation_double_resolve": {
      if (!store) {
        return skip("missing_store");
      }
      const a = resolvePresentationWithAnticipation(store);
      const b = resolvePresentationWithAnticipation(store);
      return JSON.stringify(a) === JSON.stringify(b)
        ? pass()
        : fail("nondeterministic");
    }
    case "empty_graph_integrity": {
      const graph = buildArchitectureGraph({
        engineeringRecords: [],
        proposals: [],
        opportunities: [],
        evidence: [],
      });
      return validateArchitectureIntegrity(graph).valid
        ? pass()
        : fail("integrity_invalid");
    }
    case "validate_composition_store": {
      if (!store) {
        return skip("missing_store");
      }
      const report = validateComposition(store);
      // Empty active set still yields a structured report — pass when not lineage-broken
      // for empty stores (governanceIntact true with no members).
      if (report.schemaVersion !== 1) {
        return fail("schema_mismatch");
      }
      return report.governanceIntact || report.composition.compositionOrder.length === 0
        ? pass()
        : fail("composition_failed");
    }
    case "store_architecture_integrity": {
      if (!store) {
        return skip("missing_store");
      }
      // Empty engineering/proposal sets still build a valid authority-only graph.
      return storeArchitectureIntegrityValid(store)
        ? pass()
        : fail("integrity_invalid");
    }
    case "certification_list_shape": {
      if (!store) {
        return skip("missing_store");
      }
      const certs = listCertifications(store);
      if (!Array.isArray(certs)) {
        return fail("schema_mismatch");
      }
      // Immutability: ids unique.
      const ids = certs.map((c) => c.certificationId);
      return new Set(ids).size === ids.length
        ? pass()
        : fail("schema_mismatch");
    }
    case "evidence_storage_key": {
      return EVIDENCE_STORAGE_KEY === "ws.dev.experience.evidence.v1"
        ? pass()
        : fail("schema_mismatch");
    }
    default:
      return fail("expected_mismatch");
  }
}

/**
 * Execute every invariant deterministically.
 * No partial certification: complete ≡ zero failures (skips allowed).
 */
export function runEngineeringCertification(
  store: ExperienceStoreAdapter | null = null,
  options: {
    now?: number;
    freezeDuration?: boolean;
    recordHistory?: boolean;
  } = {},
): EngineeringCertificationReport {
  const started = options.freezeDuration ? 0 : Date.now();
  const results: InvariantExecutionResult[] = [];

  for (const invariant of ENGINEERING_INVARIANTS) {
    const invStart = options.freezeDuration ? 0 : Date.now();
    const { outcome, cause } = executeInvariant(invariant, store);
    const invEnd = options.freezeDuration ? 0 : Date.now();
    results.push({
      invariantId: invariant.invariantId,
      outcome,
      cause: outcome === "passed" ? null : cause,
      owningSubsystem: invariant.owningSubsystem,
      authorityDocument: invariant.authorityDocument,
      durationMs: options.freezeDuration ? 0 : Math.max(0, invEnd - invStart),
    });
  }

  const passed = results
    .filter((r) => r.outcome === "passed")
    .map((r) => r.invariantId)
    .sort();
  const failed = results
    .filter((r) => r.outcome === "failed")
    .map((r) => r.invariantId)
    .sort();
  const skipped = results
    .filter((r) => r.outcome === "skipped")
    .map((r) => r.invariantId)
    .sort();

  const authorityReferences = [
    ...new Set(results.map((r) => r.authorityDocument)),
  ].sort();
  const subsystems = [
    ...new Set(results.map((r) => r.owningSubsystem)),
  ].sort() as EngineeringSubsystem[];

  const ended = options.freezeDuration ? 0 : Date.now();
  const certifiedAt = options.now ?? ended;
  const certificationId = contentAddressedId(
    "engcert",
    [
      String(certifiedAt),
      passed.join(","),
      failed.join(","),
      skipped.join(","),
    ].join("|"),
  );

  const report: EngineeringCertificationReport = {
    schemaVersion: 1,
    certificationId,
    certifiedAt,
    durationMs: options.freezeDuration ? 0 : Math.max(0, ended - started),
    passed,
    failed,
    skipped,
    results: results.sort((a, b) =>
      a.invariantId.localeCompare(b.invariantId),
    ),
    authorityReferences,
    subsystems,
    complete: failed.length === 0,
  };

  if (options.recordHistory !== false) {
    certificationHistory = [...certificationHistory, report].slice(-MAX_HISTORY);
  }

  return report;
}

/** Compare certified outcomes only — not implementation details. */
export function compareEngineeringCertifications(
  previous: EngineeringCertificationReport | null,
  current: EngineeringCertificationReport,
): EngineeringCertificationComparison {
  if (!previous) {
    return {
      schemaVersion: 1,
      newlyFailed: [...current.failed].sort(),
      newlyResolved: [],
      invariantsAdded: [...current.passed, ...current.failed, ...current.skipped]
        .sort(),
      invariantsRemoved: [],
    };
  }

  const prevFailed = new Set(previous.failed);
  const currFailed = new Set(current.failed);
  const prevAll = new Set([
    ...previous.passed,
    ...previous.failed,
    ...previous.skipped,
  ]);
  const currAll = new Set([
    ...current.passed,
    ...current.failed,
    ...current.skipped,
  ]);

  const newlyFailed = [...currFailed]
    .filter((id) => !prevFailed.has(id))
    .sort();
  const newlyResolved = [...prevFailed]
    .filter((id) => !currFailed.has(id))
    .sort();
  const invariantsAdded = [...currAll]
    .filter((id) => !prevAll.has(id))
    .sort();
  const invariantsRemoved = [...prevAll]
    .filter((id) => !currAll.has(id))
    .sort();

  return {
    schemaVersion: 1,
    newlyFailed,
    newlyResolved,
    invariantsAdded,
    invariantsRemoved,
  };
}

export function listEngineeringCertificationHistory(): EngineeringCertificationReport[] {
  return certificationHistory.map((r) => ({
    ...r,
    passed: [...r.passed],
    failed: [...r.failed],
    skipped: [...r.skipped],
    results: r.results.map((x) => ({ ...x })),
    authorityReferences: [...r.authorityReferences],
    subsystems: [...r.subsystems],
  }));
}

export function getLatestEngineeringCertification(): EngineeringCertificationReport | null {
  return tipOfHistory();
}

function tipOfHistory(): EngineeringCertificationReport | null {
  if (certificationHistory.length === 0) {
    return null;
  }
  return listEngineeringCertificationHistory()[certificationHistory.length - 1]!;
}

export function clearEngineeringCertificationHistory(): void {
  certificationHistory = [];
}

/** Subsystem health derived from the latest report (or a fresh run). */
export function deriveSubsystemHealth(
  report: EngineeringCertificationReport,
): Array<{
  subsystem: EngineeringSubsystem;
  passed: number;
  failed: number;
  skipped: number;
}> {
  const map = new Map<
    EngineeringSubsystem,
    { passed: number; failed: number; skipped: number }
  >();
  for (const result of report.results) {
    const entry = map.get(result.owningSubsystem) ?? {
      passed: 0,
      failed: 0,
      skipped: 0,
    };
    if (result.outcome === "passed") {
      entry.passed += 1;
    } else if (result.outcome === "failed") {
      entry.failed += 1;
    } else {
      entry.skipped += 1;
    }
    map.set(result.owningSubsystem, entry);
  }
  return [...map.entries()]
    .map(([subsystem, counts]) => ({ subsystem, ...counts }))
    .sort((a, b) => a.subsystem.localeCompare(b.subsystem));
}
