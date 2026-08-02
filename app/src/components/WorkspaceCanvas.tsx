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
        data-focus={focusedObjectId ?? ""}
        data-selected={selectedObjectId ?? ""}
        layout={!reduceMotion}
        animate={
          reduceMotion
            ? undefined
            : {
                filter: writingMode
                  ? "brightness(0.88) saturate(0.92)"
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
