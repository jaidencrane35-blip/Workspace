/**
 * Sprint 67 — Workspace Memory Evolution.
 * Derived from certified adaptations / packs / evidence — not AI autonomy.
 * No new persistence. No Runtime Core / navigation / persistence schema changes.
 * Presentation influence only (spacing, density, emphasis, grouping, motion, environment).
 */

import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  contentAddressedId,
  tipOf,
} from "../dev/governancePrimitives";
import {
  listCertifications,
  type AdaptationCertification,
} from "./adaptationCertification";
import {
  composeAdaptations,
  validateComposition,
} from "./adaptationComposition";
import {
  getActiveAdaptationPack,
  listAdaptationPacks,
  type WorkspaceAdaptationPack,
} from "./adaptationPacks";
import {
  applyPresentationLayer,
  finalizeResolvedPresentation,
  identityPresentation,
  listAdaptations,
  resolvePresentationConfiguration,
  type AdaptationTargetComponent,
  type PresentationConfiguration,
  type ResolvedPresentation,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";

export type EvolutionValidationFailure =
  | "no_certified_adaptations"
  | "missing_certification"
  | "certification_regressed"
  | "missing_evidence"
  | "missing_architecture_snapshot"
  | "missing_replay"
  | "lineage_incomplete"
  | "composition_regressed"
  | "composition_unstable"
  | "member_not_active"
  | "member_not_passed";

export interface EvolutionEvidenceLineage {
  evidenceSnapshotIds: string[];
  tipEvidenceId: string | null;
}

export interface EvolutionCertificationLineage {
  certificationIds: string[];
  tipCertificationId: string | null;
}

/** Presentation fields only — never user data / navigation / Runtime Core. */
export interface EvolutionPresentationDelta {
  density: PresentationConfiguration["density"];
  spacingScale: number;
  emphasisScale: number;
  groupingTightness: number;
  motionProfile: PresentationConfiguration["motionProfile"];
  environmentalWeight: number;
}

export interface EvolutionValidation {
  valid: boolean;
  failureReasons: EvolutionValidationFailure[];
  compositionValidationResult: "passed" | "failed";
  composedStabilityScore: number | null;
  replaySessionIds: string[];
}

/**
 * Derived evolution epoch — not persisted.
 * Built entirely from existing runtime / pack / certification / evidence state.
 */
export interface WorkspaceMemoryEvolution {
  schemaVersion: 1;
  evolutionId: string;
  affectedWorkspaceRegions: AdaptationTargetComponent[];
  originatingAdaptationIds: string[];
  evidenceLineage: EvolutionEvidenceLineage;
  certificationLineage: EvolutionCertificationLineage;
  architectureSnapshotIds: string[];
  evolutionEpoch: number;
  presentationDelta: EvolutionPresentationDelta;
  /** True only when validation passes — invalid evolution stays inactive. */
  active: boolean;
  validation: EvolutionValidation;
  packId: string | null;
  packVersion: number | null;
}

function round4(n: number): number {
  return Number(n.toFixed(4));
}

function deltaFromResolved(
  resolved: ResolvedPresentation,
): EvolutionPresentationDelta {
  return {
    density: resolved.density ?? null,
    spacingScale: resolved.spacingScale ?? 1,
    emphasisScale: resolved.emphasisScale ?? 1,
    groupingTightness: resolved.groupingTightness ?? 0.5,
    motionProfile: resolved.motionProfile ?? "standard",
    environmentalWeight: resolved.environmentalWeight ?? 1,
  };
}

function regionsOf(
  adaptations: readonly WorkspaceAdaptation[],
): AdaptationTargetComponent[] {
  return [
    ...new Set(adaptations.flatMap((a) => a.targetComponents)),
  ].sort((a, b) => a.localeCompare(b)) as AdaptationTargetComponent[];
}

function coveringClearCerts(
  adaptationId: string,
  certifications: readonly AdaptationCertification[],
): AdaptationCertification[] {
  return certifications.filter(
    (c) =>
      c.regressionStatus === "clear" &&
      c.governanceValid &&
      c.integrityValid &&
      c.compositionValidationResult === "passed" &&
      c.adaptationIds.includes(adaptationId),
  );
}

/**
 * Select certified members for evolution:
 * prefer active pack members; else active adaptations that have clear certifications.
 */
export function selectEvolutionMembers(
  store: ExperienceStoreAdapter,
): {
  members: WorkspaceAdaptation[];
  pack: WorkspaceAdaptationPack | null;
  certifications: AdaptationCertification[];
} {
  const adaptations = listAdaptations(store);
  const certifications = listCertifications(store);
  const activePack = getActiveAdaptationPack(store);

  if (activePack) {
    const members = activePack.adaptationIds
      .map((id) => adaptations.find((a) => a.adaptationId === id))
      .filter((a): a is WorkspaceAdaptation => !!a)
      .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));
    const covering = activePack.certificationIds
      .map((id) => certifications.find((c) => c.certificationId === id))
      .filter((c): c is AdaptationCertification => !!c);
    return { members, pack: activePack, certifications: covering };
  }

  const members = adaptations
    .filter(
      (a) =>
        a.rolloutState === "active" &&
        a.validation.validationResult === "passed" &&
        coveringClearCerts(a.adaptationId, certifications).length > 0,
    )
    .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));

  const certIds = new Set<string>();
  const covering: AdaptationCertification[] = [];
  for (const m of members) {
    const certs = coveringClearCerts(m.adaptationId, certifications);
    const tip = tipOf(certs);
    if (tip && !certIds.has(tip.certificationId)) {
      certIds.add(tip.certificationId);
      covering.push(tip);
    }
  }
  covering.sort((a, b) => a.certifiedAt - b.certifiedAt);
  return { members, pack: null, certifications: covering };
}

function validateMembers(
  store: ExperienceStoreAdapter,
  members: WorkspaceAdaptation[],
  certifications: AdaptationCertification[],
): EvolutionValidation {
  const failureReasons: EvolutionValidationFailure[] = [];

  if (members.length === 0) {
    failureReasons.push("no_certified_adaptations");
  }

  if (certifications.length === 0 && members.length > 0) {
    failureReasons.push("missing_certification");
  }

  for (const cert of certifications) {
    if (cert.regressionStatus !== "clear") {
      failureReasons.push("certification_regressed");
    }
  }

  for (const member of members) {
    if (member.rolloutState !== "active") {
      failureReasons.push("member_not_active");
    }
    if (member.validation.validationResult !== "passed") {
      failureReasons.push("member_not_passed");
    }
    if (!member.evidenceSnapshotId) {
      failureReasons.push("missing_evidence");
    }
    if (!member.architectureSnapshotId) {
      failureReasons.push("missing_architecture_snapshot");
    }
    if (member.validation.replaySessionIds.length === 0) {
      failureReasons.push("missing_replay");
    }
    if (coveringClearCerts(member.adaptationId, listCertifications(store)).length === 0) {
      failureReasons.push("missing_certification");
    }
  }

  const composition = validateComposition(store);
  if (composition.validationResult !== "passed") {
    failureReasons.push("composition_regressed");
  }
  if (!composition.governanceIntact) {
    failureReasons.push("lineage_incomplete");
  }
  // Stability gate: only when longitudinal scores exist for the composed set.
  if (
    composition.stabilityScores.length > 0 &&
    composition.stabilityScores.every((s) => s.score > 0) &&
    composition.composedStabilityScore < 0.7
  ) {
    failureReasons.push("composition_unstable");
  }

  const unique = [...new Set(failureReasons)].sort() as EvolutionValidationFailure[];
  const replaySessionIds = [
    ...new Set(members.flatMap((m) => m.validation.replaySessionIds)),
  ].sort();

  return {
    valid: unique.length === 0,
    failureReasons: unique,
    compositionValidationResult: composition.validationResult,
    composedStabilityScore: composition.composedStabilityScore,
    replaySessionIds,
  };
}

function buildEvolutionFromMembers(
  store: ExperienceStoreAdapter,
  members: WorkspaceAdaptation[],
  certifications: AdaptationCertification[],
  pack: WorkspaceAdaptationPack | null,
  evolutionEpoch: number,
): WorkspaceMemoryEvolution {
  const validation = validateMembers(store, members, certifications);
  const asActive = members.map((a) => ({
    ...a,
    rolloutState: "active" as const,
    validation: { ...a.validation, validationResult: "passed" as const },
  }));
  const composed = resolvePresentationConfiguration(asActive);
  const presentationDelta = deltaFromResolved(composed);

  const evidenceSnapshotIds = [
    ...new Set([
      ...members.map((m) => m.evidenceSnapshotId),
      ...certifications.map((c) => c.evidenceSnapshotId),
    ]),
  ].sort();
  const architectureSnapshotIds = [
    ...new Set(members.map((m) => m.architectureSnapshotId)),
  ].sort();
  const certificationIds = [
    ...new Set(certifications.map((c) => c.certificationId)),
  ].sort();
  const originatingAdaptationIds = members
    .map((m) => m.adaptationId)
    .sort((a, b) => a.localeCompare(b));

  const evolutionId = contentAddressedId(
    "evo",
    [
      String(evolutionEpoch),
      pack ? `${pack.packId}@${pack.version}` : "nopack",
      originatingAdaptationIds.join(","),
      certificationIds.join(","),
      evidenceSnapshotIds.join(","),
    ].join("|"),
  );

  return {
    schemaVersion: 1,
    evolutionId,
    affectedWorkspaceRegions: regionsOf(members),
    originatingAdaptationIds,
    evidenceLineage: {
      evidenceSnapshotIds,
      tipEvidenceId: tipOf(evidenceSnapshotIds),
    },
    certificationLineage: {
      certificationIds,
      tipCertificationId: tipOf(
        [...certifications].sort((a, b) => a.certifiedAt - b.certifiedAt),
      )?.certificationId ?? tipOf(certificationIds),
    },
    architectureSnapshotIds,
    evolutionEpoch,
    presentationDelta,
    active: validation.valid,
    validation,
    packId: pack?.packId ?? null,
    packVersion: pack?.version ?? null,
  };
}

/**
 * Active evolution for the current runtime state (pack → certified actives).
 * Invalid evolution is returned with `active: false` (inactive — not applied).
 */
export function deriveActiveMemoryEvolution(
  store: ExperienceStoreAdapter,
): WorkspaceMemoryEvolution | null {
  const { members, pack, certifications } = selectEvolutionMembers(store);
  if (members.length === 0 && !pack) {
    return null;
  }
  const epoch =
    pack?.version ??
    Math.max(0, ...certifications.map((c) => c.certifiedAt), 0);
  return buildEvolutionFromMembers(
    store,
    members,
    certifications,
    pack,
    epoch,
  );
}

/**
 * Evolution history derived from certification epochs and pack versions.
 * Ephemeral — not persisted.
 */
export function listMemoryEvolutions(
  store: ExperienceStoreAdapter,
): WorkspaceMemoryEvolution[] {
  const certifications = [...listCertifications(store)].sort(
    (a, b) => a.certifiedAt - b.certifiedAt,
  );
  const packs = listAdaptationPacks(store);
  const adaptations = listAdaptations(store);
  const history: WorkspaceMemoryEvolution[] = [];

  for (let i = 0; i < certifications.length; i++) {
    const cert = certifications[i]!;
    const members = cert.adaptationIds
      .map((id) => adaptations.find((a) => a.adaptationId === id))
      .filter((a): a is WorkspaceAdaptation => !!a)
      .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));
    history.push(
      buildEvolutionFromMembers(store, members, [cert], null, i + 1),
    );
  }

  for (const pack of packs) {
    const members = pack.adaptationIds
      .map((id) => adaptations.find((a) => a.adaptationId === id))
      .filter((a): a is WorkspaceAdaptation => !!a)
      .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));
    const covering = pack.certificationIds
      .map((id) => certifications.find((c) => c.certificationId === id))
      .filter((c): c is AdaptationCertification => !!c);
    history.push(
      buildEvolutionFromMembers(
        store,
        members,
        covering,
        pack,
        1000 + pack.version,
      ),
    );
  }

  // Deterministic order: epoch then evolutionId.
  return history.sort((a, b) =>
    a.evolutionEpoch === b.evolutionEpoch
      ? a.evolutionId.localeCompare(b.evolutionId)
      : a.evolutionEpoch - b.evolutionEpoch,
  );
}

export function getActiveMemoryEvolution(
  store: ExperienceStoreAdapter,
): WorkspaceMemoryEvolution | null {
  const evolution = deriveActiveMemoryEvolution(store);
  if (!evolution || !evolution.active) {
    return null;
  }
  return evolution;
}

export function validateMemoryEvolution(
  store: ExperienceStoreAdapter,
  evolution: WorkspaceMemoryEvolution,
): EvolutionValidation {
  const adaptations = listAdaptations(store);
  const members = evolution.originatingAdaptationIds
    .map((id) => adaptations.find((a) => a.adaptationId === id))
    .filter((a): a is WorkspaceAdaptation => !!a);
  const certifications = listCertifications(store).filter((c) =>
    evolution.certificationLineage.certificationIds.includes(c.certificationId),
  );
  return validateMembers(store, members, certifications);
}

/**
 * Resolver ordering:
 * Runtime State → Active Adaptation Pack → WorkspaceMemoryEvolution → Presentation
 *
 * When evolution is inactive/missing, falls back to the active adaptation set
 * (pre-Sprint-67 behaviour).
 */
export function resolvePresentationFromRuntime(
  store: ExperienceStoreAdapter,
): ResolvedPresentation {
  const evolution = deriveActiveMemoryEvolution(store);
  if (evolution?.active) {
    return resolvePresentationConfiguration([], { evolution });
  }
  return resolvePresentationConfiguration(listAdaptations(store));
}

/** Replay helper — same inputs ⇒ same presentation + evolution id. */
export function replayMemoryEvolution(
  store: ExperienceStoreAdapter,
): {
  evolution: WorkspaceMemoryEvolution | null;
  presentation: ResolvedPresentation;
} {
  const evolution = deriveActiveMemoryEvolution(store);
  const presentation = resolvePresentationFromRuntime(store);
  return { evolution, presentation };
}

/** Apply evolution delta onto identity (presentation boundaries only). */
export function presentationFromEvolution(
  evolution: WorkspaceMemoryEvolution,
): ResolvedPresentation {
  if (!evolution.active) {
    return identityPresentation();
  }
  let resolved = applyPresentationLayer(
    identityPresentation(),
    evolution.presentationDelta,
  );
  resolved.appliedAdaptationIds = [...evolution.originatingAdaptationIds].sort(
    (a, b) => a.localeCompare(b),
  );
  return finalizeResolvedPresentation(resolved);
}

/** Compose check used by tests — touches composition without new persistence. */
export function evolutionCompositionFingerprint(
  store: ExperienceStoreAdapter,
): string {
  const composition = composeAdaptations(listAdaptations(store));
  return [
    composition.compositionOrder.join(","),
    round4(composition.presentation.spacingScale ?? 1),
    round4(composition.presentation.emphasisScale ?? 1),
  ].join("|");
}
