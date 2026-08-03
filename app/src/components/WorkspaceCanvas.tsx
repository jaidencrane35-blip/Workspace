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
        layout={false}
        animate={
          reduceMotion
            ? undefined
            : {
                filter:
                  intent === "capture" || writingMode
                    ? "brightness(0.78) saturate(0.86)"
                    : intent === "restore"
                      ? "brightness(0.9) saturate(0.92)"
                      : intent === "learn"
                        ? "brightness(0.94) saturate(0.96)"
                        : `brightness(${0.92 + profile.atmosphereDepth * 0.08}) saturate(${0.96 + profile.atmosphereDepth * 0.04})`,
              }
        }
        transition={spring[profile.motion]}
        data-living={primaryObjectId && !writingMode ? "on" : "off"}
      >
        <div className="ws-canvas-root__veil" aria-hidden="true" />
        <div className="ws-canvas-root__field">{children}</div>
      </motion.div>
    </LayoutGroup>
  );
}
