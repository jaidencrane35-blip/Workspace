import { motion, useReducedMotion } from "motion/react";
import { spring } from "../../design-system";
import type { ActionPlanItem, ResumePlanPreview } from "../../types/domain";

interface ContinuePreviewBodyProps {
  preview: ResumePlanPreview;
  busy: boolean;
  onApprove: () => void;
  onCancel: () => void;
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
  describeDisposition,
}: ContinuePreviewBodyProps) {
  const reduceMotion = useReducedMotion();
  const willAttempt = preview.plan.items.filter(
    (i) => i.projected_disposition === "will_attempt",
  ).length;
  const total = preview.plan.items.length;
  const ratio = total === 0 ? 0 : willAttempt / total;
  const quality =
    ratio >= 0.85 ? "high" : ratio >= 0.5 ? "steady" : "limited";

  return (
    <div className="continue-preview-body continue-preview-body--spatial">
      <p className="continue-preview__place-line">
        Reconstructing this place
      </p>
      <div
        className="continue-window-field continue-window-field--spatial"
        role="list"
        aria-label="Windows in this place"
      >
        {preview.plan.items.map((item, index) => {
          const skip = item.projected_disposition !== "will_attempt";
          const hint = describeDisposition(item);
          const { title, app } = splitWindowLabel(item.target_summary);
          return (
            <motion.div
              key={item.item_id}
              role="listitem"
              className={[
                "continue-window-pane",
                `continue-window-pane--${index % 5}`,
                skip ? "is-skip" : "",
              ]
                .filter(Boolean)
                .join(" ")}
              initial={reduceMotion ? false : { opacity: 0, y: 4 }}
              animate={{ opacity: skip ? 0.48 : 1, y: 0 }}
              transition={spring.soft}
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

      <div className="continue-preview__footer">
        <p className="continue-preview__quality" data-quality={quality}>
          <span className="continue-preview__quality-dot" aria-hidden="true" />
          <span>
            {willAttempt === total
              ? "This place is ready"
              : `${willAttempt} of ${total} windows still open`}
          </span>
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
      </div>
    </div>
  );
}
