import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P19.S1 Tray Show ↔ ShellMode continuity", () => {
  const tray = readFileSync(
    path.join(root, "app/src-tauri/src/tray.rs"),
    "utf8",
  );
  const shellWin = readFileSync(
    path.join(root, "app/src/lib/shellWindows.ts"),
    "utf8",
  );
  const main = readFileSync(path.join(root, "app/src/main.tsx"), "utf8");
  const desktop = readFileSync(
    path.join(root, "app/src/components/operator/DesktopOperator.tsx"),
    "utf8",
  );
  const operator = readFileSync(
    path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
    "utf8",
  );

  it("tray Show emits ShellMode restore before native show", () => {
    expect(tray).toContain("workspace-show-conversation");
    expect(tray).toContain("Emitter");
    const emitAt = tray.indexOf("emit(SHOW_CONVERSATION_EVENT");
    const showAt = tray.indexOf("main.show()");
    expect(emitAt).toBeGreaterThan(-1);
    expect(showAt).toBeGreaterThan(-1);
    expect(emitAt).toBeLessThan(showAt);
  });

  it("frontend restores Conversation Form via existing ShellMode path", () => {
    expect(shellWin).toContain("restoreConversationShell");
    expect(shellWin).toContain("saveShellMode(1)");
    expect(shellWin).toContain("applyShellMode(1)");
    expect(shellWin).toContain("installTrayShowConversationRestore");
    expect(main).toContain("installTrayShowConversationRestore");
  });

  it("Desktop Operator reuses the same restore path", () => {
    expect(desktop).toContain("restoreConversationShell");
  });

  it("preserves Collapse ≠ Exit and Conversation gravity markers", () => {
    expect(operator).toContain('setMode(0)');
    expect(operator).toContain('data-gravity="conversation"');
    expect(tray).toContain("Exit Workspace");
    expect(tray).toContain("Show Conversation");
  });
});
