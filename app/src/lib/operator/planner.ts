import type { IntentAction } from "../intentBridge";
import type { OperatorPlan } from "./types";

const CAPABILITY_KINDS = new Set<IntentAction["kind"]>([
  "clipboardRead",
  "clipboardWrite",
  "appOpen",
  "appLaunch",
  "appFocus",
  "appClose",
  "appMinimize",
  "appRestore",
  "appEnumerate",
  "winEnumerate",
  "winActive",
  "winMonitors",
  "winBounds",
  "winMaximize",
  "winMinimize",
  "winRestore",
  "winSnap",
  "winCenter",
  "winMoveMonitor",
  "winFocus",
  "winResize",
]);

export function isCapabilityIntent(action: IntentAction): boolean {
  return CAPABILITY_KINDS.has(action.kind);
}

/**
 * Map accepted intents to an ordered Operator plan.
 * Multi-step plans are Operator orchestration — providers stay independent.
 */
export function planFromIntent(action: IntentAction): OperatorPlan | null {
  switch (action.kind) {
    case "clipboardRead":
      return { steps: [{ domain: "clipboard", operation: "read" }] };
    case "clipboardWrite":
      return {
        steps: [
          {
            domain: "clipboard",
            operation: "write",
            args: { text: action.text },
          },
        ],
      };
    case "appEnumerate":
      return { steps: [{ domain: "application", operation: "enumerate" }] };
    case "appLaunch":
      return {
        steps: [
          {
            domain: "application",
            operation: "launch",
            args: { query: action.query },
          },
        ],
      };
    case "appFocus":
      return {
        steps: [
          {
            domain: "application",
            operation: "focus",
            args: { query: action.query },
          },
        ],
      };
    case "appClose":
      return {
        steps: [
          {
            domain: "application",
            operation: "close",
            args: { query: action.query },
          },
        ],
      };
    case "appMinimize":
      return {
        steps: [
          {
            domain: "application",
            operation: "minimize",
            args: { query: action.query },
          },
        ],
      };
    case "appRestore":
      return {
        steps: [
          {
            domain: "application",
            operation: "restore",
            args: { query: action.query },
          },
        ],
      };
    case "appOpen":
      // Composition: find → focus | launch (Operator-orchestrated).
      return {
        compositionId: "app.open_or_focus",
        steps: [
          {
            domain: "application",
            operation: "find",
            args: { query: action.query },
          },
        ],
      };
    case "winEnumerate":
      return { steps: [{ domain: "window", operation: "enumerate" }] };
    case "winActive":
      return { steps: [{ domain: "window", operation: "active" }] };
    case "winMonitors":
      return { steps: [{ domain: "window", operation: "monitors" }] };
    case "winBounds":
      return {
        steps: [
          {
            domain: "window",
            operation: "bounds",
            args: { query: action.query },
          },
        ],
      };
    case "winMaximize":
      return {
        steps: [
          {
            domain: "window",
            operation: "maximize",
            args: { query: action.query },
          },
        ],
      };
    case "winMinimize":
      return {
        steps: [
          {
            domain: "window",
            operation: "minimize",
            args: { query: action.query },
          },
        ],
      };
    case "winRestore":
      return {
        steps: [
          {
            domain: "window",
            operation: "restore",
            args: { query: action.query },
          },
        ],
      };
    case "winCenter":
      return {
        steps: [
          {
            domain: "window",
            operation: "center",
            args: { query: action.query },
          },
        ],
      };
    case "winFocus":
      return {
        steps: [
          {
            domain: "window",
            operation: "focus",
            args: { query: action.query },
          },
        ],
      };
    case "winSnap":
      return {
        steps: [
          {
            domain: "window",
            operation: "snap",
            args: { query: action.query, snap: action.snap },
          },
        ],
      };
    case "winMoveMonitor":
      return {
        steps: [
          {
            domain: "window",
            operation: "move",
            args: {
              query: action.query,
              monitor_index: action.monitorIndex,
            },
          },
        ],
      };
    case "winResize":
      return {
        steps: [
          {
            domain: "window",
            operation: "resize",
            args: {
              query: action.query,
              width: action.width,
              height: action.height,
            },
          },
        ],
      };
    default:
      return null;
  }
}
