/**
 * Sprint 60 — Adaptation operations.
 * Deterministic catalog, batch validation, and operational health.
 * Derived views only — no duplicated governance storage.
 * No Runtime Core / navigation / persistence / production behaviour changes.
 */

import {
  listEvidenceSnapshots,
  type ExperienceEvidence,
} from "../dev/experienceEvidence";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  listArchitectureSnapshots,
  type ArchitectureSnapshot,
} from "../dev/architecturalIntegrity";
import { tipOf } from "../dev/governancePrimitives";
import {
  listAdaptations,
  loadAdaptationBundle,
  saveAdaptationBundle,
  validateAdaptationEvidence,
  type AdaptationRolloutState,
  type AdaptationValidationResult,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";
import {
  LONGITUDINAL_MIN_OBSERVATIONS,
  getLongitudinalRecord,
  isRolloutReady,
  listLongitudinalRecords,
  listStabilityReports,
  type LongitudinalAdaptationRecord,
} from "./longitudinalAdaptation";

export type CatalogDisposition =
  | "hold"
  | "rollout_candidate"
  | "reject"
  | "none";

export type BatchValidationOutcome =
  | "validated"
  | "failed"
  | "unchanged"
  | "skipped";

export type BatchValidationCause =
  | "missing_baseline"
  | "missing_after_evidence"
  | "rolled_back"
  | "improvement_unmet"
  | "rollback_triggered"
  | "identical_result";

export interface AdaptationCatalogEntry {
  schemaVersion: 1;
  adaptationId: string;
  proposalId: string;
  engineeringChangeId: string;
  lifecycleState: AdaptationRolloutState;
  rolloutReady: boolean;
  currentDisposition: CatalogDisposition;
  stabilityScore: number;
  evidenceCount: number;
  latestEvidenceSnapshotId: string | null;
  latestArchitectureSnapshotId: string | null;
}

export interface AdaptationCatalog {
  schemaVersion: 1;
  entries: AdaptationCatalogEntry[];
  lifecycleDistribution: Record<AdaptationRolloutState, number>;
}

export interface BatchValidationItem {
  adaptationId: string;
  outcome: BatchValidationOutcome;
  cause: BatchValidationCause | null;
  previousValidation: AdaptationValidationResult;
  nextValidation: AdaptationValidationResult;
}

export interface BatchValidationReport {
  schemaVersion: 1;
  validated: string[];
  failed: string[];
  unchanged: string[];
  skipped: Array<{ adaptationId: string; cause: BatchValidationCause }>;
  items: BatchValidationItem[];
}

export interface AdaptationOperationalHealth {
  schemaVersion: 1;
  adaptationCount: number;
  rolloutBacklog: number;
  validationBacklog: number;
  staleEvidence: number;
  expiredLongitudinalSamples: number;
  orphanRolloutCandidates: number;
  inactiveValidatedAdaptations: number;
  lifecycleDistribution: Record<AdaptationRolloutState, number>;
}

function emptyDistribution(): Record<AdaptationRolloutState, number> {
  return {
    inactive: 0,
    candidate: 0,
    rollout_candidate: 0,
    active: 0,
    rolled_back: 0,
  };
}

function lifecycleDistribution(
  adaptations: WorkspaceAdaptation[],
): Record<AdaptationRolloutState, number> {
  const dist = emptyDistribution();
  for (const a of adaptations) {
    dist[a.rolloutState] += 1;
  }
  return dist;
}

function latestArchitectureId(
  snapshots: ArchitectureSnapshot[],
  preferredId: string | null,
): string | null {
  if (preferredId && snapshots.some((s) => s.snapshotId === preferredId)) {
    return preferredId;
  }
  const valid = snapshots.filter((s) => s.integrity.valid);
  if (valid.length === 0) {
    return snapshots.length > 0
      ? snapshots[snapshots.length - 1]!.snapshotId
      : null;
  }
  return valid[valid.length - 1]!.snapshotId;
}

function tipEvidenceId(evidence: ExperienceEvidence[]): string | null {
  return tipOf(evidence)?.evidenceId ?? null;
}

/**
 * Derived catalog — identifiers only, no duplicated governance payloads.
 */
export function buildAdaptationCatalog(
  store: ExperienceStoreAdapter,
): AdaptationCatalog {
  const adaptations = listAdaptations(store);
  const longitudinal = listLongitudinalRecords(store);
  const reports = listStabilityReports(store);
  const evidence = listEvidenceSnapshots(store);
  const architecture = listArchitectureSnapshots(store);
  const longById = new Map(longitudinal.map((r) => [r.adaptationId, r]));
  const reportById = new Map(reports.map((r) => [r.adaptationId, r]));
  const tipArch = latestArchitectureId(architecture, null);

  const entries: AdaptationCatalogEntry[] = adaptations
    .map((adaptation) => {
      const record = longById.get(adaptation.adaptationId);
      const report = reportById.get(adaptation.adaptationId);
      const disposition: CatalogDisposition = report
        ? report.rolloutDisposition
        : "none";
      const latestEvidenceSnapshotId =
        record?.latestEvidenceId ??
        adaptation.evidenceSnapshotId ??
        tipEvidenceId(evidence);
      return {
        schemaVersion: 1 as const,
        adaptationId: adaptation.adaptationId,
        proposalId: adaptation.proposalId,
        engineeringChangeId: adaptation.engineeringChangeId,
        lifecycleState: adaptation.rolloutState,
        rolloutReady: report
          ? isRolloutReady(adaptation, report)
          : false,
        currentDisposition: disposition,
        stabilityScore: record?.stabilityScore ?? report?.stabilityScore ?? 0,
        evidenceCount: record?.observationCount ?? 0,
        latestEvidenceSnapshotId,
        latestArchitectureSnapshotId: latestArchitectureId(
          architecture,
          adaptation.architectureSnapshotId,
        ) ?? tipArch,
      };
    })
    .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));

  return {
    schemaVersion: 1,
    entries,
    lifecycleDistribution: lifecycleDistribution(adaptations),
  };
}

function resolveBaseline(
  adaptation: WorkspaceAdaptation,
  evidenceById: Map<string, ExperienceEvidence>,
  evidence: ExperienceEvidence[],
): ExperienceEvidence | null {
  const fromValidation = evidenceById.get(
    adaptation.validation.baselineEvidenceId,
  );
  if (fromValidation) {
    return fromValidation;
  }
  const fromAdapt = evidenceById.get(adaptation.evidenceSnapshotId);
  if (fromAdapt) {
    return fromAdapt;
  }
  return evidence[0] ?? null;
}

function resolveAfter(
  baseline: ExperienceEvidence,
  evidenceById: Map<string, ExperienceEvidence>,
  evidence: ExperienceEvidence[],
  record: LongitudinalAdaptationRecord | null,
): ExperienceEvidence | null {
  if (record?.latestEvidenceId) {
    const latest = evidenceById.get(record.latestEvidenceId);
    if (latest && latest.evidenceId !== baseline.evidenceId) {
      return latest;
    }
  }
  const tip = tipEvidenceId(evidence);
  if (tip && tip !== baseline.evidenceId) {
    return evidenceById.get(tip) ?? null;
  }
  // Prefer any evidence other than baseline (deterministic: sorted id).
  const others = evidence
    .filter((e) => e.evidenceId !== baseline.evidenceId)
    .sort((a, b) => a.evidenceId.localeCompare(b.evidenceId));
  return others[others.length - 1] ?? null;
}

/**
 * Validate many adaptations. Computes all outcomes first, then writes once.
 * No partial mutations — either the batch write commits or nothing changes.
 */
export function runBatchValidation(
  store: ExperienceStoreAdapter,
  options?: { adaptationIds?: string[] },
): BatchValidationReport {
  const adaptations = listAdaptations(store);
  const selected = options?.adaptationIds?.length
    ? adaptations.filter((a) =>
        options.adaptationIds!.includes(a.adaptationId),
      )
    : adaptations;
  const evidence = listEvidenceSnapshots(store);
  const evidenceById = new Map(evidence.map((e) => [e.evidenceId, e]));

  const items: BatchValidationItem[] = [];
  const pendingWrites: WorkspaceAdaptation[] = [];

  for (const adaptation of selected.sort((a, b) =>
    a.adaptationId.localeCompare(b.adaptationId),
  )) {
    const previousValidation = adaptation.validation.validationResult;

    if (adaptation.rolloutState === "rolled_back") {
      items.push({
        adaptationId: adaptation.adaptationId,
        outcome: "skipped",
        cause: "rolled_back",
        previousValidation,
        nextValidation: previousValidation,
      });
      continue;
    }

    const baseline = resolveBaseline(adaptation, evidenceById, evidence);
    if (!baseline) {
      items.push({
        adaptationId: adaptation.adaptationId,
        outcome: "skipped",
        cause: "missing_baseline",
        previousValidation,
        nextValidation: previousValidation,
      });
      continue;
    }

    const record = getLongitudinalRecord(store, adaptation.adaptationId);
    const after = resolveAfter(baseline, evidenceById, evidence, record);
    if (!after) {
      items.push({
        adaptationId: adaptation.adaptationId,
        outcome: "skipped",
        cause: "missing_after_evidence",
        previousValidation,
        nextValidation: previousValidation,
      });
      continue;
    }

    const result = validateAdaptationEvidence(adaptation, baseline, after);
    const nextValidation = result.validationResult;

    if (
      nextValidation === previousValidation &&
      result.adaptation.rolloutState === adaptation.rolloutState
    ) {
      items.push({
        adaptationId: adaptation.adaptationId,
        outcome: "unchanged",
        cause: "identical_result",
        previousValidation,
        nextValidation,
      });
      continue;
    }

    if (nextValidation === "passed") {
      items.push({
        adaptationId: adaptation.adaptationId,
        outcome: "validated",
        cause: null,
        previousValidation,
        nextValidation,
      });
      pendingWrites.push(result.adaptation);
      continue;
    }

    items.push({
      adaptationId: adaptation.adaptationId,
      outcome: "failed",
      cause: result.rollback ? "rollback_triggered" : "improvement_unmet",
      previousValidation,
      nextValidation,
    });
    pendingWrites.push(result.adaptation);
  }

  // Single atomic write for all mutations.
  if (pendingWrites.length > 0) {
    const bundle = loadAdaptationBundle(store);
    const byId = new Map(pendingWrites.map((a) => [a.adaptationId, a]));
    bundle.adaptations = bundle.adaptations.map(
      (a) => byId.get(a.adaptationId) ?? a,
    );
    // Include any new ids (should not happen for validation-only).
    for (const write of pendingWrites) {
      if (!bundle.adaptations.some((a) => a.adaptationId === write.adaptationId)) {
        bundle.adaptations.push(write);
      }
    }
    saveAdaptationBundle(store, bundle);
  }

  const validated = items
    .filter((i) => i.outcome === "validated")
    .map((i) => i.adaptationId);
  const failed = items
    .filter((i) => i.outcome === "failed")
    .map((i) => i.adaptationId);
  const unchanged = items
    .filter((i) => i.outcome === "unchanged")
    .map((i) => i.adaptationId);
  const skipped = items
    .filter((i) => i.outcome === "skipped")
    .map((i) => ({
      adaptationId: i.adaptationId,
      cause: i.cause!,
    }));

  return {
    schemaVersion: 1,
    validated,
    failed,
    unchanged,
    skipped,
    items,
  };
}

function isOrphanRolloutCandidate(
  adaptation: WorkspaceAdaptation,
  architecture: ArchitectureSnapshot[],
): boolean {
  if (adaptation.rolloutState !== "rollout_candidate") {
    return false;
  }
  if (
    !adaptation.engineeringChangeId ||
    !adaptation.proposalId ||
    !adaptation.architectureSnapshotId ||
    adaptation.validation.replaySessionIds.length === 0
  ) {
    return true;
  }
  const snap = architecture.find(
    (s) => s.snapshotId === adaptation.architectureSnapshotId,
  );
  return !snap || !snap.integrity.valid;
}

function hasExpiredLongitudinal(
  record: LongitudinalAdaptationRecord | undefined,
  evidenceById: Map<string, ExperienceEvidence>,
  adaptation: WorkspaceAdaptation,
): boolean {
  if (!record) {
    return false;
  }
  const ids = [
    record.baselineEvidenceId,
    ...record.intermediateEvidenceIds,
    record.latestEvidenceId,
  ].filter(Boolean);
  const missing = ids.some((id) => !evidenceById.has(id));
  if (missing) {
    return true;
  }
  // Incomplete series after validation passed.
  return (
    adaptation.validation.validationResult === "passed" &&
    record.observationCount > 0 &&
    record.observationCount < LONGITUDINAL_MIN_OBSERVATIONS
  );
}

function hasStaleEvidence(
  record: LongitudinalAdaptationRecord | undefined,
  tipId: string | null,
): boolean {
  if (!record || !tipId) {
    return false;
  }
  return record.latestEvidenceId !== tipId;
}

/**
 * Operational health — counts only, derived from existing artefacts.
 */
export function deriveOperationalHealth(
  store: ExperienceStoreAdapter,
): AdaptationOperationalHealth {
  const adaptations = listAdaptations(store);
  const longitudinal = listLongitudinalRecords(store);
  const reports = listStabilityReports(store);
  const evidence = listEvidenceSnapshots(store);
  const architecture = listArchitectureSnapshots(store);
  const longById = new Map(longitudinal.map((r) => [r.adaptationId, r]));
  const reportById = new Map(reports.map((r) => [r.adaptationId, r]));
  const evidenceById = new Map(evidence.map((e) => [e.evidenceId, e]));
  const tipId = tipEvidenceId(evidence);

  let rolloutBacklog = 0;
  let validationBacklog = 0;
  let staleEvidence = 0;
  let expiredLongitudinalSamples = 0;
  let orphanRolloutCandidates = 0;
  let inactiveValidatedAdaptations = 0;

  for (const adaptation of adaptations) {
    const record = longById.get(adaptation.adaptationId);
    const report = reportById.get(adaptation.adaptationId);

    if (adaptation.validation.validationResult === "pending") {
      validationBacklog += 1;
    }

    if (
      adaptation.validation.validationResult === "passed" &&
      adaptation.rolloutState === "inactive"
    ) {
      inactiveValidatedAdaptations += 1;
    }

    if (adaptation.rolloutState === "rollout_candidate") {
      rolloutBacklog += 1;
    } else if (
      adaptation.rolloutState === "candidate" &&
      report &&
      isRolloutReady(adaptation, report)
    ) {
      rolloutBacklog += 1;
    }

    if (hasStaleEvidence(record, tipId)) {
      staleEvidence += 1;
    }
    if (hasExpiredLongitudinal(record, evidenceById, adaptation)) {
      expiredLongitudinalSamples += 1;
    }
    if (isOrphanRolloutCandidate(adaptation, architecture)) {
      orphanRolloutCandidates += 1;
    }
  }

  return {
    schemaVersion: 1,
    adaptationCount: adaptations.length,
    rolloutBacklog,
    validationBacklog,
    staleEvidence,
    expiredLongitudinalSamples,
    orphanRolloutCandidates,
    inactiveValidatedAdaptations,
    lifecycleDistribution: lifecycleDistribution(adaptations),
  };
}

/** Convenience: catalog entry lookup. */
export function getCatalogEntry(
  store: ExperienceStoreAdapter,
  adaptationId: string,
): AdaptationCatalogEntry | null {
  return (
    buildAdaptationCatalog(store).entries.find(
      (e) => e.adaptationId === adaptationId,
    ) ?? null
  );
}
