/**
 * Sprint 63 — Adaptation regression certification.
 * Immutable certifications proving composed adaptations remain regression-free.
 * Verification only. No Runtime Core / navigation / persistence / new primitives.
 */

import {
  buildArchitectureGraph,
  listArchitectureSnapshots,
  validateArchitectureIntegrity,
} from "../dev/architecturalIntegrity";
import { fnv1a } from "../dev/devHash";
import {
  COMPARABLE_METRICS,
  compareEvidence,
  listEvidenceSnapshots,
  type ExperienceEvidence,
  type ExperienceEvidenceMetrics,
} from "../dev/experienceEvidence";
import { listEngineeringRecords } from "../dev/engineeringGovernance";
import { detectOpportunities } from "../dev/experienceImprovement";
import { listProposals } from "../dev/experienceGovernance";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  composeAdaptations,
  validateComposition,
  type CompositionValidationReport,
} from "./adaptationComposition";
import { listAdaptations } from "./workspaceAdaptation";

export const CERTIFICATION_STORAGE_KEY =
  "ws.experience.adaptation.certification.v1";
export const MAX_CERTIFICATIONS = 40;

export type CertificationRegressionStatus = "clear" | "regressed";

export type CertificationFailureReason =
  | "governance_invalid"
  | "integrity_invalid"
  | "composition_validation_failed"
  | "evidence_incomplete"
  | "regression_detected"
  | "stability_regressed";

export type CertificationRegressionCause =
  | "metric_regression"
  | "evidence_regression"
  | "governance_regression"
  | "integrity_regression"
  | "stability_regression";

export interface CertificationEvidenceLineage {
  evidenceSnapshotId: string;
  fingerprint: string;
  sourceSessionIds: string[];
}

export interface AdaptationCertification {
  schemaVersion: 1;
  certificationId: string;
  adaptationSetHash: string;
  certifiedAt: number;
  adaptationIds: string[];
  evidenceSnapshotId: string;
  engineeringChangeIds: string[];
  architectureSnapshotId: string;
  compositionValidationResult: "passed" | "failed";
  /** Metric values frozen from the evidence snapshot at certification time. */
  certifiedMetrics: Partial<Record<keyof ExperienceEvidenceMetrics, number>>;
  regressionStatus: CertificationRegressionStatus;
  integrityValid: boolean;
  governanceValid: boolean;
  composedStabilityScore: number;
  previousCertificationId: string | null;
  evidenceLineage: CertificationEvidenceLineage;
}

export interface CertificationRegression {
  cause: CertificationRegressionCause;
  adaptationIds: string[];
  metrics: Array<keyof ExperienceEvidenceMetrics>;
  previousEvidenceId: string;
  currentEvidenceId: string;
  /** Allowlisted field / code — not free text. */
  field: string;
}

export interface CertificationComparison {
  schemaVersion: 1;
  previousCertificationId: string;
  currentCertificationId: string;
  regressions: CertificationRegression[];
  regressionFree: boolean;
}

export interface CertificationGateResult {
  ok: boolean;
  certification: AdaptationCertification | null;
  failureReasons: CertificationFailureReason[];
  comparison: CertificationComparison | null;
  compositionValidation: CompositionValidationReport;
}

interface CertificationBundle {
  schemaVersion: 1;
  /** Append-only immutable history (oldest → newest). */
  certifications: AdaptationCertification[];
}

function emptyBundle(): CertificationBundle {
  return { schemaVersion: 1, certifications: [] };
}

function loadBundle(store: ExperienceStoreAdapter): CertificationBundle {
  const raw = store.getItem(CERTIFICATION_STORAGE_KEY);
  if (!raw) {
    return emptyBundle();
  }
  try {
    const parsed = JSON.parse(raw) as CertificationBundle;
    if (parsed?.schemaVersion !== 1 || !Array.isArray(parsed.certifications)) {
      return emptyBundle();
    }
    return {
      schemaVersion: 1,
      certifications: parsed.certifications.slice(-MAX_CERTIFICATIONS),
    };
  } catch {
    return emptyBundle();
  }
}

function saveBundle(
  store: ExperienceStoreAdapter,
  bundle: CertificationBundle,
): void {
  store.setItem(
    CERTIFICATION_STORAGE_KEY,
    JSON.stringify({
      schemaVersion: 1,
      certifications: bundle.certifications.slice(-MAX_CERTIFICATIONS),
    }),
  );
}

export function listCertifications(
  store: ExperienceStoreAdapter,
): AdaptationCertification[] {
  return loadBundle(store).certifications.map((c) => ({
    ...c,
    adaptationIds: [...c.adaptationIds],
    engineeringChangeIds: [...c.engineeringChangeIds],
    certifiedMetrics: { ...c.certifiedMetrics },
    evidenceLineage: {
      ...c.evidenceLineage,
      sourceSessionIds: [...c.evidenceLineage.sourceSessionIds],
    },
  }));
}

export function getLatestCertification(
  store: ExperienceStoreAdapter,
): AdaptationCertification | null {
  const list = listCertifications(store);
  return list.length > 0 ? list[list.length - 1]! : null;
}

export function getCertification(
  store: ExperienceStoreAdapter,
  certificationId: string,
): AdaptationCertification | null {
  return (
    listCertifications(store).find(
      (c) => c.certificationId === certificationId,
    ) ?? null
  );
}

export function clearCertificationStore(store: ExperienceStoreAdapter): void {
  store.removeItem(CERTIFICATION_STORAGE_KEY);
}

function tipEvidence(
  evidence: ExperienceEvidence[],
): ExperienceEvidence | null {
  if (evidence.length === 0) {
    return null;
  }
  return evidence[evidence.length - 1]!;
}

function extractCertifiedMetrics(
  evidence: ExperienceEvidence,
): Partial<Record<keyof ExperienceEvidenceMetrics, number>> {
  const metrics: Partial<Record<keyof ExperienceEvidenceMetrics, number>> = {};
  for (const spec of COMPARABLE_METRICS) {
    metrics[spec.key] = Number(evidence.metrics[spec.key]);
  }
  return metrics;
}

function integrityValid(store: ExperienceStoreAdapter): boolean {
  const graph = buildArchitectureGraph({
    engineeringRecords: listEngineeringRecords(store),
    proposals: listProposals(store),
    opportunities: detectOpportunities(listEvidenceSnapshots(store)),
    evidence: listEvidenceSnapshots(store),
  });
  return validateArchitectureIntegrity(graph, {
    engineeringRecords: listEngineeringRecords(store),
    proposals: listProposals(store),
  }).valid;
}

function resolveArchitectureSnapshotId(
  store: ExperienceStoreAdapter,
  compositionArchIds: string[],
): string | null {
  const snapshots = listArchitectureSnapshots(store);
  for (const id of [...compositionArchIds].sort()) {
    const snap = snapshots.find((s) => s.snapshotId === id);
    if (snap?.integrity.valid) {
      return id;
    }
  }
  const valid = snapshots.filter((s) => s.integrity.valid);
  return valid.length > 0 ? valid[valid.length - 1]!.snapshotId : null;
}

export function hashAdaptationSet(
  adaptationIds: string[],
  evidenceSnapshotId: string,
  architectureSnapshotId: string,
  composedStabilityScore: number,
): string {
  const payload = [
    [...adaptationIds].sort().join(","),
    evidenceSnapshotId,
    architectureSnapshotId,
    String(composedStabilityScore),
  ].join("|");
  return fnv1a(payload);
}

/**
 * Deterministic comparison of two certifications.
 * No heuristic scoring — explicit metric / governance / integrity / stability diffs.
 */
export function compareCertifications(
  previous: AdaptationCertification,
  current: AdaptationCertification,
  previousEvidence: ExperienceEvidence | null,
  currentEvidence: ExperienceEvidence | null,
): CertificationComparison {
  const regressions: CertificationRegression[] = [];
  const adaptationIds = [
    ...new Set([...previous.adaptationIds, ...current.adaptationIds]),
  ].sort();

  if (previousEvidence && currentEvidence) {
    const cmp = compareEvidence(previousEvidence, currentEvidence);
    for (const metric of cmp.metrics) {
      if (metric.verdict === "regressed") {
        regressions.push({
          cause: "metric_regression",
          adaptationIds,
          metrics: [metric.key],
          previousEvidenceId: previous.evidenceSnapshotId,
          currentEvidenceId: current.evidenceSnapshotId,
          field: metric.key,
        });
      }
    }
  } else if (previous.evidenceSnapshotId && !currentEvidence) {
    regressions.push({
      cause: "evidence_regression",
      adaptationIds,
      metrics: [],
      previousEvidenceId: previous.evidenceSnapshotId,
      currentEvidenceId: current.evidenceSnapshotId,
      field: "evidence_missing",
    });
  }

  if (previous.governanceValid && !current.governanceValid) {
    regressions.push({
      cause: "governance_regression",
      adaptationIds,
      metrics: [],
      previousEvidenceId: previous.evidenceSnapshotId,
      currentEvidenceId: current.evidenceSnapshotId,
      field: "governanceValid",
    });
  }

  if (previous.integrityValid && !current.integrityValid) {
    regressions.push({
      cause: "integrity_regression",
      adaptationIds,
      metrics: [],
      previousEvidenceId: previous.evidenceSnapshotId,
      currentEvidenceId: current.evidenceSnapshotId,
      field: "integrityValid",
    });
  }

  if (
    current.composedStabilityScore + 1e-9 <
    previous.composedStabilityScore
  ) {
    regressions.push({
      cause: "stability_regression",
      adaptationIds,
      metrics: [],
      previousEvidenceId: previous.evidenceSnapshotId,
      currentEvidenceId: current.evidenceSnapshotId,
      field: "composedStabilityScore",
    });
  }

  // Also compare frozen certified metrics when evidence objects unavailable.
  if (!previousEvidence || !currentEvidence) {
    for (const spec of COMPARABLE_METRICS) {
      const prev = previous.certifiedMetrics[spec.key];
      const curr = current.certifiedMetrics[spec.key];
      if (prev === undefined || curr === undefined) {
        continue;
      }
      const delta = curr - prev;
      const regressed =
        spec.direction === "lower_better"
          ? delta > spec.epsilon
          : delta < -spec.epsilon;
      if (regressed) {
        const already = regressions.some(
          (r) =>
            r.cause === "metric_regression" && r.field === spec.key,
        );
        if (!already) {
          regressions.push({
            cause: "metric_regression",
            adaptationIds,
            metrics: [spec.key],
            previousEvidenceId: previous.evidenceSnapshotId,
            currentEvidenceId: current.evidenceSnapshotId,
            field: spec.key,
          });
        }
      }
    }
  }

  regressions.sort((a, b) => {
    const c = a.cause.localeCompare(b.cause);
    if (c !== 0) return c;
    return a.field.localeCompare(b.field);
  });

  return {
    schemaVersion: 1,
    previousCertificationId: previous.certificationId,
    currentCertificationId: current.certificationId,
    regressions,
    regressionFree: regressions.length === 0,
  };
}

function buildCandidateCertification(
  store: ExperienceStoreAdapter,
  options: { now: number },
  compositionValidation: CompositionValidationReport,
  evidence: ExperienceEvidence,
  architectureSnapshotId: string,
  previous: AdaptationCertification | null,
): AdaptationCertification {
  const composition = compositionValidation.composition;
  const adaptationIds = [...composition.compositionOrder];
  const engineeringChangeIds = [
    ...composition.lineage.engineeringChangeIds,
  ].sort();
  const certifiedMetrics = extractCertifiedMetrics(evidence);
  const adaptationSetHash = hashAdaptationSet(
    adaptationIds,
    evidence.evidenceId,
    architectureSnapshotId,
    compositionValidation.composedStabilityScore,
  );
  const certificationId = `acert-${fnv1a(
    `${adaptationSetHash}|${options.now}|${previous?.certificationId ?? "none"}`,
  )}`;

  return {
    schemaVersion: 1,
    certificationId,
    adaptationSetHash,
    certifiedAt: options.now,
    adaptationIds,
    evidenceSnapshotId: evidence.evidenceId,
    engineeringChangeIds,
    architectureSnapshotId,
    compositionValidationResult: compositionValidation.validationResult,
    certifiedMetrics,
    regressionStatus: "clear",
    integrityValid: integrityValid(store),
    governanceValid: compositionValidation.governanceIntact,
    composedStabilityScore: compositionValidation.composedStabilityScore,
    previousCertificationId: previous?.certificationId ?? null,
    evidenceLineage: {
      evidenceSnapshotId: evidence.evidenceId,
      fingerprint: evidence.fingerprint,
      sourceSessionIds: [...evidence.sourceSessionIds].sort(),
    },
  };
}

/**
 * Certify the current composed adaptation set.
 * Succeeds only when governance, integrity, composition validation,
 * evidence completeness, and regression-free vs previous certification.
 * On success appends an immutable record. On failure writes nothing.
 */
export function certifyAdaptationSet(
  store: ExperienceStoreAdapter,
  options?: { now?: number },
): CertificationGateResult {
  const now = options?.now ?? 0;
  const compositionValidation = validateComposition(store);
  const failureReasons: CertificationFailureReason[] = [];
  const evidenceList = listEvidenceSnapshots(store);
  const evidence = tipEvidence(evidenceList);
  const previous = getLatestCertification(store);

  if (compositionValidation.validationResult !== "passed") {
    failureReasons.push("composition_validation_failed");
  }
  if (!compositionValidation.governanceIntact) {
    failureReasons.push("governance_invalid");
  }
  if (!evidence) {
    failureReasons.push("evidence_incomplete");
  }
  const integOk = integrityValid(store);
  if (!integOk) {
    failureReasons.push("integrity_invalid");
  }

  const architectureSnapshotId = evidence
    ? resolveArchitectureSnapshotId(
        store,
        compositionValidation.composition.lineage.architectureSnapshotIds,
      )
    : null;
  if (!architectureSnapshotId) {
    if (!failureReasons.includes("evidence_incomplete")) {
      failureReasons.push("governance_invalid");
    }
  }

  // Build candidate (not persisted) for comparison even when failing — ids needed.
  let comparison: CertificationComparison | null = null;
  let candidate: AdaptationCertification | null = null;

  if (evidence && architectureSnapshotId) {
    candidate = buildCandidateCertification(
      store,
      { now },
      compositionValidation,
      evidence,
      architectureSnapshotId,
      previous,
    );
    candidate = {
      ...candidate,
      integrityValid: integOk,
      governanceValid: compositionValidation.governanceIntact,
      compositionValidationResult: compositionValidation.validationResult,
    };

    if (previous) {
      const prevEvidence =
        evidenceList.find((e) => e.evidenceId === previous.evidenceSnapshotId) ??
        null;
      comparison = compareCertifications(
        previous,
        candidate,
        prevEvidence,
        evidence,
      );
      if (!comparison.regressionFree) {
        failureReasons.push("regression_detected");
        if (
          comparison.regressions.some((r) => r.cause === "stability_regression")
        ) {
          failureReasons.push("stability_regressed");
        }
        candidate = {
          ...candidate,
          regressionStatus: "regressed",
        };
      }
    }
  }

  const uniqueReasons = [...new Set(failureReasons)].sort();
  if (uniqueReasons.length > 0 || !candidate) {
    return {
      ok: false,
      certification: null,
      failureReasons: uniqueReasons,
      comparison,
      compositionValidation,
    };
  }

  // Immutable append — never mutate prior certifications.
  const bundle = loadBundle(store);
  const frozen: AdaptationCertification = {
    ...candidate,
    adaptationIds: [...candidate.adaptationIds],
    engineeringChangeIds: [...candidate.engineeringChangeIds],
    certifiedMetrics: { ...candidate.certifiedMetrics },
    evidenceLineage: {
      ...candidate.evidenceLineage,
      sourceSessionIds: [...candidate.evidenceLineage.sourceSessionIds],
    },
    regressionStatus: "clear",
  };
  // Guard: reject if id already exists (immutability).
  if (bundle.certifications.some((c) => c.certificationId === frozen.certificationId)) {
    return {
      ok: false,
      certification: null,
      failureReasons: ["regression_detected"],
      comparison,
      compositionValidation,
    };
  }
  bundle.certifications = [...bundle.certifications, frozen];
  saveBundle(store, bundle);

  return {
    ok: true,
    certification: frozen,
    failureReasons: [],
    comparison,
    compositionValidation,
  };
}

/** Compare latest stored certification against the prior one in history. */
export function compareLatestCertifications(
  store: ExperienceStoreAdapter,
): CertificationComparison | null {
  const list = listCertifications(store);
  if (list.length < 2) {
    return null;
  }
  const previous = list[list.length - 2]!;
  const current = list[list.length - 1]!;
  const evidence = listEvidenceSnapshots(store);
  return compareCertifications(
    previous,
    current,
    evidence.find((e) => e.evidenceId === previous.evidenceSnapshotId) ?? null,
    evidence.find((e) => e.evidenceId === current.evidenceSnapshotId) ?? null,
  );
}

/** Touch listAdaptations so certification stays coupled to live adaptation state. */
export function currentAdaptationSetIds(
  store: ExperienceStoreAdapter,
): string[] {
  return composeAdaptations(listAdaptations(store)).compositionOrder;
}
