import { MomentCard } from "./MomentCard";

interface EmptyStructureProps {
  title?: string;
  hint?: string;
}

/**
 * Honest empty scaffold — shows the shape of memory without fabricating saves.
 */
export function EmptyStructure({
  title = "Your moments will live here",
  hint = "Nothing invented. When you save, cards fill these places.",
}: EmptyStructureProps) {
  return (
    <div className="empty-structure" aria-label="Empty workspace structure">
      <div className="empty-structure__copy">
        <h3>{title}</h3>
        <p className="muted">{hint}</p>
      </div>
      <div className="dash-grid dash-grid--empty">
        <MomentCard
          variant="placeholder"
          className="span-8"
          placeholderLabel="Latest moment"
          placeholderHint="Name + what you intend next"
        />
        <MomentCard
          variant="placeholder"
          className="span-4"
          placeholderLabel="Recent"
          placeholderHint="Continues appear here"
        />
        <MomentCard
          variant="placeholder"
          className="span-4"
          placeholderLabel="Recent"
          placeholderHint="Your note, unchanged"
        />
        <MomentCard
          variant="placeholder"
          className="span-4"
          placeholderLabel="Recent"
          placeholderHint="Same Windows session restore"
        />
        <MomentCard
          variant="placeholder"
          className="span-4"
          placeholderLabel="Recent"
          placeholderHint="Still-open windows only"
        />
      </div>
    </div>
  );
}
