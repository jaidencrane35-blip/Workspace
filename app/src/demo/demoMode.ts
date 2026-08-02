/**
 * Experience demo mode gate — DEV-only, when Tauri IPC is unavailable.
 * Removable: delete `app/src/demo/` and the invokeIpc branch.
 */

/** True when running under a Tauri webview that can invoke commands. */
export function isTauriRuntimeAvailable(): boolean {
  if (typeof window === "undefined") {
    return false;
  }
  const w = window as Window & {
    __TAURI_INTERNALS__?: unknown;
    __TAURI__?: unknown;
  };
  return Boolean(w.__TAURI_INTERNALS__ || w.__TAURI__);
}

/**
 * Development builds without Tauri automatically use the demo dataset.
 * Production builds never activate. Set VITE_DISABLE_EXPERIENCE_DEMO=1 to force off.
 */
export function shouldUseExperienceDemo(): boolean {
  if (import.meta.env.PROD) {
    return false;
  }
  if (import.meta.env.VITE_DISABLE_EXPERIENCE_DEMO === "1") {
    return false;
  }
  if (import.meta.env.VITE_FORCE_EXPERIENCE_DEMO === "1") {
    return true;
  }
  return import.meta.env.DEV && !isTauriRuntimeAvailable();
}

export function isExperienceDemoActive(): boolean {
  return shouldUseExperienceDemo();
}
