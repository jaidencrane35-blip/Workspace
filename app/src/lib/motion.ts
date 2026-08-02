/** Shared motion primitives — no component invents its own animation. */

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
    case "reveal":
      // Opacity-only view swaps — y translation was a measurable CLS source.
      return {
        initial: { opacity: 0 },
        animate: { opacity: 1 },
        exit: { opacity: 0 },
        transition: spring.soft,
      };
    case "elevate":
      return {
        initial: { opacity: 0.9, y: 6 },
        animate: { opacity: 1, y: 0 },
        exit: { opacity: 0.75, y: 4 },
        transition: spring.soft,
      };
    case "settle":
      return {
        initial: { opacity: 0.92 },
        animate: { opacity: 1, y: 0 },
        exit: { opacity: 0.85 },
        transition: spring.soft,
      };
    case "dissolve":
      return {
        initial: { opacity: 1 },
        animate: { opacity: 0.4 },
        exit: { opacity: 0 },
        transition: spring.soft,
      };
    case "focus":
      return {
        initial: { opacity: 0.85, scale: 0.99 },
        animate: { opacity: 1, scale: 1 },
        exit: { opacity: 0.8, scale: 0.995 },
        transition: spring.soft,
      };
    case "restore":
      return {
        initial: { opacity: 0, y: 12 },
        animate: { opacity: 1, y: 0 },
        exit: { opacity: 0, y: 8 },
        transition: spring.lush,
      };
    case "orbit":
      return {
        initial: { opacity: 0.55, y: 6 },
        animate: { opacity: 0.82, y: 0 },
        exit: { opacity: 0.35 },
        transition: spring.soft,
      };
    case "compress":
      return {
        initial: { opacity: 1, scale: 1 },
        animate: { opacity: 0.55, scale: 0.97 },
        exit: { opacity: 0.3, scale: 0.96 },
        transition: spring.soft,
      };
    case "expand":
      return {
        initial: { opacity: 0.88, height: 0 },
        animate: { opacity: 1, height: "auto" },
        exit: { opacity: 0.7, height: 0 },
        transition: spring.lush,
      };
    default:
      return {
        animate: { opacity: 1 },
        transition: spring.soft,
      };
  }
}

export const contentTransition = (reduceMotion: boolean | null): Transition =>
  reduceMotion ? instant : { ...spring.soft, opacity: { duration: 0.22 } };

export const layoutTransition = (reduceMotion: boolean | null): Transition =>
  reduceMotion ? instant : spring.layout;
