/**
 * Experience demo adapter gate.
 *
 * Demo is not the product runtime. It is a permanent DEV/screenshot adapter
 * that implements `experienceIpcCatalog` in-memory when Tauri IPC is absent.
 *
 * Production builds never activate the adapter.
 * - `VITE_FORCE_EXPERIENCE_DEMO=1` — force adapter (screenshots / visual tests)
 * - `VITE_DISABLE_EXPERIENCE_DEMO=1` — never use adapter even in DEV
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
 * Whether the in-memory experience adapter should answer IPC.
 * Prefer production whenever Tauri is available.
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

/** Alias for UI bootstrap markers (e.g. `data-experience-demo`). */
export function isExperienceDemoActive(): boolean {
  return shouldUseExperienceDemo();
}
