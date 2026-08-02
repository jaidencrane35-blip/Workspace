import { motion, useReducedMotion } from "motion/react";
import { memo } from "react";
import { spring } from "../../design-system";
import type { WorkspaceObjectState } from "../../lib/objectState";
import { WorkspaceObject } from "../WorkspaceObject";

interface CheckInSummaryObjectProps {
  id: string;
  label: string;
  value: string;
  unit?: string;
  state?: WorkspaceObjectState;
  hero?: boolean;
}

function CheckInSummaryObjectInner({
  id,
  label,
  value,
  unit,
  state = "idle",
  hero = false,
}: CheckInSummaryObjectProps) {
  const reduceMotion = useReducedMotion();
  return (
    <WorkspaceObject
      objectId={id}
      kind="checkin-summary"
      slot="float"
      state={state}
      level="floating"
      className={hero ? "metric-orb checkin-summary is-hero" : "metric-orb checkin-summary"}
    >
      <p className="exp-kicker">{label}</p>
      <motion.p
        key={`${id}-${value}`}
        className="exp-stat metric-orb__value"
        initial={reduceMotion ? false : { opacity: 0, y: 6 }}
        animate={{ opacity: 1, y: 0 }}
        transition={spring.soft}
      >
        {value}
      </motion.p>
      {unit && <p className="muted">{unit}</p>}
      <span className="metric-ring" aria-hidden="true" />
    </WorkspaceObject>
  );
}

export const CheckInSummaryObject = memo(CheckInSummaryObjectInner);
