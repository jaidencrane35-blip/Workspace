import type { AttentionReason, DecisionReason } from "../types/domain";
import {
  resolveAttentionReasons,
  resolveDecisionReasons,
  type DisplayReason,
} from "../lib/experienceTranslation";

interface DisplayReasonListProps {
  /** Structured source reasons — always preserved; never replaced by display text. */
  reasons: AttentionReason[];
  /** When true, show unresolved keys for developer/diagnostic surfaces only. */
  showUnresolvedKey?: boolean;
}

/** User-facing Experience rendering of structured Attention reasons. */
export function DisplayReasonList({
  reasons,
  showUnresolvedKey = false,
}: DisplayReasonListProps) {
  if (reasons.length === 0) return null;
  const displayed = resolveAttentionReasons(reasons);
  return (
    <ul className="experience-reason-list muted">
      {displayed.map((display, index) => (
        <DisplayReasonItem
          key={`${display.explanation_key}-${index}`}
          display={display}
          showUnresolvedKey={showUnresolvedKey}
        />
      ))}
    </ul>
  );
}

interface DecisionReasonListProps {
  reasons: DecisionReason[];
  showUnresolvedKey?: boolean;
}

/** Decision candidates — Attention-backed reasons go through Experience first. */
export function DecisionReasonList({
  reasons,
  showUnresolvedKey = false,
}: DecisionReasonListProps) {
  if (reasons.length === 0) return null;
  const displayed = resolveDecisionReasons(reasons);
  return (
    <ul className="experience-reason-list muted">
      {displayed.map((display, index) => (
        <DisplayReasonItem
          key={`${display.explanation_key}-${index}`}
          display={display}
          showUnresolvedKey={showUnresolvedKey}
        />
      ))}
    </ul>
  );
}

function DisplayReasonItem({
  display,
  showUnresolvedKey,
}: {
  display: DisplayReason;
  showUnresolvedKey: boolean;
}) {
  return (
    <li>
      <strong>{display.title}</strong>
      <div>{display.description}</div>
      {!display.known && showUnresolvedKey ? (
        <div className="mono">untranslated: {display.explanation_key}</div>
      ) : null}
    </li>
  );
}
