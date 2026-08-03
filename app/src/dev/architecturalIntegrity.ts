/**
 * Architectural Integrity — canonical ArchitectureGraph + deterministic validation.
 * Development tooling only. No production Experience / Runtime Core changes.
 * No inferred links. No AI autonomy.
 */

import { fnv1a } from "./devHash";
import type { ExperienceEvidence } from "./experienceEvidence";
import type { ExperienceChangeProposal } from "./experienceGovernance";
import type { ExperienceOpportunity } from "./experienceImprovement";
import {
  ARCHITECTURE_AUTHORITY_DOCS,
  type ArchitectureAuthorityDoc,
  type EngineeringChangeRecord,
  isArchitectureAuthorityDoc,
} from "./engineeringGovernance";
import {
  browserStore,
  memoryStore,
  type ExperienceStoreAdapter,
} from "./experienceStore";

export const ARCHITECTURE_INTEGRITY_STORAGE_KEY =
  "ws.dev.experience.architecture.v1";
export const MAX_ARCHITECTURE_SNAPSHOTS = 40;

/** Explicit ordered authority chain — declared pairs only (not inferred). */
export const AUTHORITY_CHAIN: readonly [
  ArchitectureAuthorityDoc,
  ArchitectureAuthorityDoc,
][] = [
  ["40_Experience_Refoundation.md", "41_Perceptual_Convergence.md"],
  ["41_Perceptual_Convergence.md", "42_Experience_Validation.md"],
  ["42_Experience_Validation.md", "43_Experience_Evidence_Model.md"],
  ["43_Experience_Evidence_Model.md", "44_Experience_Improvement_Model.md"],
  ["44_Experience_Improvement_Model.md", "45_Experience_Change_Governance.md"],
  ["45_Experience_Change_Governance.md", "46_Engineering_Governance.md"],
  ["46_Engineering_Governance.md", "47_Architectural_Integrity.md"],
  ["47_Architectural_Integrity.md", "48_Adaptive_Workspace.md"],
  ["48_Adaptive_Workspace.md", "49_Adaptation_Experiments.md"],
  ["49_Adaptation_Experiments.md", "50_Longitudinal_Adaptation_Validation.md"],
  ["50_Longitudinal_Adaptation_Validation.md", "51_Adaptation_Operations.md"],
  ["51_Adaptation_Operations.md", "52_First_Production_Adaptation.md"],
  ["52_First_Production_Adaptation.md", "53_Adaptation_Composition.md"],
];

export const AUTHORITY_ROOT: ArchitectureAuthorityDoc =
  "40_Experience_Refoundation.md";

export type ArchitectureNodeKind =
  | "architecture_document"
  | "engineering_record"
  | "proposal"
  | "opportunity"
  | "evidence"
  | "replay_session"
  | "commit"
  | "module";

export type ArchitectureEdgeType =
  | "authority_precedes"
  | "references_authority"
  | "originates_from_proposal"
  | "originates_from_opportunity"
  | "supported_by_evidence"
  | "replays"
  | "implemented_by_commit"
  | "affects_module";

export interface ArchitectureNode {
  id: string;
  kind: ArchitectureNodeKind;
}

export interface ArchitectureEdge {
  id: string;
  from: string;
  to: string;
  type: ArchitectureEdgeType;
}

export interface ArchitectureGraph {
  schemaVersion: 1;
  nodes: ArchitectureNode[];
  edges: ArchitectureEdge[];
}

export type IntegrityViolationCode =
  | "duplicate_node_id"
  | "dangling_reference"
  | "orphan_node"
  | "authority_cycle"
  | "invalid_engineering_authority"
  | "proposal_missing_evidence_lineage"
  | "released_missing_implementation"
  | "unknown_edge_endpoint";

export interface IntegrityViolation {
  code: IntegrityViolationCode;
  nodeId?: string;
  edgeId?: string;
}

export interface IntegrityResult {
  valid: boolean;
  violations: IntegrityViolation[];
  orphanNodeIds: string[];
  danglingEdgeIds: string[];
}

export interface ArchitectureSnapshot {
  schemaVersion: 1;
  snapshotId: string;
  graphHash: string;
  nodeCount: number;
  edgeCount: number;
  integrity: IntegrityResult;
  t: number;
  authorityRoot: ArchitectureAuthorityDoc;
  releaseLineage: string[];
}

export interface ArchitectureGraphSource {
  engineeringRecords: EngineeringChangeRecord[];
  proposals: ExperienceChangeProposal[];
  opportunities: ExperienceOpportunity[];
  evidence: ExperienceEvidence[];
}

interface SnapshotBundle {
  schemaVersion: 1;
  snapshots: ArchitectureSnapshot[];
}

function emptyBundle(): SnapshotBundle {
  return { schemaVersion: 1, snapshots: [] };
}

function nodeId(kind: ArchitectureNodeKind, raw: string): string {
  switch (kind) {
    case "architecture_document":
      return `doc:${raw}`;
    case "engineering_record":
      return raw;
    case "proposal":
      return raw;
    case "opportunity":
      return raw;
    case "evidence":
      return raw;
    case "replay_session":
      return `replay:${raw}`;
    case "commit":
      return `commit:${raw}`;
    case "module":
      return `mod:${raw}`;
    default:
      return raw;
  }
}

function edgeId(
  type: ArchitectureEdgeType,
  from: string,
  to: string,
): string {
  return `edge:${fnv1a(`${type}|${from}|${to}`)}`;
}

function addNode(
  map: Map<string, ArchitectureNode>,
  kind: ArchitectureNodeKind,
  raw: string,
): string {
  const id = nodeId(kind, raw);
  if (!map.has(id)) {
    map.set(id, { id, kind });
  }
  return id;
}

function addEdge(
  edges: Map<string, ArchitectureEdge>,
  type: ArchitectureEdgeType,
  from: string,
  to: string,
): void {
  const id = edgeId(type, from, to);
  if (!edges.has(id)) {
    edges.set(id, { id, from, to, type });
  }
}

/**
 * Build the canonical ArchitectureGraph from declared governance artefacts.
 * Edges are created only from explicit fields / AUTHORITY_CHAIN — never inferred.
 */
export function buildArchitectureGraph(
  source: ArchitectureGraphSource,
): ArchitectureGraph {
  const nodes = new Map<string, ArchitectureNode>();
  const edges = new Map<string, ArchitectureEdge>();

  // Authority documents + declared chain.
  for (const doc of ARCHITECTURE_AUTHORITY_DOCS) {
    addNode(nodes, "architecture_document", doc);
  }
  for (const [fromDoc, toDoc] of AUTHORITY_CHAIN) {
    const from = addNode(nodes, "architecture_document", fromDoc);
    const to = addNode(nodes, "architecture_document", toDoc);
    addEdge(edges, "authority_precedes", from, to);
  }

  const proposalById = new Map(
    source.proposals.map((p) => [p.proposalId, p]),
  );
  const evidenceById = new Map(
    source.evidence.map((e) => [e.evidenceId, e]),
  );
  const opportunityById = new Map(
    source.opportunities.map((o) => [o.opportunityId, o]),
  );

  // Evidence / replay nodes are added only when explicitly referenced (no orphan isolates).

  for (const opportunity of source.opportunities) {
    const oppId = addNode(nodes, "opportunity", opportunity.opportunityId);
    for (const evidenceId of opportunity.supportingEvidenceIds) {
      if (evidenceById.has(evidenceId)) {
        const evId = addNode(nodes, "evidence", evidenceId);
        addEdge(edges, "supported_by_evidence", oppId, evId);
      }
    }
    for (const sessionId of opportunity.replaySessionIds) {
      const replayId = addNode(nodes, "replay_session", sessionId);
      addEdge(edges, "replays", oppId, replayId);
    }
  }

  for (const proposal of source.proposals) {
    const propId = addNode(nodes, "proposal", proposal.proposalId);
    for (const opportunityId of proposal.opportunityIds) {
      if (opportunityById.has(opportunityId)) {
        const oppId = addNode(nodes, "opportunity", opportunityId);
        addEdge(edges, "originates_from_opportunity", propId, oppId);
      }
    }
    for (const evidenceId of proposal.evidenceSnapshotIds) {
      if (evidenceById.has(evidenceId)) {
        const evId = addNode(nodes, "evidence", evidenceId);
        addEdge(edges, "supported_by_evidence", propId, evId);
      }
    }
    for (const sessionId of proposal.validation.replaySessionIds) {
      const replayId = addNode(nodes, "replay_session", sessionId);
      addEdge(edges, "replays", propId, replayId);
    }
  }

  for (const record of source.engineeringRecords) {
    const engId = addNode(nodes, "engineering_record", record.changeId);
    for (const proposalId of record.proposalIds) {
      if (proposalById.has(proposalId)) {
        const propId = addNode(nodes, "proposal", proposalId);
        addEdge(edges, "originates_from_proposal", engId, propId);
      }
    }
    for (const doc of record.architectureDocuments) {
      if (isArchitectureAuthorityDoc(doc)) {
        const docId = addNode(nodes, "architecture_document", doc);
        addEdge(edges, "references_authority", engId, docId);
      }
    }
    for (const commit of record.commits) {
      const commitId = addNode(nodes, "commit", commit);
      addEdge(edges, "implemented_by_commit", engId, commitId);
    }
    for (const mod of record.affectedModules) {
      const modId = addNode(nodes, "module", mod);
      addEdge(edges, "affects_module", engId, modId);
    }
  }

  const sortedNodes = [...nodes.values()].sort((a, b) =>
    a.id.localeCompare(b.id),
  );
  const sortedEdges = [...edges.values()].sort((a, b) => {
    const c = a.from.localeCompare(b.from);
    if (c !== 0) return c;
    const t = a.type.localeCompare(b.type);
    if (t !== 0) return t;
    return a.to.localeCompare(b.to);
  });

  return {
    schemaVersion: 1,
    nodes: sortedNodes,
    edges: sortedEdges,
  };
}

function hasAuthorityCycle(graph: ArchitectureGraph): boolean {
  const adj = new Map<string, string[]>();
  for (const edge of graph.edges) {
    if (edge.type !== "authority_precedes") {
      continue;
    }
    const list = adj.get(edge.from) ?? [];
    list.push(edge.to);
    adj.set(edge.from, list);
  }

  const visiting = new Set<string>();
  const visited = new Set<string>();

  const dfs = (node: string): boolean => {
    if (visiting.has(node)) {
      return true;
    }
    if (visited.has(node)) {
      return false;
    }
    visiting.add(node);
    for (const next of adj.get(node) ?? []) {
      if (dfs(next)) {
        return true;
      }
    }
    visiting.delete(node);
    visited.add(node);
    return false;
  };

  for (const node of adj.keys()) {
    if (dfs(node)) {
      return true;
    }
  }
  return false;
}

/**
 * Deterministic integrity validation. Rejects invalid graphs via valid=false.
 */
export function validateArchitectureIntegrity(
  graph: ArchitectureGraph,
  source?: Pick<ArchitectureGraphSource, "engineeringRecords" | "proposals">,
): IntegrityResult {
  const violations: IntegrityViolation[] = [];
  const nodeIds = new Set<string>();
  const seen = new Set<string>();

  for (const node of graph.nodes) {
    if (seen.has(node.id)) {
      violations.push({ code: "duplicate_node_id", nodeId: node.id });
    }
    seen.add(node.id);
    nodeIds.add(node.id);
  }

  const degree = new Map<string, number>();
  for (const id of nodeIds) {
    degree.set(id, 0);
  }

  const danglingEdgeIds: string[] = [];
  for (const edge of graph.edges) {
    if (!nodeIds.has(edge.from) || !nodeIds.has(edge.to)) {
      violations.push({
        code: "dangling_reference",
        edgeId: edge.id,
      });
      danglingEdgeIds.push(edge.id);
      continue;
    }
    degree.set(edge.from, (degree.get(edge.from) ?? 0) + 1);
    degree.set(edge.to, (degree.get(edge.to) ?? 0) + 1);
  }

  const orphanNodeIds: string[] = [];
  for (const [id, count] of degree) {
    if (count === 0) {
      orphanNodeIds.push(id);
      violations.push({ code: "orphan_node", nodeId: id });
    }
  }
  orphanNodeIds.sort((a, b) => a.localeCompare(b));
  danglingEdgeIds.sort((a, b) => a.localeCompare(b));

  if (hasAuthorityCycle(graph)) {
    violations.push({ code: "authority_cycle" });
  }

  // Engineering records must reference at least one valid authority doc node.
  if (source) {
    for (const record of source.engineeringRecords) {
      const engId = nodeId("engineering_record", record.changeId);
      const authorityEdges = graph.edges.filter(
        (e) =>
          e.from === engId &&
          e.type === "references_authority" &&
          nodeIds.has(e.to),
      );
      if (authorityEdges.length === 0) {
        violations.push({
          code: "invalid_engineering_authority",
          nodeId: engId,
        });
      }
      if (record.state === "released") {
        const hasCommit = graph.edges.some(
          (e) => e.from === engId && e.type === "implemented_by_commit",
        );
        const hasModule = graph.edges.some(
          (e) => e.from === engId && e.type === "affects_module",
        );
        if (!hasCommit || !hasModule) {
          violations.push({
            code: "released_missing_implementation",
            nodeId: engId,
          });
        }
      }
    }

    for (const proposal of source.proposals) {
      const propId = nodeId("proposal", proposal.proposalId);
      const hasEvidence = graph.edges.some(
        (e) => e.from === propId && e.type === "supported_by_evidence",
      );
      if (!hasEvidence) {
        violations.push({
          code: "proposal_missing_evidence_lineage",
          nodeId: propId,
        });
      }
    }
  }

  // Deduplicate by code+nodeId+edgeId
  const uniqueKey = (v: IntegrityViolation) =>
    `${v.code}|${v.nodeId ?? ""}|${v.edgeId ?? ""}`;
  const unique = new Map<string, IntegrityViolation>();
  for (const v of violations) {
    unique.set(uniqueKey(v), v);
  }
  const list = [...unique.values()].sort((a, b) =>
    uniqueKey(a).localeCompare(uniqueKey(b)),
  );

  return {
    valid: list.length === 0,
    violations: list,
    orphanNodeIds,
    danglingEdgeIds,
  };
}

export function hashArchitectureGraph(graph: ArchitectureGraph): string {
  return fnv1a(
    JSON.stringify({
      nodes: graph.nodes,
      edges: graph.edges,
    }),
  );
}

/**
 * Immutable snapshot from a graph. Caller supplies timestamp.
 * Snapshot identity is content-addressed (hash + counts + lineage).
 */
export function createArchitectureSnapshot(
  graph: ArchitectureGraph,
  options: {
    t: number;
    integrity?: IntegrityResult;
    source?: ArchitectureGraphSource;
  },
): ArchitectureSnapshot {
  const integrity =
    options.integrity ??
    validateArchitectureIntegrity(graph, {
      engineeringRecords: options.source?.engineeringRecords ?? [],
      proposals: options.source?.proposals ?? [],
    });
  const graphHash = hashArchitectureGraph(graph);
  const releaseLineage = (options.source?.engineeringRecords ?? [])
    .filter((r) => r.state === "released")
    .map((r) => r.changeId)
    .sort((a, b) => a.localeCompare(b));

  const snapshotId = `asnap-${fnv1a(
    `${graphHash}|${graph.nodes.length}|${graph.edges.length}|${releaseLineage.join(",")}|${integrity.valid ? 1 : 0}`,
  )}`;

  return {
    schemaVersion: 1,
    snapshotId,
    graphHash,
    nodeCount: graph.nodes.length,
    edgeCount: graph.edges.length,
    integrity: {
      valid: integrity.valid,
      violations: integrity.violations.map((v) => ({ ...v })),
      orphanNodeIds: [...integrity.orphanNodeIds],
      danglingEdgeIds: [...integrity.danglingEdgeIds],
    },
    t: options.t,
    authorityRoot: AUTHORITY_ROOT,
    releaseLineage,
  };
}

export function loadSnapshotBundle(
  store: ExperienceStoreAdapter,
): SnapshotBundle {
  const raw = store.getItem(ARCHITECTURE_INTEGRITY_STORAGE_KEY);
  if (!raw) {
    return emptyBundle();
  }
  try {
    const parsed = JSON.parse(raw) as SnapshotBundle;
    if (parsed?.schemaVersion !== 1 || !Array.isArray(parsed.snapshots)) {
      return emptyBundle();
    }
    return {
      schemaVersion: 1,
      snapshots: parsed.snapshots.slice(-MAX_ARCHITECTURE_SNAPSHOTS),
    };
  } catch {
    return emptyBundle();
  }
}

function saveSnapshotBundle(
  store: ExperienceStoreAdapter,
  bundle: SnapshotBundle,
): void {
  store.setItem(
    ARCHITECTURE_INTEGRITY_STORAGE_KEY,
    JSON.stringify({
      schemaVersion: 1,
      snapshots: bundle.snapshots.slice(-MAX_ARCHITECTURE_SNAPSHOTS),
    }),
  );
}

/**
 * Append-only snapshot persistence. Never mutates prior snapshots.
 * Idempotent on snapshotId.
 */
export function persistArchitectureSnapshot(
  store: ExperienceStoreAdapter,
  snapshot: ArchitectureSnapshot,
): ArchitectureSnapshot {
  const bundle = loadSnapshotBundle(store);
  if (bundle.snapshots.some((s) => s.snapshotId === snapshot.snapshotId)) {
    return {
      ...bundle.snapshots.find((s) => s.snapshotId === snapshot.snapshotId)!,
    };
  }
  const frozen = Object.freeze({
    ...snapshot,
    schemaVersion: 1 as const,
    integrity: Object.freeze({
      ...snapshot.integrity,
      violations: snapshot.integrity.violations.map((v) => Object.freeze({ ...v })),
      orphanNodeIds: [...snapshot.integrity.orphanNodeIds],
      danglingEdgeIds: [...snapshot.integrity.danglingEdgeIds],
    }),
    releaseLineage: [...snapshot.releaseLineage],
  });
  bundle.snapshots = [...bundle.snapshots, frozen];
  saveSnapshotBundle(store, bundle);
  return { ...frozen, integrity: { ...frozen.integrity } };
}

export function listArchitectureSnapshots(
  store: ExperienceStoreAdapter,
): ArchitectureSnapshot[] {
  return loadSnapshotBundle(store).snapshots.map((s) => ({
    ...s,
    integrity: {
      ...s.integrity,
      violations: s.integrity.violations.map((v) => ({ ...v })),
      orphanNodeIds: [...s.integrity.orphanNodeIds],
      danglingEdgeIds: [...s.integrity.danglingEdgeIds],
    },
    releaseLineage: [...s.releaseLineage],
  }));
}

export function assertSnapshotsImmutable(
  before: ArchitectureSnapshot[],
  after: ArchitectureSnapshot[],
): boolean {
  if (after.length < before.length) {
    return false;
  }
  for (let i = 0; i < before.length; i++) {
    const a = before[i]!;
    const b = after[i]!;
    if (
      a.snapshotId !== b.snapshotId ||
      a.graphHash !== b.graphHash ||
      a.nodeCount !== b.nodeCount ||
      a.edgeCount !== b.edgeCount ||
      a.t !== b.t ||
      a.authorityRoot !== b.authorityRoot ||
      a.integrity.valid !== b.integrity.valid ||
      JSON.stringify(a.releaseLineage) !== JSON.stringify(b.releaseLineage)
    ) {
      return false;
    }
  }
  return true;
}

/** Authority chain explorer helper — outgoing precedes edges from a doc node. */
export function listAuthoritySuccessors(
  graph: ArchitectureGraph,
  document: ArchitectureAuthorityDoc,
): ArchitectureAuthorityDoc[] {
  const from = nodeId("architecture_document", document);
  return graph.edges
    .filter((e) => e.from === from && e.type === "authority_precedes")
    .map((e) => e.to.replace(/^doc:/, ""))
    .filter(isArchitectureAuthorityDoc)
    .sort((a, b) => a.localeCompare(b));
}

/** Dependency explorer — outbound edges from any node id. */
export function listDependencies(
  graph: ArchitectureGraph,
  nodeIdValue: string,
): ArchitectureEdge[] {
  return graph.edges
    .filter((e) => e.from === nodeIdValue)
    .sort((a, b) => a.id.localeCompare(b.id));
}

export function clearArchitectureIntegrityStore(
  store: ExperienceStoreAdapter,
): void {
  store.removeItem(ARCHITECTURE_INTEGRITY_STORAGE_KEY);
}

export function defaultArchitectureIntegrityStore(): ExperienceStoreAdapter {
  return browserStore() ?? memoryStore();
}
