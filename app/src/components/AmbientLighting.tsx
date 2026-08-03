import { motion, useReducedMotion } from "motion/react";
import { useEffect, useState } from "react";
import { lighting, spring } from "../design-system";
import type { AttentionScene } from "../lib/attention";

export type AmbientFocus =
  | "none"
  | "workspace"
  | "dock"
  | "input"
  | "card"
  | "moment";

interface AmbientLightingProps {
  focus?: AmbientFocus;
  scene?: AttentionScene;
  /** Primary object presence deepens the living wash. */
  living?: boolean;
  active?: boolean;
}

type Spot = { x: number; y: number; strength: number; warm: number };

/**
 * Continuous environmental illumination — attention and memory, not decoration.
 */
export function AmbientLighting({
  focus = "none",
  scene = "default",
  living = false,
  active = true,
}: AmbientLightingProps) {
  const reduceMotion = useReducedMotion();
  const [spot, setSpot] = useState<Spot>({
    x: 50,
    y: 38,
    strength: 0.22,
    warm: 0,
  });

  useEffect(() => {
    const byFocus: Record<AmbientFocus, Spot> = {
      none: { x: 50, y: 40, strength: 0.14, warm: 0 },
      workspace: { x: 48, y: 34, strength: living ? 0.3 : 0.22, warm: 0.05 },
      dock: { x: 50, y: 88, strength: 0.2, warm: 0 },
      input: { x: 50, y: 46, strength: 0.36, warm: 0 },
      card: { x: 44, y: 42, strength: 0.24, warm: 0.04 },
      moment: { x: 40, y: 36, strength: living ? 0.38 : 0.3, warm: 0.08 },
    };

    let next = byFocus[focus];
    if (scene === "writing") {
      next = { x: 50, y: 44, strength: 0.4, warm: 0 };
    } else if (scene === "restore") {
      next = { x: 42, y: 40, strength: 0.34, warm: 0.16 };
    } else if (scene === "checkin") {
      next = { x: 50, y: 38, strength: 0.22, warm: 0.02 };
    } else if (scene === "guide") {
      next = { x: 46, y: 36, strength: 0.18, warm: 0 };
    } else if (scene === "empty") {
      next = { x: 50, y: 42, strength: 0.16, warm: 0 };
    }
    setSpot(next);
  }, [focus, scene, living]);

  if (!active) {
    return null;
  }

  const cool = lighting.focus;
  const warm = lighting.warm;

  return (
    <div className="ws-ambient" aria-hidden="true" data-living={living ? "on" : "off"}>
      <motion.div
        className="ws-ambient__spot"
        animate={{
          left: `${spot.x}%`,
          top: `${spot.y}%`,
          opacity: spot.strength,
        }}
        transition={reduceMotion ? { duration: 0.01 } : spring.lush}
        style={{
          background: `radial-gradient(circle at center, ${
            spot.warm > 0.1 ? warm : cool
          }, transparent 68%)`,
        }}
      />
      <motion.div
        className="ws-ambient__wash"
        animate={{
          opacity: living ? 0.38 : focus === "none" ? 0.18 : 0.28,
        }}
        transition={reduceMotion ? { duration: 0.01 } : spring.soft}
      />
      {scene === "restore" ? (
        <motion.div
          className="ws-ambient__memory"
          initial={reduceMotion ? false : { opacity: 0 }}
          animate={{ opacity: 0.22 }}
          transition={reduceMotion ? { duration: 0.01 } : spring.lush}
        />
      ) : null}
    </div>
  );
}
