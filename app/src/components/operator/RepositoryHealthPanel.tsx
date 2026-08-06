import { useEffect, useState } from "react";
import {
  formatBacklogReply,
  loadEvolutionState,
  type CapabilityEvolutionState,
} from "../../lib/capabilityEvolution";

export interface ProjectHealthSnapshot {
  constitution?: { version?: string; document?: string };
  engineeringMode?: string;
  protocol?: { version?: string };
  currentExecutionProgram?: {
    id?: string;
    title?: string;
    status?: string;
    note?: string;
  };
  repositoryHealth?: {
    overall?: string;
    build?: string;
    typecheck?: string;
    tests?: string;
    notes?: string[];
  };
  verification?: {
    status?: string;
    verifiers?: Record<string, { present?: boolean; wiredInTest?: boolean }>;
  };
  validation?: {
    status?: string;
    lastRan?: Record<string, string>;
  };
  lastMilestone?: { date?: string; title?: string; document?: string };
  remainingBacklog?: Array<{
    id?: string;
    title?: string;
    status?: string;
    phase?: string;
  }>;
  completedExecutionPrograms?: Array<{ id?: string; backlogRef?: string }>;
  architecturalRisks?: Array<{
    id?: string;
    severity?: string;
    summary?: string;
  }>;
  technicalDebtSummary?: {
    trend?: string;
    reduced?: string[];
    remaining?: string[];
  };
  handoffStatus?: string;
  nextRecommendedExecutionProgram?: { id?: string; title?: string };
  productProof?: {
    launchStatus?: string;
    productIpcCommandCount?: number;
  };
}

interface RepositoryHealthPanelProps {
  onClose: () => void;
}

export function RepositoryHealthPanel({ onClose }: RepositoryHealthPanelProps) {
  const [health, setHealth] = useState<ProjectHealthSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [evolution, setEvolution] = useState<CapabilityEvolutionState | null>(
    null,
  );

  useEffect(() => {
    let cancelled = false;
    fetch("/project-health.json", { cache: "no-store" })
      .then(async (res) => {
        if (!res.ok) {
          throw new Error(`Could not load project-health.json (${res.status})`);
        }
        return res.json() as Promise<ProjectHealthSnapshot>;
      })
      .then((data) => {
        if (!cancelled) {
          setHealth(data);
        }
      })
      .catch((err: unknown) => {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : String(err));
        }
      });
    setEvolution(loadEvolutionState());
    return () => {
      cancelled = true;
    };
  }, []);

  const verifierEntries = Object.entries(health?.verification?.verifiers ?? {});

  return (
    <aside className="op-health" aria-label="Repository health">
      <header className="op-health__head">
        <div>
          <p className="op-health__eyebrow">Developer diagnostics</p>
          <h2 className="op-health__title">Repository health</h2>
        </div>
        <button type="button" className="op-health__close" onClick={onClose}>
          Close
        </button>
      </header>
      {error && <p className="op-health__error">{error}</p>}
      {!error && !health && <p className="op-health__muted">Loading…</p>}
      {health && (
        <>
          <dl className="op-health__grid">
            <div>
              <dt>Current execution program</dt>
              <dd>{health.currentExecutionProgram?.title ?? "—"}</dd>
            </div>
            <div>
              <dt>Program status</dt>
              <dd>{health.currentExecutionProgram?.status ?? "—"}</dd>
            </div>
            <div>
              <dt>Repository health</dt>
              <dd>{health.repositoryHealth?.overall ?? "—"}</dd>
            </div>
            <div>
              <dt>Build / typecheck / tests</dt>
              <dd>
                {health.repositoryHealth?.build ?? "—"} /{" "}
                {health.repositoryHealth?.typecheck ?? "—"} /{" "}
                {health.repositoryHealth?.tests ?? "—"}
              </dd>
            </div>
            <div>
              <dt>Verification</dt>
              <dd>{health.verification?.status ?? "—"}</dd>
            </div>
            <div>
              <dt>Validation</dt>
              <dd>{health.validation?.status ?? "—"}</dd>
            </div>
            <div>
              <dt>Latest milestone</dt>
              <dd>
                {health.lastMilestone?.title ?? "—"}
                {health.lastMilestone?.date
                  ? ` (${health.lastMilestone.date})`
                  : ""}
              </dd>
            </div>
            <div>
              <dt>Handoff</dt>
              <dd>{health.handoffStatus ?? "—"}</dd>
            </div>
            <div>
              <dt>Constitution</dt>
              <dd>v{health.constitution?.version ?? "—"}</dd>
            </div>
            <div>
              <dt>Protocol</dt>
              <dd>v{health.protocol?.version ?? "—"}</dd>
            </div>
            <div>
              <dt>Next recommended</dt>
              <dd>{health.nextRecommendedExecutionProgram?.title ?? "—"}</dd>
            </div>
            <div>
              <dt>Product IPC (PP)</dt>
              <dd>
                {health.productProof?.productIpcCommandCount ?? "—"} · launch{" "}
                {health.productProof?.launchStatus ?? "—"}
              </dd>
            </div>
          </dl>

          <section className="op-health__section">
            <h3>Architectural warnings</h3>
            <ul className="op-health__list">
              {(health.architecturalRisks ?? []).map((risk) => (
                <li key={risk.id ?? risk.summary}>
                  <strong>{risk.severity}</strong> — {risk.summary}{" "}
                  <span className="op-health__muted">({risk.id})</span>
                </li>
              ))}
            </ul>
          </section>

          <section className="op-health__section">
            <h3>Backlog progress</h3>
            <p className="op-health__muted">
              Completed programs:{" "}
              {(health.completedExecutionPrograms ?? []).length}. Remaining:{" "}
              {(health.remainingBacklog ?? []).length}.
            </p>
            <ul className="op-health__list">
              {(health.remainingBacklog ?? []).map((item) => (
                <li key={item.id ?? item.title}>
                  [{item.phase ?? "—"}] {item.title ?? item.id}{" "}
                  <span className="op-health__muted">({item.status})</span>
                </li>
              ))}
            </ul>
          </section>

          <section className="op-health__section">
            <h3>Verification detail</h3>
            <ul className="op-health__list">
              {verifierEntries.map(([name, meta]) => (
                <li key={name}>
                  {name}: {meta.present ? "present" : "missing"}
                  {meta.wiredInTest ? " · wired in test" : " · not wired"}
                </li>
              ))}
            </ul>
          </section>

          <section className="op-health__section">
            <h3>Technical debt</h3>
            <p className="op-health__muted">
              Trend: {health.technicalDebtSummary?.trend ?? "—"}
            </p>
            <ul className="op-health__list">
              {(health.technicalDebtSummary?.remaining ?? []).map((item) => (
                <li key={item}>{item}</li>
              ))}
            </ul>
          </section>

          <section className="op-health__section">
            <h3>Capability evolution (local)</h3>
            <pre className="op-health__pre">
              {evolution
                ? formatBacklogReply(evolution)
                : "No local evolution state."}
            </pre>
          </section>

          {(health.repositoryHealth?.notes?.length ?? 0) > 0 && (
            <section className="op-health__section">
              <h3>Notes</h3>
              <ul className="op-health__list">
                {health.repositoryHealth?.notes?.map((note) => (
                  <li key={note}>{note}</li>
                ))}
              </ul>
            </section>
          )}
        </>
      )}
    </aside>
  );
}
