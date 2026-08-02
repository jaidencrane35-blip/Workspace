import { invoke } from "@tauri-apps/api/core";
import { shouldUseExperienceDemo } from "../demo/demoMode";
import type { IpcResponse } from "../types/workspace";

export class IpcCommandError extends Error {
  readonly code: string;

  constructor(code: string, message: string) {
    super(message);
    this.code = code;
  }
}

/**
 * Single IPC entry for the product UI.
 *
 * Production (Tauri available, not forced-demo): native `invoke` → kernel.
 * Demo adapter (DEV without Tauri, or VITE_FORCE_EXPERIENCE_DEMO): in-memory
 * fixture handler with the same command names and response shapes.
 *
 * Frozen experience panels must only call this — never branch on demo/prod
 * for layout or interaction.
 */
export async function invokeIpc<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (shouldUseExperienceDemo()) {
    const { demoInvoke } = await import("../demo/demoIpc");
    return demoInvoke<T>(command, args);
  }

  const response = await invoke<IpcResponse<T>>(command, args);

  if (response.success && response.data !== undefined) {
    return response.data;
  }

  if (response.success && response.data === null) {
    return null as T;
  }

  const code = response.error?.code ?? "unknown_error";
  const message = response.error?.message ?? "An unknown error occurred.";
  throw new IpcCommandError(code, message);
}
