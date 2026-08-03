import { useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { useActiveMoment } from "./ActiveMoment";
import { useWorkspaceComposition } from "./WorkspaceComposition";

const HINTS = [
  {
    id: "save",
    title: "Save",
    line: "Focus the note — writing expands from this Moment.",
  },
  {
    id: "continue",
    title: "Continue",
    line: "Remember this place — windows grow from the Moment itself.",
  },
  {
    id: "checkin",
    title: "Check-in",
    line: "Reflect here — answers settle back into the object.",
  },
] as const;

/**
 * Guide — contextual annotations on the persistent Moment.
 */
export function PilotHelpPanel() {
  const { density } = useWorkspaceComposition();
  const { primary, expandHost, setPresence, setExpanding } = useActiveMoment();
  const [active, setActive] = useState(0);
  const hint = HINTS[active] ?? HINTS[0];

  useEffect(() => {
    setPresence("guided");
    setExpanding(Boolean(primary));
    return () => setExpanding(false);
  }, [primary, setPresence, setExpanding]);

  const hintNode = (
    <aside className="moment-attach moment-guide-hint" aria-label="Guide hint">
      <p className="moment-attach__kicker">{hint.title}</p>
      <p className="moment-guide-hint__line">{hint.line}</p>
    </aside>
  );

  return (
    <section
      className="ws-region guide-place guide-dash guide-place--resident"
      data-testid="pilot-help"
      data-density={density}
    >
      <header className="place__identity place__identity--quiet-region">
        <p className="exp-kicker">Guide</p>
        <h1 className="place__title place__title--region">How this pilot works</h1>
        <p className="place__pulse">Hints on the Moment — never a separate surface.</p>
      </header>

      <div className="guide-experience__rail" role="list">
        {HINTS.map((item, index) => (
          <button
            key={item.id}
            type="button"
            role="listitem"
            className={
              index === active
                ? "guide-experience__chip is-active"
                : index < active
                  ? "guide-experience__chip is-done is-settled"
                  : "guide-experience__chip"
            }
            onClick={() => setActive(index)}
          >
            {item.title}
          </button>
        ))}
      </div>

      {!primary ? (
        <aside className="moment-guide-hint moment-guide-hint--fallback">
          <p className="moment-guide-hint__line">{hint.line}</p>
        </aside>
      ) : null}

      {expandHost ? createPortal(hintNode, expandHost) : null}

      <details className="exp-inspect guide-trust-recess">
        <summary>What this pilot keeps local</summary>
        <p className="quote-pane__text">
          You write the handoff. You approve every restore plan. You can inspect
          and permanently delete saved contexts. Nothing is sent off this
          computer for this pilot.
        </p>
        <p className="quote-pane__meta">{RESTORE_LIMITS_SUMMARY}</p>
      </details>
    </section>
  );
}
