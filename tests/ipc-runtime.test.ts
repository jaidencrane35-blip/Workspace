/**
 * Guards against calling @tauri-apps invoke when the desktop bridge is absent.
 * Regression for: browser Vite preview TypeError on window.__TAURI_INTERNALS__.invoke
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  IPC_RUNTIME_UNAVAILABLE_MESSAGE,
  IpcRuntimeUnavailableError,
  invokeIpc,
  isIpcRuntimeAvailable,
} from "../app/src/lib/ipc";

const rootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const appSrcDir = path.join(rootDir, "app", "src");

function listTsFiles(dir: string): string[] {
  const out: string[] = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      out.push(...listTsFiles(full));
      continue;
    }
    if (entry.name.endsWith(".ts") || entry.name.endsWith(".tsx")) {
      out.push(full);
    }
  }
  return out;
}

describe("IPC runtime availability", () => {
  it("reports unavailable outside the Tauri webview (Vitest / Node)", () => {
    expect(isIpcRuntimeAvailable()).toBe(false);
  });

  it("refuses invokeIpc with a typed runtime error instead of TypeError", async () => {
    await expect(invokeIpc("get_workspace_status")).rejects.toBeInstanceOf(
      IpcRuntimeUnavailableError,
    );
    await expect(invokeIpc("get_workspace_status")).rejects.toThrow(
      IPC_RUNTIME_UNAVAILABLE_MESSAGE,
    );
  });

  it("keeps @tauri-apps invoke confined to the IPC wrapper", () => {
    const offenders: string[] = [];
    for (const file of listTsFiles(appSrcDir)) {
      const rel = path.relative(appSrcDir, file).replaceAll("\\", "/");
      if (rel === "lib/ipc.ts") {
        continue;
      }
      const source = fs.readFileSync(file, "utf8");
      if (
        source.includes('from "@tauri-apps/api') ||
        source.includes("from '@tauri-apps/api")
      ) {
        offenders.push(`${rel}: imports @tauri-apps/api directly`);
      }
      if (/\binvoke\s*\(/.test(source) && !source.includes("invokeIpc")) {
        // allow comments mentioning invoke; flag raw invoke( calls
        if (/^\s*[^/\n]*\binvoke\s*\(/m.test(source)) {
          offenders.push(`${rel}: calls invoke( outside ipc wrapper`);
        }
      }
    }
    expect(offenders).toEqual([]);
  });
});
