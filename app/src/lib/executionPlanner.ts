/**
 * Execution Planner (P16.34) — Intent Layer.
 *
 * Goal → Intent → Capabilities → Execution Plan → (Kernel) Execution.
 * Plans are reasoned before CapabilityIntent leaves the Intent Layer.
 * Providers never see this structure; Kernel still owns composition.
 */

import { CAPABILITY_GRAPH } from "./capabilityRegistry";
import type { IntentAction } from "./intentBridge";

export type PlanStepStatus =
  | "planned"
  | "delegated_to_kernel"
  | "unsupported"
  | "recover";

export interface ExecutionPlanStep {
  id: string;
  /** Owner-facing step description — never Provider jargon. */
  goal: string;
  capabilityId: string | null;
  status: PlanStepStatus;
}

export interface ExecutionPlan {
  goal: string;
  /** Capability Registry ids involved. */
  capabilities: string[];
  steps: ExecutionPlanStep[];
  action: IntentAction;
  /** True when the plan is multi-step desktop composition. */
  multiStep: boolean;
}

function caps(...ids: string[]): string[] {
  return ids.filter((id) => CAPABILITY_GRAPH.some((n) => n.id === id));
}

function step(
  id: string,
  goal: string,
  capabilityId: string | null,
  status: PlanStepStatus = "planned",
): ExecutionPlanStep {
  return { id, goal, capabilityId, status };
}

/**
 * Build a declarative execution plan from a resolved IntentAction.
 * Does not invent capabilities — only maps to Registry ids that exist.
 */
export function buildExecutionPlan(
  utterance: string,
  action: IntentAction,
): ExecutionPlan {
  const goal = utterance.trim() || action.reply;

  switch (action.kind) {
    case "browserOpenBeside": {
      const beside = "beside" in action ? action.beside : "target window";
      const site = "url" in action ? action.url : "site";
      return {
        goal,
        capabilities: caps("browser", "focus-window", "window-state"),
        multiStep: true,
        action,
        steps: [
          step("resolve-primary", `Resolve what to open (${site})`, "browser"),
          step("locate-beside", `Locate “${beside}” on the desktop`, "focus-window"),
          step("choose-layout", "Choose side-by-side layout", "window-state"),
          step(
            "execute",
            `Open beside “${beside}”`,
            "browser",
            "delegated_to_kernel",
          ),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    }
    case "browserOpenFocus": {
      const label = "focusQuery" in action ? action.focusQuery : "site";
      return {
        goal,
        capabilities: caps("browser", "focus-window"),
        multiStep: true,
        action,
        steps: [
          step("resolve-site", `Resolve site (${label})`, "browser"),
          step("open", "Open in browser", "browser", "delegated_to_kernel"),
          step("focus", `Bring “${label}” forward`, "focus-window", "delegated_to_kernel"),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    }
    case "winFocus":
    case "winFocusMinimize":
    case "appFocus": {
      const query = "query" in action ? action.query : "window";
      return {
        goal,
        capabilities: caps("focus-window"),
        multiStep: false,
        action,
        steps: [
          step("locate", `Locate “${query}”`, "focus-window"),
          step("focus", `Bring “${query}” forward`, "focus-window", "delegated_to_kernel"),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    }
    case "winMoveMonitor": {
      const query = "query" in action ? action.query : "window";
      const idx = "monitorIndex" in action ? action.monitorIndex : "?";
      return {
        goal,
        capabilities: caps("window-state", "focus-window"),
        multiStep: true,
        action,
        steps: [
          step("locate", `Locate “${query}”`, "focus-window"),
          step(
            "move",
            `Move to monitor ${idx}`,
            "window-state",
            "delegated_to_kernel",
          ),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    }
    case "winSnap": {
      const query = "query" in action ? action.query : "window";
      return {
        goal,
        capabilities: caps("window-state", "focus-window"),
        multiStep: true,
        action,
        steps: [
          step("locate", `Locate “${query}”`, "focus-window"),
          step("snap", "Apply snap layout", "window-state", "delegated_to_kernel"),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    }
    case "browserOpen":
      return {
        goal,
        capabilities: caps("browser"),
        multiStep: false,
        action,
        steps: [
          step("resolve-site", "Resolve website", "browser"),
          step("open", "Open in browser", "browser", "delegated_to_kernel"),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    case "appOpen":
    case "appLaunch":
    case "appOpenMaximize":
      return {
        goal,
        capabilities: caps("open-app", "folders"),
        multiStep: action.kind === "appOpenMaximize",
        action,
        steps: [
          step("resolve-app", "Resolve application or folder", "open-app"),
          step(
            "open",
            action.kind === "appOpenMaximize"
              ? "Open and maximize"
              : "Open or launch",
            "open-app",
            "delegated_to_kernel",
          ),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    case "winClickControl":
      return {
        goal,
        capabilities: caps("mouse-click", "window-control-discovery", "bounded-retry"),
        multiStep: true,
        action,
        steps: [
          step(
            "observe",
            "Locate the named control",
            "window-control-discovery",
            "delegated_to_kernel",
          ),
          step("act", "Click the control", "mouse-click", "delegated_to_kernel"),
          step(
            "verify",
            "Confirm the control; bounded retry+wait only if retryable",
            "bounded-retry",
          ),
        ],
      };
    case "winTypeControl":
      return {
        goal,
        capabilities: caps("keyboard-input", "window-control-discovery", "bounded-retry"),
        multiStep: true,
        action,
        steps: [
          step(
            "observe",
            "Locate the editable control",
            "window-control-discovery",
            "delegated_to_kernel",
          ),
          step("act", "Type deterministic text", "keyboard-input", "delegated_to_kernel"),
          step(
            "verify",
            "Confirm the field; bounded retry+wait only if retryable",
            "bounded-retry",
          ),
        ],
      };
    case "winWaitCondition":
      return {
        goal,
        capabilities: caps("wait-conditions", "window-control-discovery"),
        multiStep: true,
        action,
        steps: [
          step(
            "verify",
            "Wait for the expected desktop condition (bounded timeout)",
            "wait-conditions",
            "delegated_to_kernel",
          ),
          step(
            "observe",
            "Re-check the observable state when applicable",
            "window-control-discovery",
          ),
          step("complete", "Report completed or timeout truthfully", null),
        ],
      };
    case "winFindControl":
      return {
        goal,
        capabilities: caps("window-control-discovery", "desktop-ui-tree"),
        multiStep: false,
        action,
        steps: [
          step(
            "observe",
            "Locate the named control in the resolved window",
            "window-control-discovery",
            "delegated_to_kernel",
          ),
          step("complete", "Report whether the control was found — never invent clicks", null),
        ],
      };
    case "winEnumerateControls":
      return {
        goal,
        capabilities: caps("desktop-ui-tree", "focus-window"),
        multiStep: false,
        action,
        steps: [
          step(
            "observe",
            "Read named controls in the resolved window",
            "desktop-ui-tree",
            "delegated_to_kernel",
          ),
          step("complete", "List controls truthfully — never invent clicks", null),
        ],
      };
    case "winEnumerate":
    case "winActive":
    case "winMonitors":
    case "appEnumerate":
      return {
        goal,
        capabilities: caps("focus-window", "window-state"),
        multiStep: false,
        action,
        steps: [
          step("observe", "Read current desktop state", "focus-window", "delegated_to_kernel"),
          step("complete", "Describe what is visible — never invent windows", null),
        ],
      };
    case "capabilityExplain":
      return {
        goal,
        capabilities: CAPABILITY_GRAPH.map((n) => n.id),
        multiStep: false,
        action,
        steps: [
          step("discover", "Describe capabilities from the live Registry", null),
          step("complete", "Offer ordinary-language next steps", null),
        ],
      };
    case "unknown":
      return {
        goal,
        capabilities: [],
        multiStep: false,
        action,
        steps: [
          step(
            "recover",
            "Refuse inventing actions; guide from Registry",
            null,
            "recover",
          ),
        ],
      };
    case "navigate":
    case "navigateNamed":
      return {
        goal,
        capabilities: [],
        multiStep: false,
        action,
        steps: [
          step("surface", "Open the matching Conversation surface", null, "planned"),
          step("complete", "Wait for Owner approval before restore side-effects", null),
        ],
      };
    case "compoundOpen": {
      const n = "targets" in action ? action.targets.length : 2;
      return {
        goal,
        capabilities: caps("open-app", "browser"),
        multiStep: true,
        action,
        steps: [
          step("resolve", `Resolve ${n} known open targets`, "open-app"),
          step(
            "execute",
            "Open each target in order (Kernel compound)",
            "open-app",
            "delegated_to_kernel",
          ),
          step("complete", "Report aggregate Completion Contract outcome", null),
        ],
      };
    }
    default: {
      const domainGuess =
        action.kind.startsWith("screenshot")
          ? "screenshots"
          : action.kind.startsWith("clipboard")
            ? "clipboard"
            : action.kind.startsWith("notify")
              ? "notifications"
              : action.kind.startsWith("win")
                ? "window-state"
                : null;
      return {
        goal,
        capabilities: domainGuess ? caps(domainGuess) : [],
        multiStep: false,
        action,
        steps: [
          step(
            "execute",
            action.reply || "Execute desktop action",
            domainGuess,
            "delegated_to_kernel",
          ),
          step("complete", "Present truthful desktop result", null),
        ],
      };
    }
  }
}

/** Owner-facing summary of plan steps (for evidence / audits — not Conversation chrome). */
export function summarizeExecutionPlan(plan: ExecutionPlan): string {
  return plan.steps
    .map((s, i) => `${i + 1}. ${s.goal} [${s.status}]`)
    .join(" → ");
}
