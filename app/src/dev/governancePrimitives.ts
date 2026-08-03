/**
 * Sprint 65 — Shared governance infrastructure primitives.
 * Internal only. Preserves storage keys/schemas and public APIs at call sites.
 */

import {
  buildArchitectureGraph,
  listArchitectureSnapshots,
  validateArchitectureIntegrity,
  type ArchitectureSnapshot,
} from "./architecturalIntegrity";
import { fnv1a } from "./devHash";
import { listEvidenceSnapshots } from "./experienceEvidence";
import { listEngineeringRecords } from "./engineeringGovernance";
import { detectOpportunities } from "./experienceImprovement";
import { listProposals } from "./experienceGovernance";
import type { ExperienceStoreAdapter } from "./experienceStore";

export { fnv1a } from "./devHash";
export type { ExperienceStoreAdapter } from "./experienceStore";
export { memoryStore, browserStore } from "./experienceStore";
export {
  clearJsonKey,
  loadJsonBundle,
  loadJsonBundleOrNull,
  saveJsonBundle,
} from "./governanceStore";

/** Allowlisted governance / adaptation opaque ids. */
export const GOVERNANCE_ID_RE = /^[a-z0-9_.:-]{1,96}$/i;
export const GOVERNANCE_LABEL_RE = /^[a-z0-9_.:-]{1,64}$/i;

export function tipOf<T>(items: readonly T[]): T | null {
  if (items.length === 0) {
    return null;
  }
  return items[items.length - 1]!;
}

export function stablePayloadHash(parts: readonly string[]): string {
  return fnv1a(parts.join("|"));
}

export function contentAddressedId(prefix: string, payload: string): string {
  return `${prefix}-${fnv1a(payload)}`;
}

/** Build + validate architecture integrity for the current governance store. */
export function storeArchitectureIntegrityValid(
  store: ExperienceStoreAdapter,
): boolean {
  const engineeringRecords = listEngineeringRecords(store);
  const proposals = listProposals(store);
  const evidence = listEvidenceSnapshots(store);
  const graph = buildArchitectureGraph({
    engineeringRecords,
    proposals,
    opportunities: detectOpportunities(evidence),
    evidence,
  });
  return validateArchitectureIntegrity(graph, {
    engineeringRecords,
    proposals,
  }).valid;
}

export function tipEvidenceId(store: ExperienceStoreAdapter): string | null {
  const tip = tipOf(listEvidenceSnapshots(store));
  return tip?.evidenceId ?? null;
}

export function latestValidArchitectureSnapshotId(
  store: ExperienceStoreAdapter,
  preferredIds: readonly string[] = [],
): string | null {
  const snapshots = listArchitectureSnapshots(store);
  for (const id of preferredIds) {
    const snap = snapshots.find((s) => s.snapshotId === id);
    if (snap?.integrity.valid) {
      return id;
    }
  }
  const valid = snapshots.filter((s) => s.integrity.valid);
  return tipOf(valid)?.snapshotId ?? null;
}

export interface AdaptationLineageRefFields {
  proposalId: string;
  engineeringChangeId: string;
  architectureSnapshotId: string;
  validation: { replaySessionIds: readonly string[] };
}

export interface LineageIdCatalogs {
  proposalIds: ReadonlySet<string>;
  engineeringIds: ReadonlySet<string>;
  architectureIds: ReadonlySet<string>;
}

/** Shared boolean lineage presence check (proposal/eng/arch/replay). */
export function adaptationLineageRefsPresent(
  adaptation: AdaptationLineageRefFields,
  catalogs: LineageIdCatalogs,
): boolean {
  return (
    catalogs.proposalIds.has(adaptation.proposalId) &&
    catalogs.engineeringIds.has(adaptation.engineeringChangeId) &&
    catalogs.architectureIds.has(adaptation.architectureSnapshotId) &&
    adaptation.validation.replaySessionIds.length > 0
  );
}

export function buildLineageIdCatalogs(
  store: ExperienceStoreAdapter,
): LineageIdCatalogs {
  return {
    proposalIds: new Set(listProposals(store).map((p) => p.proposalId)),
    engineeringIds: new Set(
      listEngineeringRecords(store).map((r) => r.changeId),
    ),
    architectureIds: new Set(
      listArchitectureSnapshots(store).map((s) => s.snapshotId),
    ),
  };
}

/** Shallow record clones for list APIs (deterministic copy). */
export function cloneRecords<T extends object>(items: readonly T[]): T[] {
  return items.map((item) => ({ ...item }));
}

export function architectureSnapshotById(
  store: ExperienceStoreAdapter,
  snapshotId: string,
): ArchitectureSnapshot | null {
  return (
    listArchitectureSnapshots(store).find((s) => s.snapshotId === snapshotId) ??
    null
  );
}
