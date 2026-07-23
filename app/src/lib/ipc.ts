import { invoke } from "@tauri-apps/api/core";
import type { IpcResponse } from "../types/workspace";

export class IpcCommandError extends Error {
  readonly code: string;

  constructor(code: string, message: string) {
    super(message);
    this.code = code;
  }
}

export async function invokeIpc<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  const response = await invoke<IpcResponse<T>>(command, args);

  if (response.success && response.data !== undefined) {
    return response.data;
  }

  const code = response.error?.code ?? "unknown_error";
  const message = response.error?.message ?? "An unknown error occurred.";
  throw new IpcCommandError(code, message);
}
