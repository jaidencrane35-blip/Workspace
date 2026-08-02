/** Shared Workspace Object interaction states. */

export type WorkspaceObjectState =
  | "idle"
  | "hover"
  | "focused"
  | "selected"
  | "expanded"
  | "hidden";

export type WorkspaceObjectKind =
  | "moment"
  | "intention"
  | "continue-preview"
  | "checkin-summary"
  | "guide-step"
  | "quick-action";

export type CanvasSlot = "anchor" | "float" | "orbit" | "utility" | "stage";

export function objectDepth(state: WorkspaceObjectState): number {
  switch (state) {
    case "hidden":
      return 0;
    case "idle":
      return 1;
    case "hover":
      return 2;
    case "focused":
      return 3;
    case "expanded":
      return 4;
    case "selected":
      return 5;
    default:
      return 1;
  }
}
