export type WorkspaceDensity = "focus" | "balanced" | "flow";

/** Viewport-driven density. Manual override reserved for later. */
export function densityFromViewport(width: number): WorkspaceDensity {
  if (width < 900) {
    return "focus";
  }
  if (width < 1280) {
    return "balanced";
  }
  return "flow";
}
