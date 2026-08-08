/**
 * Two-form window lifecycle (Product Gravity — P12.6).
 * Form B (1): Conversation — default launch presence; transparent host on the desktop.
 * Form A (0): Desktop Operator — collapse / idle companion.
 * Close/collapse of Conversation → Operator (never exit).
 * Exit Workspace ends the process — never overlapped with Collapse/Close.
 */

import {
  CONVERSATION_MIN_SIZE,
  CONVERSATION_SIZE,
  MAIN_POS_KEY,
  MAIN_SIZE_KEY,
  OPERATOR_POS_KEY,
  OPERATOR_SIZE,
  SHOW_CONVERSATION_EVENT,
  emitShellModeEvent,
  isTauriRuntime,
  loadPoint,
  loadSize,
  normalizeConversationSize,
  savePoint,
  saveShellMode,
  saveSize,
  setShellHidden,
  type ShellMode,
} from "./shellRuntime";
import { canTransition } from "./shellStateMachine";

async function loadWindowApi() {
  return import("@tauri-apps/api/window");
}

async function loadDpiApi() {
  return import("@tauri-apps/api/dpi");
}

export async function currentWindowLabel(): Promise<string> {
  if (!isTauriRuntime()) {
    return "main";
  }
  const { getCurrentWindow } = await loadWindowApi();
  return getCurrentWindow().label;
}

async function windowByLabel(label: string) {
  const { getAllWindows } = await loadWindowApi();
  const all = await getAllWindows();
  return all.find((w) => w.label === label) ?? null;
}

function clampPos(
  x: number,
  y: number,
  width: number,
  height: number,
): { x: number; y: number } {
  const maxX =
    typeof window !== "undefined"
      ? Math.max(8, window.screen.availWidth - width)
      : x;
  const maxY =
    typeof window !== "undefined"
      ? Math.max(8, window.screen.availHeight - height)
      : y;
  return {
    x: Math.min(Math.max(8, x), maxX),
    y: Math.min(Math.max(8, y), maxY),
  };
}

async function persistMainGeometry(
  main: Awaited<ReturnType<typeof windowByLabel>>,
): Promise<void> {
  if (!main) {
    return;
  }
  try {
    const pos = await main.outerPosition();
    savePoint(MAIN_POS_KEY, { x: pos.x, y: pos.y });
  } catch {
    /* keep */
  }
  try {
    const size = await main.outerSize();
    saveSize(
      MAIN_SIZE_KEY,
      normalizeConversationSize({ width: size.width, height: size.height }),
    );
  } catch {
    /* keep */
  }
}

/**
 * Apply shell form to native windows — mode switch, never a resize of conversation.
 * Form A (0): conversation window fully gone; operator is the only Workspace surface.
 * Form B (1): operator gone; conversation is the only Workspace surface.
 */
export async function applyShellMode(mode: ShellMode): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  const { LogicalSize, LogicalPosition } = await loadDpiApi();
  const main = await windowByLabel("main");
  const operator = await windowByLabel("operator");
  if (!main) {
    return;
  }

  setShellHidden(false);

  if (mode === 0) {
    await persistMainGeometry(main);
    // Conversation must disappear completely — not shrink into a mini window.
    try {
      await main.setSkipTaskbar(true);
    } catch {
      /* optional */
    }
    await main.hide();
    if (operator) {
      const raw = loadPoint(OPERATOR_POS_KEY, { x: 24, y: 24 });
      const pos = clampPos(
        raw.x,
        raw.y,
        OPERATOR_SIZE.width,
        OPERATOR_SIZE.height,
      );
      await operator.setSize(
        new LogicalSize(OPERATOR_SIZE.width, OPERATOR_SIZE.height),
      );
      await operator.setPosition(new LogicalPosition(pos.x, pos.y));
      try {
        await operator.setSkipTaskbar(false);
        await operator.setAlwaysOnTop(true);
        await operator.setDecorations(false);
      } catch {
        /* optional */
      }
      await operator.unminimize().catch(() => undefined);
      await operator.show();
      await operator.setFocus();
    }
    return;
  }

  if (operator) {
    try {
      const pos = await operator.outerPosition();
      savePoint(OPERATOR_POS_KEY, { x: pos.x, y: pos.y });
    } catch {
      /* keep */
    }
    try {
      await operator.setSkipTaskbar(true);
    } catch {
      /* optional */
    }
    await operator.hide();
  }

  // Compact productivity size — never from operator geometry; never beyond work area.
  const size = loadSize(MAIN_SIZE_KEY, CONVERSATION_SIZE);
  await main.setSize(new LogicalSize(size.width, size.height));
  try {
    await main.setResizable(true);
    // Product Gravity: undecorated transparent host — conversation on the desktop.
    await main.setDecorations(false);
    await main.setMinSize(
      new LogicalSize(CONVERSATION_MIN_SIZE.width, CONVERSATION_MIN_SIZE.height),
    );
  } catch {
    /* optional */
  }
  const mainPos = loadPoint(MAIN_POS_KEY, { x: -1, y: -1 });
  if (mainPos.x >= 0 && mainPos.y >= 0) {
    const clamped = clampPos(mainPos.x, mainPos.y, size.width, size.height);
    await main.setPosition(new LogicalPosition(clamped.x, clamped.y));
  } else {
    // First open: dock to lower-right of the work area — usable, not centered fullscreen.
    const availW =
      typeof window !== "undefined" ? window.screen.availWidth : 1440;
    const availH =
      typeof window !== "undefined" ? window.screen.availHeight : 900;
    const x = Math.max(16, availW - size.width - 28);
    const y = Math.max(16, availH - size.height - 48);
    await main.setPosition(new LogicalPosition(x, y));
  }
  try {
    await main.setAlwaysOnTop(false);
    await main.setSkipTaskbar(false);
  } catch {
    /* optional */
  }
  await main.unminimize().catch(() => undefined);
  await main.show();
  await main.setFocus();
}

export async function transitionShellMode(
  from: ShellMode,
  to: ShellMode,
): Promise<boolean> {
  if (!canTransition(from, to)) {
    return false;
  }
  saveShellMode(to);
  emitShellModeEvent();
  await applyShellMode(to);
  return true;
}

/** Full process exit — only explicit Exit Workspace. */
export async function exitWorkspace(): Promise<void> {
  if (!isTauriRuntime()) {
    window.close();
    return;
  }
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("exit_workspace");
  } catch {
    const main = await windowByLabel("main");
    const operator = await windowByLabel("operator");
    await operator?.close().catch(() => undefined);
    await main?.close().catch(() => undefined);
  }
}

export async function startOperatorDrag(): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  const { getCurrentWindow } = await loadWindowApi();
  await getCurrentWindow().startDragging();
}

/**
 * Close of the conversation window returns to Desktop Operator — never exits.
 */
export async function installMainCloseCollapse(): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  const { getCurrentWindow } = await loadWindowApi();
  const current = getCurrentWindow();
  if (current.label !== "main") {
    return;
  }
  await current.onCloseRequested(async (event) => {
    event.preventDefault();
    saveShellMode(0);
    emitShellModeEvent();
    await applyShellMode(0);
  });
}

/** Bootstrap: Product Gravity defaults to Conversation unless durable Operator. */
export async function bootstrapShellOnLaunch(): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  const { loadShellMode } = await import("./shellRuntime");
  setShellHidden(false);
  const mode = loadShellMode(1);
  await applyShellMode(mode);
}

/**
 * P19.S1 — restore Conversation Form B (ShellMode 1) after tray / secondary launch.
 * Same contract as Desktop Operator click: durable mode + React sync + native Form.
 * Does not remount App; navigation / Moments session authorities stay mounted.
 */
export async function restoreConversationShell(): Promise<void> {
  saveShellMode(1);
  emitShellModeEvent();
  await applyShellMode(1);
}

let trayShowRestoreInstalled = false;

/** Listen once per webview for tray Show Conversation / single-instance focus. */
export async function installTrayShowConversationRestore(): Promise<void> {
  if (!isTauriRuntime() || trayShowRestoreInstalled) {
    return;
  }
  trayShowRestoreInstalled = true;
  try {
    const { listen } = await import("@tauri-apps/api/event");
    await listen(SHOW_CONVERSATION_EVENT, () => {
      void restoreConversationShell();
    });
  } catch {
    trayShowRestoreInstalled = false;
  }
}
