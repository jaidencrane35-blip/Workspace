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
  engineeringHealth?: {
    overall?: string;
    build?: string;
    typecheck?: string;
    tests?: string;
    verification?: string;
  };
  productReadiness?: {
    status?: string;
    shellLifecycle?: string;
    compactConversation?: string;
    nativeFeel?: string;
    notes?: string[];
  };
  currentMilestone?: {
    id?: string;
    title?: string;
    status?: string;
    commit?: string;
  };
  acceptedReviews?: Array<{
    id?: string;
    title?: string;
    commit?: string;
    acceptedAt?: string;
  }>;
  outstandingProductDebt?: Array<{
    id?: string;
    summary?: string;
    track?: string;
  }>;
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
  const eng = health?.engineeringHealth;
  const product = health?.productReadiness;

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
          <section className="op-health__section">
            <h3>Current milestone</h3>
            <dl className="op-health__grid">
              <div>
                <dt>Program</dt>
                <dd>
                  {health.currentMilestone?.title ??
                    health.currentExecutionProgram?.title ??
                    "—"}
                </dd>
              </div>
              <div>
                <dt>Status</dt>
                <dd>
                  {health.currentMilestone?.status ??
                    health.currentExecutionProgram?.status ??
                    "—"}
                </dd>
              </div>
              <div>
                <dt>Commit</dt>
                <dd>{health.currentMilestone?.commit ?? "—"}</dd>
              </div>
              <div>
                <dt>Handoff</dt>
                <dd>{health.handoffStatus ?? "—"}</dd>
              </div>
            </dl>
          </section>

          <section className="op-health__section">
            <h3>Engineering health</h3>
            <dl className="op-health__grid">
              <div>
                <dt>Overall</dt>
                <dd>{eng?.overall ?? health.repositoryHealth?.overall ?? "—"}</dd>
              </div>
              <div>
                <dt>Build / typecheck / tests</dt>
                <dd>
                  {eng?.build ?? health.repositoryHealth?.build ?? "—"} /{" "}
                  {eng?.typecheck ?? health.repositoryHealth?.typecheck ?? "—"} /{" "}
                  {eng?.tests ?? health.repositoryHealth?.tests ?? "—"}
                </dd>
              </div>
              <div>
                <dt>Verification</dt>
                <dd>
                  {eng?.verification ?? health.verification?.status ?? "—"}
                </dd>
              </div>
              <div>
                <dt>Validation</dt>
                <dd>{health.validation?.status ?? "—"}</dd>
              </div>
            </dl>
          </section>

          <section className="op-health__section">
            <h3>Product readiness</h3>
            <dl className="op-health__grid">
              <div>
                <dt>Status</dt>
                <dd>{product?.status ?? "—"}</dd>
              </div>
              <div>
                <dt>Shell lifecycle</dt>
                <dd>{product?.shellLifecycle ?? "—"}</dd>
              </div>
              <div>
                <dt>Compact conversation</dt>
                <dd>{product?.compactConversation ?? "—"}</dd>
              </div>
              <div>
                <dt>Native feel</dt>
                <dd>{product?.nativeFeel ?? "—"}</dd>
              </div>
            </dl>
            {(product?.notes?.length ?? 0) > 0 && (
              <ul className="op-health__list">
                {product?.notes?.map((note) => (
                  <li key={note}>{note}</li>
                ))}
              </ul>
            )}
          </section>

          <section className="op-health__section">
            <h3>Accepted reviews</h3>
            <ul className="op-health__list">
              {(health.acceptedReviews ?? []).length === 0 && (
                <li className="op-health__muted">None recorded yet.</li>
              )}
              {(health.acceptedReviews ?? []).map((review) => (
                <li key={review.id ?? review.title}>
                  {review.title ?? review.id}
                  {review.commit ? ` · ${review.commit}` : ""}
                  {review.acceptedAt ? ` · ${review.acceptedAt}` : ""}
                </li>
              ))}
            </ul>
          </section>

          <section className="op-health__section">
            <h3>Outstanding product debt</h3>
            <ul className="op-health__list">
              {(health.outstandingProductDebt ?? []).map((item) => (
                <li key={item.id ?? item.summary}>
                  [{item.track ?? "—"}] {item.summary}{" "}
                  <span className="op-health__muted">({item.id})</span>
                </li>
              ))}
            </ul>
          </section>

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
            <h3>Capability evolution (local)</h3>
            <pre className="op-health__pre">
              {evolution
                ? formatBacklogReply(evolution)
                : "No local evolution state."}
            </pre>
          </section>
        </>
      )}
    </aside>
  );
}
