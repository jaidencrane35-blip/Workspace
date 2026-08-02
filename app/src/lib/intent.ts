/** Intent Engine — highest-level experience state (not a route). */

import type { PilotPrimaryView } from "./pilotChrome";
import type { AttentionScene } from "./attention";
import { spring } from "../design-system/tokens";

export type WorkspaceIntent =
  | "landing"
  | "capture"
  | "restore"
  | "reflect"
  | "learn";

export interface IntentCompositionProfile {
  intent: WorkspaceIntent;
  attentionScene: AttentionScene;
  dockEmphasis: number;
  atmosphereDepth: number;
  spacingScale: number;
  lightingBias: "cool" | "warm" | "neutral" | "focus";
  motion: keyof typeof spring;
  primaryHint: string;
  commands: IntentCommand[];
}

export interface IntentCommand {
  id: string;
  label: string;
  view?: PilotPrimaryView;
  action?: "create" | "save" | "continue" | "checkin" | "guide" | "home";
}

export const INTENT_LABELS: Record<WorkspaceIntent, string> = {
  landing: "What matters now",
  capture: "Capture",
  restore: "Restore",
  reflect: "Reflect",
  learn: "Learn",
};

/** Destination chrome still exists for Product Proof; intent is the orchestrator. */
export function intentFromView(view: PilotPrimaryView): WorkspaceIntent {
  switch (view) {
    case "save":
      return "capture";
    case "resume":
      return "restore";
    case "pilot":
      return "reflect";
    case "help":
      return "learn";
    case "home":
    default:
      return "landing";
  }
}

export function viewFromIntent(intent: WorkspaceIntent): PilotPrimaryView {
  switch (intent) {
    case "capture":
      return "save";
    case "restore":
      return "resume";
    case "reflect":
      return "pilot";
    case "learn":
      return "help";
    case "landing":
    default:
      return "home";
  }
}

export function compositionForIntent(
  intent: WorkspaceIntent,
  empty = false,
): IntentCompositionProfile {
  switch (intent) {
    case "capture":
      return {
        intent,
        attentionScene: "writing",
        dockEmphasis: 0.34,
        atmosphereDepth: 0.82,
        spacingScale: 1.12,
        lightingBias: "focus",
        motion: "lush",
        primaryHint: "write-surface",
        commands: [
          { id: "review", label: "Review what will be saved", action: "save" },
          { id: "home", label: "Back to Workspace", action: "home", view: "home" },
        ],
      };
    case "restore":
      return {
        intent,
        attentionScene: "restore",
        dockEmphasis: 0.7,
        atmosphereDepth: 0.9,
        spacingScale: 1.05,
        lightingBias: "warm",
        motion: "lush",
        primaryHint: "moment",
        commands: [],
      };
    case "reflect":
      return {
        intent,
        attentionScene: "checkin",
        dockEmphasis: 0.8,
        atmosphereDepth: 0.88,
        spacingScale: 1,
        lightingBias: "cool",
        motion: "soft",
        primaryHint: "checkin",
        commands: [],
      };
    case "learn":
      return {
        intent,
        attentionScene: "guide",
        dockEmphasis: 0.75,
        atmosphereDepth: 0.86,
        spacingScale: 1.08,
        lightingBias: "cool",
        motion: "soft",
        primaryHint: "guide",
        commands: [],
      };
    case "landing":
    default:
      return {
        intent: "landing",
        attentionScene: empty ? "empty" : "default",
        dockEmphasis: empty ? 0.85 : 1,
        atmosphereDepth: empty ? 0.95 : 1,
        spacingScale: empty ? 1.15 : 1,
        lightingBias: "neutral",
        motion: "soft",
        primaryHint: empty ? "home-create" : "moment",
        commands: empty
          ? [
              { id: "create", label: "Create a workspace", action: "create" },
              { id: "learn", label: "How this works", action: "guide", view: "help" },
            ]
          : [],
      };
  }
}

/**
 * Infer intent from live activity signals. Activity can override destination.
 */
export function inferIntent(signals: {
  view: PilotPrimaryView;
  writing: boolean;
  restoring: boolean;
  empty: boolean;
  reflecting: boolean;
}): WorkspaceIntent {
  if (signals.writing) {
    return "capture";
  }
  if (signals.restoring) {
    return "restore";
  }
  if (signals.reflecting && signals.view === "pilot") {
    return "reflect";
  }
  const base = intentFromView(signals.view);
  if (base === "landing" && signals.empty) {
    return "landing";
  }
  return base;
}
