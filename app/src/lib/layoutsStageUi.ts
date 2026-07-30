/**
 * Purpose: Copy helpers for the Desktop Reality Stage.
 * Owner: Frontend product shell (Product Contract V4)
 * Inputs: optional profile name
 * Outputs: Stage labels
 * Dependencies: None (pure)
 */

export function layoutsStageTitle(workspaceName: string | null | undefined): string {
  const name = workspaceName?.trim() ?? "";
  return name ? name : "Desktop";
}

export function layoutsStageEmptyAppsCopy(): {
  title: string;
  body: string;
} {
  return {
    title: "No library apps",
    body: "",
  };
}

export function layoutsStageRegistryHeading(): string {
  return "Library";
}
