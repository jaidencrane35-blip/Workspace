import type { LucideIcon } from "lucide-react";
import { memo } from "react";
import { ICON } from "../../lib/icons";
import type { WorkspaceObjectState } from "../../lib/objectState";
import { WorkspaceObject } from "../WorkspaceObject";

interface GuideStepObjectProps {
  id: string;
  step: number;
  title: string;
  line: string;
  icon: LucideIcon;
  state?: WorkspaceObjectState;
}

function GuideStepObjectInner({
  id,
  step,
  title,
  line,
  icon: Icon,
  state = "idle",
}: GuideStepObjectProps) {
  return (
    <WorkspaceObject
      objectId={id}
      kind="guide-step"
      slot="stage"
      state={state}
      className="guide-step-object"
    >
      <p className="exp-kicker">Step {step}</p>
      <div className="guide-step__icon">
        <Icon size={ICON.xl} strokeWidth={ICON.stroke} aria-hidden="true" />
      </div>
      <h3>{title}</h3>
      <p>{line}</p>
    </WorkspaceObject>
  );
}

export const GuideStepObject = memo(GuideStepObjectInner);
