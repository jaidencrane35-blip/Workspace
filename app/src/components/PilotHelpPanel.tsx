import { BookmarkPlus, ClipboardList, Play, ShieldCheck } from "lucide-react";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";

/**
 * Guide — teach by interface, not documentation.
 * Product Proof trust strings preserved.
 */
export function PilotHelpPanel() {
  return (
    <section className="layer-shell guide-dash" data-testid="pilot-help">
      <header className="continue-gallery__head">
        <p className="exp-kicker">Guide</p>
        <h1 className="dash-title">How this pilot works</h1>
      </header>

      <div className="guide-steps">
        <article className="glass-pane guide-step">
          <div className="guide-step__icon">
            <BookmarkPlus size={22} aria-hidden="true" />
          </div>
          <h3>Save</h3>
          <p>Leave a note. Confirm what is kept.</p>
        </article>
        <article className="glass-pane guide-step">
          <div className="guide-step__icon">
            <Play size={22} aria-hidden="true" />
          </div>
          <h3>Continue</h3>
          <p>Preview. Approve. Return.</p>
        </article>
        <article className="glass-pane guide-step">
          <div className="guide-step__icon">
            <ClipboardList size={22} aria-hidden="true" />
          </div>
          <h3>Check-in</h3>
          <p>Optional local pulse — only if you consent.</p>
        </article>
      </div>

      <article className="glass-pane quote-pane" style={{ marginTop: "0.5rem" }}>
        <div className="guide-step__icon" style={{ marginBottom: "0.35rem" }}>
          <ShieldCheck size={20} aria-hidden="true" />
        </div>
        <p className="quote-pane__text" style={{ fontSize: "1rem" }}>
          You write the handoff. You approve every restore plan. You can inspect
          and permanently delete saved contexts. Nothing is sent off this
          computer for this pilot.
        </p>
        <p className="quote-pane__meta">{RESTORE_LIMITS_SUMMARY}</p>
      </article>
    </section>
  );
}
