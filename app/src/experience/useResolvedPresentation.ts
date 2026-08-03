import { useEffect, useState } from "react";
import {
  ADAPTATION_CHANGED_EVENT,
  defaultAdaptationStore,
  identityPresentation,
  type ResolvedPresentation,
} from "./workspaceAdaptation";
import { resolvePresentationWithAnticipation } from "./workspaceAnticipation";

/**
 * Live presentation:
 * Runtime → Pack → Evolution → Presence → Anticipation → Presentation
 * Calibration / stability refine confidence and damping internally (no new stages).
 * Anticipation predicts only; never executes. Inactive ⇒ presence fallback.
 */
export function useResolvedPresentation(): ResolvedPresentation {
  const [presentation, setPresentation] = useState<ResolvedPresentation>(() => {
    if (typeof window === "undefined") {
      return identityPresentation();
    }
    return resolvePresentationWithAnticipation(defaultAdaptationStore());
  });

  useEffect(() => {
    const refresh = () => {
      setPresentation(
        resolvePresentationWithAnticipation(defaultAdaptationStore()),
      );
    };
    refresh();
    window.addEventListener(ADAPTATION_CHANGED_EVENT, refresh);
    window.addEventListener("storage", refresh);
    return () => {
      window.removeEventListener(ADAPTATION_CHANGED_EVENT, refresh);
      window.removeEventListener("storage", refresh);
    };
  }, []);

  return presentation;
}
