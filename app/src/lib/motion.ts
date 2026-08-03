/** Shared motion primitives — four purposes only. */

import type { Transition, TargetAndTransition } from "motion/react";
import { duration, spring } from "../design-system/tokens";

export type MotionPrimitive =
  | "reveal"
  | "elevate"
  | "settle"
  | "dissolve"
  | "focus"
  | "restore"
  | "orbit"
  | "compress"
  | "expand";

type MotionSpec = {
  initial?: TargetAndTransition;
  animate: TargetAndTransition;
  exit?: TargetAndTransition;
  transition: Transition;
};

const instant: Transition = { duration: duration.instant };

export function motionPrimitive(
  name: MotionPrimitive,
  reduceMotion = false,
): MotionSpec {
  if (reduceMotion) {
    return {
      initial: { opacity: 1 },
      animate: { opacity: 1 },
      exit: { opacity: 0 },
      transition: instant,
    };
  }

  switch (name) {
    case "focus":
      // Attention — opacity only.
      return {
        initial: { opacity: 0.88 },
        animate: { opacity: 1 },
        exit: { opacity: 0.82 },
        transition: spring.attention,
      };
    case "reveal":
      // Continuity — opacity-only view swaps.
      return {
        initial: { opacity: 0 },
        animate: { opacity: 1 },
        exit: { opacity: 0 },
        transition: spring.continuity,
      };
    case "elevate":
      return {
        initial: { opacity: 0.9, y: 6 },
        animate: { opacity: 1, y: 0 },
        exit: { opacity: 0.75, y: 4 },
        transition: spring.continuity,
      };
    case "settle":
      return {
        initial: { opacity: 0.92 },
        animate: { opacity: 1, y: 0 },
        exit: { opacity: 0.85 },
        transition: spring.continuity,
      };
    case "dissolve":
      return {
        initial: { opacity: 1 },
        animate: { opacity: 0.4 },
        exit: { opacity: 0 },
        transition: spring.continuity,
      };
    case "orbit":
      // Memory — settle into the field.
      return {
        initial: { opacity: 0.5 },
        animate: { opacity: 0.78 },
        exit: { opacity: 0.32 },
        transition: spring.memory,
      };
    case "compress":
      return {
        initial: { opacity: 1, scale: 1 },
        animate: { opacity: 0.55, scale: 0.97 },
        exit: { opacity: 0.3, scale: 0.96 },
        transition: spring.memory,
      };
    case "restore":
      // Reconstruction — rise into place.
      return {
        initial: { opacity: 0, y: 10 },
        animate: { opacity: 1, y: 0 },
        exit: { opacity: 0, y: 6 },
        transition: spring.reconstruction,
      };
    case "expand":
      return {
        initial: { opacity: 0.88, height: 0 },
        animate: { opacity: 1, height: "auto" },
        exit: { opacity: 0.7, height: 0 },
        transition: spring.reconstruction,
      };
    default:
      return {
        animate: { opacity: 1 },
        transition: spring.continuity,
      };
  }
}

export const contentTransition = (reduceMotion: boolean | null): Transition =>
  reduceMotion
    ? instant
    : { ...spring.continuity, opacity: { duration: duration.continuity } };

export const layoutTransition = (reduceMotion: boolean | null): Transition =>
  reduceMotion ? instant : spring.reconstruction;
