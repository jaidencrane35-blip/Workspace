import { useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { useActiveMoment } from "./ActiveMoment";
import { useCognitiveEngine } from "./CognitiveEngine";
import { useWorkspaceComposition } from "./WorkspaceComposition";

const HINT_LINES = {
  save: "Focus the note — writing expands from this Moment.",
  continue: "Remember this place — windows grow from the Moment itself.",
  checkin: "Reflect here — answers settle back into the object.",
} as const;

/**
 * Guide — observational. Hints only when cognitive confidence warrants.
 */
export function PilotHelpPanel() {
  const { density } = useWorkspaceComposition();
  const { primary, expandHost, setPresence, setExpanding } = useActiveMoment();
  const { guideDecision, noteGuideVisit, preferHint } = useCognitiveEngine();
  const [revealed, setRevealed] = useState(false);

  useEffect(() => {
    noteGuideVisit();
  }, [noteGuideVisit]);

  useEffect(() => {
    setPresence("guided");
    setExpanding(Boolean(primary) && guideDecision.show);
    return () => setExpanding(false);
  }, [primary, guideDecision.show, setPresence, setExpanding]);

  const hintId = preferHint ?? guideDecision.hintId;
  const line = HINT_LINES[hintId];

  const hintNode = guideDecision.show ? (
    <aside className="moment-attach moment-guide-hint" aria-label="Guide hint">
      <p className="moment-guide-hint__line">{line}</p>
    </aside>
  ) : null;

  return (
    <section
      className="ws-region guide-place guide-dash guide-place--resident guide-place--invisible guide-place--cognitive"
      data-testid="pilot-help"
      data-density={density}
      data-revealed={revealed ? "on" : "off"}
      data-hint={guideDecision.show ? "on" : "off"}
      onMouseEnter={() => setRevealed(true)}
      onFocusCapture={() => setRevealed(true)}
    >
      <h1 className="sr-only">How this pilot works</h1>
      <p className="sr-only">
        Hints appear when useful. You approve every restore plan.
      </p>

      {primary && expandHost && hintNode
        ? createPortal(hintNode, expandHost)
        : null}

      {!primary && guideDecision.show ? (
        <aside className="moment-guide-hint moment-guide-hint--fallback">
          <p className="moment-guide-hint__line">{line}</p>
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
