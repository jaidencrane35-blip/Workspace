import { motion, useReducedMotion } from "motion/react";
import { memo } from "react";
import { INTENT_LABELS } from "../lib/intent";
import { motionPrimitive } from "../lib/motion";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import { useIntentEngine } from "./IntentEngine";

interface CommandSurfaceProps {
  onNavigate: (view: PilotPrimaryView) => void;
  onCreateWorkspace?: () => void;
  busy?: boolean;
}

/**
 * Contextual next actions for the active Intent — quiet, not a toolbar.
 */
function CommandSurfaceInner({
  onNavigate,
  onCreateWorkspace,
  busy = false,
}: CommandSurfaceProps) {
  const { intent, commands, writing, restoring } = useIntentEngine();
  const reduceMotion = useReducedMotion();
  const motionSpec = motionPrimitive("settle", Boolean(reduceMotion));

  const actionable = commands.filter(
    (command) => command.view || command.action === "create",
  );

  // Continuous place: only empty-home create remains as chrome command.
  // Capture / restore / reflect / learn stay inside their region surfaces.
  if (
    actionable.length === 0 ||
    intent === "landing" ||
    intent === "restore" ||
    intent === "reflect" ||
    intent === "learn" ||
    intent === "capture"
  ) {
    return null;
  }

  return (
    <motion.div
      className="ws-command"
      data-intent={intent}
      aria-label={`${INTENT_LABELS[intent]} actions`}
      initial={motionSpec.initial}
      animate={motionSpec.animate}
      transition={motionSpec.transition}
    >
      <div className="ws-command__actions">
        {actionable.map((command, index) => (
          <button
            key={command.id}
            type="button"
            className={
              index === 0 ? "ws-command__action exp-btn primary" : "ws-command__action exp-btn ghost"
            }
            disabled={busy}
            onClick={() => {
              if (command.action === "create") {
                onCreateWorkspace?.();
                return;
              }
              if (command.view) {
                onNavigate(command.view);
              }
            }}
          >
            {command.label}
          </button>
        ))}
      </div>
      {(writing || restoring) && (
        <span className="ws-command__pulse" aria-hidden="true" />
      )}
    </motion.div>
  );
}

export const CommandSurface = memo(CommandSurfaceInner);
