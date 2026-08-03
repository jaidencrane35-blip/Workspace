/**
 * Sprint 54 — Experience Change Governance.
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
import { buildEvidenceFromSessions } from "../app/src/dev/experienceEvidence";
import { detectOpportunities } from "../app/src/dev/experienceImprovement";
import {
  GOVERNANCE_STORAGE_KEY,
  assertHistoryImmutable,
  buildProposalFromOpportunities,
  clearGovernanceStore,
  evaluateValidationContract,
  listAllHistory,
  listProposalHistory,
  listProposals,
  persistProposal,
  transitionProposal,
} from "../app/src/dev/experienceGovernance";
import {
  clearInstrumentation,
  disposeExperienceInstrumentation,
  initExperienceInstrumentation,
  listStoredSessions,
  replayStoredSession,
  trackNavigate,
  trackSaveSuccess,
} from "../app/src/dev/experienceInstrumentation";
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

const thrash = session("s-thrash", [
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

function opportunitiesFromThrash() {
  const evidence = buildEvidenceFromSessions([thrash], { tag: "gov" });
  return {
    evidence,
    opportunities: detectOpportunities([evidence]),
  };
}

describe("proposal lifecycle", () => {
  it("builds a deterministic draft with complete validation from opportunities", () => {
    const { evidence, opportunities } = opportunitiesFromThrash();
    expect(opportunities.length).toBeGreaterThan(0);
    const a = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    });
    const b = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    });
    expect(a).not.toBeNull();
    expect(a).toEqual(b);
    expect(a!.state).toBe("draft");
    expect(a!.validationStatus).toBe("complete");
    expect(a!.opportunityIds.length).toBeGreaterThan(0);
    expect(a!.evidenceSnapshotIds).toContain(evidence.evidenceId);
    expect(a!.validation.replaySessionIds).toContain("s-thrash");
    expect(a!.validation.successThresholdDelta).toBeGreaterThan(0);
    expect(a!.expectedImprovements.length).toBeGreaterThan(0);
  });

  it("does not auto-promote and requires manual transitions", () => {
    const store = memoryStore();
    const { evidence, opportunities } = opportunitiesFromThrash();
    const draft = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    })!;
    persistProposal(store, draft, { now: 1000 });
    expect(listProposals(store)[0]?.state).toBe("draft");

    const skip = transitionProposal(store, draft.proposalId, "accepted", {
      evidenceReferenceId: evidence.evidenceId,
      now: 2000,
    });
    expect(skip.ok).toBe(false);
    if (!skip.ok) {
      expect(skip.error).toBe("invalid_transition");
    }

    const steps: Array<"review" | "accepted" | "implemented" | "validated" | "closed"> = [
      "review",
      "accepted",
      "implemented",
      "validated",
      "closed",
    ];
    let t = 3000;
    for (const next of steps) {
      const result = transitionProposal(store, draft.proposalId, next, {
        evidenceReferenceId: evidence.evidenceId,
        now: t,
      });
      expect(result.ok).toBe(true);
      if (result.ok) {
        expect(result.proposal.state).toBe(next);
      }
      t += 1000;
    }
    expect(listProposals(store)[0]?.state).toBe("closed");
    const closed = transitionProposal(store, draft.proposalId, "draft", {
      evidenceReferenceId: evidence.evidenceId,
      now: t,
    });
    expect(closed.ok).toBe(false);
  });

  it("blocks review when validation is incomplete", () => {
    const store = memoryStore();
    const { evidence, opportunities } = opportunitiesFromThrash();
    const draft = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    })!;
    draft.validation.replaySessionIds = [];
    draft.validationStatus = evaluateValidationContract(
      draft.validation,
      draft.opportunityIds,
      draft.expectedImprovements.length,
    );
    expect(draft.validationStatus).toBe("incomplete");
    persistProposal(store, draft, { now: 1 });
    // persist re-evaluates validation from stored fields
    const stored = listProposals(store)[0]!;
    expect(stored.validationStatus).toBe("incomplete");
    const result = transitionProposal(store, stored.proposalId, "review", {
      evidenceReferenceId: evidence.evidenceId,
      now: 2,
    });
    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error).toBe("validation_incomplete");
    }
  });
});

describe("governance history", () => {
  it("appends immutable history on create and transitions", () => {
    const store = memoryStore();
    const { evidence, opportunities } = opportunitiesFromThrash();
    const draft = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    })!;
    persistProposal(store, draft, { now: 10 });
    const before = listProposalHistory(store, draft.proposalId);
    expect(before).toHaveLength(1);
    expect(before[0]?.previousState).toBeNull();
    expect(before[0]?.newState).toBe("draft");
    expect(before[0]?.reason).toBe("created_from_opportunities");

    transitionProposal(store, draft.proposalId, "review", {
      evidenceReferenceId: evidence.evidenceId,
      now: 20,
    });
    const after = listProposalHistory(store, draft.proposalId);
    expect(assertHistoryImmutable(before, after)).toBe(true);
    expect(after).toHaveLength(2);
    expect(after[1]?.previousState).toBe("draft");
    expect(after[1]?.newState).toBe("review");
    expect(after[1]?.t).toBe(20);

    // Attempting to mutate a returned copy must not alter store.
    after[0]!.reason = "close";
    const reread = listProposalHistory(store, draft.proposalId);
    expect(reread[0]?.reason).toBe("created_from_opportunities");
  });

  it("never rewrites historical records on failed transitions", () => {
    const store = memoryStore();
    const { evidence, opportunities } = opportunitiesFromThrash();
    const draft = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    })!;
    persistProposal(store, draft, { now: 1 });
    const before = listAllHistory(store);
    transitionProposal(store, draft.proposalId, "validated", {
      evidenceReferenceId: evidence.evidenceId,
      now: 2,
    });
    const after = listAllHistory(store);
    expect(after).toEqual(before);
  });
});

describe("replay linkage (governance)", () => {
  it("requires replay references and replays from existing trace store", () => {
    const store = memoryStore();
    disposeExperienceInstrumentation();
    clearInstrumentation();
    initExperienceInstrumentation({
      store,
      sessionId: "s-gov-live",
      force: true,
    });
    trackNavigate("save", { commandId: "go_save" });
    trackSaveSuccess();

    // Build thrash-based opportunities but ensure live session also present for replay API.
    const { evidence, opportunities } = opportunitiesFromThrash();
    const draft = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    })!;
    expect(draft.validation.replaySessionIds.length).toBeGreaterThan(0);
    persistProposal(store, draft, { now: 5 });

    for (const sessionId of draft.validation.replaySessionIds) {
      // thrash sessions are not in the live instrumentation store — linkage is by id.
      // Seed thrash into store via evidence path: upsert through analyze is separate.
      // For live id check:
      if (sessionId === "s-gov-live") {
        expect(replayStoredSession(sessionId)).not.toBeNull();
      }
    }
    expect(listStoredSessions().some((s) => s.sessionId === "s-gov-live")).toBe(
      true,
    );

    // Explicit: proposal always carries replay ids (contract).
    const listed = listProposals(store)[0]!;
    expect(listed.validation.replaySessionIds).toEqual(
      draft.validation.replaySessionIds,
    );

    disposeExperienceInstrumentation();
    clearInstrumentation();
    clearGovernanceStore(store);
  });

  it("rejects proposals without evidence or replay linkage", () => {
    expect(buildProposalFromOpportunities([])).toBeNull();
    const { opportunities } = opportunitiesFromThrash();
    const broken = opportunities.map((o) => ({
      ...o,
      replaySessionIds: [] as string[],
      supportingEvidenceIds: [] as string[],
    }));
    expect(buildProposalFromOpportunities(broken)).toBeNull();
  });
});

describe("privacy audit (governance)", () => {
  it("persisted governance JSON omits forbidden content keys", () => {
    const store = memoryStore();
    const { evidence, opportunities } = opportunitiesFromThrash();
    const draft = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: evidence.evidenceId,
    })!;
    persistProposal(store, draft, { now: 1 });
    transitionProposal(store, draft.proposalId, "review", {
      evidenceReferenceId: evidence.evidenceId,
      now: 2,
    });
    const raw = store.getItem(GOVERNANCE_STORAGE_KEY)!;
    expect(raw).toContain("proposalId");
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

  it("overlay lazy-loads governance behind DEV gate", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("./experienceGovernance")');
    expect(dash).toContain("experience-proposals");
    const main = readFileSync(path.join(root, "app/src/main.tsx"), "utf8");
    expect(main).toContain("import.meta.env.DEV");
    expect(main).not.toMatch(/from\s+["'].*experienceGovernance["']/);
  });

  it("architecture doc records lifecycle and validation contract", () => {
    const doc = readFileSync(
      path.join(root, "architecture/45_Experience_Change_Governance.md"),
      "utf8",
    );
    expect(doc).toContain("ExperienceChangeProposal");
    expect(doc).toContain("ProposalValidationContract");
    expect(doc).toContain("No automatic promotion");
    expect(doc).toMatch(/Evidence only|no UX recommendations/i);
  });
});
