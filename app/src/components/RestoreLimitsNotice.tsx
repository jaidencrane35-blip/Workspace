import {
  RESTORE_LIMITS_DOES,
  RESTORE_LIMITS_DOES_NOT,
  RESTORE_LIMITS_HEADING,
  RESTORE_LIMITS_SUMMARY,
} from "../lib/restoreLimits";

interface RestoreLimitsNoticeProps {
  /** When true, omit the heading (caller already titled the section). */
  compact?: boolean;
}

/**
 * PP-P01B — Explicit restore-limits copy for Save review and Resume flows.
 * Presentation only; does not change Action restore behaviour.
 */
export function RestoreLimitsNotice({ compact = false }: RestoreLimitsNoticeProps) {
  return (
    <section
      className="restore-limits"
      data-testid="restore-limits-notice"
      aria-label={RESTORE_LIMITS_HEADING}
    >
      {!compact && <h3>{RESTORE_LIMITS_HEADING}</h3>}
      <p className={compact ? "muted" : "lede"}>{RESTORE_LIMITS_SUMMARY}</p>
      <h4>Restored</h4>
      <ul className="list compact">
        {RESTORE_LIMITS_DOES.map((line) => (
          <li key={line}>{line}</li>
        ))}
      </ul>
      <h4>Not restored</h4>
      <ul className="list compact">
        {RESTORE_LIMITS_DOES_NOT.map((line) => (
          <li key={line}>{line}</li>
        ))}
      </ul>
    </section>
  );
}
