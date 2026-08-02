import { LayoutGroup, motion, useReducedMotion } from "motion/react";
import { type ReactNode, useEffect } from "react";
import { spring } from "../design-system";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import { useWorkspaceComposition } from "./WorkspaceComposition";

interface WorkspaceCanvasProps {
  destination: PilotPrimaryView;
  children: ReactNode;
}

/**
 * Persistent Workspace Canvas — owns composition, depth, and focus atmosphere.
 * Destinations place objects; they do not become separate pages.
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
    attentionScene,
    primaryObjectId,
  } = useWorkspaceComposition();

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
        data-scene={attentionScene}
        data-focus={focusedObjectId ?? ""}
        data-selected={selectedObjectId ?? primaryObjectId ?? ""}
        layout={!reduceMotion}
        animate={
          reduceMotion
            ? undefined
            : {
                filter: writingMode
                  ? "brightness(0.82) saturate(0.9)"
                  : attentionScene === "restore"
                    ? "brightness(0.92) saturate(0.95)"
                    : "brightness(1) saturate(1)",
              }
        }
        transition={spring.soft}
      >
        <div className="ws-canvas-root__veil" aria-hidden="true" />
        <div className="ws-canvas-root__field">{children}</div>
      </motion.div>
    </LayoutGroup>
  );
}
