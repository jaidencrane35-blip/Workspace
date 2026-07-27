import { DisplayReasonList } from "./DisplayReasonList";
import type {
  RecommendationDecisionBoundary,
  RecommendationDecisionConfirmation,
  RecommendationDecisionContext,
  RecommendationDecisionEngineAcceptance,
  RecommendationDecisionHandoffRequest,
  RecommendationDecisionIntakeAdapterPreparation,
  RecommendationDecisionIntakeCompatibility,
  RecommendationDecisionIntakeInspection,
  RecommendationDecisionIntakePackageSeal,
  RecommendationDecisionIntakeProceedDenial,
  RecommendationDecisionIntakeRequest,
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
      {item.decision_boundary ? (
        <RecommendationDecisionBoundaryBlock boundary={item.decision_boundary} />
      ) : null}
      {item.decision_confirmation ? (
        <RecommendationDecisionConfirmationBlock
          confirmation={item.decision_confirmation}
        />
      ) : null}
      {item.decision_intake ? (
        <RecommendationDecisionIntakeBlock intake={item.decision_intake} />
      ) : null}
      {item.decision_intake_inspection ? (
        <RecommendationDecisionIntakeInspectionBlock
          inspection={item.decision_intake_inspection}
        />
      ) : null}
      {item.decision_intake_compatibility ? (
        <RecommendationDecisionIntakeCompatibilityBlock
          compatibility={item.decision_intake_compatibility}
        />
      ) : null}
      {item.decision_intake_proceed_denial ? (
        <RecommendationDecisionIntakeProceedDenialBlock
          denial={item.decision_intake_proceed_denial}
        />
      ) : null}
      {item.decision_intake_package_seal ? (
        <RecommendationDecisionIntakePackageSealBlock
          seal={item.decision_intake_package_seal}
        />
      ) : null}
      {item.decision_intake_adapter_preparation ? (
        <RecommendationDecisionIntakeAdapterPreparationBlock
          preparation={item.decision_intake_adapter_preparation}
        />
      ) : null}
      {item.decision_handoff_request ? (
        <RecommendationDecisionHandoffRequestBlock
          request={item.decision_handoff_request}
        />
      ) : null}
      {item.decision_engine_acceptance ? (
        <RecommendationDecisionEngineAcceptanceBlock
          acceptance={item.decision_engine_acceptance}
        />
      ) : null}
    </div>
  );
}

/** Typed future-DE intake package — not a Decision object or handoff. */
export function RecommendationDecisionIntakeBlock({
  intake,
}: {
  intake: RecommendationDecisionIntakeRequest;
}) {
  return (
    <div className="recommendation-decision-intake muted explain-block">
      <div>
        Decision intake: <strong>{intake.intake_state}</strong>
        {" · "}
        intent: {intake.confirmation_intent}
        {" · "}
        DE object: {intake.decision_engine_object_id ?? "none"}
        {" · "}
        handoff: {intake.handoff_performed ? "performed" : "not performed"}
        {" · "}
        authority: {intake.authority_effect}
      </div>
      <div>{intake.note}</div>
      <div>
        Suggested goal (informational): {intake.suggested_goal_statement}
      </div>
      {intake.evidence_refs.length > 0 && (
        <div className="mono">
          Evidence refs: {intake.evidence_refs.slice(0, 4).join(" · ")}
        </div>
      )}
    </div>
  );
}

/** Intake integrity inspection — safe_to_inspect ≠ handoff / DE ownership. */
export function RecommendationDecisionIntakeInspectionBlock({
  inspection,
}: {
  inspection: RecommendationDecisionIntakeInspection;
}) {
  return (
    <div className="recommendation-decision-intake-inspection muted explain-block">
      <div>
        Intake inspection: <strong>{inspection.inspection_state}</strong>
        {" · "}
        safe to inspect: {inspection.safe_to_inspect ? "yes" : "no"}
        {" · "}
        handoff: {inspection.handoff_performed ? "performed" : "not performed"}
        {" · "}
        authority: {inspection.authority_effect}
      </div>
      <div>{inspection.note}</div>
      {inspection.findings.length > 0 && (
        <div>Findings: {inspection.findings.slice(0, 3).join(" · ")}</div>
      )}
    </div>
  );
}

/** Intake compatibility pin — compatible ≠ transfer / handoff / DE ownership. */
export function RecommendationDecisionIntakeCompatibilityBlock({
  compatibility,
}: {
  compatibility: RecommendationDecisionIntakeCompatibility;
}) {
  return (
    <div className="recommendation-decision-intake-compatibility muted explain-block">
      <div>
        Intake compatibility: {" "}
        <strong>{compatibility.compatible ? "compatible" : "incompatible"}</strong>
        {" · "}
        {compatibility.contract_version}
        {" · "}
        schema {compatibility.schema_version}
        {" · "}
        transfer: {compatibility.transfer_authorized ? "authorized" : "denied"}
        {" · "}
        authority: {compatibility.authority_effect}
      </div>
      <div>{compatibility.note}</div>
    </div>
  );
}

/** Compatible ≠ proceed / consume / adapter permission. */
export function RecommendationDecisionIntakeProceedDenialBlock({
  denial,
}: {
  denial: RecommendationDecisionIntakeProceedDenial;
}) {
  return (
    <div className="recommendation-decision-intake-proceed-denial muted explain-block">
      <div>
        Intake proceed: <strong>denied</strong>
        {" · "}
        {denial.eligibility_state}
        {" · "}
        adapter: {denial.adapter_invokable ? "invokable" : "blocked"}
        {" · "}
        owner: {denial.current_owner}
        {" · "}
        permission: {denial.permission_effect}
      </div>
      <div>{denial.note}</div>
    </div>
  );
}

/** Frozen intake digest — seal ≠ proceed / adapter / handoff. */
export function RecommendationDecisionIntakePackageSealBlock({
  seal,
}: {
  seal: RecommendationDecisionIntakePackageSeal;
}) {
  return (
    <div className="recommendation-decision-intake-package-seal muted explain-block">
      <div>
        Intake seal: <strong>{seal.seal_state}</strong>
        {" · "}
        match: {seal.package_matches_seal ? "yes" : "no"}
        {" · "}
        proceed: {seal.proceed_authorized ? "authorized" : "denied"}
        {" · "}
        adapter: {seal.adapter_invokable ? "invokable" : "blocked"}
        {" · "}
        owner: {seal.current_owner}
      </div>
      <div>{seal.note}</div>
    </div>
  );
}

/** Adapter preparation — prepare ≠ invoke / DE ownership. */
export function RecommendationDecisionIntakeAdapterPreparationBlock({
  preparation,
}: {
  preparation: RecommendationDecisionIntakeAdapterPreparation;
}) {
  return (
    <div className="recommendation-decision-intake-adapter-preparation muted explain-block">
      <div>
        Adapter preparation: <strong>{preparation.preparation_state}</strong>
        {" · "}
        seal aligned: {preparation.seal_aligned ? "yes" : "no"}
        {" · "}
        invoked: {preparation.adapter_invoked ? "yes" : "no"}
        {" · "}
        mapping: {preparation.mapping_performed ? "performed" : "not performed"}
        {" · "}
        owner: {preparation.current_owner}
      </div>
      <div>{preparation.note}</div>
    </div>
  );
}

/** Handoff request — request ≠ performed handoff / DE object / execution. */
export function RecommendationDecisionHandoffRequestBlock({
  request,
}: {
  request: RecommendationDecisionHandoffRequest;
}) {
  return (
    <div className="recommendation-decision-handoff-request muted explain-block">
      <div>
        Handoff request: <strong>{request.request_state}</strong>
        {" · "}
        requested: {request.handoff_requested ? "yes" : "no"}
        {" · "}
        performed: {request.handoff_performed ? "yes" : "no"}
        {" · "}
        DE object: {request.decision_engine_object_id ?? "none"}
        {" · "}
        owner: {request.current_owner}
      </div>
      <div>{request.note}</div>
    </div>
  );
}

/** DE acceptance — accept ≠ ownership transfer / DE object / execution. */
export function RecommendationDecisionEngineAcceptanceBlock({
  acceptance,
}: {
  acceptance: RecommendationDecisionEngineAcceptance;
}) {
  return (
    <div className="recommendation-decision-engine-acceptance muted explain-block">
      <div>
        DE acceptance: <strong>{acceptance.acceptance_state}</strong>
        {" · "}
        ownership: {acceptance.ownership_state}
        {" · "}
        transferred: {acceptance.ownership_transferred ? "yes" : "no"}
        {" · "}
        DE object: {acceptance.decision_engine_object_id ?? "none"}
        {" · "}
        owner: {acceptance.current_owner}
      </div>
      <div>{acceptance.note}</div>
    </div>
  );
}

/** Explicit RE↔DE boundary — accept ≠ decision created / action approved / execution. */
export function RecommendationDecisionBoundaryBlock({
  boundary,
}: {
  boundary: RecommendationDecisionBoundary;
}) {
  return (
    <div className="recommendation-decision-boundary muted explain-block">
      <div>
        Decision boundary: <strong>{boundary.transition_state}</strong>
        {" · "}
        handoff: {boundary.handoff_state}
        {" · "}
        intent kind: {boundary.user_intent_kind}
        {" · "}
        authority: {boundary.authority_effect}
      </div>
      <div>{boundary.note}</div>
      <div>
        Creates intent: {boundary.creates_intent ? "yes" : "no"}
        {" · "}
        DE object: {boundary.creates_decision_engine_object ? "yes" : "no"}
        {" · "}
        execution authorised: {boundary.grants_execution_authority ? "yes" : "no"}
      </div>
      <div className="mono">
        Owners: RE={boundary.recommendation_owner} · DE={boundary.decision_owner} ·
        Gateway={boundary.execution_owner}
      </div>
    </div>
  );
}

/** Accept ≠ confirmation — confirmation never creates Decision / Intent / execution. */
export function RecommendationDecisionConfirmationBlock({
  confirmation,
}: {
  confirmation: RecommendationDecisionConfirmation;
}) {
  return (
    <div className="recommendation-decision-confirmation muted explain-block">
      <div>
        Decision confirmation: <strong>{confirmation.confirmation_state}</strong>
        {" · "}
        intent: {confirmation.confirmation_intent}
        {" · "}
        handoff: {confirmation.handoff_performed ? "performed" : "not performed"}
        {" · "}
        authority: {confirmation.authority_effect}
      </div>
      <div>{confirmation.note}</div>
      <div>
        Creates Decision: {confirmation.creates_decision_engine_object ? "yes" : "no"}
        {" · "}
        Creates intent: {confirmation.creates_intent ? "yes" : "no"}
        {" · "}
        Execution authorised: {" "}
        {confirmation.grants_execution_authority ? "yes" : "no"}
      </div>
      <div className="mono">
        Owners: RE={confirmation.recommendation_owner} · User=
        {confirmation.confirmation_owner} · DE={confirmation.decision_owner} ·
        Gateway={confirmation.execution_owner}
      </div>
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
    <div className="recommendation-decision-context muted explain-block">
      <div>
        Decision context: {" "}
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
    <div className="recommendation-decision-readiness muted explain-block">
      <div>
        Decision readiness: <strong>{readiness.readiness_state}</strong>
        {" · "}
        future consideration eligible: {" "}
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
    <div className="recommendation-outcome muted explain-block">
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
          Experience refs (evidence only): {" "}
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
