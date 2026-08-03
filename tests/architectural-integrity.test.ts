/**
 * Sprint 56 — Architectural Integrity.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type {
  ExperienceEvent,
  ExperienceSession,
} from "../app/src/dev/experienceEvents";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import {
  buildEvidenceFromSessions,
  persistEvidenceSnapshot,
} from "../app/src/dev/experienceEvidence";
import { detectOpportunities } from "../app/src/dev/experienceImprovement";
import {
  buildProposalFromOpportunities,
  listProposals,
  persistProposal,
  transitionProposal,
} from "../app/src/dev/experienceGovernance";
import {
  buildEngineeringRecordFromProposals,
  listEngineeringRecords,
  persistEngineeringRecord,
  transitionEngineeringRecord,
} from "../app/src/dev/engineeringGovernance";
import {
  ARCHITECTURE_INTEGRITY_STORAGE_KEY,
  AUTHORITY_CHAIN,
  AUTHORITY_ROOT,
  assertSnapshotsImmutable,
  buildArchitectureGraph,
  clearArchitectureIntegrityStore,
  createArchitectureSnapshot,
  hashArchitectureGraph,
  listArchitectureSnapshots,
  listAuthoritySuccessors,
  listDependencies,
  persistArchitectureSnapshot,
  validateArchitectureIntegrity,
  type ArchitectureGraph,
} from "../app/src/dev/architecturalIntegrity";
import { memoryStore } from "../app/src/dev/experienceStore";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function session(
  id: string,
  events: ExperienceEvent[],
): ExperienceSession {
  return {
    schemaVersion: 1,
    sessionId: id,
    startedAt: 1,
    events,
  };
}

function nav(
  seq: number,
  t: number,
  from: ExperienceEvent["from"],
  destination: ExperienceEvent["destination"],
): ExperienceEvent {
  return {
    seq,
    t,
    type: "navigate",
    from,
    destination,
    commandId: "dock_navigate",
  };
}

const thrash = session("s-arch", [
  { seq: 0, t: 0, type: "session_start", destination: "home" },
  {
    seq: 1,
    t: 9000,
    type: "first_meaningful_interaction",
    destination: "home",
    modality: "pointer",
  },
  nav(2, 10000, "home", "save"),
  { seq: 3, t: 10100, type: "flow_start", flow: "save", destination: "save" },
  nav(4, 12000, "save", "resume"),
  {
    seq: 5,
    t: 12050,
    type: "flow_abandon",
    flow: "save",
    from: "save",
    destination: "resume",
  },
  nav(6, 14000, "resume", "save"),
  nav(7, 16000, "save", "resume"),
]);

function seedGovernance(store: ReturnType<typeof memoryStore>) {
  const evidence = buildEvidenceFromSessions([thrash], { tag: "arch" });
  persistEvidenceSnapshot(store, evidence);
  const opportunities = detectOpportunities([evidence]);
  const draft = buildProposalFromOpportunities(opportunities, {
    evidenceBaselineId: evidence.evidenceId,
  })!;
  persistProposal(store, draft, { now: 1 });
  for (const next of ["review", "accepted"] as const) {
    expect(
      transitionProposal(store, draft.proposalId, next, {
        evidenceReferenceId: evidence.evidenceId,
        now: next === "review" ? 2 : 3,
      }).ok,
    ).toBe(true);
  }
  const proposals = listProposals(store);
  const record = buildEngineeringRecordFromProposals(proposals, {
    commits: ["2a9230a"],
    architectureDocuments: [
      "46_Engineering_Governance.md",
      "47_Architectural_Integrity.md",
    ],
    affectedModules: ["app/src/dev/architecturalIntegrity.ts"],
    affectedTests: ["tests/architectural-integrity.test.ts"],
  })!;
  expect(
    persistEngineeringRecord(
      store,
      record,
      { proposals, evidenceIds: [evidence.evidenceId] },
      { now: 4 },
    ).ok,
  ).toBe(true);

  return {
    evidence,
    opportunities,
    proposals,
    records: listEngineeringRecords(store),
  };
}

describe("architecture graph integrity", () => {
  it("builds a deterministic graph with explicit edges only", () => {
    const store = memoryStore();
    const { evidence, opportunities, proposals, records } = seedGovernance(store);
    const source = {
      engineeringRecords: records,
      proposals,
      opportunities,
      evidence: [evidence],
    };
    const a = buildArchitectureGraph(source);
    const b = buildArchitectureGraph(source);
    expect(a).toEqual(b);
    expect(hashArchitectureGraph(a)).toBe(hashArchitectureGraph(b));
    expect(a.nodes.every((n, i, arr) => i === 0 || arr[i - 1]!.id <= n.id)).toBe(
      true,
    );
    expect(
      a.edges.every((e) => a.nodes.some((n) => n.id === e.from)),
    ).toBe(true);
    expect(a.edges.some((e) => e.type === "authority_precedes")).toBe(true);
    expect(a.edges.some((e) => e.type === "references_authority")).toBe(true);
    expect(AUTHORITY_CHAIN).toHaveLength(9);
  });

  it("passes integrity for a complete authority-linked source", () => {
    const store = memoryStore();
    const { evidence, opportunities, proposals, records } = seedGovernance(store);
    const graph = buildArchitectureGraph({
      engineeringRecords: records,
      proposals,
      opportunities,
      evidence: [evidence],
    });
    const result = validateArchitectureIntegrity(graph, {
      engineeringRecords: records,
      proposals,
    });
    expect(result.valid).toBe(true);
    expect(result.orphanNodeIds).toEqual([]);
    expect(result.danglingEdgeIds).toEqual([]);
  });

  it("rejects orphans, dangling refs, cycles, and duplicate ids", () => {
    const base = buildArchitectureGraph({
      engineeringRecords: [],
      proposals: [],
      opportunities: [],
      evidence: [],
    });
    expect(validateArchitectureIntegrity(base).valid).toBe(true);

    const withOrphan: ArchitectureGraph = {
      ...base,
      nodes: [
        ...base.nodes,
        { id: "orphan-node", kind: "module" },
      ].sort((a, b) => a.id.localeCompare(b.id)),
    };
    const orphanResult = validateArchitectureIntegrity(withOrphan);
    expect(orphanResult.valid).toBe(false);
    expect(orphanResult.violations.some((v) => v.code === "orphan_node")).toBe(
      true,
    );

    const dangling: ArchitectureGraph = {
      ...base,
      edges: [
        ...base.edges,
        {
          id: "edge:dangling",
          from: "doc:40_Experience_Refoundation.md",
          to: "missing-target",
          type: "affects_module",
        },
      ],
    };
    const dangResult = validateArchitectureIntegrity(dangling);
    expect(dangResult.valid).toBe(false);
    expect(
      dangResult.violations.some((v) => v.code === "dangling_reference"),
    ).toBe(true);

    const cycled: ArchitectureGraph = {
      ...base,
      edges: [
        ...base.edges,
        {
          id: "edge:cycle",
          from: "doc:49_Adaptation_Experiments.md",
          to: "doc:40_Experience_Refoundation.md",
          type: "authority_precedes",
        },
      ],
    };
    expect(validateArchitectureIntegrity(cycled).valid).toBe(false);
    expect(
      validateArchitectureIntegrity(cycled).violations.some(
        (v) => v.code === "authority_cycle",
      ),
    ).toBe(true);

    const dup: ArchitectureGraph = {
      ...base,
      nodes: [...base.nodes, base.nodes[0]!],
    };
    expect(
      validateArchitectureIntegrity(dup).violations.some(
        (v) => v.code === "duplicate_node_id",
      ),
    ).toBe(true);
  });

  it("requires released records to trace to implementation", () => {
    const store = memoryStore();
    const { evidence, opportunities, proposals, records } = seedGovernance(store);
    const changeId = records[0]!.changeId;
    const context = {
      proposals,
      evidenceIds: [evidence.evidenceId],
    };
    for (const next of [
      "implemented",
      "validated",
      "architecturally_accepted",
      "released",
    ] as const) {
      expect(
        transitionEngineeringRecord(store, changeId, next, context, {
          authorityReference: "47_Architectural_Integrity.md",
          now: 10,
        }).ok,
      ).toBe(true);
    }
    const released = listEngineeringRecords(store);
    const graph = buildArchitectureGraph({
      engineeringRecords: released,
      proposals,
      opportunities,
      evidence: [evidence],
    });
    const ok = validateArchitectureIntegrity(graph, {
      engineeringRecords: released,
      proposals,
    });
    expect(ok.valid).toBe(true);

    const stripped = {
      ...graph,
      edges: graph.edges.filter((e) => e.type !== "implemented_by_commit"),
    };
    const bad = validateArchitectureIntegrity(stripped, {
      engineeringRecords: released,
      proposals,
    });
    expect(
      bad.violations.some((v) => v.code === "released_missing_implementation"),
    ).toBe(true);
  });
});

describe("architecture snapshots", () => {
  it("creates content-addressed append-only snapshots", () => {
    const store = memoryStore();
    const { evidence, opportunities, proposals, records } = seedGovernance(store);
    const source = {
      engineeringRecords: records,
      proposals,
      opportunities,
      evidence: [evidence],
    };
    const graph = buildArchitectureGraph(source);
    const snapA = createArchitectureSnapshot(graph, { t: 100, source });
    const snapB = createArchitectureSnapshot(graph, { t: 200, source });
    expect(snapA.snapshotId).toBe(snapB.snapshotId);
    expect(snapA.graphHash).toBe(hashArchitectureGraph(graph));
    expect(snapA.authorityRoot).toBe(AUTHORITY_ROOT);
    expect(snapA.integrity.valid).toBe(true);

    persistArchitectureSnapshot(store, snapA);
    const before = listArchitectureSnapshots(store);
    persistArchitectureSnapshot(store, { ...snapA, t: 999 });
    const after = listArchitectureSnapshots(store);
    expect(assertSnapshotsImmutable(before, after)).toBe(true);
    expect(after).toHaveLength(1);

    // Force a second distinct snapshot by mutating release lineage via released state.
    // Use a different graph (add nothing) — same hash. Append a second by changing graph.
    const larger = buildArchitectureGraph({
      ...source,
      engineeringRecords: [
        {
          ...records[0]!,
          changeId: `${records[0]!.changeId}-x`,
          commits: ["abcdef12"],
        },
      ],
    });
    const snap2 = createArchitectureSnapshot(larger, {
      t: 300,
      source: {
        ...source,
        engineeringRecords: [
          {
            ...records[0]!,
            changeId: `${records[0]!.changeId}-x`,
            commits: ["abcdef12"],
          },
        ],
      },
    });
    persistArchitectureSnapshot(store, snap2);
    const two = listArchitectureSnapshots(store);
    expect(two.length).toBe(2);
    expect(assertSnapshotsImmutable(after, two)).toBe(true);
    clearArchitectureIntegrityStore(store);
  });
});

describe("authority validation helpers", () => {
  it("explores authority successors and dependencies", () => {
    const graph = buildArchitectureGraph({
      engineeringRecords: [],
      proposals: [],
      opportunities: [],
      evidence: [],
    });
    expect(listAuthoritySuccessors(graph, AUTHORITY_ROOT)).toEqual([
      "41_Perceptual_Convergence.md",
    ]);
    const deps = listDependencies(
      graph,
      "doc:48_Adaptive_Workspace.md",
    );
    expect(deps.some((e) => e.type === "authority_precedes")).toBe(true);
    expect(deps.some((e) => e.to === "doc:49_Adaptation_Experiments.md")).toBe(
      true,
    );
  });
});

describe("privacy audit (architectural integrity)", () => {
  it("snapshot JSON omits forbidden content keys", () => {
    const store = memoryStore();
    const graph = buildArchitectureGraph({
      engineeringRecords: [],
      proposals: [],
      opportunities: [],
      evidence: [],
    });
    const snap = createArchitectureSnapshot(graph, { t: 1 });
    persistArchitectureSnapshot(store, snap);
    const raw = store.getItem(ARCHITECTURE_INTEGRITY_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
  });

  it("dev modules introduce no network telemetry", () => {
    const dir = path.join(root, "app/src/dev");
    for (const file of readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f))) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });

  it("overlay lazy-loads integrity behind DEV gate", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("./architecturalIntegrity")');
    expect(dash).toContain("architecture-integrity");
    const main = readFileSync(path.join(root, "app/src/main.tsx"), "utf8");
    expect(main).not.toMatch(/from\s+["'].*architecturalIntegrity["']/);
  });

  it("architecture doc records graph schema and integrity rules", () => {
    const doc = readFileSync(
      path.join(root, "architecture/47_Architectural_Integrity.md"),
      "utf8",
    );
    expect(doc).toContain("ArchitectureGraph");
    expect(doc).toContain("AUTHORITY_CHAIN");
    expect(doc).toContain("No inferred links");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
