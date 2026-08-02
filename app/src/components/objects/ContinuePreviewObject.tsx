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

export function ContinuePreviewBody({
  preview,
  busy,
  onApprove,
  onCancel,
  describeDisposition,
  formatMoment,
}: ContinuePreviewBodyProps) {
  const reduceMotion = useReducedMotion();
  const willAttempt = preview.plan.items.filter(
    (i) => i.projected_disposition === "will_attempt",
  ).length;
  const total = preview.plan.items.length;
  const confidence =
    total === 0 ? 0 : Math.round((willAttempt / total) * 100);

  return (
    <div className="continue-preview-body">
      <div className="continue-preview__layers">
        <section>
          <p className="exp-kicker">Saved intention</p>
          <p className="exp-intention">
            {preview.handoff_note.trim()
              ? preview.handoff_note
              : "No handoff was recorded with this context."}
          </p>
          <p className="quote-pane__meta">
            Saved intention · not live Windows state
          </p>
        </section>
        <section>
          <p className="exp-kicker">Expected restoration</p>
          <p className="continue-preview__confidence">
            <motion.span
              key={confidence}
              initial={reduceMotion ? false : { opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={spring.soft}
            >
              {confidence}%
            </motion.span>{" "}
            <span className="muted">
              projected · {willAttempt} of {total} windows
            </span>
          </p>
          <p className="muted">
            Plan expires {formatMoment(preview.plan.expires_at)}.
          </p>
        </section>
      </div>
      <RestoreLimitsNotice />
      <details className="exp-inspect" open>
        <summary>Associated windows</summary>
        <ul className="resume-plan">
          {preview.plan.items.map((item) => (
            <li key={item.item_id}>
              <strong>{item.target_summary}</strong>
              <div>
                {item.action_type} · {describeDisposition(item)}
              </div>
              {item.reason && <div className="muted">{item.reason}</div>}
            </li>
          ))}
        </ul>
      </details>
      <div className="exp-actions">
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
          Cancel
        </button>
      </div>
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
            <p className="exp-kicker">Restoration preview</p>
            <h2 className="focus-card__title">
              Continue “{preview.saved_context_name}”
            </h2>
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
