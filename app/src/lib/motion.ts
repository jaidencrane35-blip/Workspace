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
      return {
        initial: { opacity: 0, y: 16, filter: "blur(4px)" },
        animate: { opacity: 1, y: 0, filter: "blur(0px)" },
        exit: { opacity: 0, y: -8, filter: "blur(2px)" },
        transition: spring.soft,
      };
    case "elevate":
      return {
        initial: { opacity: 0.85, y: 8, scale: 0.985 },
        animate: { opacity: 1, y: -4, scale: 1.02 },
        exit: { opacity: 0.7, y: 6, scale: 0.99 },
        transition: spring.lush,
      };
    case "settle":
      return {
        initial: { opacity: 0.9, scale: 1.02 },
        animate: { opacity: 1, scale: 1, y: 0 },
        exit: { opacity: 0.85, scale: 0.995 },
        transition: spring.soft,
      };
    case "dissolve":
      return {
        initial: { opacity: 1, filter: "blur(0px)" },
        animate: { opacity: 0.42, filter: "blur(2.5px)" },
        exit: { opacity: 0, filter: "blur(4px)" },
        transition: spring.soft,
      };
    case "focus":
      return {
        initial: { opacity: 0.8, scale: 0.97, filter: "blur(2px)" },
        animate: { opacity: 1, scale: 1.035, filter: "blur(0px)" },
        exit: { opacity: 0.75, scale: 0.99 },
        transition: spring.lush,
      };
    case "restore":
      return {
        initial: { opacity: 0, y: 20, height: 0 },
        animate: { opacity: 1, y: 0, height: "auto" },
        exit: { opacity: 0, y: 12, height: 0 },
        transition: spring.lush,
      };
    case "orbit":
      return {
        initial: { opacity: 0.5, scale: 0.96, y: 10 },
        animate: { opacity: 0.78, scale: 0.98, y: 0 },
        exit: { opacity: 0.35, scale: 0.95 },
        transition: spring.soft,
      };
    case "compress":
      return {
        initial: { opacity: 1, scale: 1 },
        animate: { opacity: 0.5, scale: 0.94, y: 6 },
        exit: { opacity: 0.3, scale: 0.92 },
        transition: spring.soft,
      };
    case "expand":
      return {
        initial: { opacity: 0.85, scale: 0.96, height: 0 },
        animate: { opacity: 1, scale: 1.02, height: "auto" },
        exit: { opacity: 0.7, scale: 0.98, height: 0 },
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
  reduceMotion ? instant : { ...spring.soft, opacity: { duration: 0.24 } };

export const layoutTransition = (reduceMotion: boolean | null): Transition =>
  reduceMotion ? instant : spring.layout;
