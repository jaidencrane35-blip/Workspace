import { DisplayReasonList } from "./DisplayReasonList";
import type {
  RecommendationDecisionContext,
  RecommendationDecisionReadiness,
  RecommendationHistoryEntry,
  RecommendationItem,
  RecommendationOutcomeView,
} from "../types/domain";

function isTerminalLifecycle(state: string | null | undefined): boolean {
  return (
    state === "accepted" ||
    state === "rejected" ||
    state === "expired" ||
    state === "superseded"
  );
}

export function isActiveRecommendation(item: RecommendationItem): boolean {
  return !isTerminalLifecycle(item.lifecycle_state);
}

interface Props {
  item: RecommendationItem;
  /** Operator diagnostics may show fingerprint; Work can omit. */
  showFingerprint?: boolean;
}

/**
 * Structured "why this recommendation is shown" — evidence + lifecycle + catalog keys.
 * Experience translates attention_reasons; never discloses chain-of-thought or grants authority.
 */
export function RecommendationExplanationBlock({
  item,
  showFingerprint = false,
}: Props) {
  const view = item.explanation;
  const lifecycle = view?.lifecycle_state ?? item.lifecycle_state ?? "available";

  return (
    <div className="recommendation-explanation">
      <div className="muted">
        Lifecycle: <strong>{lifecycle}</strong>
        {item.lifecycle_resolution_type
          ? ` · ${item.lifecycle_resolution_type}`
          : ""}
        {" · "}
        authority: {view?.authority_effect ?? item.authority_effect}
      </div>
      <div className="muted">
        {view?.lifecycle_note ??
          "Proposal only — review records a human decision, never executes."}
      </div>
      <div>
        Why shown: {view?.why_suggested ?? item.reason}
      </div>
      <div className="muted">
        Impact: {view?.impact ?? item.impact} · Confidence:{" "}
        {view?.confidence ?? item.confidence}
      </div>
      {(view?.source_domains?.length ?? item.evidence.length) > 0 && (
        <div className="muted">
          Sources:{" "}
          {(view?.source_domains ?? item.evidence.map((e) => e.source_model))
            .join(", ")}
        </div>
      )}
      {item.attention_reasons.length > 0 ? (
        <DisplayReasonList reasons={item.attention_reasons} />
      ) : null}
      {view?.explanation_keys && view.explanation_keys.length > 0 && (
        <div className="muted mono">
          Explanation keys: {view.explanation_keys.join(", ")}
        </div>
      )}
      <div className="muted">
        Evidence:{" "}
        {(view?.evidence_summaries ?? item.evidence.map((e) => e.summary)).join(
          " · ",
        ) || "none"}
      </div>
      {view?.evidence_refs && view.evidence_refs.length > 0 && (
        <div className="muted mono">
          Refs: {view.evidence_refs.slice(0, 4).join(" · ")}
        </div>
      )}
      {(view?.related_attention_id ||
        view?.related_task_id ||
        view?.related_decision_id) && (
        <div className="muted">
          Related:{" "}
          {[
            view.related_attention_id
              ? `attention ${view.related_attention_id}`
              : null,
            view.related_task_id ? `task ${view.related_task_id}` : null,
            view.related_decision_id
              ? `decision ${view.related_decision_id}`
              : null,
          ]
            .filter(Boolean)
            .join(" · ")}
        </div>
      )}
      {showFingerprint && view?.continuity_fingerprint && (
        <div className="muted mono">
          Continuity fingerprint: {view.continuity_fingerprint.slice(0, 80)}
          {view.continuity_fingerprint.length > 80 ? "…" : ""}
        </div>
      )}
      {view?.experience_trace_match_keys &&
        view.experience_trace_match_keys.length > 0 && (
          <div className="muted mono">
            Experience match keys (provenance only):{" "}
            {view.experience_trace_match_keys.slice(0, 3).join(", ")}
          </div>
        )}
      {item.outcome ? <RecommendationOutcomeBlock outcome={item.outcome} /> : null}
      {item.decision_context ? (
        <RecommendationDecisionContextBlock context={item.decision_context} />
      ) : null}
      {item.decision_readiness ? (
        <RecommendationDecisionReadinessBlock readiness={item.decision_readiness} />
      ) : null}
    </div>
  );
}

/** Observational future-DE intake context — never a DE object or handoff. */
export function RecommendationDecisionContextBlock({
  context,
}: {
  context: RecommendationDecisionContext;
}) {
  return (
    <div className="recommendation-decision-context muted" style={{ marginTop: 4 }}>
      <div>
        Decision context:{" "}
        <strong>{context.complete ? "complete" : "incomplete"}</strong>
        {" · "}
        handoff performed: {context.handoff_performed ? "yes" : "no"}
        {" · "}
        DE object: {context.decision_engine_object_id ?? "none"}
        {" · "}
        authority: {context.authority_effect}
      </div>
      <div>{context.note}</div>
      {context.user_decision && (
        <div>
          User decision record: {context.user_decision}
          {context.result_kind ? ` · ${context.result_kind}` : ""}
        </div>
      )}
      {context.missing.length > 0 && (
        <div className="mono">Missing: {context.missing.join(", ")}</div>
      )}
      {context.outcome_history_refs.length > 0 && (
        <div className="mono">
          Outcome history refs: {context.outcome_history_refs.slice(0, 3).join(", ")}
        </div>
      )}
    </div>
  );
}

/** Read-only RE→DE readiness — never handoff, never commands. */
export function RecommendationDecisionReadinessBlock({
  readiness,
}: {
  readiness: RecommendationDecisionReadiness;
}) {
  return (
    <div className="recommendation-decision-readiness muted" style={{ marginTop: 4 }}>
      <div>
        Decision readiness: <strong>{readiness.readiness_state}</strong>
        {" · "}
        future consideration eligible:{" "}
        {readiness.ready_for_future_handoff ? "yes" : "no"}
        {" · "}
        not a Decision Engine object · authority: {readiness.authority_effect}
      </div>
      <div>{readiness.note}</div>
      {readiness.missing.length > 0 && (
        <div className="mono">Missing: {readiness.missing.join(", ")}</div>
      )}
    </div>
  );
}

/** Immutable outcome feedback — never execution, never scoring. */
export function RecommendationOutcomeBlock({
  outcome,
}: {
  outcome: RecommendationOutcomeView;
}) {
  return (
    <div className="recommendation-outcome muted" style={{ marginTop: 4 }}>
      <div>
        Outcome: <strong>{outcome.user_decision}</strong> · {outcome.result_kind}
        {" · "}
        authority: {outcome.authority_effect}
        {outcome.is_system_failure ? " · system failure" : " · not a system failure"}
      </div>
      <div className="mono">Recorded {outcome.recorded_at}</div>
      {outcome.explanation_keys.length > 0 && (
        <div className="mono">
          Provenance keys: {outcome.explanation_keys.join(", ")}
        </div>
      )}
      {outcome.evidence_refs.length > 0 && (
        <div className="mono">
          Evidence refs: {outcome.evidence_refs.slice(0, 4).join(" · ")}
        </div>
      )}
      {outcome.experience_trace_match_keys.length > 0 && (
        <div className="mono">
          Experience refs (evidence only):{" "}
          {outcome.experience_trace_match_keys.slice(0, 3).join(", ")}
        </div>
      )}
    </div>
  );
}

export function RecommendationHistoryList({
  history,
}: {
  history: RecommendationHistoryEntry[];
}) {
  if (!history.length) return null;
  return (
    <ul className="intelligence-list">
      {history.map((entry) => (
        <li key={`${entry.native_id}:${entry.outcome.outcome_id}:${entry.outcome.recorded_at}`}>
          <strong>
            [{entry.lifecycle_state}] {entry.native_id}
          </strong>
          <RecommendationOutcomeBlock outcome={entry.outcome} />
        </li>
      ))}
    </ul>
  );
}
