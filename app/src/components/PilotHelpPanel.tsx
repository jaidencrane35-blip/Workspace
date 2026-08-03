import { useEffect, useState } from "react";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { useActiveMoment } from "./ActiveMoment";
import { useCognitiveEngine } from "./CognitiveEngine";
import { useWorkspaceComposition } from "./WorkspaceComposition";

/**
 * Guide — observational. Cluster annotations live on the semantic field;
 * this surface keeps trust recess and destination presence only.
 */
export function PilotHelpPanel() {
  const { density } = useWorkspaceComposition();
  const { primary, setPresence, setExpanding } = useActiveMoment();
  const { guideDecision, noteGuideVisit } = useCognitiveEngine();
  const [revealed, setRevealed] = useState(false);

  useEffect(() => {
    noteGuideVisit();
  }, [noteGuideVisit]);

  useEffect(() => {
    setPresence("guided");
    // Hints attach to the semantic cluster — do not expand the Moment.
    setExpanding(false);
    return () => setExpanding(false);
  }, [setPresence, setExpanding]);

  return (
    <section
      className="ws-region guide-place guide-dash guide-place--resident guide-place--invisible guide-place--cognitive"
      data-testid="pilot-help"
      data-density={density}
      data-revealed={revealed ? "on" : "off"}
      data-hint={guideDecision.show ? "on" : "off"}
      data-semantic-guide="cluster"
      onMouseEnter={() => setRevealed(true)}
      onFocusCapture={() => setRevealed(true)}
    >
      <h1 className="sr-only">How this pilot works</h1>
      <p className="sr-only">
        Hints appear when useful. You approve every restore plan.
      </p>

      {!primary && guideDecision.show ? (
        <aside className="moment-guide-hint moment-guide-hint--fallback">
          <p className="moment-guide-hint__line">
            Focus the note — writing expands from this Moment.
          </p>
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
