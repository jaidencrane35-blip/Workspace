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

export async function invokeIpc<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
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
