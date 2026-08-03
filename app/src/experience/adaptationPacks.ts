/**
 * Sprint 64 — Certified adaptation packs.
 * Curated, immutable collections of certified adaptations activated together.
 * No new adaptation primitives. No Runtime Core / navigation / persistence changes.
 */

import { fnv1a } from "../dev/devHash";
import { listEvidenceSnapshots } from "../dev/experienceEvidence";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  adaptationLineageRefsPresent,
  buildLineageIdCatalogs,
  clearJsonKey,
  loadJsonBundle,
  saveJsonBundle,
  stablePayloadHash,
  storeArchitectureIntegrityValid,
  tipOf,
} from "../dev/governancePrimitives";
import {
  getLatestCertification,
  listCertifications,
  type AdaptationCertification,
} from "./adaptationCertification";
import {
  analyzeAdaptationConflicts,
  composeAdaptations,
  type CompositionResult,
} from "./adaptationComposition";
import {
  activateAdaptation,
  deactivateAdaptation,
  getAdaptation,
  listAdaptations,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";

export const PACK_STORAGE_KEY = "ws.experience.adaptation.packs.v1";
export const MAX_PACKS = 40;

export type PackRolloutStatus = "certified";

export interface PackStabilitySummary {
  composedStabilityScore: number;
  adaptationCount: number;
}

export interface PackEvidenceSummary {
  evidenceSnapshotIds: string[];
  tipEvidenceId: string | null;
}

/** Derived collection of certified adaptations — ids only, no duplicated payloads. */
export interface WorkspaceAdaptationPack {
  schemaVersion: 1;
  packId: string;
  version: number;
  adaptationIds: string[];
  certificationIds: string[];
  compositionHash: string;
  stabilitySummary: PackStabilitySummary;
  evidenceSummary: PackEvidenceSummary;
  rolloutStatus: PackRolloutStatus;
}

export type PackCertificationFailureReason =
  | "empty_adaptation_set"
  | "adaptation_not_found"
  | "adaptation_not_certified"
  | "adaptation_not_passed"
  | "composition_invalid"
  | "unresolved_conflicts"
  | "governance_incomplete"
  | "integrity_invalid"
  | "duplicate_pack";

export type PackActivationError =
  | "not_found"
  | "not_certified"
  | "member_not_found"
  | "member_invalid_state"
  | "member_activate_failed"
  | "partial_prevented";

interface PackBundle {
  schemaVersion: 1;
  packs: WorkspaceAdaptationPack[];
  activePackId: string | null;
}

function emptyBundle(): PackBundle {
  return { schemaVersion: 1, packs: [], activePackId: null };
}

function isPackBundle(parsed: unknown): parsed is PackBundle {
  if (!parsed || typeof parsed !== "object") {
    return false;
  }
  const p = parsed as PackBundle;
  return p.schemaVersion === 1 && Array.isArray(p.packs);
}

function loadBundle(store: ExperienceStoreAdapter): PackBundle {
  const loaded = loadJsonBundle(
    store,
    PACK_STORAGE_KEY,
    emptyBundle,
    isPackBundle,
  );
  return {
    schemaVersion: 1,
    packs: loaded.packs.slice(-MAX_PACKS),
    activePackId: loaded.activePackId ?? null,
  };
}

function saveBundle(store: ExperienceStoreAdapter, bundle: PackBundle): void {
  saveJsonBundle(store, PACK_STORAGE_KEY, {
    schemaVersion: 1 as const,
    packs: bundle.packs.slice(-MAX_PACKS),
    activePackId: bundle.activePackId,
  });
}

export function listAdaptationPacks(
  store: ExperienceStoreAdapter,
): WorkspaceAdaptationPack[] {
  return loadBundle(store).packs.map((p) => ({
    ...p,
    adaptationIds: [...p.adaptationIds],
    certificationIds: [...p.certificationIds],
    stabilitySummary: { ...p.stabilitySummary },
    evidenceSummary: {
      ...p.evidenceSummary,
      evidenceSnapshotIds: [...p.evidenceSummary.evidenceSnapshotIds],
    },
  }));
}

export function getAdaptationPack(
  store: ExperienceStoreAdapter,
  packId: string,
  version?: number,
): WorkspaceAdaptationPack | null {
  const packs = listAdaptationPacks(store).filter((p) => p.packId === packId);
  if (packs.length === 0) {
    return null;
  }
  if (version !== undefined) {
    return packs.find((p) => p.version === version) ?? null;
  }
  return packs.sort((a, b) => b.version - a.version)[0] ?? null;
}

export function getActiveAdaptationPack(
  store: ExperienceStoreAdapter,
): WorkspaceAdaptationPack | null {
  const bundle = loadBundle(store);
  if (!bundle.activePackId) {
    return null;
  }
  // activePackId encodes packId@version
  const [packId, verStr] = bundle.activePackId.split("@");
  if (!packId) {
    return null;
  }
  const version = verStr ? Number(verStr) : undefined;
  return getAdaptationPack(store, packId, version);
}

export function clearAdaptationPackStore(store: ExperienceStoreAdapter): void {
  clearJsonKey(store, PACK_STORAGE_KEY);
}

function coveringCertifications(
  adaptationId: string,
  certifications: AdaptationCertification[],
): AdaptationCertification[] {
  return certifications.filter(
    (c) =>
      c.regressionStatus === "clear" &&
      c.governanceValid &&
      c.integrityValid &&
      c.adaptationIds.includes(adaptationId),
  );
}

/** Treat selected members as active for composition analysis only (not persisted). */
export function composePackCandidate(
  members: WorkspaceAdaptation[],
): CompositionResult {
  const asActive = members.map((a) => ({
    ...a,
    rolloutState: "active" as const,
    validation: {
      ...a.validation,
      validationResult: "passed" as const,
    },
  }));
  return composeAdaptations(asActive);
}

function governanceComplete(
  adaptation: WorkspaceAdaptation,
  store: ExperienceStoreAdapter,
): boolean {
  return adaptationLineageRefsPresent(
    adaptation,
    buildLineageIdCatalogs(store),
  );
}

export function packActivationReady(
  store: ExperienceStoreAdapter,
  pack: WorkspaceAdaptationPack,
): boolean {
  if (pack.rolloutStatus !== "certified") {
    return false;
  }
  for (const id of pack.adaptationIds) {
    const adaptation = getAdaptation(store, id);
    if (!adaptation) {
      return false;
    }
    if (adaptation.validation.validationResult !== "passed") {
      return false;
    }
    if (adaptation.rolloutState === "rolled_back") {
      return false;
    }
  }
  return true;
}

export interface PackCertificationResult {
  ok: boolean;
  pack: WorkspaceAdaptationPack | null;
  failureReasons: PackCertificationFailureReason[];
  composition: CompositionResult | null;
}

/**
 * Certify an adaptation pack. Writes only on full success (immutable record).
 */
export function certifyAdaptationPack(
  store: ExperienceStoreAdapter,
  options?: { adaptationIds?: string[]; now?: number },
): PackCertificationResult {
  const failureReasons: PackCertificationFailureReason[] = [];
  const certifications = listCertifications(store);
  const latest = getLatestCertification(store);

  let adaptationIds = options?.adaptationIds?.length
    ? [...options.adaptationIds].sort((a, b) => a.localeCompare(b))
    : latest
      ? [...latest.adaptationIds].sort((a, b) => a.localeCompare(b))
      : listAdaptations(store)
          .filter((a) => a.validation.validationResult === "passed")
          .map((a) => a.adaptationId)
          .sort((a, b) => a.localeCompare(b));

  adaptationIds = [...new Set(adaptationIds)].sort((a, b) =>
    a.localeCompare(b),
  );

  if (adaptationIds.length === 0) {
    return {
      ok: false,
      pack: null,
      failureReasons: ["empty_adaptation_set"],
      composition: null,
    };
  }

  const members: WorkspaceAdaptation[] = [];
  const certificationIds = new Set<string>();

  for (const id of adaptationIds) {
    const adaptation = getAdaptation(store, id);
    if (!adaptation) {
      failureReasons.push("adaptation_not_found");
      continue;
    }
    if (adaptation.validation.validationResult !== "passed") {
      failureReasons.push("adaptation_not_passed");
      continue;
    }
    const covering = coveringCertifications(id, certifications);
    if (covering.length === 0) {
      failureReasons.push("adaptation_not_certified");
      continue;
    }
    // Prefer latest covering certification.
    const chosen = covering[covering.length - 1]!;
    certificationIds.add(chosen.certificationId);
    if (!governanceComplete(adaptation, store)) {
      failureReasons.push("governance_incomplete");
      continue;
    }
    members.push(adaptation);
  }

  if (members.length !== adaptationIds.length) {
    return {
      ok: false,
      pack: null,
      failureReasons: [...new Set(failureReasons)].sort(),
      composition: null,
    };
  }

  const composition = composePackCandidate(members);
  const conflicts = analyzeAdaptationConflicts(
    members.map((a) => ({
      ...a,
      rolloutState: "active" as const,
      validation: { ...a.validation, validationResult: "passed" as const },
    })),
  );

  // Unresolved = any conflict missing a resolution strategy (should not occur),
  // or composition order empty when members exist.
  const unresolved = conflicts.conflicts.some((c) => !c.resolutionStrategy);
  if (unresolved) {
    failureReasons.push("unresolved_conflicts");
  }

  // Pack requires conflict-compatible OR all conflicts explicitly strategized.
  // Explicit strategies always present; additionally reject if strategies empty on conflicts.
  if (
    conflicts.conflicts.length > 0 &&
    conflicts.conflicts.some(
      (c) =>
        c.resolutionStrategy !== "merge_multiply" &&
        c.resolutionStrategy !== "priority_replace" &&
        c.resolutionStrategy !== "union_targets" &&
        c.resolutionStrategy !== "clamp_range",
    )
  ) {
    failureReasons.push("unresolved_conflicts");
  }

  if (composition.compositionOrder.length !== members.length) {
    failureReasons.push("composition_invalid");
  }

  if (!storeArchitectureIntegrityValid(store)) {
    failureReasons.push("integrity_invalid");
  }

  const uniqueReasons = [...new Set(failureReasons)].sort();
  if (uniqueReasons.length > 0) {
    return {
      ok: false,
      pack: null,
      failureReasons: uniqueReasons,
      composition,
    };
  }

  const compositionHash = stablePayloadHash([
    composition.compositionOrder.join(","),
    String(composition.presentation.spacingScale ?? 1),
    String(composition.presentation.emphasisScale ?? 1),
    String(composition.presentation.environmentalWeight ?? 1),
    String(composition.presentation.density ?? "null"),
    String(composition.presentation.motionProfile ?? "standard"),
    String(conflicts.conflicts.length),
  ]);

  const evidenceSnapshotIds = [
    ...new Set(
      [...certificationIds].flatMap((cid) => {
        const cert = certifications.find((c) => c.certificationId === cid);
        return cert ? [cert.evidenceSnapshotId] : [];
      }),
    ),
  ].sort();
  const tipEvidenceId =
    tipOf(listEvidenceSnapshots(store))?.evidenceId ??
    tipOf(evidenceSnapshotIds) ??
    null;

  const stabilityScores = [...certificationIds].map((cid) => {
    const cert = certifications.find((c) => c.certificationId === cid)!;
    return cert.composedStabilityScore;
  });
  const composedStabilityScore =
    stabilityScores.length === 0
      ? 0
      : Number(
          (
            stabilityScores.reduce((a, b) => a + b, 0) / stabilityScores.length
          ).toFixed(4),
        );

  const packId = `pack-${fnv1a(adaptationIds.join(","))}`;
  const bundle = loadBundle(store);
  const priorVersions = bundle.packs.filter((p) => p.packId === packId);
  const version =
    priorVersions.length === 0
      ? 1
      : Math.max(...priorVersions.map((p) => p.version)) + 1;

  // Identical content already certified (same composition hash at latest version).
  const latestSame = priorVersions.sort((a, b) => b.version - a.version)[0];
  if (latestSame && latestSame.compositionHash === compositionHash) {
    return {
      ok: false,
      pack: null,
      failureReasons: ["duplicate_pack"],
      composition,
    };
  }

  const pack: WorkspaceAdaptationPack = {
    schemaVersion: 1,
    packId,
    version,
    adaptationIds,
    certificationIds: [...certificationIds].sort(),
    compositionHash,
    stabilitySummary: {
      composedStabilityScore,
      adaptationCount: adaptationIds.length,
    },
    evidenceSummary: {
      evidenceSnapshotIds,
      tipEvidenceId,
    },
    rolloutStatus: "certified",
  };

  // Immutable append — never mutate prior pack versions.
  bundle.packs = [...bundle.packs, pack];
  saveBundle(store, bundle);

  return {
    ok: true,
    pack,
    failureReasons: [],
    composition,
  };
}

export interface PackActivationResult {
  ok: boolean;
  pack: WorkspaceAdaptationPack | null;
  activatedIds: string[];
  error: PackActivationError | null;
}

/**
 * Activate a certified pack via existing activateAdaptation for each member.
 * Manual only. No second activation mechanism.
 */
export function activateAdaptationPack(
  store: ExperienceStoreAdapter,
  packId: string,
  version?: number,
): PackActivationResult {
  const pack = getAdaptationPack(store, packId, version);
  if (!pack) {
    return { ok: false, pack: null, activatedIds: [], error: "not_found" };
  }
  if (pack.rolloutStatus !== "certified") {
    return { ok: false, pack, activatedIds: [], error: "not_certified" };
  }

  // Preflight — no partial activation.
  for (const id of pack.adaptationIds) {
    const adaptation = getAdaptation(store, id);
    if (!adaptation) {
      return {
        ok: false,
        pack,
        activatedIds: [],
        error: "member_not_found",
      };
    }
    if (adaptation.validation.validationResult !== "passed") {
      return {
        ok: false,
        pack,
        activatedIds: [],
        error: "member_invalid_state",
      };
    }
    if (adaptation.rolloutState === "rolled_back") {
      return {
        ok: false,
        pack,
        activatedIds: [],
        error: "member_invalid_state",
      };
    }
  }

  const packSet = new Set(pack.adaptationIds);
  // Deactivate actives outside the pack (still via existing deactivate).
  for (const adaptation of listAdaptations(store)) {
    if (
      adaptation.rolloutState === "active" &&
      !packSet.has(adaptation.adaptationId)
    ) {
      const deactivated = deactivateAdaptation(store, adaptation.adaptationId);
      if (!deactivated.ok) {
        return {
          ok: false,
          pack,
          activatedIds: [],
          error: "partial_prevented",
        };
      }
    }
  }

  const activatedIds: string[] = [];
  for (const id of pack.adaptationIds) {
    const adaptation = getAdaptation(store, id)!;
    if (adaptation.rolloutState === "active") {
      activatedIds.push(id);
      continue;
    }
    const result = activateAdaptation(store, id);
    if (!result.ok) {
      // Roll back members activated in this call.
      for (const activatedId of activatedIds) {
        if (packSet.has(activatedId)) {
          deactivateAdaptation(store, activatedId);
        }
      }
      return {
        ok: false,
        pack,
        activatedIds: [],
        error: "member_activate_failed",
      };
    }
    activatedIds.push(id);
  }

  const bundle = loadBundle(store);
  bundle.activePackId = `${pack.packId}@${pack.version}`;
  saveBundle(store, bundle);

  return {
    ok: true,
    pack,
    activatedIds: [...activatedIds].sort(),
    error: null,
  };
}

/**
 * Deactivate pack members via existing deactivateAdaptation; clear active pack pointer.
 */
export function deactivateAdaptationPack(
  store: ExperienceStoreAdapter,
  packId: string,
  version?: number,
): PackActivationResult {
  const pack = getAdaptationPack(store, packId, version);
  if (!pack) {
    return { ok: false, pack: null, activatedIds: [], error: "not_found" };
  }

  const deactivatedIds: string[] = [];
  for (const id of pack.adaptationIds) {
    const adaptation = getAdaptation(store, id);
    if (!adaptation) {
      continue;
    }
    if (adaptation.rolloutState === "active") {
      const result = deactivateAdaptation(store, id);
      if (!result.ok) {
        return {
          ok: false,
          pack,
          activatedIds: deactivatedIds,
          error: "member_invalid_state",
        };
      }
      deactivatedIds.push(id);
    }
  }

  const bundle = loadBundle(store);
  if (bundle.activePackId === `${pack.packId}@${pack.version}`) {
    bundle.activePackId = null;
    saveBundle(store, bundle);
  }

  return {
    ok: true,
    pack,
    activatedIds: deactivatedIds.sort(),
    error: null,
  };
}
