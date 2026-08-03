import { BookmarkPlus, ClipboardList, Play } from "lucide-react";
import { useEffect, useState } from "react";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { GuideStepObject } from "./objects/GuideStepObject";
import { useWorkspaceComposition } from "./WorkspaceComposition";

const STEPS = [
  {
    id: "save",
    icon: BookmarkPlus,
    title: "Save",
    line: "Focus the note — the place quiets around writing.",
  },
  {
    id: "continue",
    icon: Play,
    title: "Continue",
    line: "Step into a Moment — the place remembers itself.",
  },
  {
    id: "checkin",
    icon: ClipboardList,
    title: "Check-in",
    line: "Answer one question — earlier answers settle into history.",
  },
] as const;

/**
 * Guide — quiet resident of the living workspace.
 */
export function PilotHelpPanel() {
  const {
    density,
    setPrimaryObject,
    setSecondaryObjects,
    setAttentionScene,
    setAmbient,
  } = useWorkspaceComposition();
  const [active, setActive] = useState(0);

  useEffect(() => {
    setAttentionScene("guide");
    setPrimaryObject(STEPS[active]?.id ?? "save");
    setSecondaryObjects(
      STEPS.filter((_, index) => index !== active).map((step) => step.id),
    );
    setAmbient("workspace");
  }, [
    active,
    setAttentionScene,
    setPrimaryObject,
    setSecondaryObjects,
    setAmbient,
  ]);

  const current = STEPS[active];

  return (
    <section
      className="ws-region guide-place guide-dash guide-walk guide-place--resident attention-field"
      data-testid="pilot-help"
      data-density={density}
    >
      <header className="place__identity place__identity--place place__identity--quiet-region">
        <p className="exp-kicker">Guide</p>
        <h1 className="place__title place__title--region">How this pilot works</h1>
        <p className="place__pulse">Hints when useful — never an interruption.</p>
      </header>

      <div className="guide-experience guide-experience--embedded">
        <div className="guide-experience__stage">
          <GuideStepObject
            id={current.id}
            step={active + 1}
            title={current.title}
            line={current.line}
            icon={current.icon}
            state="expanded"
          />
        </div>

        <div className="guide-experience__rail" role="list">
          {STEPS.map((item, index) => (
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
      </div>

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
