/**
 * Purpose: Copy helpers for the Desktop Reality Stage.
 * Owner: Frontend product shell (Product Contract V3)
 * Inputs: optional profile name
 * Outputs: Stage labels
 * Dependencies: None (pure)
 * Non-responsibilities: IPC, window control, essays, canvas practice copy
 */

export function layoutsStageEyebrow(): string {
  return "Desktop reality";
}

export function layoutsStageTitle(workspaceName: string | null | undefined): string {
  const name = workspaceName?.trim() ?? "";
  return name ? `${name} — desktop` : "Your desktop";
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
