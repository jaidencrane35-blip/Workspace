/**
 * Shell window transitions — Mode 0 lives in a separate always-on-top window.
 * Zero-Trap: close of main collapses to operator; Exit ends the process.
 */

import {
  COMPACT_SIZE,
  EXPANDED_SIZE,
  MAIN_POS_KEY,
  OPERATOR_POS_KEY,
  OPERATOR_SIZE,
  emitShellModeEvent,
  isTauriRuntime,
  loadPoint,
  savePoint,
  saveShellMode,
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
    typeof window !== "undefined" ? Math.max(8, window.screen.availWidth - width) : x;
  const maxY =
    typeof window !== "undefined"
      ? Math.max(8, window.screen.availHeight - height)
      : y;
  return {
    x: Math.min(Math.max(8, x), maxX),
    y: Math.min(Math.max(8, y), maxY),
  };
}

/**
 * Apply shell mode to native windows.
 * Mode 0 → hide main, show operator (taskbar-visible for recovery).
 * Modes 1–3 → hide operator, show main at compact/expanded size.
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
    try {
      const pos = await main.outerPosition();
      savePoint(MAIN_POS_KEY, { x: pos.x, y: pos.y });
    } catch {
      /* keep stored */
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
      } catch {
        /* optional APIs */
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
    await operator.hide();
  }

  const size = mode === 1 ? COMPACT_SIZE : EXPANDED_SIZE;
  await main.setSize(new LogicalSize(size.width, size.height));
  const mainPos = loadPoint(MAIN_POS_KEY, { x: -1, y: -1 });
  if (mainPos.x >= 0 && mainPos.y >= 0) {
    const clamped = clampPos(mainPos.x, mainPos.y, size.width, size.height);
    await main.setPosition(new LogicalPosition(clamped.x, clamped.y));
  }
  try {
    await main.setAlwaysOnTop(false);
    await main.setSkipTaskbar(false);
  } catch {
    /* optional APIs */
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

/** Hide both surfaces; keep process + operator taskbar entry for recovery. */
export async function hideShellToTaskbar(): Promise<void> {
  if (!isTauriRuntime()) {
    setShellHidden(true);
    return;
  }
  setShellHidden(true);
  saveShellMode(0);
  emitShellModeEvent();
  const main = await windowByLabel("main");
  const operator = await windowByLabel("operator");
  if (main) {
    try {
      const pos = await main.outerPosition();
      savePoint(MAIN_POS_KEY, { x: pos.x, y: pos.y });
    } catch {
      /* ignore */
    }
    await main.hide();
  }
  if (operator) {
    try {
      const pos = await operator.outerPosition();
      savePoint(OPERATOR_POS_KEY, { x: pos.x, y: pos.y });
    } catch {
      /* ignore */
    }
    try {
      await operator.setSkipTaskbar(false);
    } catch {
      /* ignore */
    }
    await operator.minimize();
  }
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
 * Close of the conversation window must collapse to the operator — never trap.
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

/** Bootstrap windows from durable shell mode on launch. */
export async function bootstrapShellOnLaunch(): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  const { loadShellMode, isShellHidden } = await import("./shellRuntime");
  const mode = loadShellMode(1);
  if (isShellHidden()) {
    await hideShellToTaskbar();
    return;
  }
  await applyShellMode(mode);
}
