import { motion, useReducedMotion } from "motion/react";
import { useEffect, useState } from "react";
import { lighting, spring } from "../design-system";

export type AmbientFocus =
  | "none"
  | "workspace"
  | "dock"
  | "input"
  | "card"
  | "moment";

interface AmbientLightingProps {
  focus?: AmbientFocus;
  active?: boolean;
}

/**
 * Soft ambient illumination that follows interaction focus.
 * Fades naturally — no hard neon glows.
 */
export function AmbientLighting({
  focus = "none",
  active = true,
}: AmbientLightingProps) {
  const reduceMotion = useReducedMotion();
  const [spot, setSpot] = useState({ x: 50, y: 38, strength: 0.22 });

  useEffect(() => {
    const map: Record<AmbientFocus, { x: number; y: number; strength: number }> =
      {
        none: { x: 50, y: 40, strength: 0.16 },
        workspace: { x: 48, y: 32, strength: 0.28 },
        dock: { x: 50, y: 88, strength: 0.24 },
        input: { x: 50, y: 48, strength: 0.32 },
        card: { x: 42, y: 44, strength: 0.26 },
        moment: { x: 38, y: 36, strength: 0.34 },
      };
    setSpot(map[focus]);
  }, [focus]);

  if (!active) {
    return null;
  }

  return (
    <div className="ws-ambient" aria-hidden="true">
      <motion.div
        className="ws-ambient__spot"
        animate={{
          left: `${spot.x}%`,
          top: `${spot.y}%`,
          opacity: spot.strength,
        }}
        transition={reduceMotion ? { duration: 0.01 } : spring.lush}
        style={{
          background: `radial-gradient(circle at center, ${lighting.focus}, transparent 68%)`,
        }}
      />
      <motion.div
        className="ws-ambient__wash"
        animate={{ opacity: focus === "none" ? 0.2 : 0.35 }}
        transition={reduceMotion ? { duration: 0.01 } : spring.soft}
      />
    </div>
  );
}
