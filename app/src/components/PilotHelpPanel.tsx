import { BookmarkPlus, ClipboardList, Play, ShieldCheck } from "lucide-react";
import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { useState } from "react";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { ElevatedCard } from "./ElevatedCard";
import { useWorkspaceComposition } from "./WorkspaceComposition";

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
 * Guide — progressive onboarding story inside the shell.
 */
export function PilotHelpPanel() {
  const { density } = useWorkspaceComposition();
  const reduceMotion = useReducedMotion();
  const [step, setStep] = useState(0);
  const current = STEPS[step];
  const Icon = current.icon;

  return (
    <section
      className="spatial-frame guide-dash guide-story"
      data-testid="pilot-help"
      data-density={density}
    >
      <header className="spatial-header">
        <p className="exp-kicker">Guide</p>
        <h1 className="spatial-title">How this pilot works</h1>
      </header>

      <div className="guide-story__rail" role="tablist" aria-label="Guide steps">
        {STEPS.map((item, index) => (
          <button
            key={item.id}
            type="button"
            role="tab"
            className={
              index === step
                ? "guide-story__dot is-active"
                : "guide-story__dot"
            }
            aria-selected={index === step}
            onClick={() => setStep(index)}
          >
            <span>{index + 1}</span>
            {density !== "focus" && <em>{item.title}</em>}
          </button>
        ))}
      </div>

      <AnimatePresence mode="wait">
        <motion.div
          key={current.id}
          initial={reduceMotion ? false : { opacity: 0, y: 16, scale: 0.98 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={reduceMotion ? undefined : { opacity: 0, y: -10, scale: 0.99 }}
          transition={{ type: "spring", stiffness: 320, damping: 32 }}
          className="guide-story__stage"
        >
          <ElevatedCard
            tone="hero"
            elevation={3}
            padding="xl"
            className="guide-story__card"
            layout
          >
            <div className="guide-step__icon">
              <Icon size={26} aria-hidden="true" />
            </div>
            <h3>{current.title}</h3>
            <p>{current.line}</p>
            <div className="exp-actions">
              <button
                type="button"
                className="exp-btn"
                disabled={step === 0}
                onClick={() => setStep((value) => Math.max(0, value - 1))}
              >
                Back
              </button>
              <button
                type="button"
                className="exp-btn primary"
                disabled={step >= STEPS.length - 1}
                onClick={() =>
                  setStep((value) => Math.min(STEPS.length - 1, value + 1))
                }
              >
                Next
              </button>
            </div>
          </ElevatedCard>
        </motion.div>
      </AnimatePresence>

      {density === "flow" && (
        <div className="guide-steps guide-steps--ghost">
          {STEPS.map((item) => {
            const StepIcon = item.icon;
            return (
              <ElevatedCard
                key={item.id}
                tone="ghost"
                padding="md"
                className="guide-step"
              >
                <div className="guide-step__icon">
                  <StepIcon size={18} aria-hidden="true" />
                </div>
                <h3>{item.title}</h3>
                <p>{item.line}</p>
              </ElevatedCard>
            );
          })}
        </div>
      )}

      <ElevatedCard tone="soft" elevation={2} padding="md" className="quote-pane">
        <div className="guide-step__icon" style={{ marginBottom: "0.35rem" }}>
          <ShieldCheck size={20} aria-hidden="true" />
        </div>
        <p className="quote-pane__text" style={{ fontSize: "1rem" }}>
          You write the handoff. You approve every restore plan. You can inspect
          and permanently delete saved contexts. Nothing is sent off this
          computer for this pilot.
        </p>
        <p className="quote-pane__meta">{RESTORE_LIMITS_SUMMARY}</p>
      </ElevatedCard>
    </section>
  );
}
