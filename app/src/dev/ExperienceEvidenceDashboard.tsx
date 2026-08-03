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
  maxWidth: 460,
  maxHeight: "72vh",
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

export function ExperienceEvidenceDashboard() {
  const [open, setOpen] = useState(false);
  const [tick, setTick] = useState(0);
  const [improvementApi, setImprovementApi] = useState<ImprovementApi | null>(
    null,
  );
  const [lastReplay, setLastReplay] = useState<string | null>(null);
  const store = useMemo(() => defaultEvidenceStore(), []);

  const refresh = useCallback(() => setTick((n) => n + 1), []);

  useEffect(() => {
    if (!open || improvementApi) {
      return;
    }
    let cancelled = false;
    void import("./experienceImprovement").then((mod) => {
      if (!cancelled) {
        setImprovementApi(mod);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [open, improvementApi]);

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
        <button type="button" style={btnStyle} onClick={refresh}>
          Refresh
        </button>
      </div>

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
