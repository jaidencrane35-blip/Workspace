import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";

/**
 * PP-P01D — Minimal pilot help.
 *
 * Explains the Product Proof loop without exposing engine surfaces.
 * Presentation redesigned for Experience fidelity; content remains truthful.
 */
export function PilotHelpPanel() {
  return (
    <section className="exp-stage" data-testid="pilot-help">
      <header className="exp-home-header">
        <div>
          <p className="exp-kicker">Guide</p>
          <h2>How this pilot works</h2>
          <p className="exp-lede">
            Workspace helps you leave work and return to it with less friction.
            You stay in control: nothing is captured or restored until you
            confirm.
          </p>
        </div>
      </header>

      <div className="exp-card-grid">
        <article className="exp-card">
          <h3>Save</h3>
          <p>
            Name the context you are in and write what you intend to do next.
            Review what would be recorded, then confirm. Workspace does not invent
            your next step.
          </p>
        </article>
        <article className="exp-card">
          <h3>Continue</h3>
          <p>
            Choose a saved context, inspect what was kept, preview the restore
            plan, and approve before anything moves. You can delete a saved
            context after an explicit confirmation.
          </p>
        </article>
        <article className="exp-card">
          <h3>What restore does not do</h3>
          <p className="muted">{RESTORE_LIMITS_SUMMARY}</p>
        </article>
        <article className="exp-card">
          <h3>Check-in</h3>
          <p>
            Under Check-in, you may consent to local evaluation records: baseline
            minutes, leave→resume times you enter, correction notes, and interview
            answers. Nothing is uploaded, and nothing is recorded without consent.
          </p>
        </article>
        <article className="exp-card featured">
          <h3>Your control</h3>
          <ul className="list compact">
            <li>You write the handoff; Workspace does not rewrite it.</li>
            <li>You approve every restore plan before it runs.</li>
            <li>You can inspect and permanently delete saved contexts.</li>
            <li>Nothing is sent off this computer for this pilot.</li>
          </ul>
        </article>
      </div>
    </section>
  );
}
