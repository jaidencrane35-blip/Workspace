/**
 * Runtime shell modes for the Conversational Desktop Operator.
 * Modes are window/runtime states — not page layouts.
 */

export type ShellMode = 0 | 1 | 2 | 3;

export const SHELL_MODE_LABEL: Record<ShellMode, string> = {
  0: "Desktop Operator",
  1: "Compact Conversation",
  2: "Expanded Workspace",
  3: "Specialized Work Surface",
};

export const COMPACT_SIZE = { width: 360, height: 520 };
export const EXPANDED_SIZE = { width: 1100, height: 720 };
export const OPERATOR_SIZE = { width: 56, height: 56 };

export const MODE_STORAGE_KEY = "workspace.shell.mode";
export const OPERATOR_POS_KEY = "workspace.shell.operatorPos";
export const MAIN_POS_KEY = "workspace.shell.mainPos";
export const HIDDEN_KEY = "workspace.shell.hidden";

export function isShellMode(value: unknown): value is ShellMode {
  return value === 0 || value === 1 || value === 2 || value === 3;
}

export function loadShellMode(fallback: ShellMode = 1): ShellMode {
  try {
    const raw = localStorage.getItem(MODE_STORAGE_KEY);
    const n = raw == null ? NaN : Number(raw);
    return isShellMode(n) ? n : fallback;
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
