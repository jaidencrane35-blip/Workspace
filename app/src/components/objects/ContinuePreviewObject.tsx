import { motion, useReducedMotion } from "motion/react";
import { spring } from "../../design-system";
import { sortWindowsByImportance } from "../../lib/cognitive";
import type { ActionPlanItem, ResumePlanPreview } from "../../types/domain";

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
  const ordered = sortWindowsByImportance(preview.plan.items);
  const willAttempt = ordered.filter(
    (i) => i.projected_disposition === "will_attempt",
  ).length;
  const total = ordered.length;
  const ratio = total === 0 ? 0 : willAttempt / total;
  const quality =
    ratio >= 0.85 ? "high" : ratio >= 0.5 ? "steady" : "limited";

  return (
    <div
      className="continue-preview-body continue-preview-body--spatial continue-preview-body--remember continue-preview-body--invisible continue-preview-body--cognitive"
      data-quality={quality}
    >
      <p className="sr-only">
        Reconstructing this place. {willAttempt} of {total} windows still open.
      </p>
      <div
        className="continue-window-field continue-window-field--spatial"
        role="list"
        aria-label="Windows in this place"
      >
        {ordered.map((item, index) => {
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
                `continue-window-pane--${index % 5}`,
                skip ? "is-skip" : "",
                "continue-window-pane--remember",
              ]
                .filter(Boolean)
                .join(" ")}
              initial={
                reduceMotion
                  ? false
                  : { opacity: 0, y: 10, filter: "blur(6px)" }
              }
              animate={{
                opacity: skip ? 0.42 : 1,
                y: 0,
                filter: "blur(0px)",
              }}
              transition={{
                ...spring.lush,
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
          ...spring.soft,
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
