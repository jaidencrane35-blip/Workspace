import { ArrowRight, Clock3, Layers } from "lucide-react";
import { memo } from "react";
import type { WorkspaceObjectState } from "../lib/objectState";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext } from "../types/domain";
import { WorkspaceObject } from "./WorkspaceObject";
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
 * Moment spatial object — layout-preserving states on the Workspace Canvas.
 */
function MomentCardInner({
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
  const objectState = toObjectState(variant, state);
  const showActions =
    objectState === "expanded" ||
    objectState === "selected" ||
    variant === "hero";

  return (
    <WorkspaceObject
      objectId={context.id}
      kind="moment"
      slot={variant === "hero" ? "anchor" : "orbit"}
      state={objectState}
      layoutId={layoutId ?? `moment-${context.id}`}
      lit={
        state === "selected" || state === "preview" || state === "restoring"
      }
      onActivate={onSelect}
      className={`moment-card moment-object moment-card--${variant} is-${state} ${className}`.trim()}
    >
      <div className="moment-card__top">
        <p className="moment-card__kicker">
          {state === "restoring"
            ? "Restoring"
            : state === "preview"
              ? "Preview"
              : variant === "hero"
                ? "Pick up here"
                : "Moment"}
        </p>
        <h3 className="moment-card__title">{context.name}</h3>
      </div>
      <p
        className={
          variant === "compact" && state === "collapsed"
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
    </WorkspaceObject>
  );
}

export const MomentCard = memo(MomentCardInner);
