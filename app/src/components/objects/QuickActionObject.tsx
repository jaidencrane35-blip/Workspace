import type { LucideIcon } from "lucide-react";
import { memo } from "react";
import { ICON } from "../../lib/icons";
import { WorkspaceObject } from "../WorkspaceObject";

interface QuickActionObjectProps {
  id: string;
  label: string;
  icon: LucideIcon;
  disabled?: boolean;
  primary?: boolean;
  onClick: () => void;
}

function QuickActionObjectInner({
  id,
  label,
  icon: Icon,
  disabled = false,
  primary = false,
  onClick,
}: QuickActionObjectProps) {
  return (
    <WorkspaceObject
      objectId={id}
      kind="quick-action"
      slot="utility"
      state="idle"
      interactive={!disabled}
      className={primary ? "quick-action is-primary" : "quick-action"}
    >
      <button
        type="button"
        className={primary ? "exp-btn primary" : "exp-btn"}
        disabled={disabled}
        onClick={onClick}
      >
        <Icon size={ICON.md} strokeWidth={ICON.stroke} aria-hidden="true" />
        {label}
      </button>
    </WorkspaceObject>
  );
}

export const QuickActionObject = memo(QuickActionObjectInner);
