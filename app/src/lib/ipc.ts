import { invoke } from "@tauri-apps/api/core";
import type { IpcErrorCode } from "../types/ipc-errors";
import type { IpcResponse } from "../types/workspace";

export class IpcCommandError extends Error {
  readonly code: IpcErrorCode;

  constructor(code: IpcErrorCode, message: string) {
    super(message);
    this.name = "IpcCommandError";
    this.code = code;
  }
}

/** True when Tauri IPC invoke is available (false in standalone browser Vite). */
export function isIpcRuntimeAvailable(): boolean {
  if (typeof window === "undefined") {
    return false;
  }
  const internals = (
    window as Window & {
      __TAURI_INTERNALS__?: { invoke?: unknown };
    }
  ).__TAURI_INTERNALS__;
  return typeof internals?.invoke === "function";
}

export class IpcRuntimeUnavailableError extends Error {
  constructor() {
    super(
      "Workspace desktop runtime is unavailable. Open the native app to use data actions.",
    );
    this.name = "IpcRuntimeUnavailableError";
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
