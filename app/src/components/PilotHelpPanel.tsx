import { useEffect, useMemo, useState } from "react";
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
 * Guide — idle chrome nearly gone; one contextual hint when useful.
 */
export function PilotHelpPanel() {
  const { density } = useWorkspaceComposition();
  const { primary, expandHost, neighbours, setPresence, setExpanding } =
    useActiveMoment();
  const [revealed, setRevealed] = useState(false);

  const hint = useMemo(() => {
    if (!primary) {
      return HINTS[0];
    }
    if (neighbours.length === 0) {
      return HINTS[0];
    }
    return HINTS[1];
  }, [primary, neighbours.length]);

  useEffect(() => {
    setPresence("guided");
    setExpanding(Boolean(primary));
    return () => setExpanding(false);
  }, [primary, setPresence, setExpanding]);

  const hintNode = (
    <aside className="moment-attach moment-guide-hint" aria-label="Guide hint">
      <p className="moment-guide-hint__line">{hint.line}</p>
    </aside>
  );

  return (
    <section
      className="ws-region guide-place guide-dash guide-place--resident guide-place--invisible"
      data-testid="pilot-help"
      data-density={density}
      data-revealed={revealed ? "on" : "off"}
      onMouseEnter={() => setRevealed(true)}
      onFocusCapture={() => setRevealed(true)}
    >
      <h1 className="sr-only">How this pilot works</h1>
      <p className="sr-only">
        Hints appear when useful. You approve every restore plan.
      </p>

      {primary && expandHost ? createPortal(hintNode, expandHost) : null}

      {!primary ? (
        <aside className="moment-guide-hint moment-guide-hint--fallback">
          <p className="moment-guide-hint__line">{hint.line}</p>
        </aside>
      ) : null}

      <details className="exp-inspect guide-trust-recess guide-trust-recess--quiet">
        <summary>Local trust</summary>
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
