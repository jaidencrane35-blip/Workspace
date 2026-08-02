import { motion, useReducedMotion } from "motion/react";
import type { HTMLAttributes, ReactNode } from "react";

export type ElevatedCardTone =
  | "default"
  | "hero"
  | "soft"
  | "ghost"
  | "solid"
  | "float";
export type ElevatedCardPadding = "sm" | "md" | "lg" | "xl";
export type ElevatedCardElevation = 1 | 2 | 3 | 4;

interface ElevatedCardProps extends HTMLAttributes<HTMLElement> {
  as?: "article" | "aside" | "div" | "section";
  tone?: ElevatedCardTone;
  padding?: ElevatedCardPadding;
  elevation?: ElevatedCardElevation;
  interactive?: boolean;
  layout?: boolean;
  children: ReactNode;
}

/**
 * Glass surface primitive — carved from light, not outlined.
 */
export function ElevatedCard({
  as: Tag = "article",
  tone = "default",
  padding = "md",
  elevation = 2,
  interactive = false,
  layout = false,
  className = "",
  children,
  ...rest
}: ElevatedCardProps) {
  const reduceMotion = useReducedMotion();
  const classes = [
    "elevated-card",
    `elevated-card--${tone}`,
    `elevated-card--pad-${padding}`,
    `elevated-card--e${elevation}`,
    interactive ? "elevated-card--interactive" : "",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  if (layout && !reduceMotion) {
    return (
      <motion.div
        layout
        className={classes}
        transition={{ type: "spring", stiffness: 340, damping: 36 }}
        {...(rest as object)}
      >
        <span className="elevated-card__edge" aria-hidden="true" />
        <span className="elevated-card__shine" aria-hidden="true" />
        <div className="elevated-card__body">{children}</div>
      </motion.div>
    );
  }

  return (
    <Tag className={classes} {...rest}>
      <span className="elevated-card__edge" aria-hidden="true" />
      <span className="elevated-card__shine" aria-hidden="true" />
      <div className="elevated-card__body">{children}</div>
    </Tag>
  );
}
