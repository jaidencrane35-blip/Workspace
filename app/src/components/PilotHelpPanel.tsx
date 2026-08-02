import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";

/**
 * PP-P01D — Minimal pilot help.
 *
 * Explains the Product Proof loop without exposing engine surfaces.
 */
export function PilotHelpPanel() {
  return (
    <section className="assistant-hero" data-testid="pilot-help">
      <p className="assistant-kicker">Help</p>
      <h2>How this pilot works</h2>
      <p className="lede">
        Workspace helps you leave work and return to it with less friction. You
        stay in control: nothing is captured or restored until you confirm.
      </p>

      <section>
        <h3>Save</h3>
        <p>
          Name the context you are in and write what you intend to do next.
          Review what would be recorded, then confirm. Workspace does not invent
          your next step.
        </p>
      </section>

      <section>
        <h3>Resume</h3>
        <p>
          Choose a saved context, inspect what was kept, preview the restore
          plan, and approve before anything moves. You can delete a saved context
          after an explicit confirmation.
        </p>
      </section>

      <section>
        <h3>What restore does not do</h3>
        <p className="muted">{RESTORE_LIMITS_SUMMARY}</p>
      </section>

      <section>
        <h3>Pilot measurement</h3>
        <p>
          Under Pilot, you may consent to local evaluation records: baseline
          minutes, leave→resume times you enter, correction notes, and interview
          answers. Nothing is uploaded, and nothing is recorded without consent.
        </p>
      </section>

      <section>
        <h3>Your control</h3>
        <ul className="list compact">
          <li>You write the handoff; Workspace does not rewrite it.</li>
          <li>You approve every restore plan before it runs.</li>
          <li>You can inspect and permanently delete saved contexts.</li>
          <li>Nothing is sent off this computer for this pilot.</li>
        </ul>
      </section>
    </section>
  );
}
