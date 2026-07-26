import { DisplayReasonList } from "./DisplayReasonList";
import type { RecommendationItem } from "../types/domain";

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
    </div>
  );
}
