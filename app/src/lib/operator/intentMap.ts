import type { IntentAction } from "../intentBridge";
import type { CapabilityIntent } from "./types";

/**
 * Intent Layer ownership ends at CapabilityIntent.
 * No orchestration, provider choice, or execution order here.
 */
export function toCapabilityIntent(action: IntentAction): CapabilityIntent | null {
  switch (action.kind) {
    case "clipboardRead":
      return { domain: "clipboard", operation: "read" };
    case "clipboardWrite":
      return { domain: "clipboard", operation: "write", text: action.text };
    case "notifyStatus":
      return { domain: "notifications", operation: "status" };
    case "notifyShow":
      return {
        domain: "notifications",
        operation: "show",
        title: action.title ?? "Workspace",
        text: action.text,
        category: action.category,
        priority: action.priority,
        duration: action.duration,
      };
    case "notifyDismiss":
      return {
        domain: "notifications",
        operation: "dismiss",
        query: action.id,
      };
    case "browserStatus":
      return { domain: "browser", operation: "status" };
    case "browserOpen":
      return {
        domain: "browser",
        operation: "open",
        path: action.url,
        query: action.url,
      };
    case "browserOpenBeside":
      return {
        domain: "browser",
        operation: "open_beside",
        path: action.url,
        query: action.url,
        title: action.beside,
        // Prefer matching the browser window that typically appears after URL open.
        snap: "Chrome",
      };
    case "screenshotStatus":
      return { domain: "screenshots", operation: "status" };
    case "screenshotDesktop":
      return { domain: "screenshots", operation: "capture_desktop" };
    case "screenshotWindow":
      return {
        domain: "screenshots",
        operation: "capture_window",
        query: action.query,
      };
    case "screenshotMonitor":
      return {
        domain: "screenshots",
        operation: "capture_monitor",
        monitorIndex: action.monitorIndex,
      };
    case "screenshotSave":
      return { domain: "screenshots", operation: "save_png" };
    case "screenshotCopy":
      return {
        domain: "screenshots",
        operation: "copy_clipboard",
        path: action.path,
      };
    case "screenshotCaptureAndCopy":
      return {
        domain: "screenshots",
        operation: "capture_and_copy",
        query: action.query,
        monitorIndex: action.monitorIndex,
      };
    case "appEnumerate":
      return { domain: "application", operation: "enumerate" };
    case "appLaunch":
      return { domain: "application", operation: "launch", query: action.query };
    case "appFocus":
      return { domain: "application", operation: "focus", query: action.query };
    case "appClose":
      return { domain: "application", operation: "close", query: action.query };
    case "appMinimize":
      return {
        domain: "application",
        operation: "minimize",
        query: action.query,
      };
    case "appRestore":
      return {
        domain: "application",
        operation: "restore",
        query: action.query,
      };
    case "appOpen":
      return { domain: "application", operation: "open", query: action.query };
    case "winEnumerate":
      return { domain: "window", operation: "enumerate" };
    case "winActive":
      return { domain: "window", operation: "active" };
    case "winMonitors":
      return { domain: "window", operation: "monitors" };
    case "winBounds":
      return { domain: "window", operation: "bounds", query: action.query };
    case "winMaximize":
      return { domain: "window", operation: "maximize", query: action.query };
    case "winMinimize":
      return { domain: "window", operation: "minimize", query: action.query };
    case "winRestore":
      return { domain: "window", operation: "restore", query: action.query };
    case "winCenter":
      return { domain: "window", operation: "center", query: action.query };
    case "winFocus":
      return { domain: "window", operation: "focus", query: action.query };
    case "winSnap":
      return {
        domain: "window",
        operation: "snap",
        query: action.query,
        snap: action.snap,
      };
    case "winMoveMonitor":
      return {
        domain: "window",
        operation: "move",
        query: action.query,
        monitorIndex: action.monitorIndex,
      };
    case "winResize":
      return {
        domain: "window",
        operation: "resize",
        query: action.query,
        width: action.width,
        height: action.height,
      };
    default:
      return null;
  }
}

export function isCapabilityIntentAction(action: IntentAction): boolean {
  return toCapabilityIntent(action) !== null;
}
