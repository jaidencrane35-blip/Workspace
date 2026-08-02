# Pilot Participant #1 — Local Build Notes

Status: Operational note (LEDGER-0032)
Authority: Pilot execution support subordinate to LEDGER-0013 / LEDGER-0031
Date: 2026-08-02

Participant #1 is the developer dogfooding Workspace as the primary interruption
recovery tool. This is not Product Proof success evidence by itself.

---

## Installable artefacts

After a release Tauri build (for example
`pnpm --filter @workspace/app exec tauri build`):

- `target/release/bundle/nsis/Workspace_0.1.0_x64-setup.exe` (recommended)
- `target/release/bundle/msi/Workspace_0.1.0_x64_en-US.msi`
- `target/release/workspace-app.exe` (portable run without installer)

Local database (persists across restarts):

- `%APPDATA%\com.workspace.app\workspace.db`

---

## Daily Product Proof loop

1. Open **Save** — create a workspace if none exists.
2. Name the context, write the handoff, review scope, confirm.
3. Leave the work, keep windows open in the same Windows session.
4. Open **Resume** — inspect, preview, approve restore.
5. Optionally record leave→resume minutes under **Pilot** (after consent).
6. Use **Help** for restore limits and trust framing.

---

## Recovery

- App fails closed if the database cannot initialise (no silent empty schema).
- Migrations are embedded in the binary; a clean machine does not need the git
  checkout present for schema apply.
- To reset local pilot/product data for a clean trial: quit Workspace, delete
  `%APPDATA%\com.workspace.app\workspace.db`, relaunch (recreates schema).

---

## Not claimed

- Cross-session / relaunch restore
- Network sync or telemetry
- Hypothesis proof from developer-only use
