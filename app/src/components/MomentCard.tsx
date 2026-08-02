import { ArrowRight, Clock3, Layers } from "lucide-react";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext } from "../types/domain";

export type MomentCardVariant = "hero" | "standard" | "compact" | "placeholder";

interface MomentCardProps {
  variant?: MomentCardVariant;
  context?: SavedContext;
  busy?: boolean;
  onContinue?: () => void;
  onInspect?: () => void;
  placeholderLabel?: string;
  placeholderHint?: string;
  className?: string;
}

/**
 * Floating moment tile — handoff and time first; metadata stays out.
 */
export function MomentCard({
  variant = "standard",
  context,
  busy = false,
  onContinue,
  onInspect,
  placeholderLabel = "Saved moment",
  placeholderHint = "Appears when you save",
  className = "",
}: MomentCardProps) {
  if (variant === "placeholder" || !context) {
    return (
      <article
        className={`moment-card moment-card--placeholder ${className}`.trim()}
        aria-hidden={variant === "placeholder"}
      >
        <p className="moment-card__kicker">Waiting</p>
        <h3>{placeholderLabel}</h3>
        <p className="moment-card__hint">{placeholderHint}</p>
      </article>
    );
  }

  const handoff = context.handoff_note.trim() || "No handoff was recorded.";
  const windowLabel =
    context.windows.length === 1
      ? "1 window"
      : `${context.windows.length} windows`;

  return (
    <article
      className={`moment-card moment-card--${variant} ${className}`.trim()}
    >
      <div className="moment-card__top">
        <p className="moment-card__kicker">
          {variant === "hero" ? "Pick up here" : "Moment"}
        </p>
        <h3 className="moment-card__title">{context.name}</h3>
      </div>
      <p
        className={
          variant === "compact"
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
      <div className="moment-card__actions">
        {onContinue && (
          <button
            type="button"
            className="exp-btn primary"
            disabled={busy}
            onClick={onContinue}
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
            onClick={onInspect}
          >
            Inspect
          </button>
        )}
      </div>
    </article>
  );
}
