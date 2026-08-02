import { BookmarkPlus, Play, ShieldCheck, ClipboardList } from "lucide-react";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";

/**
 * Guide — product onboarding tone; truthful Product Proof content.
 */
export function PilotHelpPanel() {
  return (
    <section className="dash guide-dash" data-testid="pilot-help">
      <header className="dash-chrome">
        <div>
          <p className="exp-kicker">Guide</p>
          <h1 className="dash-title">How this pilot works</h1>
          <p className="dash-summary">
            Stay in control at every step — save a note, leave, continue when you
            return.
          </p>
        </div>
      </header>

      <div className="dash-grid">
        <article className="moment-card moment-card--hero span-8 guide-hero">
          <p className="exp-kicker">In one breath</p>
          <h3>Save a note. Leave. Continue when you return.</h3>
          <p className="exp-lede short">
            Workspace helps you leave work and return to it with less friction.
            Nothing is captured or restored until you confirm.
          </p>
        </article>
        <aside className="dash-rail span-4">
          <article className="action-card action-card--soft">
            <ShieldCheck size={20} aria-hidden="true" />
            <h3>Your control</h3>
            <ul className="list compact">
              <li>You write the handoff; Workspace does not rewrite it.</li>
              <li>You approve every restore plan before it runs.</li>
              <li>You can inspect and permanently delete saved contexts.</li>
              <li>Nothing is sent off this computer for this pilot.</li>
            </ul>
          </article>
        </aside>

        <article className="exp-card span-4">
          <BookmarkPlus size={18} aria-hidden="true" />
          <h3>Save</h3>
          <p>
            Name the context you are in and write what you intend to do next.
            Review what would be recorded, then confirm. Workspace does not invent
            your next step.
          </p>
        </article>
        <article className="exp-card span-4">
          <Play size={18} aria-hidden="true" />
          <h3>Continue</h3>
          <p>
            Choose a saved context, inspect what was kept, preview the restore
            plan, and approve before anything moves. You can delete a saved
            context after an explicit confirmation.
          </p>
        </article>
        <article className="exp-card span-4">
          <ClipboardList size={18} aria-hidden="true" />
          <h3>Check-in</h3>
          <p>
            Under Check-in, you may consent to local evaluation records: baseline
            minutes, leave→resume times you enter, correction notes, and interview
            answers. Nothing is uploaded, and nothing is recorded without consent.
          </p>
        </article>

        <article className="exp-card span-12">
          <h3>What restore does not do</h3>
          <p className="muted">{RESTORE_LIMITS_SUMMARY}</p>
        </article>
      </div>
    </section>
  );
}
