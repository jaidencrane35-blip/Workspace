#!/usr/bin/env node
/**
 * Development Environment Recovery — terminate orphaned Workspace/Vite/Tauri
 * child processes left after Product Proof launches or crashed `tauri dev`.
 *
 * Conversation history is not touched. Only live OS processes are considered.
 *
 * Usage: node scripts/dev-env-cleanup.mjs
 */
import { execSync } from "node:child_process";

const isWin = process.platform === "win32";

function run(cmd) {
  try {
    return execSync(cmd, {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
      windowsHide: true,
    });
  } catch (err) {
    return err.stdout?.toString?.() ?? "";
  }
}

function cleanupWindows() {
  const before = run(
    `powershell -NoProfile -Command "(Get-Process -Name 'workspace-app' -ErrorAction SilentlyContinue | Measure-Object).Count"`,
  ).trim();
  run(
    `powershell -NoProfile -Command "Get-NetTCPConnection -LocalPort 1420 -State Listen -ErrorAction SilentlyContinue | ForEach-Object { Stop-Process -Id $_.OwningProcess -Force -ErrorAction SilentlyContinue }; Get-Process -Name 'workspace-app' -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue"`,
  );
  const after = run(
    `powershell -NoProfile -Command "(Get-Process -Name 'workspace-app' -ErrorAction SilentlyContinue | Measure-Object).Count"`,
  ).trim();
  console.log(
    `dev-env-cleanup: workspace-app before=${before || 0} after=${after || 0}; port 1420 listeners cleared`,
  );
}

function cleanupUnix() {
  run("pkill -f 'workspace-app' 2>/dev/null || true");
  run("fuser -k 1420/tcp 2>/dev/null || true");
  console.log("dev-env-cleanup: attempted unix cleanup of workspace-app / :1420");
}

if (isWin) {
  cleanupWindows();
} else {
  cleanupUnix();
}
