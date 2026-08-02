import { BookmarkPlus, ClipboardList, Play, ShieldCheck } from "lucide-react";
import { motion, useReducedMotion, useScroll, useTransform } from "motion/react";
import { useRef } from "react";
import { spring } from "../design-system";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceSurface } from "./WorkspaceSurface";

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
 * Guide — interactive walkthrough with scroll reveal.
 */
export function PilotHelpPanel() {
  const { density } = useWorkspaceComposition();
  const reduceMotion = useReducedMotion();
  const ref = useRef<HTMLElement>(null);
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
        <motion.div className="guide-walk__progress" style={{ width: progressWidth }} />
      </header>

      <div className="guide-walk__chapters">
        {STEPS.map((item, index) => {
          const Icon = item.icon;
          return (
            <motion.div
              key={item.id}
              className="guide-walk__chapter"
              initial={reduceMotion ? false : { opacity: 0.25, y: 28 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ amount: 0.45, once: true }}
              transition={spring.soft}
            >
              <WorkspaceSurface
                level="floating"
                tone={index === 0 ? "hero" : "default"}
                padding="xl"
                className="guide-story__card"
                layout
              >
                <p className="exp-kicker">Step {index + 1}</p>
                <div className="guide-step__icon">
                  <Icon size={26} aria-hidden="true" />
                </div>
                <h3>{item.title}</h3>
                <p>{item.line}</p>
              </WorkspaceSurface>
            </motion.div>
          );
        })}
      </div>

      <WorkspaceSurface
        level="floating"
        tone="soft"
        padding="md"
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
      </WorkspaceSurface>
    </section>
  );
}
