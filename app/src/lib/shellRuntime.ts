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

/** Compact productivity default — must not dominate the desktop. */
export const CONVERSATION_SIZE = { width: 340, height: 480 };
export const CONVERSATION_MIN_SIZE = { width: 300, height: 400 };
/** Soft ceiling before clamp to work area (rejects prototype-scale persists). */
export const CONVERSATION_SOFT_MAX = { width: 520, height: 720 };
/** @deprecated Use CONVERSATION_SIZE */
export const COMPACT_SIZE = CONVERSATION_SIZE;
/** Companion hit-target — not a miniature application frame. */
export const OPERATOR_SIZE = { width: 44, height: 44 };

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

/**
 * Product Gravity (P12.6): unset mode opens Conversation.
 * Durable mode still restores Operator after Collapse.
 */
export function loadShellMode(fallback: ShellMode = 1): ShellMode {
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

function workArea(): { width: number; height: number } {
  if (typeof window === "undefined") {
    return { width: 1440, height: 900 };
  }
  return {
    width: window.screen.availWidth || 1440,
    height: window.screen.availHeight || 900,
  };
}

/**
 * Compact-first size: migrate oversized legacy defaults, clamp to work area.
 * Never larger than the usable desktop.
 */
export function normalizeConversationSize(size: {
  width: number;
  height: number;
}): { width: number; height: number } {
  const area = workArea();
  let { width, height } = size;

  // Migrate prior product defaults that felt like a prototype panel.
  const legacyDefault =
    (Math.abs(width - 420) <= 8 && Math.abs(height - 560) <= 8) ||
    (Math.abs(width - 360) <= 8 && Math.abs(height - 520) <= 8);
  if (legacyDefault) {
    width = CONVERSATION_SIZE.width;
    height = CONVERSATION_SIZE.height;
  }

  const maxW = Math.max(
    CONVERSATION_MIN_SIZE.width,
    Math.min(
      CONVERSATION_SOFT_MAX.width,
      area.width - 32,
      Math.floor(area.width * 0.42),
    ),
  );
  const maxH = Math.max(
    CONVERSATION_MIN_SIZE.height,
    Math.min(
      CONVERSATION_SOFT_MAX.height,
      area.height - 48,
      Math.floor(area.height * 0.72),
    ),
  );

  return {
    width: Math.min(Math.max(width, CONVERSATION_MIN_SIZE.width), maxW),
    height: Math.min(Math.max(height, CONVERSATION_MIN_SIZE.height), maxH),
  };
}

export function loadSize(
  key: string,
  fallback: { width: number; height: number },
): { width: number; height: number } {
  try {
    const raw = localStorage.getItem(key);
    if (!raw) {
      return normalizeConversationSize(fallback);
    }
    const parsed = JSON.parse(raw) as { width?: number; height?: number };
    if (typeof parsed.width !== "number" || typeof parsed.height !== "number") {
      return normalizeConversationSize(fallback);
    }
    return normalizeConversationSize({
      width: parsed.width,
      height: parsed.height,
    });
  } catch {
    return normalizeConversationSize(fallback);
  }
}

export function saveSize(
  key: string,
  size: { width: number; height: number },
): void {
  try {
    const normalized = normalizeConversationSize(size);
    localStorage.setItem(key, JSON.stringify(normalized));
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
