import { motion, useReducedMotion } from "motion/react";
import { motionPrimitive } from "../lib/motion";
import { MomentCard } from "./MomentCard";

interface EmptyStructureProps {
  title?: string;
  hint?: string;
}

/**
 * Empty field — static atmosphere, no decorative pulse.
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
      <div className="empty-possibility__halo" aria-hidden="true" />
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
          placeholderHint="Your next note waits here"
        />
        <MomentCard
          variant="placeholder"
          className="span-4 orbit-item orbit-item--1"
          placeholderLabel="Recent"
          placeholderHint="Quieter until you save"
        />
      </motion.div>
    </div>
  );
}
