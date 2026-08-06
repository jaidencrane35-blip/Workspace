/**
 * Two-form shell runtime — Desktop Operator | Conversation Window.
 * Legacy modes 2–3 migrate to Conversation (1).
 */

/** 0 = Desktop Operator (Form A). 1 = Conversation Window (Form B). */
export type ShellMode = 0 | 1;

export const SHELL_MODE_LABEL: Record<ShellMode, string> = {
  0: "Desktop Operator",
  1: "Conversation Window",
};

export const CONVERSATION_SIZE = { width: 420, height: 560 };
/** @deprecated Use CONVERSATION_SIZE — kept for test/compat aliases. */
export const COMPACT_SIZE = CONVERSATION_SIZE;
export const OPERATOR_SIZE = { width: 52, height: 52 };

export const MODE_STORAGE_KEY = "workspace.shell.mode";
export const OPERATOR_POS_KEY = "workspace.shell.operatorPos";
export const MAIN_POS_KEY = "workspace.shell.mainPos";
export const MAIN_SIZE_KEY = "workspace.shell.mainSize";
export const HIDDEN_KEY = "workspace.shell.hidden";

export function isShellMode(value: unknown): value is ShellMode {
  return value === 0 || value === 1;
}

/** Normalize legacy 2/3 (expanded/specialized) → Conversation. */
export function normalizeShellMode(value: number): ShellMode {
  if (value === 0) {
    return 0;
  }
  return 1;
}

/** Default idle form is Desktop Operator. */
export function loadShellMode(fallback: ShellMode = 0): ShellMode {
  try {
    const raw = localStorage.getItem(MODE_STORAGE_KEY);
    const n = raw == null ? NaN : Number(raw);
    if (Number.isFinite(n)) {
      return normalizeShellMode(n);
    }
    return fallback;
  } catch {
    return fallback;
  }
}

export function saveShellMode(mode: ShellMode): void {
  try {
    localStorage.setItem(MODE_STORAGE_KEY, String(mode));
    localStorage.setItem(HIDDEN_KEY, "0");
  } catch {
    /* ignore */
  }
}

export function loadPoint(
  key: string,
  fallback: { x: number; y: number },
): { x: number; y: number } {
  try {
    const raw = localStorage.getItem(key);
    if (!raw) {
      return fallback;
    }
    const parsed = JSON.parse(raw) as { x?: number; y?: number };
    return {
      x: typeof parsed.x === "number" ? parsed.x : fallback.x,
      y: typeof parsed.y === "number" ? parsed.y : fallback.y,
    };
  } catch {
    return fallback;
  }
}

export function savePoint(key: string, pos: { x: number; y: number }): void {
  try {
    localStorage.setItem(key, JSON.stringify(pos));
  } catch {
    /* ignore */
  }
}

export function loadSize(
  key: string,
  fallback: { width: number; height: number },
): { width: number; height: number } {
  try {
    const raw = localStorage.getItem(key);
    if (!raw) {
      return fallback;
    }
    const parsed = JSON.parse(raw) as { width?: number; height?: number };
    return {
      width:
        typeof parsed.width === "number" && parsed.width >= 280
          ? parsed.width
          : fallback.width,
      height:
        typeof parsed.height === "number" && parsed.height >= 360
          ? parsed.height
          : fallback.height,
    };
  } catch {
    return fallback;
  }
}

export function saveSize(
  key: string,
  size: { width: number; height: number },
): void {
  try {
    localStorage.setItem(key, JSON.stringify(size));
  } catch {
    /* ignore */
  }
}

export function setShellHidden(hidden: boolean): void {
  try {
    localStorage.setItem(HIDDEN_KEY, hidden ? "1" : "0");
  } catch {
    /* ignore */
  }
}

export function isShellHidden(): boolean {
  try {
    return localStorage.getItem(HIDDEN_KEY) === "1";
  } catch {
    return false;
  }
}

/** True when running inside a Tauri webview. */
export function isTauriRuntime(): boolean {
  return (
    typeof window !== "undefined" &&
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    typeof (window as any).__TAURI_INTERNALS__ !== "undefined"
  );
}

export const SHELL_MODE_EVENT = "workspace-shell-mode";

export function emitShellModeEvent(): void {
  window.dispatchEvent(new Event(SHELL_MODE_EVENT));
}
