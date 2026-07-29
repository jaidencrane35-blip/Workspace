import { invoke, isTauri } from "@tauri-apps/api/core";
import type { IpcErrorCode } from "../types/ipc-errors";
import type { IpcResponse } from "../types/workspace";

/** Shown when the UI runs outside the Tauri webview (e.g. Vite in a browser). */
export const IPC_RUNTIME_UNAVAILABLE_MESSAGE =
  "Workspace requires the Tauri desktop runtime. Browser-only Vite preview cannot call IPC.";

type TauriInternals = {
  invoke: (cmd: string, args?: unknown, options?: unknown) => Promise<unknown>;
};

function hasTauriInternals(): boolean {
  if (typeof window === "undefined") {
    return false;
  }
  const internals = (
    window as Window & { __TAURI_INTERNALS__?: TauriInternals }
  ).__TAURI_INTERNALS__;
  return internals != null && typeof internals.invoke === "function";
}

/**
 * True only when the Tauri IPC bridge is actually present.
 * `@tauri-apps/api` `invoke` assumes `window.__TAURI_INTERNALS__` exists;
 * calling it in a plain browser throws TypeError on undefined `.invoke`.
 */
export function isIpcRuntimeAvailable(): boolean {
  return isTauri() && hasTauriInternals();
}

export class IpcRuntimeUnavailableError extends Error {
  constructor(message = IPC_RUNTIME_UNAVAILABLE_MESSAGE) {
    super(message);
    this.name = "IpcRuntimeUnavailableError";
  }
}

export class IpcCommandError extends Error {
  readonly code: IpcErrorCode;

  constructor(code: IpcErrorCode, message: string) {
    super(message);
    this.name = "IpcCommandError";
    this.code = code;
  }
}

export async function invokeIpc<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!isIpcRuntimeAvailable()) {
    throw new IpcRuntimeUnavailableError();
  }

  const response = await invoke<IpcResponse<T>>(command, args);

  if (response.success === true) {
    return response.data;
  }

  if (response.success === false && response.error) {
    throw new IpcCommandError(response.error.code, response.error.message);
  }

  throw new IpcCommandError(
    "invalid_ipc_response",
    "Workspace returned an invalid IPC response.",
  );
}
