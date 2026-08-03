import { useEffect, useState } from "react";
import {
  ADAPTATION_CHANGED_EVENT,
  defaultAdaptationStore,
  identityPresentation,
  listAdaptations,
  resolvePresentationConfiguration,
  type ResolvedPresentation,
} from "./workspaceAdaptation";

/**
 * Live presentation configuration from approved active adaptations.
 * Defaults to identity (no change) when none are active.
 */
export function useResolvedPresentation(): ResolvedPresentation {
  const [presentation, setPresentation] = useState<ResolvedPresentation>(() => {
    if (typeof window === "undefined") {
      return identityPresentation();
    }
    return resolvePresentationConfiguration(
      listAdaptations(defaultAdaptationStore()),
    );
  });

  useEffect(() => {
    const refresh = () => {
      setPresentation(
        resolvePresentationConfiguration(
          listAdaptations(defaultAdaptationStore()),
        ),
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
