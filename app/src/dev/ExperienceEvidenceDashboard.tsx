/**
 * Development-only Experience Evidence viewer.
 * Never mounted in production builds (dynamic import + DEV gate).
 * Improvement engine modules lazy-load when the overlay opens.
 */

import { useCallback, useEffect, useMemo, useState, type CSSProperties } from "react";
import {
  buildEvidenceFromSessions,
  compareEvidence,
  defaultEvidenceStore,
  getEvidenceBaseline,
  getReplayInvocationCount,
  listEvidenceSnapshots,
  persistEvidenceSnapshot,
  recordReplayInvocation,
  setEvidenceBaseline,
  type EvidenceComparison,
  type ExperienceEvidence,
} from "./experienceEvidence";
import { listStoredSessions, replayStoredSession } from "./experienceInstrumentation";
import { isExperienceValidationEnabled } from "./experienceValidationGate";
import type {
  BaselineEvolution,
  ExperienceOpportunity,
} from "./experienceImprovement";
import type {
  ExperienceChangeProposal,
  GovernanceHistoryEntry,
  ProposalLifecycle,
} from "./experienceGovernance";
import type {
  EngineeringChangeRecord,
  EngineeringHistoryEntry,
  EngineeringLifecycle,
  ReleaseTraceability,
} from "./engineeringGovernance";
import type {
  ArchitectureGraph,
  ArchitectureSnapshot,
  IntegrityResult,
} from "./architecturalIntegrity";
import type {
  AdaptationStabilityReport,
  LongitudinalAdaptationRecord,
  WorkspaceAdaptation,
} from "../experience/workspaceAdaptation";
import type {
  AdaptationExperimentResult,
  ExperimentRunSummary,
} from "../experience/adaptationExperiments";
import type {
  AdaptationCatalog,
  AdaptationOperationalHealth,
  BatchValidationReport,
} from "../experience/adaptationOperations";
import type { ProductionActivationRecord } from "../experience/productionAdaptation";
import type {
  AdaptationConflictReport,
  CompositionResult,
  CompositionValidationReport,
} from "../experience/adaptationComposition";
import type {
  AdaptationCertification,
  CertificationComparison,
  CertificationGateResult,
} from "../experience/adaptationCertification";
import type {
  PackCertificationResult,
  WorkspaceAdaptationPack,
} from "../experience/adaptationPacks";
import { fnv1a } from "./devHash";

const panelStyle: CSSProperties = {
  position: "fixed",
  right: 12,
  bottom: 12,
  zIndex: 2147483000,
  fontFamily: "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace",
  fontSize: 11,
  color: "#e8e8e8",
  background: "rgba(18, 20, 24, 0.94)",
  border: "1px solid rgba(255,255,255,0.12)",
  borderRadius: 6,
  boxShadow: "0 8px 24px rgba(0,0,0,0.35)",
  maxWidth: 480,
  maxHeight: "74vh",
  overflow: "auto",
  padding: 10,
};

const btnStyle: CSSProperties = {
  font: "inherit",
  color: "#e8e8e8",
  background: "rgba(255,255,255,0.08)",
  border: "1px solid rgba(255,255,255,0.16)",
  borderRadius: 4,
  padding: "4px 8px",
  cursor: "pointer",
  marginRight: 6,
  marginBottom: 6,
};

function verdictColor(v: string): string {
  if (v === "improved" || v === "improving") return "#6dcea0";
  if (v === "regressed" || v === "degrading") return "#e08a8a";
  return "#a0a0a0";
}

function severityColor(s: string): string {
  if (s === "high") return "#e08a8a";
  if (s === "medium") return "#e0c56d";
  return "#a0a0a0";
}

type ImprovementApi = typeof import("./experienceImprovement");
type GovernanceApi = typeof import("./experienceGovernance");
type EngineeringApi = typeof import("./engineeringGovernance");
type IntegrityApi = typeof import("./architecturalIntegrity");
type AdaptationApi = typeof import("../experience/workspaceAdaptation");
type ExperimentApi = typeof import("../experience/adaptationExperiments");
type LongitudinalApi = typeof import("../experience/longitudinalAdaptation");
type OperationsApi = typeof import("../experience/adaptationOperations");
type ProductionApi = typeof import("../experience/productionAdaptation");
type CompositionApi = typeof import("../experience/adaptationComposition");
type CertificationApi = typeof import("../experience/adaptationCertification");
type PacksApi = typeof import("../experience/adaptationPacks");
type ValidationApi = typeof import("../experience/adaptationValidation");
type EvolutionApi = typeof import("../experience/workspaceMemoryEvolution");
type PresenceApi = typeof import("../experience/workspacePresence");

const NEXT_STATE: Partial<Record<ProposalLifecycle, ProposalLifecycle>> = {
  draft: "review",
  review: "accepted",
  accepted: "implemented",
  implemented: "validated",
  validated: "closed",
};

const ENG_NEXT_STATE: Partial<
  Record<EngineeringLifecycle, EngineeringLifecycle>
> = {
  draft: "implemented",
  implemented: "validated",
  validated: "architecturally_accepted",
  architecturally_accepted: "released",
};

export function ExperienceEvidenceDashboard() {
  const [open, setOpen] = useState(false);
  const [tick, setTick] = useState(0);
  const [improvementApi, setImprovementApi] = useState<ImprovementApi | null>(
    null,
  );
  const [governanceApi, setGovernanceApi] = useState<GovernanceApi | null>(
    null,
  );
  const [engineeringApi, setEngineeringApi] = useState<EngineeringApi | null>(
    null,
  );
  const [integrityApi, setIntegrityApi] = useState<IntegrityApi | null>(null);
  const [adaptationApi, setAdaptationApi] = useState<AdaptationApi | null>(
    null,
  );
  const [experimentApi, setExperimentApi] = useState<ExperimentApi | null>(
    null,
  );
  const [longitudinalApi, setLongitudinalApi] =
    useState<LongitudinalApi | null>(null);
  const [operationsApi, setOperationsApi] = useState<OperationsApi | null>(
    null,
  );
  const [productionApi, setProductionApi] = useState<ProductionApi | null>(
    null,
  );
  const [compositionApi, setCompositionApi] =
    useState<CompositionApi | null>(null);
  const [certificationApi, setCertificationApi] =
    useState<CertificationApi | null>(null);
  const [packsApi, setPacksApi] = useState<PacksApi | null>(null);
  const [validationApi, setValidationApi] = useState<ValidationApi | null>(
    null,
  );
  const [evolutionApi, setEvolutionApi] = useState<EvolutionApi | null>(null);
  const [presenceApi, setPresenceApi] = useState<PresenceApi | null>(null);
  const [batchReport, setBatchReport] = useState<BatchValidationReport | null>(
    null,
  );
  const [lastCertGate, setLastCertGate] =
    useState<CertificationGateResult | null>(null);
  const [lastPackGate, setLastPackGate] =
    useState<PackCertificationResult | null>(null);
  const [exploreNode, setExploreNode] = useState<string>(
    "doc:40_Experience_Refoundation.md",
  );
  const [lastReplay, setLastReplay] = useState<string | null>(null);
  const [govMessage, setGovMessage] = useState<string | null>(null);
  const store = useMemo(() => defaultEvidenceStore(), []);
  const govStore = useMemo(() => {
    // Same adapter medium; governance / engineering keys are separate.
    return store;
  }, [store]);

  const refresh = useCallback(() => setTick((n) => n + 1), []);

  useEffect(() => {
    if (!open) {
      return;
    }
    let cancelled = false;
    if (!improvementApi) {
      void import("./experienceImprovement").then((mod) => {
        if (!cancelled) {
          setImprovementApi(mod);
        }
      });
    }
    if (!governanceApi) {
      void import("./experienceGovernance").then((mod) => {
        if (!cancelled) {
          setGovernanceApi(mod);
        }
      });
    }
    if (!engineeringApi) {
      void import("./engineeringGovernance").then((mod) => {
        if (!cancelled) {
          setEngineeringApi(mod);
        }
      });
    }
    if (!integrityApi) {
      void import("./architecturalIntegrity").then((mod) => {
        if (!cancelled) {
          setIntegrityApi(mod);
        }
      });
    }
    if (!adaptationApi) {
      void import("../experience/workspaceAdaptation").then((mod) => {
        if (!cancelled) {
          setAdaptationApi(mod);
        }
      });
    }
    if (!experimentApi) {
      void import("../experience/adaptationExperiments").then((mod) => {
        if (!cancelled) {
          setExperimentApi(mod);
        }
      });
    }
    if (!longitudinalApi) {
      void import("../experience/longitudinalAdaptation").then((mod) => {
        if (!cancelled) {
          setLongitudinalApi(mod);
        }
      });
    }
    if (!operationsApi) {
      void import("../experience/adaptationOperations").then((mod) => {
        if (!cancelled) {
          setOperationsApi(mod);
        }
      });
    }
    if (!productionApi) {
      void import("../experience/productionAdaptation").then((mod) => {
        if (!cancelled) {
          setProductionApi(mod);
        }
      });
    }
    if (!compositionApi) {
      void import("../experience/adaptationComposition").then((mod) => {
        if (!cancelled) {
          setCompositionApi(mod);
        }
      });
    }
    if (!certificationApi) {
      void import("../experience/adaptationCertification").then((mod) => {
        if (!cancelled) {
          setCertificationApi(mod);
        }
      });
    }
    if (!packsApi) {
      void import("../experience/adaptationPacks").then((mod) => {
        if (!cancelled) {
          setPacksApi(mod);
        }
      });
    }
    if (!validationApi) {
      void import("../experience/adaptationValidation").then((mod) => {
        if (!cancelled) {
          setValidationApi(mod);
        }
      });
    }
    if (!evolutionApi) {
      void import("../experience/workspaceMemoryEvolution").then((mod) => {
        if (!cancelled) {
          setEvolutionApi(mod);
        }
      });
    }
    if (!presenceApi) {
      void import("../experience/workspacePresence").then((mod) => {
        if (!cancelled) {
          setPresenceApi(mod);
        }
      });
    }
    return () => {
      cancelled = true;
    };
  }, [
    open,
    improvementApi,
    governanceApi,
    engineeringApi,
    integrityApi,
    adaptationApi,
    experimentApi,
    longitudinalApi,
    operationsApi,
    productionApi,
    compositionApi,
    certificationApi,
    packsApi,
    validationApi,
    evolutionApi,
    presenceApi,
  ]);

  const sessions = useMemo(() => {
    void tick;
    return listStoredSessions();
  }, [tick]);

  const snapshots = useMemo(() => {
    void tick;
    return listEvidenceSnapshots(store);
  }, [store, tick]);

  const baseline = useMemo(() => {
    void tick;
    return getEvidenceBaseline(store);
  }, [store, tick]);

  const replayCount = useMemo(() => {
    void tick;
    return getReplayInvocationCount(store);
  }, [store, tick]);

  const latest = snapshots[snapshots.length - 1] ?? null;

  const comparison: EvidenceComparison | null = useMemo(() => {
    if (!baseline || !latest || baseline.evidenceId === latest.evidenceId) {
      return null;
    }
    return compareEvidence(baseline, latest);
  }, [baseline, latest]);

  const opportunities: ExperienceOpportunity[] = useMemo(() => {
    if (!improvementApi || snapshots.length === 0) {
      return [];
    }
    return improvementApi.detectOpportunities(snapshots);
  }, [improvementApi, snapshots]);

  const evolution: BaselineEvolution | null = useMemo(() => {
    if (!improvementApi || snapshots.length === 0) {
      return null;
    }
    return improvementApi.evolveBaselines(snapshots);
  }, [improvementApi, snapshots]);

  const proposals: ExperienceChangeProposal[] = useMemo(() => {
    void tick;
    if (!governanceApi) {
      return [];
    }
    return governanceApi.listProposals(govStore);
  }, [governanceApi, govStore, tick]);

  const historyByProposal = useMemo(() => {
    void tick;
    if (!governanceApi) {
      return new Map<string, GovernanceHistoryEntry[]>();
    }
    const map = new Map<string, GovernanceHistoryEntry[]>();
    for (const p of proposals) {
      map.set(p.proposalId, governanceApi.listProposalHistory(govStore, p.proposalId));
    }
    return map;
  }, [governanceApi, govStore, proposals, tick]);

  const engContext = useMemo(
    () => ({
      proposals,
      evidenceIds: snapshots.map((s) => s.evidenceId),
    }),
    [proposals, snapshots],
  );

  const engineeringRecords: EngineeringChangeRecord[] = useMemo(() => {
    void tick;
    if (!engineeringApi) {
      return [];
    }
    return engineeringApi.listEngineeringRecords(govStore);
  }, [engineeringApi, govStore, tick]);

  const engHistoryByChange = useMemo(() => {
    void tick;
    if (!engineeringApi) {
      return new Map<string, EngineeringHistoryEntry[]>();
    }
    const map = new Map<string, EngineeringHistoryEntry[]>();
    for (const r of engineeringRecords) {
      map.set(
        r.changeId,
        engineeringApi.listEngineeringHistory(govStore, r.changeId),
      );
    }
    return map;
  }, [engineeringApi, engineeringRecords, govStore, tick]);

  const traceabilityByChange = useMemo(() => {
    if (!engineeringApi) {
      return new Map<string, ReleaseTraceability>();
    }
    const map = new Map<string, ReleaseTraceability>();
    for (const r of engineeringRecords) {
      map.set(
        r.changeId,
        engineeringApi.buildReleaseTraceability(r, engContext),
      );
    }
    return map;
  }, [engineeringApi, engineeringRecords, engContext]);

  const graphSource = useMemo(
    () => ({
      engineeringRecords,
      proposals,
      opportunities,
      evidence: snapshots,
    }),
    [engineeringRecords, proposals, opportunities, snapshots],
  );

  const architectureGraph: ArchitectureGraph | null = useMemo(() => {
    if (!integrityApi) {
      return null;
    }
    return integrityApi.buildArchitectureGraph(graphSource);
  }, [integrityApi, graphSource]);

  const integrityResult: IntegrityResult | null = useMemo(() => {
    if (!integrityApi || !architectureGraph) {
      return null;
    }
    return integrityApi.validateArchitectureIntegrity(architectureGraph, {
      engineeringRecords,
      proposals,
    });
  }, [integrityApi, architectureGraph, engineeringRecords, proposals]);

  const architectureSnapshots: ArchitectureSnapshot[] = useMemo(() => {
    void tick;
    if (!integrityApi) {
      return [];
    }
    return integrityApi.listArchitectureSnapshots(govStore);
  }, [integrityApi, govStore, tick]);

  const authoritySuccessors = useMemo(() => {
    if (!integrityApi || !architectureGraph) {
      return [];
    }
    return integrityApi.listAuthoritySuccessors(
      architectureGraph,
      "40_Experience_Refoundation.md",
    );
  }, [integrityApi, architectureGraph]);

  const exploredDeps = useMemo(() => {
    if (!integrityApi || !architectureGraph) {
      return [];
    }
    return integrityApi.listDependencies(architectureGraph, exploreNode);
  }, [integrityApi, architectureGraph, exploreNode]);

  const adaptations: WorkspaceAdaptation[] = useMemo(() => {
    void tick;
    if (!adaptationApi) {
      return [];
    }
    return adaptationApi.listAdaptations(govStore);
  }, [adaptationApi, govStore, tick]);

  const resolvedPresentation = useMemo(() => {
    if (!adaptationApi) {
      return null;
    }
    return adaptationApi.resolvePresentationConfiguration(adaptations);
  }, [adaptationApi, adaptations]);

  const experimentSummary: ExperimentRunSummary | null = useMemo(() => {
    void tick;
    if (!experimentApi) {
      return null;
    }
    return experimentApi.getExperimentSummary(govStore);
  }, [experimentApi, govStore, tick]);

  const experimentResults: AdaptationExperimentResult[] = useMemo(() => {
    void tick;
    if (!experimentApi) {
      return [];
    }
    return experimentApi.listExperimentResults(govStore);
  }, [experimentApi, govStore, tick]);

  const longitudinalRecords: LongitudinalAdaptationRecord[] = useMemo(() => {
    void tick;
    if (!longitudinalApi) {
      return [];
    }
    return longitudinalApi.listLongitudinalRecords(govStore);
  }, [longitudinalApi, govStore, tick]);

  const stabilityReports: AdaptationStabilityReport[] = useMemo(() => {
    void tick;
    if (!longitudinalApi) {
      return [];
    }
    return longitudinalApi.listStabilityReports(govStore);
  }, [longitudinalApi, govStore, tick]);

  const rolloutCandidates: WorkspaceAdaptation[] = useMemo(() => {
    void tick;
    if (!longitudinalApi) {
      return [];
    }
    return longitudinalApi.listRolloutCandidates(govStore);
  }, [longitudinalApi, govStore, tick]);

  const adaptationCatalog: AdaptationCatalog | null = useMemo(() => {
    void tick;
    if (!operationsApi) {
      return null;
    }
    return operationsApi.buildAdaptationCatalog(govStore);
  }, [operationsApi, govStore, tick]);

  const operationalHealth: AdaptationOperationalHealth | null = useMemo(() => {
    void tick;
    if (!operationsApi) {
      return null;
    }
    return operationsApi.deriveOperationalHealth(govStore);
  }, [operationsApi, govStore, tick]);

  const productionRecord: ProductionActivationRecord | null = useMemo(() => {
    void tick;
    if (!productionApi) {
      return null;
    }
    return productionApi.getProductionActivationRecord(govStore);
  }, [productionApi, govStore, tick]);

  const activeAdaptations = useMemo(
    () => adaptations.filter((a) => a.rolloutState === "active"),
    [adaptations],
  );

  const compositionResult: CompositionResult | null = useMemo(() => {
    void tick;
    if (!compositionApi) {
      return null;
    }
    return compositionApi.composeAdaptations(adaptations);
  }, [compositionApi, adaptations, tick]);

  const compositionValidation: CompositionValidationReport | null =
    useMemo(() => {
      void tick;
      if (!compositionApi) {
        return null;
      }
      return compositionApi.validateComposition(govStore);
    }, [compositionApi, govStore, tick]);

  const conflictReport: AdaptationConflictReport | null =
    compositionResult?.conflictReport ?? null;

  const certifications: AdaptationCertification[] = useMemo(() => {
    void tick;
    if (!certificationApi) {
      return [];
    }
    return certificationApi.listCertifications(govStore);
  }, [certificationApi, govStore, tick]);

  const currentCertification: AdaptationCertification | null = useMemo(() => {
    void tick;
    if (!certificationApi) {
      return null;
    }
    return certificationApi.getLatestCertification(govStore);
  }, [certificationApi, govStore, tick]);

  const previousCertification: AdaptationCertification | null = useMemo(() => {
    if (certifications.length < 2) {
      return null;
    }
    return certifications[certifications.length - 2]!;
  }, [certifications]);

  const certificationComparison: CertificationComparison | null =
    useMemo(() => {
      void tick;
      if (!certificationApi) {
        return null;
      }
      return certificationApi.compareLatestCertifications(govStore);
    }, [certificationApi, govStore, tick]);

  const adaptationPacks: WorkspaceAdaptationPack[] = useMemo(() => {
    void tick;
    if (!packsApi) {
      return [];
    }
    return packsApi.listAdaptationPacks(govStore);
  }, [packsApi, govStore, tick]);

  const activePack: WorkspaceAdaptationPack | null = useMemo(() => {
    void tick;
    if (!packsApi) {
      return null;
    }
    return packsApi.getActiveAdaptationPack(govStore);
  }, [packsApi, govStore, tick]);

  const realWorldValidation = useMemo(() => {
    void tick;
    if (!validationApi) {
      return null;
    }
    return validationApi.buildRealWorldValidationBundle(govStore, {
      now: Date.now(),
    });
  }, [validationApi, govStore, tick]);

  const memoryEvolutions = useMemo(() => {
    void tick;
    if (!evolutionApi) {
      return [];
    }
    return evolutionApi.listMemoryEvolutions(govStore);
  }, [evolutionApi, govStore, tick]);

  const activeMemoryEvolution = useMemo(() => {
    void tick;
    if (!evolutionApi) {
      return null;
    }
    return evolutionApi.getActiveMemoryEvolution(govStore);
  }, [evolutionApi, govStore, tick]);

  const evolvedPresentation = useMemo(() => {
    void tick;
    if (!evolutionApi) {
      return null;
    }
    return evolutionApi.resolvePresentationFromRuntime(govStore);
  }, [evolutionApi, govStore, tick]);

  const activePresence = useMemo(() => {
    void tick;
    if (!presenceApi) {
      return null;
    }
    return presenceApi.getActiveWorkspacePresence(govStore);
  }, [presenceApi, govStore, tick]);

  const presenceReplay = useMemo(() => {
    void tick;
    if (!presenceApi) {
      return null;
    }
    return presenceApi.replayWorkspacePresence(govStore);
  }, [presenceApi, govStore, tick]);

  const analyze = () => {
    if (sessions.length === 0) {
      return;
    }
    const evidence = buildEvidenceFromSessions(sessions, {
      tag: `agg_${sessions.length}`,
    });
    persistEvidenceSnapshot(store, evidence);
    refresh();
  };

  const markBaseline = (evidence: ExperienceEvidence) => {
    setEvidenceBaseline(store, evidence.evidenceId);
    refresh();
  };

  const replayAll = () => {
    for (const session of sessions) {
      replayStoredSession(session.sessionId);
      recordReplayInvocation(store);
    }
    setLastReplay(`all:${sessions.length}`);
    refresh();
  };

  const replayOpportunity = (opp: ExperienceOpportunity) => {
    const played: string[] = [];
    for (const sessionId of opp.replaySessionIds) {
      const result = replayStoredSession(sessionId);
      if (result) {
        played.push(sessionId);
        recordReplayInvocation(store);
      }
    }
    setLastReplay(`${opp.opportunityId}:${played.join(",")}`);
    refresh();
  };

  const replaySessionLink = (sessionId: string) => {
    const result = replayStoredSession(sessionId);
    if (result) {
      recordReplayInvocation(store);
      setLastReplay(sessionId);
      refresh();
    }
  };

  const draftFromOpportunities = () => {
    if (!governanceApi || opportunities.length === 0) {
      setGovMessage("no_opportunities");
      return;
    }
    const draft = governanceApi.buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId:
        baseline?.evidenceId ?? opportunities[0]?.supportingEvidenceIds[0],
    });
    if (!draft) {
      setGovMessage("proposal_build_failed");
      return;
    }
    governanceApi.persistProposal(govStore, draft);
    setGovMessage(`drafted:${draft.proposalId}`);
    refresh();
  };

  const advanceProposal = (proposal: ExperienceChangeProposal) => {
    if (!governanceApi) {
      return;
    }
    const next = NEXT_STATE[proposal.state];
    if (!next) {
      setGovMessage("no_next_state");
      return;
    }
    const result = governanceApi.transitionProposal(
      govStore,
      proposal.proposalId,
      next,
      {
        evidenceReferenceId:
          proposal.validation.evidenceBaselineId ||
          baseline?.evidenceId ||
          "unknown",
      },
    );
    setGovMessage(
      result.ok ? `${proposal.proposalId}->${next}` : `err:${result.error}`,
    );
    refresh();
  };

  const rejectProposal = (proposal: ExperienceChangeProposal) => {
    if (!governanceApi || proposal.state !== "review") {
      return;
    }
    const result = governanceApi.transitionProposal(
      govStore,
      proposal.proposalId,
      "draft",
      {
        evidenceReferenceId: proposal.validation.evidenceBaselineId,
        reason: "reject_to_draft",
      },
    );
    setGovMessage(
      result.ok ? `${proposal.proposalId}->draft` : `err:${result.error}`,
    );
    refresh();
  };

  const eligibleProposals = proposals.filter((p) =>
    ["accepted", "implemented", "validated", "closed"].includes(p.state),
  );

  const draftEngineeringRecord = () => {
    if (!engineeringApi) {
      setGovMessage("eng_loading");
      return;
    }
    if (eligibleProposals.length === 0) {
      setGovMessage("no_eligible_proposals");
      return;
    }
    const commitPlaceholder = fnv1a(
      eligibleProposals
        .map((p) => p.proposalId)
        .sort()
        .join(","),
    );
    const draft = engineeringApi.buildEngineeringRecordFromProposals(
      eligibleProposals,
      {
        commits: [commitPlaceholder],
        architectureDocuments: [
          "45_Experience_Change_Governance.md",
          "46_Engineering_Governance.md",
          "47_Architectural_Integrity.md",
        ],
        affectedModules: [
          "app/src/dev/engineeringGovernance.ts",
          "app/src/dev/architecturalIntegrity.ts",
        ],
        affectedTests: ["tests/architectural-integrity.test.ts"],
        releaseImpact: "dev_tooling",
      },
    );
    if (!draft) {
      setGovMessage("eng_build_failed");
      return;
    }
    const result = engineeringApi.persistEngineeringRecord(
      govStore,
      draft,
      engContext,
    );
    setGovMessage(
      result.ok
        ? `eng_drafted:${result.record.changeId}`
        : `eng_err:${result.error}`,
    );
    refresh();
  };

  const advanceEngineering = (record: EngineeringChangeRecord) => {
    if (!engineeringApi) {
      return;
    }
    const next = ENG_NEXT_STATE[record.state];
    if (!next) {
      setGovMessage("eng_no_next");
      return;
    }
    const result = engineeringApi.transitionEngineeringRecord(
      govStore,
      record.changeId,
      next,
      engContext,
      {
        authorityReference: "46_Engineering_Governance.md",
      },
    );
    setGovMessage(
      result.ok
        ? `${record.changeId}->${next}`
        : `eng_err:${result.error}`,
    );
    refresh();
  };

  const snapshotArchitecture = () => {
    if (!integrityApi || !architectureGraph) {
      setGovMessage("integrity_loading");
      return;
    }
    const snapshot = integrityApi.createArchitectureSnapshot(architectureGraph, {
      t: Date.now(),
      integrity: integrityResult ?? undefined,
      source: graphSource,
    });
    integrityApi.persistArchitectureSnapshot(govStore, snapshot);
    setGovMessage(`asnap:${snapshot.snapshotId}:${snapshot.integrity.valid ? "ok" : "fail"}`);
    refresh();
  };

  const draftAdaptation = () => {
    if (!adaptationApi || !integrityApi) {
      setGovMessage("adapt_loading");
      return;
    }
    const released = engineeringRecords.filter((r) =>
      ["architecturally_accepted", "released"].includes(r.state),
    );
    const eng = released[released.length - 1];
    const proposal = proposals.find(
      (p) => eng?.proposalIds.includes(p.proposalId),
    );
    const evidence =
      snapshots.find((s) => s.evidenceId === eng?.validationEvidence.evidenceSnapshotIds[0]) ??
      latest;
    let snap = architectureSnapshots[architectureSnapshots.length - 1];
    if (!snap && architectureGraph) {
      snap = integrityApi.createArchitectureSnapshot(architectureGraph, {
        t: Date.now(),
        source: graphSource,
      });
      integrityApi.persistArchitectureSnapshot(govStore, snap);
    }
    if (!eng || !proposal || !evidence || !snap) {
      setGovMessage("adapt_lineage_incomplete");
      return;
    }
    const built = adaptationApi.buildAdaptationFromLineage(
      {
        engineering: eng,
        proposal,
        evidence,
        architectureSnapshot: snap,
      },
      {
        targetComponents: ["shell", "canvas"],
        scopes: ["spacing", "density", "motion", "environment"],
        presentation: {
          density: "balanced",
          spacingScale: 0.94,
          emphasisScale: 1.04,
          groupingTightness: 0.62,
          motionProfile: "standard",
          environmentalWeight: 0.96,
        },
        expectedMetric: "meanFrictionScore",
        expectedImprovementDelta: 0.03,
        expectedDirection: "lower_better",
        rollbackCriteria: ["friction_regression", "abandon_increase"],
      },
    );
    if (!built) {
      setGovMessage("adapt_build_failed");
      return;
    }
    const result = adaptationApi.persistAdaptation(govStore, built, {
      engineering: eng,
      proposal,
      evidence,
      architectureSnapshot: snap,
    });
    setGovMessage(
      result.ok
        ? `adapt_drafted:${result.adaptation.adaptationId}`
        : `adapt_err:${result.error}`,
    );
    refresh();
  };

  const validateAdaptation = (adaptation: WorkspaceAdaptation) => {
    if (!adaptationApi || !latest) {
      return;
    }
    const baseline =
      snapshots.find(
        (s) => s.evidenceId === adaptation.validation.baselineEvidenceId,
      ) ?? snapshots[0];
    if (!baseline) {
      setGovMessage("adapt_no_baseline");
      return;
    }
    // Compare baseline vs latest aggregate as after evidence.
    const result = adaptationApi.validateAdaptationEvidence(
      adaptation,
      baseline,
      latest,
    );
    adaptationApi.upsertValidatedAdaptation(govStore, result.adaptation);
    if (longitudinalApi) {
      longitudinalApi.appendLongitudinalObservation(
        govStore,
        result.adaptation,
        baseline,
      );
      longitudinalApi.appendLongitudinalObservation(
        govStore,
        result.adaptation,
        latest,
      );
    }
    setGovMessage(
      `adapt_val:${adaptation.adaptationId}:${result.validationResult}${result.rollback ? ":rollback" : ""}`,
    );
    refresh();
  };

  const runLongitudinal = (adaptation: WorkspaceAdaptation) => {
    if (!longitudinalApi) {
      setGovMessage("long_loading");
      return;
    }
    if (latest) {
      longitudinalApi.appendLongitudinalObservation(
        govStore,
        adaptation,
        latest,
      );
    }
    const { report } = longitudinalApi.runLongitudinalValidation(
      govStore,
      adaptation,
    );
    setGovMessage(
      `long_val:${adaptation.adaptationId}:score=${report.stabilityScore}:disp=${report.rolloutDisposition}`,
    );
    refresh();
  };

  const promoteRollout = (adaptation: WorkspaceAdaptation) => {
    if (!longitudinalApi) {
      setGovMessage("long_loading");
      return;
    }
    const result = longitudinalApi.promoteToRolloutCandidate(
      govStore,
      adaptation.adaptationId,
    );
    setGovMessage(
      result.ok
        ? `long_promote:${adaptation.adaptationId}`
        : `long_err:${result.error}`,
    );
    refresh();
  };

  const runBatchValidate = () => {
    if (!operationsApi) {
      setGovMessage("ops_loading");
      return;
    }
    const report = operationsApi.runBatchValidation(govStore);
    setBatchReport(report);
    setGovMessage(
      `ops_batch:v=${report.validated.length}:f=${report.failed.length}:u=${report.unchanged.length}:s=${report.skipped.length}`,
    );
    refresh();
  };

  const runProductionActivation = () => {
    if (!productionApi) {
      setGovMessage("prod_loading");
      return;
    }
    const record = productionApi.runFirstProductionAdaptation(govStore, {
      now: Date.now(),
    });
    setGovMessage(
      `prod:${record.outcome}:${record.adaptationId ?? "none"}:${record.blockReasons.join(",") || "ok"}`,
    );
    refresh();
  };

  const rollbackProduction = () => {
    if (!productionApi) {
      setGovMessage("prod_loading");
      return;
    }
    const result = productionApi.rollbackProductionAdaptation(govStore);
    setGovMessage(
      result.ok
        ? `prod_rollback:${result.record.adaptationId}`
        : `prod_err:${result.error}`,
    );
    refresh();
  };

  const runCertification = () => {
    if (!certificationApi) {
      setGovMessage("cert_loading");
      return;
    }
    const result = certificationApi.certifyAdaptationSet(govStore, {
      now: Date.now(),
    });
    setLastCertGate(result);
    setGovMessage(
      result.ok
        ? `cert_ok:${result.certification?.certificationId}`
        : `cert_fail:${result.failureReasons.join(",")}`,
    );
    refresh();
  };

  const runPackCertification = () => {
    if (!packsApi) {
      setGovMessage("pack_loading");
      return;
    }
    const result = packsApi.certifyAdaptationPack(govStore, {
      now: Date.now(),
    });
    setLastPackGate(result);
    setGovMessage(
      result.ok
        ? `pack_ok:${result.pack?.packId}@${result.pack?.version}`
        : `pack_fail:${result.failureReasons.join(",")}`,
    );
    refresh();
  };

  const activatePack = (pack: WorkspaceAdaptationPack) => {
    if (!packsApi) {
      setGovMessage("pack_loading");
      return;
    }
    const result = packsApi.activateAdaptationPack(
      govStore,
      pack.packId,
      pack.version,
    );
    setGovMessage(
      result.ok
        ? `pack_active:${pack.packId}@${pack.version}`
        : `pack_err:${result.error}`,
    );
    refresh();
  };

  const deactivatePack = (pack: WorkspaceAdaptationPack) => {
    if (!packsApi) {
      setGovMessage("pack_loading");
      return;
    }
    const result = packsApi.deactivateAdaptationPack(
      govStore,
      pack.packId,
      pack.version,
    );
    setGovMessage(
      result.ok
        ? `pack_inactive:${pack.packId}@${pack.version}`
        : `pack_err:${result.error}`,
    );
    refresh();
  };

  const activateAdaptation = (adaptation: WorkspaceAdaptation) => {
    if (!adaptationApi) {
      return;
    }
    const result = adaptationApi.activateAdaptation(
      govStore,
      adaptation.adaptationId,
    );
    setGovMessage(
      result.ok
        ? `adapt_active:${adaptation.adaptationId}`
        : `adapt_err:${result.error}`,
    );
    refresh();
  };

  const rollbackAdaptation = (adaptation: WorkspaceAdaptation) => {
    if (!adaptationApi) {
      return;
    }
    const result = adaptationApi.rollbackAdaptation(
      govStore,
      adaptation.adaptationId,
    );
    setGovMessage(
      result.ok
        ? `adapt_rollback:${adaptation.adaptationId}`
        : `adapt_err:${result.error}`,
    );
    refresh();
  };

  const runExperiments = () => {
    if (!experimentApi) {
      setGovMessage("exp_loading");
      return;
    }
    const summary = experimentApi.runAdaptationExperiments(govStore);
    setGovMessage(
      `exp_run:selected=${summary.experimentsSelected}:validated=${summary.results.filter((r) => r.experimentStatus === "validated").length}`,
    );
    refresh();
  };

  const toggleExperiment = (adaptationId: string) => {
    if (!experimentApi || !adaptationId) {
      return;
    }
    const result = experimentApi.toggleAdaptationExperiment(
      govStore,
      adaptationId,
    );
    setGovMessage(
      result.ok
        ? `exp_toggle:${adaptationId}:${result.adaptation.rolloutState}`
        : `exp_err:${result.error}`,
    );
    refresh();
  };

  if (!isExperienceValidationEnabled()) {
    return null;
  }

  if (!open) {
    return (
      <button
        type="button"
        data-testid="experience-evidence-toggle"
        aria-label="Open experience evidence"
        style={{
          ...btnStyle,
          position: "fixed",
          right: 12,
          bottom: 12,
          zIndex: 2147483000,
          margin: 0,
        }}
        onClick={() => setOpen(true)}
      >
        Evidence
      </button>
    );
  }

  return (
    <aside
      data-testid="experience-evidence-dashboard"
      style={panelStyle}
      aria-label="Experience evidence dashboard"
    >
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 8 }}>
        <strong>Experience Evidence</strong>
        <button type="button" style={btnStyle} onClick={() => setOpen(false)}>
          Close
        </button>
      </div>

      <div style={{ marginBottom: 8 }}>
        <button type="button" style={btnStyle} onClick={analyze}>
          Analyze traces
        </button>
        <button type="button" style={btnStyle} onClick={replayAll}>
          Replay sessions
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="draft-proposal"
          onClick={draftFromOpportunities}
        >
          Draft proposal
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="draft-engineering"
          onClick={draftEngineeringRecord}
        >
          Draft engineering
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="snapshot-architecture"
          onClick={snapshotArchitecture}
        >
          Snapshot architecture
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="draft-adaptation"
          onClick={draftAdaptation}
        >
          Draft adaptation
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="run-adaptation-experiments"
          onClick={runExperiments}
        >
          Run experiments
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="run-batch-validation"
          onClick={runBatchValidate}
        >
          Batch validate
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="run-production-adaptation"
          onClick={runProductionActivation}
        >
          Activate production adaptation
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="run-adaptation-certification"
          onClick={runCertification}
        >
          Certify adaptation set
        </button>
        <button
          type="button"
          style={btnStyle}
          data-testid="run-pack-certification"
          onClick={runPackCertification}
        >
          Certify adaptation pack
        </button>
        <button type="button" style={btnStyle} onClick={refresh}>
          Refresh
        </button>
      </div>
      {govMessage ? <div style={{ marginBottom: 8 }}>gov: {govMessage}</div> : null}

      <section style={{ marginBottom: 10 }}>
        <div>Sessions in store: {sessions.length}</div>
        <div>Evidence snapshots: {snapshots.length}</div>
        <div>Replay invocations: {replayCount}</div>
        <div>Baseline: {baseline?.evidenceId ?? "—"}</div>
        {lastReplay ? <div>Last replay: {lastReplay}</div> : null}
      </section>

      {latest ? (
        <section style={{ marginBottom: 10 }}>
          <div style={{ marginBottom: 4 }}>
            <strong>Latest aggregate</strong> ({latest.evidenceId})
          </div>
          <div>median TTC: {latest.metrics.medianTimeToConfidenceMs} ms</div>
          <div>mean friction: {latest.metrics.meanFrictionScore}</div>
          <div>
            abandons: {latest.metrics.abandonedFlowTotal} · loops:{" "}
            {latest.metrics.navigationLoopCount} · hotspots:{" "}
            {latest.metrics.hesitationHotspotCount}
          </div>
          <button
            type="button"
            style={btnStyle}
            onClick={() => markBaseline(latest)}
          >
            Set as baseline
          </button>
        </section>
      ) : (
        <section style={{ marginBottom: 10, opacity: 0.7 }}>
          No evidence snapshots yet. Analyze traces to create one.
        </section>
      )}

      <section style={{ marginBottom: 10 }} data-testid="experience-opportunities">
        <div style={{ marginBottom: 4 }}>
          <strong>Opportunities</strong>
          {!improvementApi ? " (loading…)" : ` (${opportunities.length})`}
        </div>
        {improvementApi && opportunities.length === 0 ? (
          <div style={{ opacity: 0.7 }}>None above thresholds.</div>
        ) : null}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
          {opportunities.map((opp) => (
            <li key={opp.opportunityId} style={{ marginBottom: 8 }}>
              <div style={{ color: severityColor(opp.severity) }}>
                {opp.opportunityId} · {opp.metric} · {opp.workflow} ·{" "}
                {opp.severity}
              </div>
              <div>
                value {opp.observedValue} / thr {opp.threshold} · excess{" "}
                {opp.excess} · n={opp.evidenceCount} · conf {opp.confidence} ·
                repro {opp.reproducibilityScore}
              </div>
              <div>
                evidence: {opp.supportingEvidenceIds.join(", ")}
              </div>
              <div>
                replay:{" "}
                {opp.replaySessionIds.map((id) => (
                  <button
                    key={id}
                    type="button"
                    style={btnStyle}
                    data-testid={`replay-link-${id}`}
                    onClick={() => replaySessionLink(id)}
                  >
                    {id}
                  </button>
                ))}
                <button
                  type="button"
                  style={btnStyle}
                  data-testid={`replay-opp-${opp.opportunityId}`}
                  onClick={() => replayOpportunity(opp)}
                >
                  Replay all
                </button>
              </div>
            </li>
          ))}
        </ul>
      </section>

      <section style={{ marginBottom: 10 }} data-testid="experience-proposals">
        <div style={{ marginBottom: 4 }}>
          <strong>Proposal queue</strong>
          {!governanceApi ? " (loading…)" : ` (${proposals.length})`}
        </div>
        {governanceApi && proposals.length === 0 ? (
          <div style={{ opacity: 0.7 }}>
            Empty. Draft from opportunities after evidence exists.
          </div>
        ) : null}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
          {proposals.map((p) => {
            const hist = historyByProposal.get(p.proposalId) ?? [];
            return (
              <li key={p.proposalId} style={{ marginBottom: 10 }}>
                <div>
                  {p.proposalId} · <strong>{p.state}</strong> · validation{" "}
                  {p.validationStatus} · scope {p.implementationScope} · conf{" "}
                  {p.confidence}
                </div>
                <div>
                  workflows: {p.workflows.join(", ")} · components:{" "}
                  {p.affectedComponents.join(", ")}
                </div>
                <div>
                  opportunities: {p.opportunityIds.join(", ") || "—"}
                </div>
                <div>
                  evidence: {p.evidenceSnapshotIds.join(", ") || "—"} · baseline{" "}
                  {p.validation.evidenceBaselineId}
                </div>
                <div>
                  success: {p.validation.successMetric} Δ≥
                  {p.validation.successThresholdDelta} (
                  {p.validation.successDirection})
                </div>
                <div>
                  replay:{" "}
                  {p.validation.replaySessionIds.map((id) => (
                    <button
                      key={id}
                      type="button"
                      style={btnStyle}
                      onClick={() => replaySessionLink(id)}
                    >
                      {id}
                    </button>
                  ))}
                </div>
                <div>
                  history:{" "}
                  {hist
                    .map(
                      (h) =>
                        `${h.seq}:${h.previousState ?? "∅"}→${h.newState}`,
                    )
                    .join(" · ") || "—"}
                </div>
                <div>
                  {NEXT_STATE[p.state] ? (
                    <button
                      type="button"
                      style={btnStyle}
                      data-testid={`advance-${p.proposalId}`}
                      onClick={() => advanceProposal(p)}
                    >
                      → {NEXT_STATE[p.state]}
                    </button>
                  ) : null}
                  {p.state === "review" ? (
                    <button
                      type="button"
                      style={btnStyle}
                      onClick={() => rejectProposal(p)}
                    >
                      → draft
                    </button>
                  ) : null}
                </div>
              </li>
            );
          })}
        </ul>
      </section>

      <section style={{ marginBottom: 10 }} data-testid="engineering-records">
        <div style={{ marginBottom: 4 }}>
          <strong>Engineering records</strong>
          {!engineeringApi
            ? " (loading…)"
            : ` (${engineeringRecords.length})`}
        </div>
        {engineeringApi && engineeringRecords.length === 0 ? (
          <div style={{ opacity: 0.7 }}>
            Empty. Advance a proposal to accepted+, then draft engineering.
          </div>
        ) : null}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
          {engineeringRecords.map((r) => {
            const hist = engHistoryByChange.get(r.changeId) ?? [];
            const trace = traceabilityByChange.get(r.changeId);
            return (
              <li key={r.changeId} style={{ marginBottom: 10 }}>
                <div>
                  {r.changeId} · <strong>{r.state}</strong> · consistency{" "}
                  {r.consistencyStatus} · impact {r.releaseImpact}
                </div>
                <div>
                  commits: {r.commits.join(", ")} · proposals:{" "}
                  {r.proposalIds.join(", ")}
                </div>
                <div>
                  architecture: {r.architectureDocuments.join(", ")}
                </div>
                <div>
                  modules: {r.affectedModules.join(", ")}
                </div>
                <div>tests: {r.affectedTests.join(", ")}</div>
                <div>
                  validation: proposals=
                  {r.validationEvidence.proposalValidationComplete
                    ? "complete"
                    : "incomplete"}{" "}
                  · evidence{" "}
                  {r.validationEvidence.evidenceSnapshotIds.length} · replay{" "}
                  {r.validationEvidence.replaySessionIds.length}
                </div>
                <div>
                  release readiness:{" "}
                  {trace?.complete ? "ready" : `blocked:${trace?.missing.join(",") ?? "—"}`}
                </div>
                <div>
                  lineage: commit→
                  {trace?.proposalIds[0] ?? "?"}→
                  {trace?.opportunityIds[0] ?? "?"}→
                  {trace?.evidenceSnapshotIds[0] ?? "?"}→
                  {trace?.replaySessionIds[0] ?? "?"}→
                  {trace?.interactionSessionIds[0] ?? "?"}
                </div>
                <div>
                  history:{" "}
                  {hist
                    .map(
                      (h) =>
                        `${h.seq}:${h.previousState ?? "∅"}→${h.newState}`,
                    )
                    .join(" · ") || "—"}
                </div>
                {ENG_NEXT_STATE[r.state] ? (
                  <button
                    type="button"
                    style={btnStyle}
                    data-testid={`advance-eng-${r.changeId}`}
                    onClick={() => advanceEngineering(r)}
                  >
                    → {ENG_NEXT_STATE[r.state]}
                  </button>
                ) : null}
              </li>
            );
          })}
        </ul>
      </section>

      <section style={{ marginBottom: 10 }} data-testid="adaptation-experiments">
        <div style={{ marginBottom: 4 }}>
          <strong>Adaptation experiments</strong>
          {!experimentApi
            ? " (loading…)"
            : ` (${experimentResults.length})`}
        </div>
        {experimentSummary ? (
          <div style={{ marginBottom: 6, opacity: 0.85 }}>
            considered {experimentSummary.proposalsConsidered} · selected{" "}
            {experimentSummary.experimentsSelected}
            <div>{experimentSummary.selectionNote}</div>
          </div>
        ) : (
          <div style={{ opacity: 0.7, marginBottom: 6 }}>
            No experiment run yet. Run experiments to exercise the pipeline.
          </div>
        )}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
          {experimentResults.map((r) => {
            const ready = experimentApi
              ? experimentApi.experimentRollbackReady(govStore, r.experimentId)
              : false;
            return (
              <li key={r.experimentId} style={{ marginBottom: 8 }}>
                <div>
                  {r.experimentKey} · <strong>{r.experimentStatus}</strong> ·{" "}
                  {r.validationOutcome} · rollout {r.rolloutDisposition}
                </div>
                <div>
                  {r.expectedMetric}: {r.baselineMetricValue} →{" "}
                  {r.observedMetricValue} (Δ {r.observedDelta}, expect Δ
                  {r.expectedDirection === "lower_better" ? "≤-" : "≥"}
                  {r.expectedImprovementDelta})
                </div>
                <div>
                  confidence {r.confidence} · regression {r.regressionCheck} ·
                  rollback {ready ? "ready" : "n/a"}
                </div>
                <div>
                  lineage prop {r.proposalId} · eng {r.engineeringChangeId}
                </div>
                <div>
                  evidence {r.evidenceBaselineId} → {r.evidenceAfterId}
                </div>
                {r.adaptationId ? (
                  <button
                    type="button"
                    style={btnStyle}
                    data-testid={`toggle-exp-${r.experimentKey}`}
                    onClick={() => toggleExperiment(r.adaptationId)}
                  >
                    Toggle adaptation
                  </button>
                ) : null}
              </li>
            );
          })}
        </ul>
      </section>

      <section style={{ marginBottom: 10 }} data-testid="adaptation-packs">
        <div style={{ marginBottom: 4 }}>
          <strong>Adaptation packs</strong>
          {!packsApi
            ? " (loading…)"
            : ` · ${adaptationPacks.length}`}
        </div>
        {activePack ? (
          <div data-testid="active-pack">
            active pack {activePack.packId}@{activePack.version} · hash{" "}
            {activePack.compositionHash}
          </div>
        ) : (
          <div style={{ opacity: 0.7 }}>No active pack.</div>
        )}
        {lastPackGate && !lastPackGate.ok ? (
          <div>last pack gate fail: {lastPackGate.failureReasons.join(", ")}</div>
        ) : null}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
          {adaptationPacks.map((pack) => {
            const ready = packsApi
              ? packsApi.packActivationReady(govStore, pack)
              : false;
            const isActive =
              activePack?.packId === pack.packId &&
              activePack?.version === pack.version;
            return (
              <li
                key={`${pack.packId}@${pack.version}`}
                style={{ marginBottom: 8 }}
              >
                <div>
                  {pack.packId}@{pack.version} ·{" "}
                  <strong>{pack.rolloutStatus}</strong> · ready{" "}
                  {ready ? "yes" : "no"}
                  {isActive ? " · ACTIVE" : ""}
                </div>
                <div>
                  adaptations {pack.adaptationIds.join(", ") || "—"}
                </div>
                <div>
                  certs {pack.certificationIds.join(", ") || "—"} · hash{" "}
                  {pack.compositionHash}
                </div>
                <div>
                  stability {pack.stabilitySummary.composedStabilityScore} ·
                  evidence tip {pack.evidenceSummary.tipEvidenceId ?? "—"}
                </div>
                {ready && !isActive ? (
                  <button
                    type="button"
                    style={btnStyle}
                    data-testid={`activate-pack-${pack.packId}-v${pack.version}`}
                    onClick={() => activatePack(pack)}
                  >
                    Activate pack
                  </button>
                ) : null}
                {isActive ? (
                  <button
                    type="button"
                    style={btnStyle}
                    data-testid={`deactivate-pack-${pack.packId}-v${pack.version}`}
                    onClick={() => deactivatePack(pack)}
                  >
                    Deactivate pack
                  </button>
                ) : null}
              </li>
            );
          })}
        </ul>
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="workspace-presence"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Workspace presence</strong>
          {!presenceApi ? " (loading…)" : ""}
        </div>
        {activePresence ? (
          <div data-testid="active-presence-profile">
            active {activePresence.presenceId}
            <div>
              calm {activePresence.environmentalCalm} · continuity{" "}
              {activePresence.spatialContinuity} · focal{" "}
              {activePresence.focalGravity} · atmosphere{" "}
              {activePresence.contextualAtmosphere} · breath{" "}
              {activePresence.visualBreathingRhythm}
            </div>
            <div data-testid="presence-contributing-adaptations">
              adaptations{" "}
              {activePresence.contributingAdaptationIds.join(", ") || "—"}
            </div>
            <div data-testid="presence-evolution-lineage">
              evolution {activePresence.evolutionId ?? "—"} · epoch{" "}
              {activePresence.evolutionEpoch ?? "—"} · certs{" "}
              {activePresence.certificationIds.join(", ") || "—"}
            </div>
            <div data-testid="presence-resolved-environment">
              lighting {activePresence.resolvedEnvironment.lighting} · spacing{" "}
              {activePresence.resolvedEnvironment.spacingRhythm} · atmosphere{" "}
              {activePresence.resolvedEnvironment.atmosphericIntensity} ·
              motion {activePresence.resolvedEnvironment.motionCadence} · focal{" "}
              {activePresence.resolvedEnvironment.focalEmphasis} · depth{" "}
              {activePresence.resolvedEnvironment.depthWeighting}
            </div>
            <div data-testid="presence-validation-status">
              validation valid · composition{" "}
              {activePresence.validation.compositionValidationResult} ·
              stability{" "}
              {activePresence.validation.composedStabilityScore ?? "—"}
            </div>
          </div>
        ) : (
          <div style={{ opacity: 0.7 }} data-testid="active-presence-profile">
            No active presence
            {presenceApi && presenceReplay?.presence
              ? ` · inactive (${presenceReplay.presence.validation.failureReasons.join(",") || "—"})`
              : "."}
          </div>
        )}
        {presenceReplay ? (
          <div data-testid="presence-replay-comparison">
            replay presentation spacing{" "}
            {presenceReplay.presentation.spacingScale} · baseline{" "}
            {presenceReplay.baselinePresentation.spacingScale} · match{" "}
            {presenceReplay.presentation.spacingScale ===
            presenceReplay.baselinePresentation.spacingScale
              ? "yes"
              : "no"}
          </div>
        ) : null}
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="workspace-memory-evolution"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Workspace memory evolution</strong>
          {!evolutionApi
            ? " (loading…)"
            : ` · history ${memoryEvolutions.length}`}
        </div>
        {activeMemoryEvolution ? (
          <div data-testid="active-memory-evolution">
            active {activeMemoryEvolution.evolutionId} · epoch{" "}
            {activeMemoryEvolution.evolutionEpoch}
            {activeMemoryEvolution.packId
              ? ` · pack ${activeMemoryEvolution.packId}@${activeMemoryEvolution.packVersion}`
              : ""}
            <div>
              adaptations{" "}
              {activeMemoryEvolution.originatingAdaptationIds.join(", ") ||
                "—"}
            </div>
            <div data-testid="evolution-evidence-lineage">
              evidence{" "}
              {activeMemoryEvolution.evidenceLineage.evidenceSnapshotIds.join(
                ", ",
              ) || "—"}{" "}
              · tip{" "}
              {activeMemoryEvolution.evidenceLineage.tipEvidenceId ?? "—"}
            </div>
            <div data-testid="evolution-replay-lineage">
              replay{" "}
              {activeMemoryEvolution.validation.replaySessionIds.join(", ") ||
                "—"}
            </div>
            <div data-testid="evolution-presentation-delta">
              delta spacing {activeMemoryEvolution.presentationDelta.spacingScale}{" "}
              · emphasis{" "}
              {activeMemoryEvolution.presentationDelta.emphasisScale} · group{" "}
              {activeMemoryEvolution.presentationDelta.groupingTightness} · env{" "}
              {activeMemoryEvolution.presentationDelta.environmentalWeight} ·
              motion {activeMemoryEvolution.presentationDelta.motionProfile} ·
              density{" "}
              {activeMemoryEvolution.presentationDelta.density ?? "null"}
            </div>
            <div>
              regions{" "}
              {activeMemoryEvolution.affectedWorkspaceRegions.join(", ") ||
                "—"}
            </div>
          </div>
        ) : (
          <div style={{ opacity: 0.7 }} data-testid="active-memory-evolution">
            No active evolution (inactive or uncertified).
          </div>
        )}
        {evolvedPresentation ? (
          <div>
            resolved applied{" "}
            {evolvedPresentation.appliedAdaptationIds.join(", ") || "(none)"} ·
            spacing {evolvedPresentation.spacingScale}
          </div>
        ) : null}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }} data-testid="evolution-history">
          {memoryEvolutions.map((evo) => (
            <li key={evo.evolutionId} style={{ marginBottom: 6 }}>
              <div>
                {evo.evolutionId} · epoch {evo.evolutionEpoch} ·{" "}
                <strong>{evo.active ? "active" : "inactive"}</strong>
                {evo.validation.failureReasons.length
                  ? ` · ${evo.validation.failureReasons.join(",")}`
                  : ""}
              </div>
              <div>
                adaptations {evo.originatingAdaptationIds.join(", ") || "—"} ·
                certs{" "}
                {evo.certificationLineage.certificationIds.join(", ") || "—"}
              </div>
            </li>
          ))}
        </ul>
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="real-world-adaptation-validation"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Real-world adaptation validation</strong>
          {!validationApi
            ? " (loading…)"
            : ` · snapshots ${realWorldValidation?.inventory.snapshotCount ?? 0}`}
        </div>
        {realWorldValidation ? (
          <>
            <div data-testid="evidence-inventory">
              <strong>Evidence inventory</strong>
              <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
                {realWorldValidation.inventory.entries.map((e) => (
                  <li key={e.evidenceId} style={{ marginBottom: 4 }}>
                    #{e.sequenceIndex + 1} {e.evidenceId} · t{" "}
                    {e.timestampMs ?? "—"} · interactions {e.interactionCount} ·
                    replay {e.replayCount} · activeAdapt{" "}
                    {e.activeAdaptationIds.join(",") || "—"} · cert{" "}
                    {e.certificationId ?? "—"}
                  </li>
                ))}
              </ul>
            </div>
            <div data-testid="adaptation-performance">
              <strong>Adaptation performance</strong>
              <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
                {realWorldValidation.performance.adaptations.map((p) => (
                  <li key={p.subjectId} style={{ marginBottom: 4 }}>
                    {p.subjectId} · act {p.activationCount} · obs{" "}
                    {p.observationCount} · stability {p.stabilityScore ?? "—"} ·
                    trend {p.stabilityTrend} · persist{" "}
                    {p.improvementPersistence ?? "—"} · regFreq{" "}
                    {p.regressionFrequency ?? "—"} · growth {p.evidenceGrowth}
                    <div>
                      confidence [{p.confidenceEvolution.join(", ") || "—"}]
                    </div>
                  </li>
                ))}
                {realWorldValidation.performance.packs.map((p) => (
                  <li
                    key={`${p.subjectId}@${p.packVersion}`}
                    style={{ marginBottom: 4 }}
                  >
                    pack {p.subjectId}@{p.packVersion} · act {p.activationCount}{" "}
                    · obs {p.observationCount} · stability{" "}
                    {p.stabilityScore ?? "—"} · trend {p.stabilityTrend} ·
                    growth {p.evidenceGrowth} · rollout{" "}
                    {p.rolloutSuccess ? "yes" : "no"}
                  </li>
                ))}
              </ul>
            </div>
            <div data-testid="pack-effectiveness">
              <strong>Pack effectiveness</strong>
              <div>
                singles n={realWorldValidation.packEffectiveness.singles.subjectCount}{" "}
                · stability{" "}
                {realWorldValidation.packEffectiveness.singles.meanStabilityScore ??
                  "—"}{" "}
                · regFreq{" "}
                {realWorldValidation.packEffectiveness.singles
                  .meanRegressionFrequency ?? "—"}{" "}
                · growth{" "}
                {realWorldValidation.packEffectiveness.singles.totalEvidenceGrowth}{" "}
                · rolloutRate{" "}
                {realWorldValidation.packEffectiveness.singles.rolloutSuccessRate ??
                  "—"}
              </div>
              <div>
                packs n={realWorldValidation.packEffectiveness.packs.subjectCount}{" "}
                · stability{" "}
                {realWorldValidation.packEffectiveness.packs.meanStabilityScore ??
                  "—"}{" "}
                · regFreq{" "}
                {realWorldValidation.packEffectiveness.packs
                  .meanRegressionFrequency ?? "—"}{" "}
                · growth{" "}
                {realWorldValidation.packEffectiveness.packs.totalEvidenceGrowth}{" "}
                · rolloutRate{" "}
                {realWorldValidation.packEffectiveness.packs.rolloutSuccessRate ??
                  "—"}
              </div>
              <div>
                delta stability{" "}
                {realWorldValidation.packEffectiveness.delta.meanStabilityScore ??
                  "—"}{" "}
                · delta regFreq{" "}
                {realWorldValidation.packEffectiveness.delta
                  .meanRegressionFrequency ?? "—"}{" "}
                · delta growth{" "}
                {realWorldValidation.packEffectiveness.delta.totalEvidenceGrowth}{" "}
                · delta rollout{" "}
                {realWorldValidation.packEffectiveness.delta.rolloutSuccessRate ??
                  "—"}
              </div>
            </div>
            <div data-testid="longitudinal-trend-graphs">
              <strong>Longitudinal trend</strong>
              <div>
                friction [
                {realWorldValidation.longitudinalTrend.points
                  .map((p) => p.meanFrictionScore)
                  .join(", ") || "—"}
                ]
              </div>
              <div>
                ttc [
                {realWorldValidation.longitudinalTrend.points
                  .map((p) => p.medianTimeToConfidenceMs)
                  .join(", ") || "—"}
                ]
              </div>
              <div>
                sessions [
                {realWorldValidation.longitudinalTrend.points
                  .map((p) => p.sessionCount)
                  .join(", ") || "—"}
                ]
              </div>
            </div>
            <div data-testid="certification-longevity">
              <strong>Certification longevity</strong>
              <div>
                meanLongevityMs{" "}
                {realWorldValidation.certificationLongevity.meanLongevityMs ??
                  "—"}
              </div>
              <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
                {realWorldValidation.certificationLongevity.entries.map((e) => (
                  <li key={e.certificationId}>
                    {e.certificationId} · at {e.certifiedAt} · longevity{" "}
                    {e.longevityMs ?? "tip"} · {e.regressionStatus}
                  </li>
                ))}
              </ul>
            </div>
          </>
        ) : (
          <div style={{ opacity: 0.7 }}>Awaiting validation module…</div>
        )}
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="adaptation-certification"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Adaptation certification</strong>
          {!certificationApi
            ? " (loading…)"
            : ` · history ${certifications.length}`}
        </div>
        {currentCertification ? (
          <div data-testid="current-certification">
            current {currentCertification.certificationId} · at{" "}
            {currentCertification.certifiedAt} · hash{" "}
            {currentCertification.adaptationSetHash} · regression{" "}
            {currentCertification.regressionStatus}
            <div>
              adaptations{" "}
              {currentCertification.adaptationIds.join(", ") || "(none)"} ·
              evidence {currentCertification.evidenceSnapshotId}
            </div>
            <div>
              eng{" "}
              {currentCertification.engineeringChangeIds.join(", ") || "—"} ·
              asnap {currentCertification.architectureSnapshotId} ·
              stability {currentCertification.composedStabilityScore}
            </div>
            <div>
              integrity{" "}
              {currentCertification.integrityValid ? "valid" : "invalid"} ·
              governance{" "}
              {currentCertification.governanceValid ? "valid" : "invalid"}
            </div>
          </div>
        ) : (
          <div style={{ opacity: 0.7 }}>No certification yet.</div>
        )}
        {previousCertification ? (
          <div data-testid="previous-certification">
            previous {previousCertification.certificationId} · hash{" "}
            {previousCertification.adaptationSetHash}
          </div>
        ) : null}
        {certificationComparison ? (
          <div data-testid="certification-comparison">
            comparison regressionFree{" "}
            {certificationComparison.regressionFree ? "yes" : "no"} ·
            regressions {certificationComparison.regressions.length}
            {certificationComparison.regressions.length > 0 ? (
              <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
                {certificationComparison.regressions.map((r) => (
                  <li key={`${r.cause}:${r.field}`}>
                    {r.cause} · {r.field} · {r.metrics.join(",") || "—"} ·{" "}
                    {r.previousEvidenceId} → {r.currentEvidenceId}
                  </li>
                ))}
              </ul>
            ) : null}
          </div>
        ) : null}
        {lastCertGate && !lastCertGate.ok ? (
          <div>
            last gate fail: {lastCertGate.failureReasons.join(", ")}
          </div>
        ) : null}
        <div style={{ marginTop: 4 }}>
          <strong>History</strong>
        </div>
        <ul style={{ paddingLeft: 16, margin: "4px 0" }} data-testid="certification-history">
          {certifications.map((c) => (
            <li key={c.certificationId}>
              {c.certificationId} · {c.certifiedAt} · set{" "}
              {c.adaptationIds.length} · {c.regressionStatus}
            </li>
          ))}
        </ul>
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="adaptation-composition"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Adaptation composition</strong>
          {!compositionApi
            ? " (loading…)"
            : compositionResult
              ? ` · stack ${compositionResult.compositionOrder.length}`
              : ""}
        </div>
        {compositionResult ? (
          <>
            <div data-testid="composition-order">
              order:{" "}
              {compositionResult.compositionOrder.join(" → ") || "(empty)"}
            </div>
            <div>
              priority:{" "}
              {compositionResult.priority
                .map((p) => `${p.adaptationId}#${p.rank}`)
                .join(", ") || "—"}
            </div>
            <div data-testid="composition-resolved">
              resolved: density{" "}
              {String(compositionResult.presentation.density)} · space{" "}
              {compositionResult.presentation.spacingScale} · motion{" "}
              {compositionResult.presentation.motionProfile} · env{" "}
              {compositionResult.presentation.environmentalWeight}
            </div>
            <div>
              lineage props{" "}
              {compositionResult.lineage.proposalIds.join(", ") || "—"} · eng{" "}
              {compositionResult.lineage.engineeringChangeIds.join(", ") ||
                "—"}
            </div>
            {conflictReport ? (
              <div data-testid="composition-conflicts">
                conflicts {conflictReport.conflicts.length} · compatible{" "}
                {conflictReport.compatible ? "yes" : "no"}
                {conflictReport.conflicts.length > 0 ? (
                  <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
                    {conflictReport.conflicts.map((c) => (
                      <li
                        key={`${c.cause}:${c.adaptationIds.join("|")}:${c.field}`}
                      >
                        {c.cause} · {c.field} · {c.adaptationIds.join("+")} ·{" "}
                        {c.resolutionStrategy}
                        {c.winnerAdaptationId
                          ? ` → ${c.winnerAdaptationId}`
                          : ""}
                      </li>
                    ))}
                  </ul>
                ) : null}
              </div>
            ) : null}
            {compositionValidation ? (
              <div>
                composition validation{" "}
                {compositionValidation.validationResult} · stability{" "}
                {compositionValidation.composedStabilityScore} · governance{" "}
                {compositionValidation.governanceIntact ? "intact" : "broken"}
              </div>
            ) : null}
          </>
        ) : (
          <div style={{ opacity: 0.7 }}>Composition module loading…</div>
        )}
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="production-adaptation"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Production adaptation</strong>
          {!productionApi
            ? " (loading…)"
            : productionRecord
              ? ` · ${productionRecord.outcome}`
              : " · none"}
        </div>
        {activeAdaptations.length > 0 ? (
          <div data-testid="active-adaptation">
            active:{" "}
            {activeAdaptations.map((a) => a.adaptationId).join(", ")}
          </div>
        ) : (
          <div style={{ opacity: 0.7 }}>No active adaptation.</div>
        )}
        {productionRecord ? (
          <>
            <div>
              activatedAt {productionRecord.activatedAt ?? "—"} · rollback{" "}
              {productionRecord.rollbackAvailable ? "available" : "n/a"}
            </div>
            <div>
              evidence {productionRecord.preEvidenceId ?? "—"} →{" "}
              {productionRecord.postEvidenceId ?? "—"}
            </div>
            <div>
              {productionRecord.expectedMetric ?? "metric"}:{" "}
              {productionRecord.preMetricValue ?? "—"} →{" "}
              {productionRecord.postMetricValue ?? "—"} (Δ{" "}
              {productionRecord.metricDelta ?? "—"})
            </div>
            <div>
              regression {productionRecord.regression ? "yes" : "no"} ·
              governance {productionRecord.governanceIntact ? "intact" : "broken"}{" "}
              · integrity{" "}
              {productionRecord.integrityValid ? "valid" : "invalid"}
            </div>
            <div>
              lineage prop {productionRecord.proposalId ?? "—"} · eng{" "}
              {productionRecord.engineeringChangeId ?? "—"} · asnap{" "}
              {productionRecord.architectureSnapshotId ?? "—"}
            </div>
            <div>
              replay{" "}
              {productionRecord.replaySessionIds.join(", ") || "—"} ·
              stability {productionRecord.stabilityScore ?? "—"}
            </div>
            {productionRecord.blockReasons.length > 0 ? (
              <div>block: {productionRecord.blockReasons.join(", ")}</div>
            ) : null}
            {productionRecord.rollbackAvailable &&
            productionRecord.outcome === "activated" ? (
              <button
                type="button"
                style={btnStyle}
                data-testid="rollback-production-adaptation"
                onClick={rollbackProduction}
              >
                Rollback production adaptation
              </button>
            ) : null}
          </>
        ) : (
          <div style={{ opacity: 0.7, marginBottom: 6 }}>
            No production activation record. Run activate production adaptation.
          </div>
        )}
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="adaptation-operations"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Adaptation operations</strong>
          {!operationsApi
            ? " (loading…)"
            : adaptationCatalog
              ? ` (${adaptationCatalog.entries.length})`
              : ""}
        </div>
        {operationalHealth ? (
          <div style={{ marginBottom: 6 }} data-testid="operational-health">
            <div>
              health · adaptations {operationalHealth.adaptationCount} ·
              rollout backlog {operationalHealth.rolloutBacklog} · validation
              backlog {operationalHealth.validationBacklog}
            </div>
            <div>
              stale evidence {operationalHealth.staleEvidence} · expired
              longitudinal {operationalHealth.expiredLongitudinalSamples} ·
              orphan rollout {operationalHealth.orphanRolloutCandidates} ·
              inactive validated {operationalHealth.inactiveValidatedAdaptations}
            </div>
            <div>
              lifecycle inactive {operationalHealth.lifecycleDistribution.inactive}{" "}
              · candidate {operationalHealth.lifecycleDistribution.candidate} ·
              rollout_candidate{" "}
              {operationalHealth.lifecycleDistribution.rollout_candidate} ·
              active {operationalHealth.lifecycleDistribution.active} ·
              rolled_back {operationalHealth.lifecycleDistribution.rolled_back}
            </div>
          </div>
        ) : null}
        {batchReport ? (
          <div style={{ marginBottom: 6 }} data-testid="batch-validation-report">
            batch · validated {batchReport.validated.length} · failed{" "}
            {batchReport.failed.length} · unchanged {batchReport.unchanged.length}{" "}
            · skipped {batchReport.skipped.length}
            {batchReport.skipped.length > 0 ? (
              <div>
                skipped causes:{" "}
                {batchReport.skipped
                  .map((s) => `${s.adaptationId}:${s.cause}`)
                  .join(", ")}
              </div>
            ) : null}
            {batchReport.failed.length > 0 ? (
              <div>
                failed causes:{" "}
                {batchReport.items
                  .filter((i) => i.outcome === "failed")
                  .map((i) => `${i.adaptationId}:${i.cause}`)
                  .join(", ")}
              </div>
            ) : null}
          </div>
        ) : (
          <div style={{ opacity: 0.7, marginBottom: 6 }}>
            No batch report yet. Run batch validate.
          </div>
        )}
        <div style={{ marginBottom: 4 }}>
          <strong>Catalog</strong>
        </div>
        <ul style={{ paddingLeft: 16, margin: "6px 0" }} data-testid="adaptation-catalog">
          {(adaptationCatalog?.entries ?? []).map((e) => (
            <li key={e.adaptationId} style={{ marginBottom: 6 }}>
              <div>
                {e.adaptationId} · <strong>{e.lifecycleState}</strong> · ready{" "}
                {e.rolloutReady ? "yes" : "no"} · disposition{" "}
                {e.currentDisposition}
              </div>
              <div>
                prop {e.proposalId} · eng {e.engineeringChangeId}
              </div>
              <div>
                stability {e.stabilityScore} · evidence {e.evidenceCount} ·
                latest evd {e.latestEvidenceSnapshotId ?? "—"} · asnap{" "}
                {e.latestArchitectureSnapshotId ?? "—"}
              </div>
            </li>
          ))}
        </ul>
      </section>

      <section
        style={{ marginBottom: 10 }}
        data-testid="longitudinal-adaptation"
      >
        <div style={{ marginBottom: 4 }}>
          <strong>Longitudinal validation</strong>
          {!longitudinalApi
            ? " (loading…)"
            : ` (${longitudinalRecords.length}) · rollout_candidates ${rolloutCandidates.length}`}
        </div>
        {longitudinalRecords.length === 0 ? (
          <div style={{ opacity: 0.7, marginBottom: 6 }}>
            No longitudinal series yet. Validate adaptations across multiple
            evidence snapshots.
          </div>
        ) : null}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
          {longitudinalRecords.map((rec) => {
            const report = stabilityReports.find(
              (r) => r.adaptationId === rec.adaptationId,
            );
            const adaptation = adaptations.find(
              (a) => a.adaptationId === rec.adaptationId,
            );
            return (
              <li key={rec.adaptationId} style={{ marginBottom: 8 }}>
                <div>
                  {rec.adaptationId} · score {rec.stabilityScore} · obs{" "}
                  {rec.observationCount} · regressions {rec.regressionCount}
                </div>
                <div>
                  timeline {rec.baselineEvidenceId}
                  {rec.intermediateEvidenceIds.length
                    ? ` → ${rec.intermediateEvidenceIds.join(" → ")}`
                    : ""}
                  {" → "}
                  {rec.latestEvidenceId}
                </div>
                {report ? (
                  <>
                    <div>
                      trend {report.confidenceTrend} · consistency{" "}
                      {report.improvementConsistency} · variance{" "}
                      {report.metricVariance} · freq{" "}
                      {report.regressionFrequency}
                    </div>
                    <div>
                      confidence [{report.confidenceEvolution.join(", ")}] ·
                      disposition {report.rolloutDisposition}
                    </div>
                    {report.regressionEvents.length > 0 ? (
                      <div>
                        regressions:{" "}
                        {report.regressionEvents
                          .map((e) => `${e.evidenceId}:${e.criterion}`)
                          .join(", ")}
                      </div>
                    ) : (
                      <div>regressions: none</div>
                    )}
                  </>
                ) : null}
                {adaptation ? (
                  <div>
                    <button
                      type="button"
                      style={btnStyle}
                      data-testid={`long-run-${adaptation.adaptationId}`}
                      onClick={() => runLongitudinal(adaptation)}
                    >
                      Re-analyse
                    </button>
                    {adaptation.rolloutState === "candidate" ? (
                      <button
                        type="button"
                        style={btnStyle}
                        data-testid={`long-promote-${adaptation.adaptationId}`}
                        onClick={() => promoteRollout(adaptation)}
                      >
                        Promote rollout_candidate
                      </button>
                    ) : null}
                    <span style={{ opacity: 0.7 }}>
                      {" "}
                      state {adaptation.rolloutState}
                    </span>
                  </div>
                ) : null}
              </li>
            );
          })}
        </ul>
      </section>

      <section style={{ marginBottom: 10 }} data-testid="workspace-adaptations">
        <div style={{ marginBottom: 4 }}>
          <strong>Adaptations</strong>
          {!adaptationApi
            ? " (loading…)"
            : ` (${adaptations.length})`}
        </div>
        {resolvedPresentation ? (
          <div>
            resolved: density {String(resolvedPresentation.density)} · space{" "}
            {resolvedPresentation.spacingScale} · motion{" "}
            {resolvedPresentation.motionProfile} · active{" "}
            {resolvedPresentation.appliedAdaptationIds.length}
          </div>
        ) : null}
        {adaptationApi && adaptations.length === 0 ? (
          <div style={{ opacity: 0.7 }}>
            Empty. Need accepted engineering + architecture snapshot, then draft.
          </div>
        ) : null}
        <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
          {adaptations.map((a) => (
            <li key={a.adaptationId} style={{ marginBottom: 8 }}>
              <div>
                {a.adaptationId} · <strong>{a.rolloutState}</strong> · validation{" "}
                {a.validation.validationResult}
              </div>
              <div>
                eng {a.engineeringChangeId} · prop {a.proposalId}
              </div>
              <div>
                evidence {a.evidenceSnapshotId} · asnap{" "}
                {a.architectureSnapshotId}
              </div>
              <div>
                scopes {a.scopes.join(",")} · metric {a.expectedMetric} Δ
                {a.expectedImprovementDelta}
              </div>
              <div>
                replay {a.validation.replaySessionIds.join(", ") || "—"}
              </div>
              <div>
                {a.validation.validationResult !== "passed" ? (
                  <button
                    type="button"
                    style={btnStyle}
                    onClick={() => validateAdaptation(a)}
                  >
                    Validate vs latest
                  </button>
                ) : null}
                {a.validation.validationResult === "passed" &&
                a.rolloutState !== "active" &&
                a.rolloutState !== "rolled_back" ? (
                  <button
                    type="button"
                    style={btnStyle}
                    data-testid={`activate-adapt-${a.adaptationId}`}
                    onClick={() => activateAdaptation(a)}
                  >
                    Activate
                  </button>
                ) : null}
                {a.rolloutState === "active" ? (
                  <button
                    type="button"
                    style={btnStyle}
                    onClick={() => rollbackAdaptation(a)}
                  >
                    Rollback
                  </button>
                ) : null}
                <span style={{ opacity: 0.7 }}>
                  {a.rolloutState === "rolled_back" ? " rolled_back" : ""}
                  {a.rolloutState === "inactive" ? " inactive" : ""}
                  {a.rolloutState === "candidate" ? " candidate" : ""}
                  {a.rolloutState === "rollout_candidate"
                    ? " rollout_candidate"
                    : ""}
                </span>
              </div>
            </li>
          ))}
        </ul>
      </section>

      <section style={{ marginBottom: 10 }} data-testid="architecture-integrity">
        <div style={{ marginBottom: 4 }}>
          <strong>Architectural integrity</strong>
          {!integrityApi ? " (loading…)" : ""}
        </div>
        {architectureGraph && integrityResult ? (
          <>
            <div>
              graph: nodes {architectureGraph.nodes.length} · edges{" "}
              {architectureGraph.edges.length} · hash{" "}
              {integrityApi
                ? integrityApi.hashArchitectureGraph(architectureGraph)
                : "—"}
            </div>
            <div
              style={{
                color: integrityResult.valid ? "#6dcea0" : "#e08a8a",
              }}
            >
              integrity: {integrityResult.valid ? "valid" : "invalid"} ·
              violations {integrityResult.violations.length} · orphans{" "}
              {integrityResult.orphanNodeIds.length} · dangling{" "}
              {integrityResult.danglingEdgeIds.length}
            </div>
            {integrityResult.orphanNodeIds.length > 0 ? (
              <div>
                orphans: {integrityResult.orphanNodeIds.slice(0, 8).join(", ")}
              </div>
            ) : null}
            <div>
              authority root → {authoritySuccessors.join(" → ") || "—"}
            </div>
            <div style={{ marginTop: 6 }}>
              explorer:{" "}
              <select
                value={exploreNode}
                onChange={(e) => setExploreNode(e.target.value)}
                style={{
                  font: "inherit",
                  color: "#e8e8e8",
                  background: "rgba(0,0,0,0.35)",
                  border: "1px solid rgba(255,255,255,0.16)",
                  maxWidth: "100%",
                }}
              >
                {(architectureGraph?.nodes ?? []).map((n) => (
                  <option key={n.id} value={n.id}>
                    {n.kind}:{n.id}
                  </option>
                ))}
              </select>
            </div>
            <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
              {exploredDeps.map((e) => (
                <li key={e.id}>
                  {e.type} → {e.to}
                </li>
              ))}
            </ul>
            <div style={{ marginTop: 6 }}>
              <strong>Snapshots</strong> ({architectureSnapshots.length})
            </div>
            <ol style={{ paddingLeft: 16, margin: "4px 0" }}>
              {[...architectureSnapshots].reverse().map((s) => (
                <li key={s.snapshotId}>
                  {s.snapshotId} · hash {s.graphHash} · n={s.nodeCount} e=
                  {s.edgeCount} ·{" "}
                  {s.integrity.valid ? "valid" : "invalid"} · releases{" "}
                  {s.releaseLineage.length}
                </li>
              ))}
            </ol>
          </>
        ) : (
          <div style={{ opacity: 0.7 }}>Loading graph…</div>
        )}
      </section>

      {evolution ? (
        <section style={{ marginBottom: 10 }} data-testid="experience-evolution">
          <div style={{ marginBottom: 4 }}>
            <strong>Baseline evolution</strong>
          </div>
          <div>
            improving {evolution.summary.improving} · stable{" "}
            {evolution.summary.stable} · degrading {evolution.summary.degrading}
          </div>
          {evolution.improvements.length > 0 ? (
            <div style={{ marginTop: 6 }}>
              <div>Improvements</div>
              <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
                {evolution.improvements.map((m) => (
                  <li key={m.key} style={{ color: verdictColor("improving") }}>
                    {m.key}: {m.first} → {m.last} (Δ {m.delta})
                  </li>
                ))}
              </ul>
            </div>
          ) : null}
          {evolution.regressions.length > 0 ? (
            <div style={{ marginTop: 6 }}>
              <div>Regressions</div>
              <ul style={{ paddingLeft: 16, margin: "4px 0" }}>
                {evolution.regressions.map((m) => (
                  <li key={m.key} style={{ color: verdictColor("degrading") }}>
                    {m.key}: {m.first} → {m.last} (Δ {m.delta})
                  </li>
                ))}
              </ul>
            </div>
          ) : null}
        </section>
      ) : null}

      {comparison ? (
        <section style={{ marginBottom: 10 }}>
          <div style={{ marginBottom: 4 }}>
            <strong>Baseline vs latest</strong>
          </div>
          <div>
            improved {comparison.summary.improved} · unchanged{" "}
            {comparison.summary.unchanged} · regressed{" "}
            {comparison.summary.regressed}
          </div>
          <ul style={{ paddingLeft: 16, margin: "6px 0" }}>
            {comparison.metrics.map((m) => (
              <li key={m.key} style={{ color: verdictColor(m.verdict) }}>
                {m.key}: {m.baseline} → {m.candidate} ({m.verdict}, Δ {m.delta})
              </li>
            ))}
          </ul>
        </section>
      ) : null}

      <section data-testid="experience-timeline">
        <div style={{ marginBottom: 4 }}>
          <strong>Evidence timeline</strong>
        </div>
        {snapshots.length === 0 ? (
          <div style={{ opacity: 0.7 }}>—</div>
        ) : (
          <ol style={{ paddingLeft: 16, margin: 0 }}>
            {snapshots.map((s, index) => (
              <li key={s.evidenceId} style={{ marginBottom: 4 }}>
                #{index + 1} {s.tag} · {s.evidenceId} · friction{" "}
                {s.metrics.meanFrictionScore} · n={s.metrics.sessionCount}
                {baseline?.evidenceId === s.evidenceId ? " [baseline]" : ""}
                {s.sourceSessionIds.length > 0 ? (
                  <div>
                    {s.sourceSessionIds.map((id) => (
                      <button
                        key={id}
                        type="button"
                        style={btnStyle}
                        onClick={() => replaySessionLink(id)}
                      >
                        replay {id}
                      </button>
                    ))}
                  </div>
                ) : null}
              </li>
            ))}
          </ol>
        )}
      </section>
    </aside>
  );
}
