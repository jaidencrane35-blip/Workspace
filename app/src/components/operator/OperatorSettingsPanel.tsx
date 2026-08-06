/**
 * Mode 3 specialized Settings — honest product preferences, not a feature catalogue.
 */

interface OperatorSettingsPanelProps {
  onClose: () => void;
  onOpenGuide: () => void;
}

export function OperatorSettingsPanel({
  onClose,
  onOpenGuide,
}: OperatorSettingsPanelProps) {
  return (
    <aside className="op-settings" aria-label="Workspace settings">
      <header className="op-settings__head">
        <div>
          <p className="op-settings__eyebrow">Workspace</p>
          <h2 className="op-settings__title">Settings</h2>
        </div>
        <button type="button" className="op-health__close" onClick={onClose}>
          Close
        </button>
      </header>

      <section className="op-settings__section">
        <h3>Shell</h3>
        <p className="op-settings__copy">
          Collapse leaves the floating desktop operator. Closing the conversation
          window returns to that operator — it does not quit Workspace.
        </p>
        <p className="op-settings__copy">
          Use Exit Workspace from the operator menu when you want to quit
          completely.
        </p>
      </section>

      <section className="op-settings__section">
        <h3>Trust</h3>
        <p className="op-settings__copy">
          Desktop observation and restore still require your approval. Ambient
          watching stays off by default.
        </p>
        <button type="button" className="op-shell__btn" onClick={onOpenGuide}>
          Open Guide
        </button>
      </section>

      <section className="op-settings__section">
        <h3>Not here yet</h3>
        <p className="op-settings__copy">
          Appearance themes, hotkeys, voice, and screenshot capture are not
          wired in this build. Ask in conversation to propose them — Workspace
          will not pretend they exist.
        </p>
      </section>
    </aside>
  );
}
