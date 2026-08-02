import { ArrowRight, Clock3, Layers } from "lucide-react";
import { motion, useReducedMotion } from "motion/react";
import { spring } from "../design-system";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext } from "../types/domain";
import { WorkspaceSurface } from "./WorkspaceSurface";

export type MomentCardVariant = "hero" | "standard" | "compact" | "placeholder";
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
}

/**
 * Moment spatial object — not a list row.
 * States morph via shared layout animation.
 */
export function MomentCard({
  variant = "standard",
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
}: MomentCardProps) {
  const reduceMotion = useReducedMotion();
  const objectState =
    variant === "hero" && state === "collapsed" ? "expanded" : state;

  if (variant === "placeholder" || !context) {
    return (
      <WorkspaceSurface
        level="surface"
        tone="ghost"
        padding="md"
        className={`moment-card moment-object moment-card--placeholder ${className}`.trim()}
        aria-hidden={variant === "placeholder" ? true : undefined}
      >
        <p className="moment-card__kicker">Waiting</p>
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
  const level =
    objectState === "selected" ||
    objectState === "preview" ||
    objectState === "restoring" ||
    variant === "hero"
      ? "floating"
      : "surface";
  const padding =
    variant === "compact"
      ? "sm"
      : variant === "hero" || objectState === "expanded"
        ? "lg"
        : "md";

  return (
    <motion.div
      layout={!reduceMotion}
      layoutId={layoutId ?? `moment-${context.id}`}
      className={`moment-object-wrap ${className}`.trim()}
      transition={spring.layout}
      data-state={objectState}
    >
      <WorkspaceSurface
        level={level}
        tone={variant === "hero" ? "hero" : "default"}
        padding={padding}
        interactive
        lit={
          objectState === "selected" ||
          objectState === "preview" ||
          objectState === "restoring"
        }
        className={`moment-card moment-object moment-card--${variant} is-${objectState}`}
        onClick={onSelect}
      >
        <div className="moment-card__top">
          <p className="moment-card__kicker">
            {objectState === "restoring"
              ? "Restoring"
              : objectState === "preview"
                ? "Preview"
                : variant === "hero"
                  ? "Pick up here"
                  : "Moment"}
          </p>
          <h3 className="moment-card__title">{context.name}</h3>
        </div>
        <p
          className={
            variant === "compact" && objectState === "collapsed"
              ? "moment-card__handoff moment-card__handoff--compact"
              : "moment-card__handoff"
          }
        >
          {handoff}
        </p>
        <div className="moment-card__meta">
          <span>
            <Clock3 size={14} aria-hidden="true" />
            {formatRelativeTime(context.created_at)}
          </span>
          <span>
            <Layers size={14} aria-hidden="true" />
            {windowLabel}
          </span>
        </div>
        {(objectState !== "collapsed" || variant === "hero") && (
          <div className="moment-card__actions">
            {onContinue && (
              <button
                type="button"
                className="exp-btn primary"
                disabled={busy || objectState === "restoring"}
                onClick={(event) => {
                  event.stopPropagation();
                  onContinue();
                }}
              >
                Continue
                <ArrowRight size={16} aria-hidden="true" />
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
      </WorkspaceSurface>
    </motion.div>
  );
}
