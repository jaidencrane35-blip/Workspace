import { ArrowRight } from "lucide-react";
import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { memo, type ReactNode } from "react";
import { ICON } from "../lib/icons";
import { motionPrimitive } from "../lib/motion";
import type { WorkspaceObjectState } from "../lib/objectState";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext } from "../types/domain";
import { WorkspaceObject } from "./WorkspaceObject";
import { WorkspaceSurface } from "./WorkspaceSurface";

export type MomentCardVariant = "hero" | "compact" | "placeholder" | "ambient";
export type MomentObjectState =
  | "collapsed"
  | "expanded"
  | "selected"
  | "preview"
  | "restoring";

interface MomentCardProps {
  variant?: MomentCardVariant;
  state?: MomentObjectState;
  context?: SavedContext;
  busy?: boolean;
  onContinue?: () => void;
  onInspect?: () => void;
  onSelect?: () => void;
  placeholderLabel?: string;
  placeholderHint?: string;
  className?: string;
  layoutId?: string;
  /** Progressive restore reveal rendered inside the expanding moment. */
  expandContent?: ReactNode;
  /** Supporting satellite — title-led, minimal chrome. */
  sparse?: boolean;
  /** Hero: show time only, omit window count. */
  sparseMeta?: boolean;
  /** Override attention weight (satellites should stay readable). */
  attentionWeight?: number;
}

function toObjectState(
  variant: MomentCardVariant,
  state: MomentObjectState,
): WorkspaceObjectState {
  if (state === "selected" || state === "preview" || state === "restoring") {
    return "selected";
  }
  if (state === "expanded" || variant === "hero") {
    return "expanded";
  }
  return "idle";
}

/**
 * Moment spatial artifact — attention-driven; expands in place for restore.
 */
function MomentCardInner({
  variant = "compact",
  state = "collapsed",
  context,
  busy = false,
  onContinue,
  onInspect,
  onSelect,
  placeholderLabel = "Saved moment",
  placeholderHint = "Appears when you save",
  className = "",
  layoutId,
  expandContent,
  sparse = false,
  sparseMeta = false,
  attentionWeight,
}: MomentCardProps) {
  const reduceMotion = useReducedMotion();
  const expandMotion = motionPrimitive("expand", Boolean(reduceMotion));

  if (variant === "placeholder" || !context) {
    return (
      <WorkspaceSurface
        level="surface"
        tone="ghost"
        padding="md"
        className={`moment-card moment-object moment-card--placeholder ${className}`.trim()}
        aria-hidden={variant === "placeholder" ? true : undefined}
      >
        <h3>{placeholderLabel}</h3>
        <p className="moment-card__hint">{placeholderHint}</p>
      </WorkspaceSurface>
    );
  }

  const handoff = context.handoff_note.trim() || "No handoff was recorded.";
  const windowLabel =
    context.windows.length === 1
      ? "1 window"
      : `${context.windows.length} windows`;
  const ambient = variant === "ambient";
  const objectState = toObjectState(ambient ? "compact" : variant, state);
  const showActions =
    !ambient &&
    (objectState === "expanded" ||
      objectState === "selected" ||
      variant === "hero") &&
    !expandContent &&
    !sparse;
  const revealing = Boolean(expandContent) && state === "preview";
  const showHandoff = ambient || !sparse || variant === "compact";
  const showWindows = !ambient && !sparse && !sparseMeta;
  // Hero sparseMeta: handoff carries meaning; drop orphan time row (parity H-06).
  const showMeta = !ambient && !sparse && !sparseMeta;
  const showKicker = !ambient && !sparse && state === "restoring";

  return (
    <WorkspaceObject
      objectId={context.id}
      kind="moment"
      slot={variant === "hero" || revealing ? "anchor" : "orbit"}
      state={objectState}
      layoutId={layoutId ?? `moment-${context.id}`}
      attentionWeight={attentionWeight ?? (ambient ? 0.36 : undefined)}
      lit={state === "preview" || state === "restoring"}
      // Avoid nested button roles when Continue/Inspect actions render.
      interactive={!showActions}
      onActivate={onSelect ?? onContinue}
      className={[
        "moment-card",
        "moment-object",
        `moment-card--${variant}`,
        `is-${state}`,
        sparse || ambient ? "moment-card--sparse" : "",
        ambient ? "moment-card--ambient" : "",
        className,
      ]
        .filter(Boolean)
        .join(" ")}
    >
      <div className="moment-card__top">
        {showKicker && <p className="moment-card__kicker">Restoring</p>}
        <h3 className="moment-card__title">{context.name}</h3>
      </div>
      {showHandoff && (
        <p
          className={
            sparse || (variant === "compact" && state === "collapsed" && !revealing)
              ? "moment-card__handoff moment-card__handoff--compact"
              : "moment-card__handoff"
          }
        >
          {handoff}
        </p>
      )}
      {showMeta && (
        <div className="moment-card__meta">
          <span>{formatRelativeTime(context.created_at)}</span>
          {showWindows && <span>{windowLabel}</span>}
        </div>
      )}
      {showActions && (
        <div className="moment-card__actions">
          {onContinue && (
            <button
              type="button"
              className="exp-btn primary"
              disabled={busy || state === "restoring"}
              onClick={(event) => {
                event.stopPropagation();
                onContinue();
              }}
            >
              Continue
              <ArrowRight
                size={ICON.md}
                strokeWidth={ICON.stroke}
                aria-hidden="true"
              />
            </button>
          )}
          {onInspect && (
            <button
              type="button"
              className="exp-btn ghost"
              disabled={busy}
              onClick={(event) => {
                event.stopPropagation();
                onInspect();
              }}
            >
              Inspect
            </button>
          )}
        </div>
      )}
      <AnimatePresence>
        {revealing && (
          <motion.div
            className="moment-expand"
            initial={expandMotion.initial}
            animate={expandMotion.animate}
            exit={expandMotion.exit}
            transition={expandMotion.transition}
          >
            {expandContent}
          </motion.div>
        )}
      </AnimatePresence>
    </WorkspaceObject>
  );
}

export const MomentCard = memo(MomentCardInner);
