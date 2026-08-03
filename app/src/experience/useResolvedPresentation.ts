import { useEffect, useState } from "react";
import {
  ADAPTATION_CHANGED_EVENT,
  defaultAdaptationStore,
  identityPresentation,
  type ResolvedPresentation,
} from "./workspaceAdaptation";
import { resolvePresentationWithPresence } from "./workspacePresence";

/**
 * Live presentation:
 * Runtime → Pack → MemoryEvolution → Presence → Presentation
 * Presence inactive ⇒ evolution / active adaptation fallback.
 */
export function useResolvedPresentation(): ResolvedPresentation {
  const [presentation, setPresentation] = useState<ResolvedPresentation>(() => {
    if (typeof window === "undefined") {
      return identityPresentation();
    }
    return resolvePresentationWithPresence(defaultAdaptationStore());
  });

  useEffect(() => {
    const refresh = () => {
      setPresentation(resolvePresentationWithPresence(defaultAdaptationStore()));
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
