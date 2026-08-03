/**
 * Sprint 57 — Governed Adaptive Workspace.
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
  buildArchitectureGraph,
  createArchitectureSnapshot,
  listArchitectureSnapshots,
  persistArchitectureSnapshot,
} from "../app/src/dev/architecturalIntegrity";
import {
  ADAPTATION_STORAGE_KEY,
  activateAdaptation,
  buildAdaptationFromLineage,
  clearAdaptationStore,
  identityPresentation,
  listAdaptations,
  persistAdaptation,
  presentationToShellStyle,
  resolvePresentationConfiguration,
  rollbackAdaptation,
  upsertValidatedAdaptation,
  validateAdaptationEvidence,
  verifyAdaptationLineage,
} from "../app/src/experience/workspaceAdaptation";
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

const thrash = session("s-adapt", [
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

const calm = session("s-adapt-calm", [
  { seq: 0, t: 0, type: "session_start", destination: "home" },
  {
    seq: 1,
    t: 120,
    type: "first_meaningful_interaction",
    destination: "home",
    modality: "pointer",
  },
  nav(2, 200, "home", "save"),
  { seq: 3, t: 250, type: "flow_start", flow: "save", destination: "save" },
  { seq: 4, t: 400, type: "save_success", flow: "save", destination: "save" },
]);

function seedLineage(store: ReturnType<typeof memoryStore>) {
  const evidence = buildEvidenceFromSessions([thrash], { tag: "adapt" });
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
        now: 2,
      }).ok,
    ).toBe(true);
  }
  const proposals = listProposals(store);
  const record = buildEngineeringRecordFromProposals(proposals, {
    commits: ["4e707f3"],
    architectureDocuments: [
      "47_Architectural_Integrity.md",
      "48_Adaptive_Workspace.md",
    ],
    affectedModules: ["app/src/experience/workspaceAdaptation.ts"],
    affectedTests: ["tests/workspace-adaptation.test.ts"],
  })!;
  expect(
    persistEngineeringRecord(
      store,
      record,
      { proposals, evidenceIds: [evidence.evidenceId] },
      { now: 3 },
    ).ok,
  ).toBe(true);
  const changeId = listEngineeringRecords(store)[0]!.changeId;
  for (const next of [
    "implemented",
    "validated",
    "architecturally_accepted",
  ] as const) {
    expect(
      transitionEngineeringRecord(
        store,
        changeId,
        next,
        { proposals, evidenceIds: [evidence.evidenceId] },
        {
          authorityReference: "48_Adaptive_Workspace.md",
          now: 4,
        },
      ).ok,
    ).toBe(true);
  }
  const engineering = listEngineeringRecords(store)[0]!;
  const graph = buildArchitectureGraph({
    engineeringRecords: [engineering],
    proposals,
    opportunities,
    evidence: [evidence],
  });
  const snapshot = createArchitectureSnapshot(graph, {
    t: 5,
    source: {
      engineeringRecords: [engineering],
      proposals,
      opportunities,
      evidence: [evidence],
    },
  });
  persistArchitectureSnapshot(store, snapshot);

  return {
    evidence,
    afterEvidence: buildEvidenceFromSessions([calm], { tag: "after" }),
    opportunities,
    proposal: listProposals(store)[0]!,
    engineering: listEngineeringRecords(store)[0]!,
    architectureSnapshot: listArchitectureSnapshots(store)[0]!,
  };
}

describe("adaptation resolver", () => {
  it("resolves identity when no active adaptations", () => {
    const resolved = resolvePresentationConfiguration([]);
    expect(resolved).toEqual(identityPresentation());
    const style = presentationToShellStyle(1, 1, resolved);
    expect(style["--intent-space"]).toBe("1");
  });

  it("applies active+passed adaptations deterministically", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    const a = buildAdaptationFromLineage(
      {
        engineering: lineage.engineering,
        proposal: lineage.proposal,
        evidence: lineage.evidence,
        architectureSnapshot: lineage.architectureSnapshot,
      },
      {
        targetComponents: ["shell"],
        scopes: ["spacing", "density"],
        presentation: {
          density: "focus",
          spacingScale: 0.9,
          motionProfile: "reduced",
        },
        expectedMetric: "meanFrictionScore",
        expectedImprovementDelta: 0.01,
        expectedDirection: "lower_better",
        rollbackCriteria: ["friction_regression"],
      },
    )!;
    const validated = validateAdaptationEvidence(
      a,
      lineage.evidence,
      lineage.afterEvidence,
    );
    expect(validated.validationResult).toBe("passed");
    upsertValidatedAdaptation(store, validated.adaptation);
    expect(
      activateAdaptation(store, validated.adaptation.adaptationId).ok,
    ).toBe(true);

    const list = listAdaptations(store);
    const r1 = resolvePresentationConfiguration(list);
    const r2 = resolvePresentationConfiguration(list);
    expect(r1).toEqual(r2);
    expect(r1.density).toBe("focus");
    expect(r1.spacingScale).toBe(0.9);
    expect(r1.motionProfile).toBe("reduced");
    expect(r1.appliedAdaptationIds).toEqual([validated.adaptation.adaptationId]);
    const style = presentationToShellStyle(1.2, 1.0, r1);
    expect(style["--motion-fast"]).toBe("0ms");
    expect(Number(style["--intent-space"])).toBeCloseTo(1.08, 4);
  });
});

describe("governance linkage", () => {
  it("rejects adaptations without complete lineage", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    const built = buildAdaptationFromLineage(
      {
        engineering: lineage.engineering,
        proposal: lineage.proposal,
        evidence: lineage.evidence,
        architectureSnapshot: lineage.architectureSnapshot,
      },
      {
        targetComponents: ["shell"],
        scopes: ["spacing"],
        presentation: { spacingScale: 0.95 },
        expectedMetric: "meanFrictionScore",
        expectedImprovementDelta: 0.02,
        expectedDirection: "lower_better",
        rollbackCriteria: ["friction_regression"],
      },
    );
    expect(built).not.toBeNull();
    expect(
      verifyAdaptationLineage(built!, {
        engineering: null,
        proposal: lineage.proposal,
        evidence: lineage.evidence,
        architectureSnapshot: lineage.architectureSnapshot,
      }),
    ).toBe("missing_engineering");

    const refused = persistAdaptation(store, built!, {
      engineering: {
        ...lineage.engineering,
        state: "draft",
      },
      proposal: lineage.proposal,
      evidence: lineage.evidence,
      architectureSnapshot: lineage.architectureSnapshot,
    });
    expect(refused.ok).toBe(false);

    const ok = persistAdaptation(store, built!, {
      engineering: lineage.engineering,
      proposal: lineage.proposal,
      evidence: lineage.evidence,
      architectureSnapshot: lineage.architectureSnapshot,
    });
    expect(ok.ok).toBe(true);
    if (ok.ok) {
      expect(ok.adaptation.rolloutState).toBe("inactive");
      expect(ok.adaptation.validation.validationResult).not.toBe("passed");
    }
  });

  it("does not auto-activate on create or validation pass", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    const built = buildAdaptationFromLineage(
      {
        engineering: lineage.engineering,
        proposal: lineage.proposal,
        evidence: lineage.evidence,
        architectureSnapshot: lineage.architectureSnapshot,
      },
      {
        targetComponents: ["shell", "canvas"],
        scopes: ["spacing", "environment"],
        presentation: { spacingScale: 0.92, environmentalWeight: 0.9 },
        expectedMetric: "meanFrictionScore",
        expectedImprovementDelta: 0.01,
        expectedDirection: "lower_better",
        rollbackCriteria: ["friction_regression"],
      },
    )!;
    persistAdaptation(store, built, {
      engineering: lineage.engineering,
      proposal: lineage.proposal,
      evidence: lineage.evidence,
      architectureSnapshot: lineage.architectureSnapshot,
    });
    const validated = validateAdaptationEvidence(
      built,
      lineage.evidence,
      lineage.afterEvidence,
    );
    expect(validated.validationResult).toBe("passed");
    expect(validated.adaptation.rolloutState).toBe("candidate");
    upsertValidatedAdaptation(store, validated.adaptation);
    expect(resolvePresentationConfiguration(listAdaptations(store)).appliedAdaptationIds).toEqual(
      [],
    );
    const blocked = activateAdaptation(store, "missing");
    expect(blocked.ok).toBe(false);
  });
});

describe("rollback", () => {
  it("rolls back without deleting the record and removes presentation effect", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    const built = buildAdaptationFromLineage(
      {
        engineering: lineage.engineering,
        proposal: lineage.proposal,
        evidence: lineage.evidence,
        architectureSnapshot: lineage.architectureSnapshot,
      },
      {
        targetComponents: ["shell"],
        scopes: ["density"],
        presentation: { density: "flow" },
        expectedMetric: "meanFrictionScore",
        expectedImprovementDelta: 0.01,
        expectedDirection: "lower_better",
        rollbackCriteria: ["friction_regression"],
      },
    )!;
    const validated = validateAdaptationEvidence(
      built,
      lineage.evidence,
      lineage.afterEvidence,
    );
    upsertValidatedAdaptation(store, validated.adaptation);
    activateAdaptation(store, validated.adaptation.adaptationId);
    expect(
      resolvePresentationConfiguration(listAdaptations(store)).density,
    ).toBe("flow");

    const rb = rollbackAdaptation(store, validated.adaptation.adaptationId);
    expect(rb.ok).toBe(true);
    expect(listAdaptations(store)[0]?.rolloutState).toBe("rolled_back");
    expect(
      resolvePresentationConfiguration(listAdaptations(store)).appliedAdaptationIds,
    ).toEqual([]);

    // Failed validation with active → rolled_back
    const again = buildAdaptationFromLineage(
      {
        engineering: lineage.engineering,
        proposal: lineage.proposal,
        evidence: lineage.evidence,
        architectureSnapshot: lineage.architectureSnapshot,
      },
      {
        targetComponents: ["dock"],
        scopes: ["motion"],
        presentation: { motionProfile: "expressive" },
        expectedMetric: "meanFrictionScore",
        expectedImprovementDelta: 0.5,
        expectedDirection: "lower_better",
        rollbackCriteria: ["friction_regression"],
      },
    )!;
    // Force pass then activate, then validate with worse after (thrash as after).
    const pass = {
      ...again,
      validation: { ...again.validation, validationResult: "passed" as const },
      rolloutState: "candidate" as const,
    };
    upsertValidatedAdaptation(store, pass);
    activateAdaptation(store, pass.adaptationId);
    const fail = validateAdaptationEvidence(
      listAdaptations(store).find((a) => a.adaptationId === pass.adaptationId)!,
      lineage.afterEvidence,
      lineage.evidence,
    );
    expect(fail.validationResult).toBe("failed");
    expect(fail.adaptation.rolloutState).toBe("rolled_back");
    clearAdaptationStore(store);
  });
});

describe("privacy + integrity audit (adaptation)", () => {
  it("adaptation JSON omits forbidden content keys", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    const built = buildAdaptationFromLineage(
      {
        engineering: lineage.engineering,
        proposal: lineage.proposal,
        evidence: lineage.evidence,
        architectureSnapshot: lineage.architectureSnapshot,
      },
      {
        targetComponents: ["shell"],
        scopes: ["spacing"],
        presentation: { spacingScale: 0.97 },
        expectedMetric: "meanFrictionScore",
        expectedImprovementDelta: 0.01,
        expectedDirection: "lower_better",
        rollbackCriteria: ["ttc_regression"],
      },
    )!;
    persistAdaptation(store, built, {
      engineering: lineage.engineering,
      proposal: lineage.proposal,
      evidence: lineage.evidence,
      architectureSnapshot: lineage.architectureSnapshot,
    });
    const raw = store.getItem(ADAPTATION_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
  });

  it("experience + dev modules introduce no network telemetry", () => {
    for (const rel of ["app/src/dev", "app/src/experience"]) {
      const dir = path.join(root, rel);
      for (const file of readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f))) {
        const src = readFileSync(path.join(dir, file), "utf8");
        expect(src).not.toMatch(/\bfetch\s*\(/);
        expect(src).not.toMatch(/sendBeacon/);
        expect(src).not.toMatch(/XMLHttpRequest/);
        expect(src).not.toMatch(/WebSocket/);
      }
    }
  });

  it("overlay lazy-loads adaptations; shell applies presentation only", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/workspaceAdaptation")');
    expect(dash).toContain("workspace-adaptations");
    const shell = readFileSync(
      path.join(root, "app/src/components/WorkspaceShell.tsx"),
      "utf8",
    );
    expect(shell).toContain("useResolvedPresentation");
    expect(shell).toContain("presentationToShellStyle");
    expect(shell).toContain("data-adapt-motion");
  });

  it("architecture doc records schema, resolver, rollback", () => {
    const doc = readFileSync(
      path.join(root, "architecture/48_Adaptive_Workspace.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspaceAdaptation");
    expect(doc).toContain("resolvePresentationConfiguration");
    expect(doc).toContain("Rollback model");
    expect(doc).toMatch(/Evidence only|no design recommendations/i);
  });
});
