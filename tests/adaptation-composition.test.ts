/**
 * Sprint 62 — Adaptation composition.
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
  AUTHORITY_CHAIN,
} from "../app/src/dev/architecturalIntegrity";
import { memoryStore } from "../app/src/dev/experienceStore";
import {
  ADAPTATION_STORAGE_KEY,
  activateAdaptation,
  buildAdaptationFromLineage,
  listAdaptations,
  resolvePresentationConfiguration,
  upsertValidatedAdaptation,
  validateAdaptationEvidence,
  type WorkspaceAdaptation,
} from "../app/src/experience/workspaceAdaptation";
import {
  analyzeAdaptationConflicts,
  composeAdaptations,
  selectComposableAdaptations,
  validateComposition,
} from "../app/src/experience/adaptationComposition";

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

const thrash = session("s-comp-thrash", [
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

const calm = session("s-comp-calm", [
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
  const evidence = buildEvidenceFromSessions([thrash], { tag: "comp" });
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
    commits: ["39b30dc"],
    architectureDocuments: [
      "52_First_Production_Adaptation.md",
      "53_Adaptation_Composition.md",
    ],
    affectedModules: ["app/src/experience/adaptationComposition.ts"],
    affectedTests: ["tests/adaptation-composition.test.ts"],
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
          authorityReference: "53_Adaptation_Composition.md",
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
    afterEvidence: buildEvidenceFromSessions([calm], { tag: "comp_after" }),
    proposal: listProposals(store)[0]!,
    engineering: listEngineeringRecords(store)[0]!,
    architectureSnapshot: listArchitectureSnapshots(store)[0]!,
  };
}

function activateBuilt(
  store: ReturnType<typeof memoryStore>,
  lineage: ReturnType<typeof seedLineage>,
  presentation: WorkspaceAdaptation["presentation"],
  scopes: WorkspaceAdaptation["scopes"],
  targets: WorkspaceAdaptation["targetComponents"],
): WorkspaceAdaptation {
  const built = buildAdaptationFromLineage(
    {
      engineering: lineage.engineering,
      proposal: lineage.proposal,
      evidence: lineage.evidence,
      architectureSnapshot: lineage.architectureSnapshot,
    },
    {
      targetComponents: targets,
      scopes,
      presentation,
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
  expect(validated.validationResult).toBe("passed");
  upsertValidatedAdaptation(store, validated.adaptation);
  expect(activateAdaptation(store, validated.adaptation.adaptationId).ok).toBe(
    true,
  );
  return listAdaptations(store).find(
    (a) => a.adaptationId === validated.adaptation.adaptationId,
  )!;
}

describe("adaptation composition", () => {
  it("composes multiple active adaptations deterministically regardless of input order", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    const a = activateBuilt(
      store,
      lineage,
      {
        density: "focus",
        spacingScale: 0.9,
        motionProfile: "reduced",
      },
      ["spacing", "density", "motion"],
      ["shell"],
    );
    const b = activateBuilt(
      store,
      lineage,
      {
        density: "balanced",
        spacingScale: 0.95,
        environmentalWeight: 0.9,
        motionProfile: "expressive",
      },
      ["density", "environment", "motion"],
      ["shell", "canvas"],
    );

    const list = listAdaptations(store);
    const forward = composeAdaptations(list);
    const reversed = composeAdaptations([...list].reverse());
    expect(forward.presentation).toEqual(reversed.presentation);
    expect(forward.compositionOrder).toEqual(reversed.compositionOrder);
    expect(forward.conflictReport).toEqual(reversed.conflictReport);

    // Order is lexicographic adaptationId — not registration order.
    expect(forward.compositionOrder).toEqual(
      [a.adaptationId, b.adaptationId].sort((x, y) => x.localeCompare(y)),
    );
    expect(forward.presentation.appliedAdaptationIds).toEqual(
      forward.compositionOrder,
    );
    expect(resolvePresentationConfiguration(list)).toEqual(
      forward.presentation,
    );

    // Later id wins density/motion.
    const winner = forward.compositionOrder[forward.compositionOrder.length - 1]!;
    const winnerAdapt = list.find((x) => x.adaptationId === winner)!;
    expect(forward.presentation.density).toBe(winnerAdapt.presentation.density);
    expect(forward.presentation.motionProfile).toBe(
      winnerAdapt.presentation.motionProfile,
    );
  });

  it("reports overlapping targets and contradictory presentation values", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    activateBuilt(
      store,
      lineage,
      {
        density: "focus",
        motionProfile: "reduced",
        spacingScale: 0.9,
        environmentalWeight: 1.1,
      },
      ["density", "motion", "spacing", "environment"],
      ["shell"],
    );
    activateBuilt(
      store,
      lineage,
      {
        density: "balanced",
        motionProfile: "expressive",
        spacingScale: 1.1,
        environmentalWeight: 0.85,
        groupingTightness: 0.7,
      },
      ["density", "motion", "spacing", "environment", "grouping"],
      ["shell", "home"],
    );

    const ordered = selectComposableAdaptations(listAdaptations(store));
    const report = analyzeAdaptationConflicts(ordered);
    expect(report.compatible).toBe(false);
    expect(report.conflicts.some((c) => c.cause === "overlapping_targets")).toBe(
      true,
    );
    expect(
      report.conflicts.some((c) => c.cause === "contradictory_density"),
    ).toBe(true);
    expect(
      report.conflicts.some((c) => c.cause === "contradictory_motion"),
    ).toBe(true);
    expect(
      report.conflicts.some((c) => c.cause === "contradictory_spacing"),
    ).toBe(true);
    expect(
      report.conflicts.some((c) => c.cause === "contradictory_environment"),
    ).toBe(true);

    for (const conflict of report.conflicts) {
      expect(conflict.adaptationIds[0] <= conflict.adaptationIds[1]).toBe(true);
      expect(conflict.resolutionStrategy).toBeTruthy();
      if (
        conflict.resolutionStrategy === "priority_replace" &&
        conflict.cause !== "overlapping_targets"
      ) {
        expect(conflict.winnerAdaptationId).toBeTruthy();
      }
    }
  });

  it("marks compatible when presentations do not contradict", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    activateBuilt(
      store,
      lineage,
      { spacingScale: 0.94 },
      ["spacing"],
      ["shell"],
    );
    activateBuilt(
      store,
      lineage,
      { emphasisScale: 1.05 },
      ["emphasis"],
      ["canvas"],
    );
    const composition = composeAdaptations(listAdaptations(store));
    // Different targets → may still have no contradictory field conflicts.
    expect(
      composition.conflictReport.conflicts.every(
        (c) => c.cause === "overlapping_targets",
      ),
    ).toBe(true);
    // Non-overlapping targets → compatible or only non-field conflicts.
    expect(composition.conflictReport.conflicts.length).toBe(0);
    expect(composition.conflictReport.compatible).toBe(true);
    expect(composition.presentation.spacingScale).toBeCloseTo(0.94, 4);
    expect(composition.presentation.emphasisScale).toBeCloseTo(1.05, 4);
  });
});

describe("composition validation", () => {
  it("validates composed governance lineage", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    activateBuilt(
      store,
      lineage,
      { spacingScale: 0.92 },
      ["spacing"],
      ["shell"],
    );
    activateBuilt(
      store,
      lineage,
      { environmentalWeight: 0.96 },
      ["environment"],
      ["canvas"],
    );
    const validation = validateComposition(store);
    expect(validation.validationResult).toBe("passed");
    expect(validation.governanceIntact).toBe(true);
    expect(validation.missingLineageAdaptationIds).toEqual([]);
    expect(validation.composition.lineage.proposalIds).toContain(
      lineage.proposal.proposalId,
    );
    expect(validation.composition.lineage.engineeringChangeIds).toContain(
      lineage.engineering.changeId,
    );
    expect(validation.composition.compositionOrder.length).toBe(2);
  });
});

describe("governance + authority", () => {
  it("extends authority chain through composition doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "52_First_Production_Adaptation.md" &&
          b === "53_Adaptation_Composition.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (adaptation composition)", () => {
  it("composition is derived — does not add forbidden keys to adaptation store", () => {
    const store = memoryStore();
    const lineage = seedLineage(store);
    activateBuilt(
      store,
      lineage,
      { density: "focus", motionProfile: "reduced" },
      ["density", "motion"],
      ["shell"],
    );
    activateBuilt(
      store,
      lineage,
      { density: "balanced", motionProfile: "standard" },
      ["density", "motion"],
      ["shell"],
    );
    composeAdaptations(listAdaptations(store));
    validateComposition(store);
    expect(store.getItem("ws.experience.adaptation.composition.v1")).toBeNull();
    const raw = store.getItem(ADAPTATION_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
  });

  it("experience modules introduce no network telemetry", () => {
    const dir = path.join(root, "app/src/experience");
    for (const file of readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f))) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });

  it("overlay lazy-loads composition module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/adaptationComposition")');
    expect(dash).toContain("adaptation-composition");
    expect(dash).toContain("composition-order");
    expect(dash).toContain("composition-conflicts");
  });

  it("architecture doc records composition and conflict model", () => {
    const doc = readFileSync(
      path.join(root, "architecture/53_Adaptation_Composition.md"),
      "utf8",
    );
    expect(doc).toContain("composeAdaptations");
    expect(doc).toContain("AdaptationConflictReport");
    expect(doc).toContain("priority_replace");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
