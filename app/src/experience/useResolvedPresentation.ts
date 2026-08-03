import { useEffect, useState } from "react";
import {
  ADAPTATION_CHANGED_EVENT,
  defaultAdaptationStore,
  identityPresentation,
  type ResolvedPresentation,
} from "./workspaceAdaptation";
import { resolvePresentationFromRuntime } from "./workspaceMemoryEvolution";

/**
 * Live presentation from Runtime → Pack → MemoryEvolution → Presentation.
 * Falls back to the active adaptation set when evolution is inactive.
 * Defaults to identity when nothing applies.
 */
export function useResolvedPresentation(): ResolvedPresentation {
  const [presentation, setPresentation] = useState<ResolvedPresentation>(() => {
    if (typeof window === "undefined") {
      return identityPresentation();
    }
    return resolvePresentationFromRuntime(defaultAdaptationStore());
  });

  useEffect(() => {
    const refresh = () => {
      setPresentation(resolvePresentationFromRuntime(defaultAdaptationStore()));
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
