import { motion, useReducedMotion } from "motion/react";
import type { CSSProperties } from "react";
import { spring } from "../../design-system";
import { composeSemanticWindowField } from "../../lib/cognitive";
import type { ActionPlanItem, ResumePlanPreview } from "../../types/domain";
import { useActiveMoment } from "../ActiveMoment";
import { useCognitiveEngine } from "../CognitiveEngine";

interface ContinuePreviewBodyProps {
  preview: ResumePlanPreview;
  busy: boolean;
  onApprove: () => void;
  onCancel: () => void;
  onInspect?: () => void;
  describeDisposition: (item: ActionPlanItem) => string;
}

function splitWindowLabel(summary: string): { title: string; app: string } {
  const trimmed = summary.trim();
  for (const sep of [" — ", " - ", " · "]) {
    const cut = trimmed.indexOf(sep);
    if (cut > 0) {
      return {
        title: trimmed.slice(0, cut),
        app: trimmed.slice(cut + sep.length),
      };
    }
  }
  return {
    title: trimmed.length > 40 ? `${trimmed.slice(0, 38)}…` : trimmed,
    app: "",
  };
}

export function ContinuePreviewBody({
  preview,
  busy,
  onApprove,
  onCancel,
  onInspect,
  describeDisposition,
}: ContinuePreviewBodyProps) {
  const reduceMotion = useReducedMotion();
  const { primaryPhase } = useActiveMoment();
  const { resumeAffinity, permanenceById } = useCognitiveEngine();
  const momentId = preview.saved_context_id;
  const temporalConfidence = Math.min(
    1,
    (resumeAffinity[momentId] ?? 0) * 0.55 +
      (permanenceById[momentId] ?? 0) * 0.2 +
      (primaryPhase === "resumed"
        ? 0.35
        : primaryPhase === "dormant"
          ? 0.08
          : primaryPhase === "evolving"
            ? 0.22
            : 0.15),
  );
  const field = composeSemanticWindowField(
    preview.plan.items,
    5,
    temporalConfidence,
  );
  const willAttempt = field.filter(
    (entry) => entry.item.projected_disposition === "will_attempt",
  ).length;
  const total = field.length;
  const ratio = total === 0 ? 0 : willAttempt / total;
  const quality =
    ratio >= 0.85 ? "high" : ratio >= 0.5 ? "steady" : "limited";

  return (
    <div
      className="continue-preview-body continue-preview-body--spatial continue-preview-body--remember continue-preview-body--invisible continue-preview-body--cognitive continue-preview-body--semantic"
      data-quality={quality}
      data-temporal-confidence={temporalConfidence.toFixed(2)}
      data-temporal={primaryPhase ?? "waiting"}
    >
      <p className="sr-only">
        Reconstructing this place. {willAttempt} of {total} windows still open.
      </p>
      <div
        className="continue-window-field continue-window-field--spatial continue-window-field--semantic"
        role="list"
        aria-label="Windows in this place"
        data-testid="semantic-restore-field"
      >
        {field.map(({ item, placement }, index) => {
          const skip = item.projected_disposition !== "will_attempt";
          const hint = describeDisposition(item);
          const { title, app } = splitWindowLabel(item.target_summary);
          const delay = reduceMotion ? 0 : 0.08 + index * 0.11;
          return (
            <motion.div
              key={item.item_id}
              role="listitem"
              className={[
                "continue-window-pane",
                "continue-window-pane--semantic",
                skip ? "is-skip" : "",
                "continue-window-pane--remember",
              ]
                .filter(Boolean)
                .join(" ")}
              data-importance={placement.importance.toFixed(2)}
              style={
                {
                  "--sem-x": `${placement.x}px`,
                  "--sem-y": `${placement.y}px`,
                  "--sem-scale": String(placement.scale),
                  "--sem-opacity": String(placement.opacity),
                  zIndex: placement.zIndex,
                } as CSSProperties
              }
              initial={reduceMotion ? false : { opacity: 0, y: 8 }}
              animate={{
                opacity: placement.opacity,
                y: 0,
              }}
              transition={{
                ...spring.reconstruction,
                delay,
              }}
            >
              <span className="continue-window-pane__chrome" aria-hidden="true" />
              <span className="continue-window-pane__title">{title}</span>
              {app ? (
                <span className="continue-window-pane__app">{app}</span>
              ) : null}
              {hint ? (
                <span className="continue-window-pane__hint">{hint}</span>
              ) : null}
            </motion.div>
          );
        })}
      </div>

      <motion.div
        className="continue-preview__footer continue-preview__footer--quiet"
        initial={reduceMotion ? false : { opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{
          ...spring.continuity,
          delay: reduceMotion ? 0 : 0.12 + total * 0.11,
        }}
      >
        <p
          className="continue-preview__quality"
          data-quality={quality}
          aria-hidden="true"
        >
          <span className="continue-preview__quality-dot" />
        </p>
        <div className="exp-actions continue-preview__actions">
          <button
            type="button"
            className="exp-btn primary"
            disabled={busy}
            onClick={onApprove}
          >
            Approve and restore
          </button>
          <button
            type="button"
            className="exp-btn ghost"
            disabled={busy}
            onClick={onCancel}
          >
            Not now
          </button>
        </div>
        {onInspect ? (
          <details className="exp-inspect continue-inspect-inline">
            <summary>Details</summary>
            <button
              type="button"
              className="exp-btn ghost"
              disabled={busy}
              onClick={onInspect}
            >
              Inspect this place
            </button>
          </details>
        ) : null}
      </motion.div>
    </div>
  );
}
