import { motion, useReducedMotion } from "motion/react";
import { motionPrimitive } from "../lib/motion";
import { MomentCard } from "./MomentCard";

interface EmptyStructureProps {
  title?: string;
  hint?: string;
}

/**
 * Premium empty field — possibility through atmosphere, not instructions.
 */
export function EmptyStructure({
  title = "Moments will float here",
  hint = "Ghost tiles only. Real notes appear when you save.",
}: EmptyStructureProps) {
  const reduceMotion = useReducedMotion();
  const orbit = motionPrimitive("orbit", Boolean(reduceMotion));

  return (
    <div
      className="empty-structure empty-possibility span-12"
      aria-label="Empty workspace structure"
    >
      <motion.div
        className="empty-possibility__halo"
        aria-hidden="true"
        animate={
          reduceMotion
            ? undefined
            : { opacity: [0.25, 0.45, 0.3], scale: [1, 1.04, 1.01] }
        }
        transition={
          reduceMotion
            ? undefined
            : { duration: 12, repeat: Infinity, ease: "easeInOut" }
        }
      />
      <div className="empty-structure__copy sr-only">
        <h3>{title}</h3>
        <p className="muted">{hint}</p>
      </div>
      <motion.div
        className="dash-grid dash-grid--empty attention-orbit attention-orbit--ghost"
        initial={orbit.initial}
        animate={orbit.animate}
        transition={orbit.transition}
      >
        <MomentCard
          variant="placeholder"
          className="span-8 orbit-item orbit-item--0"
          placeholderLabel="Latest moment"
          placeholderHint="Name + what you intend next"
        />
        <MomentCard
          variant="placeholder"
          className="span-4 orbit-item orbit-item--1"
          placeholderLabel="Recent"
          placeholderHint="Continues appear here"
        />
        <MomentCard
          variant="placeholder"
          className="span-4 orbit-item orbit-item--2"
          placeholderLabel="Recent"
          placeholderHint="Your note, unchanged"
        />
        <MomentCard
          variant="placeholder"
          className="span-4 orbit-item orbit-item--3"
          placeholderLabel="Recent"
          placeholderHint="Same Windows session restore"
        />
        <MomentCard
          variant="placeholder"
          className="span-4 orbit-item orbit-item--4"
          placeholderLabel="Recent"
          placeholderHint="Still-open windows only"
        />
      </motion.div>
    </div>
  );
}
