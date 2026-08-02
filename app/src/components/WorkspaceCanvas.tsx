import { LayoutGroup, motion, useReducedMotion } from "motion/react";
import { type ReactNode, useEffect } from "react";
import { spring } from "../design-system";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import { useIntentEngine } from "./IntentEngine";
import { useWorkspaceComposition } from "./WorkspaceComposition";

interface WorkspaceCanvasProps {
  destination: PilotPrimaryView;
  children: ReactNode;
}

/**
 * Persistent Workspace Canvas — reshaped continuously by Intent profiles.
 */
export function WorkspaceCanvas({
  destination,
  children,
}: WorkspaceCanvasProps) {
  const reduceMotion = useReducedMotion();
  const {
    density,
    writingMode,
    ambient,
    setDestination,
    focusedObjectId,
    selectedObjectId,
    primaryObjectId,
  } = useWorkspaceComposition();
  const { intent, profile } = useIntentEngine();

  useEffect(() => {
    setDestination(destination);
  }, [destination, setDestination]);

  return (
    <LayoutGroup id="workspace-canvas">
      <motion.div
        className="ws-canvas-root"
        data-destination={destination}
        data-density={density}
        data-writing={writingMode ? "on" : "off"}
        data-ambient={ambient}
        data-intent={intent}
        data-scene={profile.attentionScene}
        data-light={profile.lightingBias}
        data-focus={focusedObjectId ?? ""}
        data-selected={selectedObjectId ?? primaryObjectId ?? ""}
        layout={!reduceMotion}
        animate={
          reduceMotion
            ? undefined
            : {
                filter:
                  intent === "capture"
                    ? "brightness(0.82) saturate(0.9)"
                    : intent === "restore"
                      ? "brightness(0.92) saturate(0.95)"
                      : `brightness(${0.9 + profile.atmosphereDepth * 0.1}) saturate(1)`,
              }
        }
        transition={spring[profile.motion]}
      >
        <div className="ws-canvas-root__veil" aria-hidden="true" />
        <div className="ws-canvas-root__field">{children}</div>
      </motion.div>
    </LayoutGroup>
  );
}
