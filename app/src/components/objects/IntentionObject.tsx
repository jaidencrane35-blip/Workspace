import { memo } from "react";
import type { WorkspaceObjectState } from "../../lib/objectState";
import { WorkspaceObject } from "../WorkspaceObject";

interface IntentionObjectProps {
  id: string;
  text: string;
  meta?: string;
  state?: WorkspaceObjectState;
  className?: string;
}

function IntentionObjectInner({
  id,
  text,
  meta,
  state = "idle",
  className = "",
}: IntentionObjectProps) {
  return (
    <WorkspaceObject
      objectId={id}
      kind="intention"
      slot="float"
      state={state}
      className={`intention-object ${className}`.trim()}
    >
      <div className="quote-pane__mark" aria-hidden="true">
        “
      </div>
      <p className="quote-pane__text">{text}</p>
      {meta && <p className="quote-pane__meta">{meta}</p>}
    </WorkspaceObject>
  );
}

export const IntentionObject = memo(IntentionObjectInner);
