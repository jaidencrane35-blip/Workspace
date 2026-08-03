/**
 * Sprint 55 — Engineering Governance.
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
  ENGINEERING_STORAGE_KEY,
  assertEngineeringHistoryImmutable,
  assertNoOrphanReleasedRecords,
  buildEngineeringRecordFromProposals,
  buildReleaseTraceability,
  clearEngineeringStore,
  listEngineeringHistory,
  listEngineeringRecords,
  persistEngineeringRecord,
  transitionEngineeringRecord,
  verifyEngineeringConsistency,
} from "../app/src/dev/engineeringGovernance";
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

const thrash = session("s-eng", [
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

function seedAcceptedProposal(store: ReturnType<typeof memoryStore>) {
  const evidence = buildEvidenceFromSessions([thrash], { tag: "eng" });
  persistEvidenceSnapshot(store, evidence);
  const opportunities = detectOpportunities([evidence]);
  const draft = buildProposalFromOpportunities(opportunities, {
    evidenceBaselineId: evidence.evidenceId,
  })!;
  persistProposal(store, draft, { now: 1 });
  for (const next of ["review", "accepted"] as const) {
    const result = transitionProposal(store, draft.proposalId, next, {
      evidenceReferenceId: evidence.evidenceId,
      now: next === "review" ? 2 : 3,
    });
    expect(result.ok).toBe(true);
  }
  return {
    evidence,
    opportunities,
    proposals: listProposals(store),
  };
}

describe("engineering change records", () => {
  it("builds deterministic records reusing proposal identifiers", () => {
    const store = memoryStore();
    const { evidence, proposals } = seedAcceptedProposal(store);
    const opts = {
      commits: ["5f7800b"],
      architectureDocuments: [
        "45_Experience_Change_Governance.md" as const,
        "46_Engineering_Governance.md" as const,
      ],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
      releaseImpact: "dev_tooling" as const,
    };
    const a = buildEngineeringRecordFromProposals(proposals, opts);
    const b = buildEngineeringRecordFromProposals(proposals, opts);
    expect(a).not.toBeNull();
    expect(a).toEqual(b);
    expect(a!.proposalIds).toEqual(proposals.map((p) => p.proposalId));
    expect(a!.validationEvidence.evidenceSnapshotIds).toContain(
      evidence.evidenceId,
    );
    expect(a!.validationEvidence.replaySessionIds).toContain("s-eng");
    expect(a!.state).toBe("draft");
  });

  it("rejects incomplete records on persist", () => {
    const store = memoryStore();
    const { proposals } = seedAcceptedProposal(store);
    const record = buildEngineeringRecordFromProposals(proposals, {
      commits: ["5f7800b"],
      architectureDocuments: ["46_Engineering_Governance.md"],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
    })!;
    const incomplete = verifyEngineeringConsistency(record, {
      proposals: [],
      evidenceIds: [],
    });
    expect(incomplete.status).toBe("incomplete");
    expect(incomplete.errors.length).toBeGreaterThan(0);

    const refused = persistEngineeringRecord(
      store,
      record,
      { proposals: [], evidenceIds: [] },
      { now: 10 },
    );
    expect(refused.ok).toBe(false);
  });

  it("persists when authority chain is complete", () => {
    const store = memoryStore();
    const { evidence, proposals } = seedAcceptedProposal(store);
    const record = buildEngineeringRecordFromProposals(proposals, {
      commits: ["5f7800babc"],
      architectureDocuments: [
        "45_Experience_Change_Governance.md",
        "46_Engineering_Governance.md",
      ],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
    })!;
    const result = persistEngineeringRecord(
      store,
      record,
      {
        proposals,
        evidenceIds: [evidence.evidenceId],
      },
      { now: 20 },
    );
    expect(result.ok).toBe(true);
    if (result.ok) {
      expect(result.record.consistencyStatus).toBe("complete");
    }
    expect(listEngineeringRecords(store)).toHaveLength(1);
  });
});

describe("engineering lifecycle + history", () => {
  it("advances manually with immutable history and no auto-promotion", () => {
    const store = memoryStore();
    const { evidence, proposals } = seedAcceptedProposal(store);
    const context = {
      proposals,
      evidenceIds: [evidence.evidenceId],
    };
    const record = buildEngineeringRecordFromProposals(proposals, {
      commits: ["abcdef1"],
      architectureDocuments: ["46_Engineering_Governance.md"],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
    })!;
    persistEngineeringRecord(store, record, context, { now: 1 });

    const skip = transitionEngineeringRecord(
      store,
      record.changeId,
      "released",
      context,
      {
        authorityReference: "46_Engineering_Governance.md",
        now: 2,
      },
    );
    expect(skip.ok).toBe(false);

    const before = listEngineeringHistory(store, record.changeId);
    const steps = [
      "implemented",
      "validated",
      "architecturally_accepted",
      "released",
    ] as const;
    let t = 10;
    for (const next of steps) {
      const result = transitionEngineeringRecord(
        store,
        record.changeId,
        next,
        context,
        {
          authorityReference: "46_Engineering_Governance.md",
          now: t,
        },
      );
      expect(result.ok).toBe(true);
      t += 1;
    }
    const after = listEngineeringHistory(store, record.changeId);
    expect(assertEngineeringHistoryImmutable(before, after)).toBe(true);
    expect(after.length).toBe(before.length + 4);
    expect(listEngineeringRecords(store)[0]?.state).toBe("released");
    expect(assertNoOrphanReleasedRecords(store, context)).toBe(true);
  });
});

describe("release traceability", () => {
  it("requires full commit→proposal→opportunity→evidence→replay→traces chain", () => {
    const store = memoryStore();
    const { evidence, proposals } = seedAcceptedProposal(store);
    const record = buildEngineeringRecordFromProposals(proposals, {
      commits: ["1234567"],
      architectureDocuments: ["46_Engineering_Governance.md"],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
    })!;
    const trace = buildReleaseTraceability(record, {
      proposals,
      evidenceIds: [evidence.evidenceId],
    });
    expect(trace.complete).toBe(true);
    expect(trace.commits).toContain("1234567");
    expect(trace.proposalIds).toEqual(record.proposalIds);
    expect(trace.opportunityIds.length).toBeGreaterThan(0);
    expect(trace.evidenceSnapshotIds).toContain(evidence.evidenceId);
    expect(trace.replaySessionIds).toContain("s-eng");
    expect(trace.interactionSessionIds).toEqual(trace.replaySessionIds);

    const broken = buildReleaseTraceability(record, {
      proposals: [],
      evidenceIds: [evidence.evidenceId],
    });
    expect(broken.complete).toBe(false);
    expect(broken.missing).toContain("missing_proposal");
  });
});

describe("authority validation", () => {
  it("requires architecture authority documents from the Version 2 set", () => {
    const store = memoryStore();
    const { proposals } = seedAcceptedProposal(store);
    const built = buildEngineeringRecordFromProposals(proposals, {
      commits: ["7654321"],
      architectureDocuments: [],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
    });
    expect(built).toBeNull();

    const record = buildEngineeringRecordFromProposals(proposals, {
      commits: ["7654321"],
      architectureDocuments: ["46_Engineering_Governance.md"],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
    })!;
    // Forge an invalid authority list on a copy.
    const forged = {
      ...record,
      architectureDocuments: [] as typeof record.architectureDocuments,
      validationEvidence: {
        ...record.validationEvidence,
        architectureAuthorityIds: [],
      },
    };
    const result = verifyEngineeringConsistency(forged, {
      proposals,
      evidenceIds: record.validationEvidence.evidenceSnapshotIds,
    });
    expect(result.status).toBe("incomplete");
    expect(result.errors).toContain("missing_architecture_authority");
  });
});

describe("privacy audit (engineering governance)", () => {
  it("persisted engineering JSON omits forbidden content keys", () => {
    const store = memoryStore();
    const { evidence, proposals } = seedAcceptedProposal(store);
    const record = buildEngineeringRecordFromProposals(proposals, {
      commits: ["5f7800b"],
      architectureDocuments: ["46_Engineering_Governance.md"],
      affectedModules: ["app/src/dev/engineeringGovernance.ts"],
      affectedTests: ["tests/experience-engineering.test.ts"],
    })!;
    persistEngineeringRecord(
      store,
      record,
      { proposals, evidenceIds: [evidence.evidenceId] },
      { now: 1 },
    );
    const raw = store.getItem(ENGINEERING_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
    clearEngineeringStore(store);
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

  it("overlay lazy-loads engineering governance", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("./engineeringGovernance")');
    expect(dash).toContain("engineering-records");
    const main = readFileSync(path.join(root, "app/src/main.tsx"), "utf8");
    expect(main).not.toMatch(/from\s+["'].*engineeringGovernance["']/);
  });

  it("architecture doc records schema and release traceability", () => {
    const doc = readFileSync(
      path.join(root, "architecture/46_Engineering_Governance.md"),
      "utf8",
    );
    expect(doc).toContain("EngineeringChangeRecord");
    expect(doc).toContain("Release traceability");
    expect(doc).toContain("No automatic advancement");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
