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
 * Lightweight contextual command surface — adapts to Intent, not a toolbar.
 */
function CommandSurfaceInner({
  onNavigate,
  onCreateWorkspace,
  busy = false,
}: CommandSurfaceProps) {
  const { intent, commands, writing, restoring } = useIntentEngine();
  const reduceMotion = useReducedMotion();
  const motionSpec = motionPrimitive("settle", Boolean(reduceMotion));

  if (commands.length === 0) {
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
      <p className="ws-command__intent">{INTENT_LABELS[intent]}</p>
      <div className="ws-command__actions">
        {commands.map((command) => (
          <button
            key={command.id}
            type="button"
            className="ws-command__action"
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
