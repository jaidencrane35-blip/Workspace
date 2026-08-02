import { BookmarkPlus, ClipboardList, Play, ShieldCheck } from "lucide-react";
import { motion, useReducedMotion, useScroll, useTransform } from "motion/react";
import { useRef, useState } from "react";
import { spring } from "../design-system";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { GuideStepObject } from "./objects/GuideStepObject";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceObject } from "./WorkspaceObject";

const STEPS = [
  {
    id: "save",
    icon: BookmarkPlus,
    title: "Save",
    line: "Leave a note. Confirm what is kept.",
  },
  {
    id: "continue",
    icon: Play,
    title: "Continue",
    line: "Preview. Approve. Return.",
  },
  {
    id: "checkin",
    icon: ClipboardList,
    title: "Check-in",
    line: "Optional local pulse — only if you consent.",
  },
] as const;

/**
 * Guide — interactive product tour on the Workspace Canvas.
 */
export function PilotHelpPanel() {
  const { density, setFocusedObjectId, setAmbient } = useWorkspaceComposition();
  const reduceMotion = useReducedMotion();
  const ref = useRef<HTMLElement>(null);
  const [active, setActive] = useState(0);
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start start", "end end"],
  });
  const progressWidth = useTransform(scrollYProgress, [0, 1], ["8%", "100%"]);

  return (
    <section
      ref={ref}
      className="spatial-frame guide-dash guide-walk"
      data-testid="pilot-help"
      data-density={density}
    >
      <header className="spatial-header">
        <p className="exp-kicker">Guide</p>
        <h1 className="spatial-title">How this pilot works</h1>
        <motion.div
          className="guide-walk__progress"
          style={{ width: progressWidth }}
        />
      </header>

      <div className="guide-walk__chapters ws-compose ws-compose--tour">
        {STEPS.map((item, index) => (
          <motion.div
            key={item.id}
            className="guide-walk__chapter"
            initial={reduceMotion ? false : { opacity: 0.2, y: 32, scale: 0.98 }}
            whileInView={{ opacity: 1, y: 0, scale: 1 }}
            viewport={{ amount: 0.5, once: false }}
            transition={spring.soft}
            onViewportEnter={() => {
              setActive(index);
              setFocusedObjectId(item.id);
              setAmbient("card");
            }}
          >
            <GuideStepObject
              id={item.id}
              step={index + 1}
              title={item.title}
              line={item.line}
              icon={item.icon}
              state={active === index ? "expanded" : "idle"}
            />
          </motion.div>
        ))}
      </div>

      <WorkspaceObject
        objectId="guide-trust"
        kind="guide-step"
        slot="stage"
        state="expanded"
        className="quote-pane"
      >
        <div className="guide-step__icon" style={{ marginBottom: "0.35rem" }}>
          <ShieldCheck size={20} aria-hidden="true" />
        </div>
        <p className="quote-pane__text" style={{ fontSize: "1rem" }}>
          You write the handoff. You approve every restore plan. You can inspect
          and permanently delete saved contexts. Nothing is sent off this
          computer for this pilot.
        </p>
        <p className="quote-pane__meta">{RESTORE_LIMITS_SUMMARY}</p>
      </WorkspaceObject>
    </section>
  );
}
