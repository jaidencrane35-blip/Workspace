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
import {
  modulateByIntentKind,
  visualFromCognitiveWeight,
} from "../lib/cognitive";
import { layoutTransition } from "../lib/motion";
import type {
  CanvasSlot,
  WorkspaceObjectKind,
  WorkspaceObjectState,
} from "../lib/objectState";
import { useAttentionRegistration } from "./AttentionEngine";
import { useIntentEngine } from "./IntentEngine";
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
  attentionWeight?: number;
  onActivate?: () => void;
  children: ReactNode;
}

/**
 * Spatial Workspace Object — attention + intent ecosystem relationships.
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
  const { influenceObjectId, setInfluenceObjectId, intent, profile } =
    useIntentEngine();

  const baseWeight = attentionWeight ?? attention.weight;
  const weight = modulateByIntentKind(
    baseWeight,
    intent,
    kind,
    objectId,
    influenceObjectId,
    slot,
  );
  const visual = visualFromCognitiveWeight(weight);

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
      setInfluenceObjectId(objectId);
      onMouseEnter?.(event);
    },
    [controlled, onMouseEnter, objectId, setInfluenceObjectId, state],
  );

  const onLeave = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (!controlled && state === "hover") {
        setLocal("idle");
      }
      setInfluenceObjectId(null);
      onMouseLeave?.(event);
    },
    [controlled, onMouseLeave, setInfluenceObjectId, state],
  );

  const canKeyboardActivate = Boolean(interactive && onActivate && !hidden);

  return (
    <motion.div
      layout={false}
      layoutId={layoutId ?? `ws-obj-${objectId}`}
      className={[
        "ws-object",
        `ws-object--${kind}`,
        `ws-object--${slot}`,
        `ws-object--${visual.tier}`,
        `is-${state}`,
        influenceObjectId === objectId ? "is-influencing" : "",
        className,
      ]
        .filter(Boolean)
        .join(" ")}
      data-object-id={objectId}
      data-kind={kind}
      data-state={state}
      data-attention={visual.tier}
      data-intent={intent}
      data-weight={weight.toFixed(2)}
      role={canKeyboardActivate ? "button" : undefined}
      tabIndex={canKeyboardActivate ? 0 : undefined}
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
              scale: visual.scale * (profile.spacingScale > 1.1 ? 1.01 : 1),
              y: visual.y * -0.35,
              filter: `blur(${visual.blur}px)`,
            }
      }
      transition={layoutTransition(reduceMotion)}
      onMouseEnter={onEnter}
      onMouseLeave={onLeave}
      onFocus={(event: FocusEvent<HTMLDivElement>) => {
        if (!controlled) {
          setLocal("focused");
        }
        setInfluenceObjectId(objectId);
        onFocus?.(event);
      }}
      onBlur={(event: FocusEvent<HTMLDivElement>) => {
        if (!controlled && state === "focused") {
          setLocal("idle");
        }
        setInfluenceObjectId(null);
        onBlur?.(event);
      }}
      onKeyDown={(event) => {
        if (!canKeyboardActivate) {
          return;
        }
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          onActivate?.();
        }
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
        lit={lit || visual.lit || influenceObjectId === objectId}
        layout={false}
      >
        {children}
      </WorkspaceSurface>
    </motion.div>
  );
}

export const WorkspaceObject = memo(WorkspaceObjectInner);
