/**
 * Purpose: Shared product-shell presentation helpers (runtime banners, initials).
 * Owner: Frontend product shell (Milestone A.1)
 * Inputs: Error-like values, application/workspace display names
 * Outputs: Banner classification + monogram initials
 * Dependencies: ipc runtime error types
 * Non-responsibilities: IPC, permissions, Assistant reasoning
 */

import { IpcRuntimeUnavailableError } from "./ipc";

/** Max zones loaded with workspace canvas context (IPC page bound). */
export const ZONE_CONTEXT_LIMIT = 200;

export type BannerKind = "error" | "runtime" | "ok";

/** User-facing copy when Tauri IPC is unavailable (browser preview). */
export const DESKTOP_PREVIEW_BANNER =
  "Desktop preview mode — open the Workspace app to create, switch, launch, and restore.";

export function classifyBanner(error: unknown): {
  kind: BannerKind;
  text: string;
} {
  if (typeof error === "string") {
    if (/desktop runtime is unavailable/i.test(error)) {
      return { kind: "runtime", text: DESKTOP_PREVIEW_BANNER };
    }
    return { kind: "error", text: error };
  }
  if (error instanceof IpcRuntimeUnavailableError) {
    return {
      kind: "runtime",
      text: DESKTOP_PREVIEW_BANNER,
    };
  }
  if (error instanceof Error) {
    if (
      error.name === "IpcRuntimeUnavailableError" ||
      /desktop runtime is unavailable/i.test(error.message)
    ) {
      return {
        kind: "runtime",
        text: DESKTOP_PREVIEW_BANNER,
      };
    }
    return { kind: "error", text: error.message };
  }
  return { kind: "error", text: String(error) };
}

/** Two-letter monogram for app/workspace cards (placeholder icon). */
export function monogramFromName(name: string): string {
  const parts = name
    .trim()
    .split(/\s+/)
    .filter(Boolean);
  if (parts.length === 0) {
    return "·";
  }
  if (parts.length === 1) {
    return parts[0].slice(0, 2).toUpperCase();
  }
  return `${parts[0][0] ?? ""}${parts[1][0] ?? ""}`.toUpperCase();
}
