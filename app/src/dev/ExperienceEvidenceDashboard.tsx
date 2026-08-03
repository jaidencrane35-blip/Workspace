/**
 * Development-only Experience Evidence viewer.
 * Never mounted in production builds (dynamic import + DEV gate).
 */

import { useCallback, useMemo, useState, type CSSProperties } from "react";
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
  maxWidth: 420,
  maxHeight: "70vh",
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
  if (v === "improved") return "#6dcea0";
  if (v === "regressed") return "#e08a8a";
  return "#a0a0a0";
}

export function ExperienceEvidenceDashboard() {
  const [open, setOpen] = useState(false);
  const [tick, setTick] = useState(0);
  const store = useMemo(() => defaultEvidenceStore(), []);

  const refresh = useCallback(() => setTick((n) => n + 1), []);

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
        <button type="button" style={btnStyle} onClick={refresh}>
          Refresh
        </button>
      </div>

      <section style={{ marginBottom: 10 }}>
        <div>Sessions in store: {sessions.length}</div>
        <div>Evidence snapshots: {snapshots.length}</div>
        <div>Replay invocations: {replayCount}</div>
        <div>Baseline: {baseline?.evidenceId ?? "—"}</div>
      </section>

      {latest ? (
        <section style={{ marginBottom: 10 }}>
          <div style={{ marginBottom: 4 }}>
            <strong>Latest aggregate</strong> ({latest.evidenceId})
          </div>
          <div>median TTC: {latest.metrics.medianTimeToConfidenceMs} ms</div>
          <div>mean friction: {latest.metrics.meanFrictionScore}</div>
          <div>
            friction [{latest.metrics.frictionMin} … {latest.metrics.frictionMax}]
            p25={latest.metrics.frictionP25} p75={latest.metrics.frictionP75}
          </div>
          <div>
            abandons: {latest.metrics.abandonedFlowTotal} (save{" "}
            {latest.metrics.abandonedSave} / continue{" "}
            {latest.metrics.abandonedContinue})
          </div>
          <div>
            recovery rate: {latest.metrics.recoverySuccessRate} (
            {latest.metrics.recoveryCount}/{latest.metrics.interruptionCount})
          </div>
          <div>
            loops: {latest.metrics.navigationLoopCount} · hotspots:{" "}
            {latest.metrics.hesitationHotspotCount}
            {latest.metrics.topHesitationDestination
              ? ` (top ${latest.metrics.topHesitationDestination})`
              : ""}
          </div>
          <div>
            replay divergence: {latest.metrics.replayDivergenceRate} · saves:{" "}
            {latest.metrics.saveSuccessTotal} · continues:{" "}
            {latest.metrics.continueSuccessTotal}
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

      {comparison ? (
        <section style={{ marginBottom: 10 }}>
          <div style={{ marginBottom: 4 }}>
            <strong>Regression summary</strong>
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

      <section>
        <div style={{ marginBottom: 4 }}>
          <strong>Validation history</strong>
        </div>
        {snapshots.length === 0 ? (
          <div style={{ opacity: 0.7 }}>—</div>
        ) : (
          <ol style={{ paddingLeft: 16, margin: 0 }}>
            {[...snapshots].reverse().map((s) => (
              <li key={s.evidenceId} style={{ marginBottom: 4 }}>
                {s.tag} · {s.evidenceId} · friction{" "}
                {s.metrics.meanFrictionScore} · n={s.metrics.sessionCount}
                {baseline?.evidenceId === s.evidenceId ? " [baseline]" : ""}
              </li>
            ))}
          </ol>
        )}
      </section>
    </aside>
  );
}
