import { BookmarkPlus, ClipboardList, Play, ShieldCheck } from "lucide-react";
import { motion, useReducedMotion } from "motion/react";
import { useEffect, useState } from "react";
import { spring } from "../design-system";
import { ICON } from "../lib/icons";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { GuideStepObject } from "./objects/GuideStepObject";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceObject } from "./WorkspaceObject";

const STEPS = [
  {
    id: "save",
    icon: BookmarkPlus,
    title: "Save",
    line: "Try it: focus the note field — Workspace quiets around writing.",
    demo: "write",
  },
  {
    id: "continue",
    icon: Play,
    title: "Continue",
    line: "Try it: a Moment expands — restore confidence appears inside it.",
    demo: "expand",
  },
  {
    id: "checkin",
    icon: ClipboardList,
    title: "Check-in",
    line: "Try it: one question at a time — completed answers gently recede.",
    demo: "flow",
  },
] as const;

/**
 * Guide — experiential tour driven by attention, not explanation.
 */
export function PilotHelpPanel() {
  const {
    density,
    setPrimaryObject,
    setSecondaryObjects,
    setAttentionScene,
    setAmbient,
  } = useWorkspaceComposition();
  const reduceMotion = useReducedMotion();
  const [active, setActive] = useState(0);

  useEffect(() => {
    setAttentionScene("guide");
    setPrimaryObject(STEPS[active]?.id ?? "save");
    setSecondaryObjects(
      STEPS.filter((_, index) => index !== active).map((step) => step.id),
    );
    setAmbient("card");
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
      className="spatial-frame guide-dash guide-walk attention-field"
      data-testid="pilot-help"
      data-density={density}
    >
      <header className="spatial-header">
        <p className="exp-kicker">Guide</p>
        <h1 className="spatial-title">How this pilot works</h1>
      </header>

      <div className="guide-experience">
        <div className="guide-experience__stage">
          <GuideStepObject
            id={current.id}
            step={active + 1}
            title={current.title}
            line={current.line}
            icon={current.icon}
            state="expanded"
          />
          <motion.div
            key={current.demo}
            className={`guide-demo guide-demo--${current.demo}`}
            initial={reduceMotion ? false : { opacity: 0, scale: 0.96 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={spring.soft}
            aria-hidden="true"
          >
            <span className="guide-demo__pulse" />
            <span className="guide-demo__pulse guide-demo__pulse--b" />
            <span className="guide-demo__label">{current.title}</span>
          </motion.div>
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
                    ? "guide-experience__chip is-done"
                    : "guide-experience__chip"
              }
              onClick={() => setActive(index)}
            >
              {item.title}
            </button>
          ))}
        </div>
      </div>

      <WorkspaceObject
        objectId="guide-trust"
        kind="guide-step"
        slot="stage"
        state="expanded"
        className="quote-pane"
      >
        <div className="guide-step__icon">
          <ShieldCheck
            size={ICON.lg}
            strokeWidth={ICON.stroke}
            aria-hidden="true"
          />
        </div>
        <p className="quote-pane__text">
          You write the handoff. You approve every restore plan. You can inspect
          and permanently delete saved contexts. Nothing is sent off this
          computer for this pilot.
        </p>
        <p className="quote-pane__meta">{RESTORE_LIMITS_SUMMARY}</p>
      </WorkspaceObject>
    </section>
  );
}
