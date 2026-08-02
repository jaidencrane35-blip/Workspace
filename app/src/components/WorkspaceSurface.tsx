import { motion, useReducedMotion } from "motion/react";
import type { HTMLAttributes, ReactNode } from "react";
import { spring } from "../design-system";

export type WorkspaceSurfaceLevel = "surface" | "floating" | "overlay";
export type WorkspaceSurfacePadding = "sm" | "md" | "lg" | "xl";
export type WorkspaceSurfaceTone =
  | "default"
  | "hero"
  | "soft"
  | "ghost"
  | "solid"
  | "write";

interface WorkspaceSurfaceProps extends HTMLAttributes<HTMLElement> {
  as?: "article" | "aside" | "div" | "section";
  level?: WorkspaceSurfaceLevel;
  tone?: WorkspaceSurfaceTone;
  padding?: WorkspaceSurfacePadding;
  interactive?: boolean;
  lit?: boolean;
  layout?: boolean;
  children: ReactNode;
}

/**
 * WorkspaceSurface — glass material at Surface / Floating / Overlay.
 */
export function WorkspaceSurface({
  as: Tag = "article",
  level = "surface",
  tone = "default",
  padding = "md",
  interactive = false,
  lit = false,
  layout = false,
  className = "",
  children,
  onMouseEnter,
  onMouseLeave,
  ...rest
}: WorkspaceSurfaceProps) {
  const reduceMotion = useReducedMotion();
  const classes = [
    "ws-surface",
    `ws-surface--${level}`,
    `ws-surface--${tone}`,
    `ws-surface--pad-${padding}`,
    interactive ? "ws-surface--interactive" : "",
    lit ? "ws-surface--lit" : "",
    // Compat for prior elevated-card consumers during migration
    "elevated-card",
    `elevated-card--${tone === "write" ? "hero" : tone}`,
    `elevated-card--pad-${padding}`,
    className,
  ]
    .filter(Boolean)
    .join(" ");

  const body = (
    <>
      <span className="ws-surface__edge elevated-card__edge" aria-hidden="true" />
      <span className="ws-surface__shine elevated-card__shine" aria-hidden="true" />
      <div className="ws-surface__body elevated-card__body">{children}</div>
    </>
  );

  if (layout && !reduceMotion) {
    return (
      <motion.div
        layout
        className={classes}
        transition={spring.layout}
        onMouseEnter={onMouseEnter as never}
        onMouseLeave={onMouseLeave as never}
        {...(rest as object)}
      >
        {body}
      </motion.div>
    );
  }

  return (
    <Tag
      className={classes}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      {...rest}
    >
      {body}
    </Tag>
  );
}

/** @deprecated Prefer WorkspaceSurface */
export const ElevatedCard = WorkspaceSurface;
