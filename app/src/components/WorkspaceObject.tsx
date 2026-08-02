import { motion, useReducedMotion } from "motion/react";
import {
  memo,
  useCallback,
  useState,
  type FocusEvent,
  type HTMLAttributes,
  type MouseEvent,
  type ReactNode,
} from "react";
import { opacity, spring } from "../design-system";
import {
  objectDepth,
  type CanvasSlot,
  type WorkspaceObjectKind,
  type WorkspaceObjectState,
} from "../lib/objectState";
import { WorkspaceSurface, type WorkspaceSurfaceLevel } from "./WorkspaceSurface";

export interface WorkspaceObjectProps
  extends Omit<HTMLAttributes<HTMLDivElement>, "onSelect"> {
  objectId: string;
  kind: WorkspaceObjectKind;
  slot?: CanvasSlot;
  state?: WorkspaceObjectState;
  level?: WorkspaceSurfaceLevel;
  layoutId?: string;
  lit?: boolean;
  interactive?: boolean;
  onActivate?: () => void;
  children: ReactNode;
}

/**
 * Spatial Workspace Object — state morphs via shared layout; never remounts.
 */
function WorkspaceObjectInner({
  objectId,
  kind,
  slot = "stage",
  state: controlled,
  level,
  layoutId,
  lit = false,
  interactive = true,
  onActivate,
  className = "",
  children,
  onMouseEnter,
  onMouseLeave,
  onFocus,
  onBlur,
  onClick,
  style,
  "aria-hidden": ariaHidden,
  title,
}: WorkspaceObjectProps) {
  const reduceMotion = useReducedMotion();
  const [local, setLocal] = useState<WorkspaceObjectState>("idle");
  const state = controlled ?? local;
  const hidden = state === "hidden";

  const resolvedLevel: WorkspaceSurfaceLevel =
    level ??
    (state === "selected" || state === "expanded" || state === "focused"
      ? "floating"
      : "surface");

  const onEnter = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (!controlled && state !== "selected" && state !== "expanded") {
        setLocal("hover");
      }
      onMouseEnter?.(event);
    },
    [controlled, onMouseEnter, state],
  );

  const onLeave = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (!controlled && state === "hover") {
        setLocal("idle");
      }
      onMouseLeave?.(event);
    },
    [controlled, onMouseLeave, state],
  );

  return (
    <motion.div
      layout={!reduceMotion}
      layoutId={layoutId ?? `ws-obj-${objectId}`}
      className={[
        "ws-object",
        `ws-object--${kind}`,
        `ws-object--${slot}`,
        `is-${state}`,
        className,
      ]
        .filter(Boolean)
        .join(" ")}
      data-object-id={objectId}
      data-kind={kind}
      data-state={state}
      aria-hidden={ariaHidden}
      title={title}
      style={{
        zIndex: objectDepth(state),
        opacity: hidden ? 0 : state === "idle" ? opacity.strong : opacity.full,
        pointerEvents: hidden ? "none" : undefined,
        ...style,
      }}
      animate={
        reduceMotion
          ? undefined
          : {
              scale:
                state === "hover"
                  ? 1.015
                  : state === "selected" || state === "expanded"
                    ? 1.02
                    : 1,
              y: state === "hover" ? -2 : state === "selected" ? -4 : 0,
              filter:
                state === "idle" && slot === "orbit"
                  ? "saturate(0.85) brightness(0.92)"
                  : "none",
            }
      }
      transition={spring.layout}
      onMouseEnter={onEnter}
      onMouseLeave={onLeave}
      onFocus={(event: FocusEvent<HTMLDivElement>) => {
        if (!controlled) {
          setLocal("focused");
        }
        onFocus?.(event);
      }}
      onBlur={(event: FocusEvent<HTMLDivElement>) => {
        if (!controlled && state === "focused") {
          setLocal("idle");
        }
        onBlur?.(event);
      }}
      onClick={(event) => {
        onClick?.(event);
        onActivate?.();
      }}
    >
      <WorkspaceSurface
        level={resolvedLevel}
        tone={
          kind === "intention"
            ? "soft"
            : kind === "quick-action"
              ? "default"
              : state === "expanded" || state === "selected"
                ? "hero"
                : "default"
        }
        padding={
          kind === "quick-action"
            ? "sm"
            : state === "expanded" || state === "selected"
              ? "lg"
              : "md"
        }
        interactive={interactive}
        lit={lit || state === "selected" || state === "focused"}
        layout={false}
      >
        {children}
      </WorkspaceSurface>
    </motion.div>
  );
}

export const WorkspaceObject = memo(WorkspaceObjectInner);
