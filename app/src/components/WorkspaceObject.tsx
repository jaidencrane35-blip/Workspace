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
import { spring } from "../design-system";
import type {
  CanvasSlot,
  WorkspaceObjectKind,
  WorkspaceObjectState,
} from "../lib/objectState";
import { useAttentionRegistration } from "./AttentionEngine";
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
  /** Optional attention weight override (0–1). Engine used when omitted. */
  attentionWeight?: number;
  onActivate?: () => void;
  children: ReactNode;
}

/**
 * Spatial Workspace Object — attention-weighted; layout-preserving morphs.
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
  attentionWeight,
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
  const attention = useAttentionRegistration(objectId);

  const weight = attentionWeight ?? attention.weight;
  const visual =
    attentionWeight != null
      ? {
          ...attention,
          weight: attentionWeight,
          scale: 0.94 + attentionWeight * 0.1,
          opacity: 0.38 + attentionWeight * 0.62,
          blur: (1 - attentionWeight) * 3.2,
          y: (1 - attentionWeight) * 6,
          elevation: (attentionWeight >= 0.85
            ? 3
            : attentionWeight >= 0.48
              ? 2
              : 1) as 1 | 2 | 3,
          lit: attentionWeight >= 0.82,
          tier: attention.tier,
        }
      : attention;

  const resolvedLevel: WorkspaceSurfaceLevel =
    level ??
    (visual.elevation === 3
      ? "overlay"
      : visual.elevation === 2
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
        `ws-object--${visual.tier}`,
        `is-${state}`,
        className,
      ]
        .filter(Boolean)
        .join(" ")}
      data-object-id={objectId}
      data-kind={kind}
      data-state={state}
      data-attention={visual.tier}
      data-weight={weight.toFixed(2)}
      aria-hidden={ariaHidden}
      title={title}
      style={{
        zIndex: visual.interactionPriority,
        pointerEvents: hidden ? "none" : undefined,
        ...style,
      }}
      animate={
        reduceMotion
          ? {
              opacity: hidden ? 0 : visual.opacity,
              scale: 1,
              y: 0,
              filter: "blur(0px)",
            }
          : {
              opacity: hidden ? 0 : visual.opacity,
              scale: visual.scale,
              y: visual.y * -0.35,
              filter: `blur(${visual.blur}px)`,
            }
      }
      transition={reduceMotion ? { duration: 0.01 } : spring.lush}
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
              : visual.tier === "primary"
                ? "hero"
                : "default"
        }
        padding={
          kind === "quick-action"
            ? "sm"
            : visual.tier === "primary"
              ? "lg"
              : "md"
        }
        interactive={interactive && visual.tier !== "context"}
        lit={lit || visual.lit}
        layout={false}
      >
        {children}
      </WorkspaceSurface>
    </motion.div>
  );
}

export const WorkspaceObject = memo(WorkspaceObjectInner);
