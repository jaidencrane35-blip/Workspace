import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { memo } from "react";
import { spring } from "../../design-system";
import type { ActionPlanItem, ResumePlanPreview } from "../../types/domain";
import type { WorkspaceObjectState } from "../../lib/objectState";
import { RestoreLimitsNotice } from "../RestoreLimitsNotice";
import { WorkspaceObject } from "../WorkspaceObject";

interface ContinuePreviewBodyProps {
  preview: ResumePlanPreview;
  busy: boolean;
  onApprove: () => void;
  onCancel: () => void;
  describeDisposition: (item: ActionPlanItem) => string;
  formatMoment: (iso: string) => string;
}

function shortTitle(summary: string): string {
  const trimmed = summary.trim();
  const cut = trimmed.indexOf(" — ");
  if (cut > 0) {
    return trimmed.slice(0, cut);
  }
  const dash = trimmed.indexOf(" - ");
  if (dash > 0) {
    return trimmed.slice(0, dash);
  }
  return trimmed.length > 36 ? `${trimmed.slice(0, 34)}…` : trimmed;
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
    <div className="continue-preview-body">
      <div
        className="continue-window-field"
        role="list"
        aria-label="Windows in this restore"
      >
        {preview.plan.items.map((item, index) => {
          const skip = item.projected_disposition !== "will_attempt";
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
              initial={reduceMotion ? false : { opacity: 0, y: 10, scale: 0.97 }}
              animate={{ opacity: skip ? 0.45 : 1, y: 0, scale: 1 }}
              transition={{ ...spring.soft, delay: reduceMotion ? 0 : index * 0.04 }}
            >
              <span className="continue-window-pane__title">
                {shortTitle(item.target_summary)}
              </span>
              <span className="continue-window-pane__hint">
                {describeDisposition(item)}
              </span>
            </motion.div>
          );
        })}
      </div>

      <p className="continue-preview__quality" data-quality={quality}>
        <span className="continue-preview__quality-dot" aria-hidden="true" />
        <span>
          {willAttempt === total
            ? "Ready to place the open windows"
            : `Can place ${willAttempt} of ${total} windows still open`}
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

      <details className="exp-inspect continue-preview__limits">
        <summary>What restore does</summary>
        <RestoreLimitsNotice compact />
      </details>
    </div>
  );
}

interface ContinuePreviewObjectProps {
  preview: ResumePlanPreview | null;
  busy: boolean;
  state?: WorkspaceObjectState;
  onApprove: () => void;
  onCancel: () => void;
  describeDisposition: (item: ActionPlanItem) => string;
  formatMoment: (iso: string) => string;
}

function ContinuePreviewObjectInner({
  preview,
  busy,
  state = "expanded",
  onApprove,
  onCancel,
  describeDisposition,
  formatMoment,
}: ContinuePreviewObjectProps) {
  const reduceMotion = useReducedMotion();

  return (
    <AnimatePresence mode="sync">
      {preview && (
        <motion.div
          key={preview.saved_context_id}
          className="continue-preview-slot"
          initial={reduceMotion ? false : { opacity: 0, y: 24, height: 0 }}
          animate={{ opacity: 1, y: 0, height: "auto" }}
          exit={reduceMotion ? undefined : { opacity: 0, y: 12, height: 0 }}
          transition={spring.lush}
        >
          <WorkspaceObject
            objectId={`preview-${preview.saved_context_id}`}
            kind="continue-preview"
            slot="stage"
            state={state}
            level="overlay"
            lit
            className="continue-preview-object"
          >
            <ContinuePreviewBody
              preview={preview}
              busy={busy}
              onApprove={onApprove}
              onCancel={onCancel}
              describeDisposition={describeDisposition}
              formatMoment={formatMoment}
            />
          </WorkspaceObject>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

export const ContinuePreviewObject = memo(ContinuePreviewObjectInner);
