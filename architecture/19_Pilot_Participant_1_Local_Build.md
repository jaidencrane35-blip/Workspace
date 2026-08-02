# Pilot Participant #1 — Local Build Notes

Status: Operational note (LEDGER-0032); environment install 2026-08-02
Authority: Pilot execution support subordinate to LEDGER-0013 / LEDGER-0031
Date: 2026-08-02

Participant #1 is the developer dogfooding Workspace as the primary interruption
recovery tool. This is not Product Proof success evidence by itself.

---

## Installed environment (Participant #1 machine)

Installer used:

- `target/release/bundle/nsis/Workspace_0.1.0_x64-setup.exe`
  (silent install verified with `/S`, exit code 0)

Installation path:

- `C:\Users\Jaide\AppData\Local\Workspace\`
- Executable: `workspace-app.exe`
- Uninstaller: `uninstall.exe`
- Registry: HKCU uninstall entry DisplayName `Workspace` 0.1.0

Shortcuts (created by installer; both verified to launch):

- Desktop: `%USERPROFILE%\Desktop\Workspace.lnk`
- Start Menu: `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Workspace.lnk`
- Target: `C:\Users\Jaide\AppData\Local\Workspace\workspace-app.exe`
- Working directory: `C:\Users\Jaide\AppData\Local\Workspace`

Local database:

- `%APPDATA%\com.workspace.app\workspace.db`
- WebView data: `%LOCALAPPDATA%\com.workspace.app\EBWebView\`

Daily launch: double-click the Desktop or Start Menu shortcut. No Cursor
terminal, `pnpm`, or git checkout is required for schema apply (LEDGER-0032).

---

## Installable artefacts (rebuild)

After a release Tauri build (for example
`pnpm --filter @workspace/app exec tauri build`):

- `target/release/bundle/nsis/Workspace_0.1.0_x64-setup.exe` (recommended)
- `target/release/bundle/msi/Workspace_0.1.0_x64_en-US.msi`
- `target/release/workspace-app.exe` (portable run without installer)

---

## Validation results (2026-08-02)

| Check | Result |
|---|---|
| Install succeeds | Pass |
| Desktop shortcut exists | Pass (installer-created) |
| Start Menu shortcut launches | Pass |
| Double-click launch | Pass — window title `Workspace`, path under Local\Workspace |
| Production experience (not Vite/dev) | Pass — no `localhost:1420` listener; command line is the installed exe only |
| Database initializes | Pass — 43 embedded migrations applied through `044_pilot_measurement` |
| DB persists across app restart | Pass — same `workspace.db` retained |
| Pilot chrome | Pass — Save / Resume / Pilot / Help tabs; no Canvas/Work/Assistant/Diagnostic |
| In-session Save | Pass — named context + handoff saved to SQLite |
| Icon | Pass — icon extractable from installed exe |

### Active workspace persistence

**Fixed in LEDGER-0033** (`589a8e2`). `update_settings` accepts
`active_workspace_id`-only updates. Rebuild/reinstall after Experience
convergence (LEDGER-0034) so Participant #1 runs the fixed binary.

Reboot persistence of the SQLite file itself was not exercised by a full Windows
reboot; process-quit/relaunch retention was verified for the database file.

---

## Daily Product Proof loop (once active-workspace defect is fixed)

1. Open **Workspace** from Desktop or Start Menu.
2. Open **Save** — create a workspace if none exists.
3. Name the context, write the handoff, review scope, confirm.
4. Leave the work, keep windows open in the same Windows session.
5. Open **Resume** — inspect, preview, approve restore.
6. Optionally record leave→resume minutes under **Pilot** (after consent).
7. Use **Help** for restore limits and trust framing.

---

## Recovery

- App fails closed if the database cannot initialise (no silent empty schema).
- Migrations are embedded in the binary; a clean machine does not need the git
  checkout present for schema apply.
- To reset local pilot/product data for a clean trial: quit Workspace, delete
  `%APPDATA%\com.workspace.app\workspace.db`, relaunch (recreates schema).
  Optionally clear `%LOCALAPPDATA%\com.workspace.app\EBWebView` if WebView
  local state must be reset too.

---

## Not claimed

- Cross-session / relaunch window restore
- Network sync or telemetry
- Hypothesis proof from developer-only use
- Code signing / SmartScreen silence on unsigned local builds
