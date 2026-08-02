import type { HTMLAttributes, ReactNode } from "react";

export type ElevatedCardTone = "default" | "hero" | "soft" | "ghost" | "solid";
export type ElevatedCardPadding = "sm" | "md" | "lg";

interface ElevatedCardProps extends HTMLAttributes<HTMLElement> {
  as?: "article" | "aside" | "div" | "section";
  tone?: ElevatedCardTone;
  padding?: ElevatedCardPadding;
  interactive?: boolean;
  children: ReactNode;
}

/**
 * Single elevated glass card primitive for the Workspace shell.
 * All product surfaces should compose this — not invent card CSS.
 */
export function ElevatedCard({
  as: Tag = "article",
  tone = "default",
  padding = "md",
  interactive = false,
  className = "",
  children,
  ...rest
}: ElevatedCardProps) {
  const classes = [
    "elevated-card",
    `elevated-card--${tone}`,
    `elevated-card--pad-${padding}`,
    interactive ? "elevated-card--interactive" : "",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <Tag className={classes} {...rest}>
      <span className="elevated-card__shine" aria-hidden="true" />
      <div className="elevated-card__body">{children}</div>
    </Tag>
  );
}
