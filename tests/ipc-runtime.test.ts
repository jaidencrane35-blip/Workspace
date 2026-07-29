import { describe, expect, it } from "vitest";
import {
  isIpcRuntimeAvailable,
  IpcRuntimeUnavailableError,
  invokeIpc,
} from "../app/src/lib/ipc";

describe("IPC runtime availability", () => {
  it("reports unavailable outside Tauri", () => {
    expect(isIpcRuntimeAvailable()).toBe(false);
  });

  it("rejects invoke when runtime is missing without crashing callers", async () => {
    await expect(invokeIpc("list_desktop_arrangements")).rejects.toBeInstanceOf(
      IpcRuntimeUnavailableError,
    );
  });
});
