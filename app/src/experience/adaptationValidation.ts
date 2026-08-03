/**
 * Sprint 66 — Real-world adaptation validation.
 * Derived reports from existing evidence / adaptation / certification / pack stores.
 * No new persistence. No new governance / evidence / adaptation abstractions.
 * No Runtime Core / navigation / persistence schema changes.
 */

import {
  getReplayInvocationCount,
  listEvidenceSnapshots,
  type ExperienceEvidence,
} from "../dev/experienceEvidence";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import { tipOf } from "../dev/governancePrimitives";
import {
  listCertifications,
  type AdaptationCertification,
} from "./adaptationCertification";
import {
  getActiveAdaptationPack,
  listAdaptationPacks,
  type WorkspaceAdaptationPack,
} from "./adaptationPacks";
import { getProductionActivationRecord } from "./productionAdaptation";
import {
  listLongitudinalRecords,
  listStabilityReports,
} from "./longitudinalAdaptation";
import {
  listAdaptations,
  type AdaptationStabilityReport,
  type LongitudinalAdaptationRecord,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";
import { meanOf, round4 } from "./experienceMath";

export type StabilityTrend = "rising" | "stable" | "falling" | "unknown";

/** One row per locally available ExperienceEvidence snapshot. */
export interface EvidenceInventoryEntry {
  schemaVersion: 1;
  evidenceId: string;
  /** Append order in the evidence store (0-based). Temporal proxy when wall clock absent. */
  sequenceIndex: number;
  /**
   * Wall-clock ms when derivable from certification / production activation.
   * Evidence snapshots themselves carry no timestamp — null when underivable.
   */
  timestampMs: number | null;
  /** Session-derived interaction proxy (metrics.sessionCount). No user content. */
  interactionCount: number;
  /** Snapshot metric replayCount. */
  replayCount: number;
  /** Bundle-level replay invocation counter at harvest time (same for all rows). */
  bundleReplayInvocations: number;
  /** Adaptations currently active that reference this snapshot in lineage/timeline. */
  activeAdaptationIds: string[];
  /** Latest certification that freezes this evidenceSnapshotId, if any. */
  certificationId: string | null;
  fingerprint: string;
  tag: string;
}

export interface EvidenceInventory {
  schemaVersion: 1;
  harvestedAt: number;
  snapshotCount: number;
  entries: EvidenceInventoryEntry[];
}

export interface AdaptationPerformanceEntry {
  schemaVersion: 1;
  subjectKind: "adaptation" | "pack";
  subjectId: string;
  /** Pack version when subjectKind === "pack"; else null. */
  packVersion: number | null;
  adaptationIds: string[];
  activationCount: number;
  observationCount: number;
  stabilityScore: number | null;
  stabilityTrend: StabilityTrend;
  /** improvementConsistency from stability report (0–1); null if unavailable. */
  improvementPersistence: number | null;
  regressionFrequency: number | null;
  confidenceEvolution: number[];
  evidenceGrowth: number;
  rolloutSuccess: boolean;
  certificationIds: string[];
}

export interface AdaptationPerformanceReport {
  schemaVersion: 1;
  generatedAt: number;
  evidenceSnapshotCount: number;
  adaptations: AdaptationPerformanceEntry[];
  packs: AdaptationPerformanceEntry[];
}

export interface PackEffectivenessCohort {
  kind: "single_adaptations" | "certified_packs";
  subjectCount: number;
  meanStabilityScore: number | null;
  meanRegressionFrequency: number | null;
  totalEvidenceGrowth: number;
  rolloutSuccessCount: number;
  rolloutSuccessRate: number | null;
}

export interface PackEffectivenessReport {
  schemaVersion: 1;
  generatedAt: number;
  singles: PackEffectivenessCohort;
  packs: PackEffectivenessCohort;
  /** Deterministic deltas: pack − single (null when either side lacks samples). */
  delta: {
    meanStabilityScore: number | null;
    meanRegressionFrequency: number | null;
    totalEvidenceGrowth: number;
    rolloutSuccessRate: number | null;
  };
}

export interface CertificationLongevityEntry {
  certificationId: string;
  certifiedAt: number;
  previousCertificationId: string | null;
  /** ms to next certification; null if tip. */
  longevityMs: number | null;
  adaptationSetHash: string;
  regressionStatus: AdaptationCertification["regressionStatus"];
  evidenceSnapshotId: string;
}

export interface CertificationLongevityReport {
  schemaVersion: 1;
  entries: CertificationLongevityEntry[];
  meanLongevityMs: number | null;
}

export interface LongitudinalTrendPoint {
  evidenceId: string;
  sequenceIndex: number;
  sessionCount: number;
  meanFrictionScore: number;
  medianTimeToConfidenceMs: number;
  replayCount: number;
}

export interface LongitudinalTrendReport {
  schemaVersion: 1;
  points: LongitudinalTrendPoint[];
}

function certificationTimestampForEvidence(
  evidenceId: string,
  certifications: readonly AdaptationCertification[],
): { timestampMs: number; certificationId: string } | null {
  const matches = certifications
    .filter((c) => c.evidenceSnapshotId === evidenceId)
    .sort((a, b) => b.certifiedAt - a.certifiedAt);
  const tip = tipOf(matches);
  if (!tip) {
    return null;
  }
  return { timestampMs: tip.certifiedAt, certificationId: tip.certificationId };
}

function activeAdaptationIdsForEvidence(
  evidenceId: string,
  adaptations: readonly WorkspaceAdaptation[],
  longitudinal: readonly LongitudinalAdaptationRecord[],
  stability: readonly AdaptationStabilityReport[],
): string[] {
  const ids = new Set<string>();
  for (const adaptation of adaptations) {
    if (adaptation.rolloutState !== "active") {
      continue;
    }
    if (adaptation.evidenceSnapshotId === evidenceId) {
      ids.add(adaptation.adaptationId);
      continue;
    }
    if (adaptation.validation.baselineEvidenceId === evidenceId) {
      ids.add(adaptation.adaptationId);
      continue;
    }
    const record = longitudinal.find(
      (r) => r.adaptationId === adaptation.adaptationId,
    );
    if (
      record &&
      (record.baselineEvidenceId === evidenceId ||
        record.latestEvidenceId === evidenceId ||
        record.intermediateEvidenceIds.includes(evidenceId))
    ) {
      ids.add(adaptation.adaptationId);
      continue;
    }
    const report = stability.find(
      (r) => r.adaptationId === adaptation.adaptationId,
    );
    if (report?.evidenceTimeline.includes(evidenceId)) {
      ids.add(adaptation.adaptationId);
    }
  }
  return [...ids].sort();
}

/**
 * PHASE 1 — Deterministic inventory of every local ExperienceEvidence snapshot.
 * Does not inspect user content.
 */
export function buildEvidenceInventory(
  store: ExperienceStoreAdapter,
  options: { now?: number } = {},
): EvidenceInventory {
  const snapshots = listEvidenceSnapshots(store);
  const certifications = listCertifications(store);
  const adaptations = listAdaptations(store);
  const longitudinal = listLongitudinalRecords(store);
  const stability = listStabilityReports(store);
  const production = getProductionActivationRecord(store);
  const bundleReplayInvocations = getReplayInvocationCount(store);
  const now = options.now ?? 0;

  const entries: EvidenceInventoryEntry[] = snapshots.map((snapshot, index) => {
    const fromCert = certificationTimestampForEvidence(
      snapshot.evidenceId,
      certifications,
    );
    let timestampMs = fromCert?.timestampMs ?? null;
    if (
      timestampMs === null &&
      production &&
      (production.preEvidenceId === snapshot.evidenceId ||
        production.postEvidenceId === snapshot.evidenceId) &&
      typeof production.activatedAt === "number"
    ) {
      timestampMs = production.activatedAt;
    }

    return {
      schemaVersion: 1,
      evidenceId: snapshot.evidenceId,
      sequenceIndex: index,
      timestampMs,
      interactionCount: snapshot.metrics.sessionCount,
      replayCount: snapshot.metrics.replayCount,
      bundleReplayInvocations,
      activeAdaptationIds: activeAdaptationIdsForEvidence(
        snapshot.evidenceId,
        adaptations,
        longitudinal,
        stability,
      ),
      certificationId: fromCert?.certificationId ?? null,
      fingerprint: snapshot.fingerprint,
      tag: snapshot.tag,
    };
  });

  return {
    schemaVersion: 1,
    harvestedAt: now,
    snapshotCount: entries.length,
    entries,
  };
}

function evidenceGrowthForAdaptation(
  adaptationId: string,
  longitudinal: readonly LongitudinalAdaptationRecord[],
  stability: readonly AdaptationStabilityReport[],
): number {
  const report = stability.find((r) => r.adaptationId === adaptationId);
  if (report) {
    return report.evidenceTimeline.length;
  }
  const record = longitudinal.find((r) => r.adaptationId === adaptationId);
  if (!record) {
    return 0;
  }
  return (
    1 +
    record.intermediateEvidenceIds.length +
    (record.latestEvidenceId !== record.baselineEvidenceId ? 1 : 0)
  );
}

function activationCountForAdaptation(
  adaptation: WorkspaceAdaptation,
  productionAdaptationId: string | null,
): number {
  let count = 0;
  if (adaptation.rolloutState === "active") {
    count += 1;
  }
  if (
    productionAdaptationId &&
    productionAdaptationId === adaptation.adaptationId
  ) {
    // Production activation is a distinct governed activation event.
    // Count it only when not already counted as currently active, or always
    // as +1 historical — sprint asks activation count; use 1 if ever activated
    // via production OR currently active, maxing at reflecting both signals
    // without inventing history: active + production match → up to 2 when
    // production activated and still active (same activation). Cap as:
    // if production match, at least 1; if active, at least 1; if both, 1.
    count = Math.max(count, 1);
  }
  return count;
}

function performanceFromAdaptation(
  adaptation: WorkspaceAdaptation,
  longitudinal: readonly LongitudinalAdaptationRecord[],
  stability: readonly AdaptationStabilityReport[],
  certifications: readonly AdaptationCertification[],
  productionAdaptationId: string | null,
): AdaptationPerformanceEntry {
  const record = longitudinal.find(
    (r) => r.adaptationId === adaptation.adaptationId,
  );
  const report = stability.find(
    (r) => r.adaptationId === adaptation.adaptationId,
  );
  const certIds = certifications
    .filter((c) => c.adaptationIds.includes(adaptation.adaptationId))
    .map((c) => c.certificationId)
    .sort();

  const rolloutSuccess =
    adaptation.rolloutState === "active" ||
    adaptation.rolloutState === "rollout_candidate" ||
    report?.rolloutDisposition === "rollout_candidate";

  return {
    schemaVersion: 1,
    subjectKind: "adaptation",
    subjectId: adaptation.adaptationId,
    packVersion: null,
    adaptationIds: [adaptation.adaptationId],
    activationCount: activationCountForAdaptation(
      adaptation,
      productionAdaptationId,
    ),
    observationCount: record?.observationCount ?? 0,
    stabilityScore: report?.stabilityScore ?? record?.stabilityScore ?? null,
    stabilityTrend: report?.confidenceTrend ?? "unknown",
    improvementPersistence: report?.improvementConsistency ?? null,
    regressionFrequency:
      report?.regressionFrequency ??
      (record
        ? round4(
            record.observationCount > 0
              ? record.regressionCount / record.observationCount
              : 0,
          )
        : null),
    confidenceEvolution: report ? [...report.confidenceEvolution] : [],
    evidenceGrowth: evidenceGrowthForAdaptation(
      adaptation.adaptationId,
      longitudinal,
      stability,
    ),
    rolloutSuccess: Boolean(rolloutSuccess),
    certificationIds: certIds,
  };
}

function performanceFromPack(
  pack: WorkspaceAdaptationPack,
  adaptations: readonly WorkspaceAdaptation[],
  longitudinal: readonly LongitudinalAdaptationRecord[],
  stability: readonly AdaptationStabilityReport[],
  activePack: WorkspaceAdaptationPack | null,
): AdaptationPerformanceEntry {
  const members = pack.adaptationIds
    .map((id) => adaptations.find((a) => a.adaptationId === id))
    .filter((a): a is WorkspaceAdaptation => !!a);

  const memberReports = members
    .map((m) => stability.find((r) => r.adaptationId === m.adaptationId))
    .filter((r): r is AdaptationStabilityReport => !!r);

  const observationCount = members.reduce((sum, m) => {
    const record = longitudinal.find((r) => r.adaptationId === m.adaptationId);
    return sum + (record?.observationCount ?? 0);
  }, 0);

  const stabilityScores = memberReports.map((r) => r.stabilityScore);
  if (stabilityScores.length === 0 && pack.stabilitySummary) {
    stabilityScores.push(pack.stabilitySummary.composedStabilityScore);
  }

  const regressionFreqs = memberReports.map((r) => r.regressionFrequency);
  const persistences = memberReports.map((r) => r.improvementConsistency);
  const trends = memberReports.map((r) => r.confidenceTrend);
  let stabilityTrend: StabilityTrend = "unknown";
  if (trends.includes("falling")) {
    stabilityTrend = "falling";
  } else if (trends.includes("rising") && trends.every((t) => t !== "falling")) {
    stabilityTrend = "rising";
  } else if (trends.length > 0 && trends.every((t) => t === "stable")) {
    stabilityTrend = "stable";
  } else if (trends.length > 0) {
    stabilityTrend = "stable";
  }

  const confidenceEvolution =
    tipOf(memberReports)?.confidenceEvolution.slice() ?? [];

  const evidenceIds = new Set<string>();
  for (const id of pack.evidenceSummary.evidenceSnapshotIds) {
    evidenceIds.add(id);
  }
  for (const m of members) {
    const report = stability.find((r) => r.adaptationId === m.adaptationId);
    for (const eid of report?.evidenceTimeline ?? []) {
      evidenceIds.add(eid);
    }
  }

  const isActive =
    !!activePack &&
    activePack.packId === pack.packId &&
    activePack.version === pack.version;

  const anyMemberActive = members.some((m) => m.rolloutState === "active");

  return {
    schemaVersion: 1,
    subjectKind: "pack",
    subjectId: pack.packId,
    packVersion: pack.version,
    adaptationIds: [...pack.adaptationIds].sort(),
    activationCount: isActive || anyMemberActive ? 1 : 0,
    observationCount,
    stabilityScore: meanOf(stabilityScores),
    stabilityTrend,
    improvementPersistence: meanOf(persistences),
    regressionFrequency: meanOf(regressionFreqs),
    confidenceEvolution,
    evidenceGrowth: evidenceIds.size,
    rolloutSuccess: pack.rolloutStatus === "certified" && (isActive || anyMemberActive),
    certificationIds: [...pack.certificationIds].sort(),
  };
}

/**
 * PHASE 2 — Performance for every active adaptation and every certified pack.
 * Derived only from existing stores / evidence.
 */
export function buildAdaptationPerformanceReport(
  store: ExperienceStoreAdapter,
  options: { now?: number } = {},
): AdaptationPerformanceReport {
  const adaptations = listAdaptations(store);
  const longitudinal = listLongitudinalRecords(store);
  const stability = listStabilityReports(store);
  const certifications = listCertifications(store);
  const packs = listAdaptationPacks(store);
  const activePack = getActiveAdaptationPack(store);
  const production = getProductionActivationRecord(store);
  const productionId =
    production?.outcome === "activated" ? production.adaptationId : null;
  const evidenceSnapshotCount = listEvidenceSnapshots(store).length;
  const now = options.now ?? 0;

  const activeAdaptations = adaptations
    .filter((a) => a.rolloutState === "active")
    .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));

  const adaptationEntries = activeAdaptations.map((a) =>
    performanceFromAdaptation(
      a,
      longitudinal,
      stability,
      certifications,
      productionId,
    ),
  );

  const packEntries = [...packs]
    .sort((a, b) =>
      a.packId === b.packId
        ? a.version - b.version
        : a.packId.localeCompare(b.packId),
    )
    .map((pack) =>
      performanceFromPack(
        pack,
        adaptations,
        longitudinal,
        stability,
        activePack,
      ),
    );

  return {
    schemaVersion: 1,
    generatedAt: now,
    evidenceSnapshotCount,
    adaptations: adaptationEntries,
    packs: packEntries,
  };
}

function cohortFromEntries(
  kind: PackEffectivenessCohort["kind"],
  entries: readonly AdaptationPerformanceEntry[],
): PackEffectivenessCohort {
  const stabilities = entries
    .map((e) => e.stabilityScore)
    .filter((n): n is number => n !== null);
  const regressions = entries
    .map((e) => e.regressionFrequency)
    .filter((n): n is number => n !== null);
  const rolloutSuccessCount = entries.filter((e) => e.rolloutSuccess).length;
  const totalEvidenceGrowth = entries.reduce(
    (sum, e) => sum + e.evidenceGrowth,
    0,
  );
  return {
    kind,
    subjectCount: entries.length,
    meanStabilityScore: meanOf(stabilities),
    meanRegressionFrequency: meanOf(regressions),
    totalEvidenceGrowth,
    rolloutSuccessCount,
    rolloutSuccessRate:
      entries.length === 0
        ? null
        : round4(rolloutSuccessCount / entries.length),
  };
}

function deltaNullable(
  pack: number | null,
  single: number | null,
): number | null {
  if (pack === null || single === null) {
    return null;
  }
  return round4(pack - single);
}

/**
 * PHASE 3 — Compare single adaptations vs certified packs on objective measures only.
 */
export function buildPackEffectivenessReport(
  store: ExperienceStoreAdapter,
  options: { now?: number } = {},
): PackEffectivenessReport {
  const performance = buildAdaptationPerformanceReport(store, options);
  // Singles = per-adaptation performance rows; packs = pack aggregate rows.
  // Overlap is expected when active adaptations are pack members — comparison
  // is individual pathway vs composed pack pathway, not disjoint sets.
  const singleCohort = cohortFromEntries(
    "single_adaptations",
    performance.adaptations,
  );
  const packCohort = cohortFromEntries("certified_packs", performance.packs);

  return {
    schemaVersion: 1,
    generatedAt: options.now ?? 0,
    singles: singleCohort,
    packs: packCohort,
    delta: {
      meanStabilityScore: deltaNullable(
        packCohort.meanStabilityScore,
        singleCohort.meanStabilityScore,
      ),
      meanRegressionFrequency: deltaNullable(
        packCohort.meanRegressionFrequency,
        singleCohort.meanRegressionFrequency,
      ),
      totalEvidenceGrowth:
        packCohort.totalEvidenceGrowth - singleCohort.totalEvidenceGrowth,
      rolloutSuccessRate: deltaNullable(
        packCohort.rolloutSuccessRate,
        singleCohort.rolloutSuccessRate,
      ),
    },
  };
}

/** Certification longevity along the append-only certification chain. */
export function buildCertificationLongevityReport(
  store: ExperienceStoreAdapter,
): CertificationLongevityReport {
  const certifications = [...listCertifications(store)].sort(
    (a, b) => a.certifiedAt - b.certifiedAt,
  );
  const entries: CertificationLongevityEntry[] = certifications.map(
    (cert, index) => {
      const next = certifications[index + 1];
      return {
        certificationId: cert.certificationId,
        certifiedAt: cert.certifiedAt,
        previousCertificationId: cert.previousCertificationId,
        longevityMs: next ? next.certifiedAt - cert.certifiedAt : null,
        adaptationSetHash: cert.adaptationSetHash,
        regressionStatus: cert.regressionStatus,
        evidenceSnapshotId: cert.evidenceSnapshotId,
      };
    },
  );
  const longevities = entries
    .map((e) => e.longevityMs)
    .filter((n): n is number => n !== null);
  return {
    schemaVersion: 1,
    entries,
    meanLongevityMs: meanOf(longevities),
  };
}

/** Longitudinal metric trend across evidence snapshots (append order). */
export function buildLongitudinalTrendReport(
  store: ExperienceStoreAdapter,
): LongitudinalTrendReport {
  const snapshots = listEvidenceSnapshots(store);
  return {
    schemaVersion: 1,
    points: snapshots.map((s: ExperienceEvidence, index) => ({
      evidenceId: s.evidenceId,
      sequenceIndex: index,
      sessionCount: s.metrics.sessionCount,
      meanFrictionScore: s.metrics.meanFrictionScore,
      medianTimeToConfidenceMs: s.metrics.medianTimeToConfidenceMs,
      replayCount: s.metrics.replayCount,
    })),
  };
}

/** Convenience aggregate for DEV overlay / tests. */
export function buildRealWorldValidationBundle(
  store: ExperienceStoreAdapter,
  options: { now?: number } = {},
) {
  return {
    inventory: buildEvidenceInventory(store, options),
    performance: buildAdaptationPerformanceReport(store, options),
    packEffectiveness: buildPackEffectivenessReport(store, options),
    certificationLongevity: buildCertificationLongevityReport(store),
    longitudinalTrend: buildLongitudinalTrendReport(store),
  };
}
