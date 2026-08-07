# Workspace Installer (NSIS) — Slice 1

| Field | Value |
| --- | --- |
| **Program** | P16.PI1 — Production Implementation |
| **Slice** | 1 — Installer Foundation |
| **Bundle** | NSIS (`-setup.exe`) via Tauri 2 |
| **Config** | `app/src-tauri/tauri.conf.json` |
| **Hooks** | `app/src-tauri/windows/hooks.nsh` |
| **Signing** | Not in this slice (remaining — certificate + Release Engineering) |

---

## What this slice delivers

| Capability | How |
| --- | --- |
| Installation | NSIS setup installs under current user (`installMode: currentUser`) |
| Upgrade path | Same product identity; `allowDowngrades: false` blocks silent downgrade |
| Repair | Re-run the same (or newer) setup over an existing install |
| Uninstall | Windows Apps & features / Start Menu uninstall; closes running app first |
| Uninstall cleanup | Removes install dir leftovers; **asks** before deleting user data |
| Version detection | `$INSTDIR\install-manifest.json` written post-install |
| Prerequisite validation | Embedded WebView2 bootstrapper + `minimumWebview2Version` |
| Installer verification | `pnpm verify:installer-foundation` |

---

## Build the installer package

Requires Windows + Rust stable + NSIS tooling via Tauri CLI.

```powershell
pnpm installer:build
```

Equivalent:

```powershell
pnpm --filter @workspace/app exec tauri build --bundles nsis
```

Artifact location (typical):

```
target/release/bundle/nsis/Workspace_*_x64-setup.exe
```

Silent install / uninstall (NSIS conventions):

```powershell
.\Workspace_0.1.0_x64-setup.exe /S
.\Workspace_0.1.0_x64-setup.exe /S /D=C:\Custom\Path   # if supported by bundler
```

Uninstall: use Windows Settings → Apps, or the uninstaller registered by NSIS.

---

## User data policy

| Path | Default on uninstall |
| --- | --- |
| Program files / `$INSTDIR` | Removed |
| `%APPDATA%\com.workspace.app` | **Preserved** unless user chooses Yes on cleanup prompt |
| `%LOCALAPPDATA%\com.workspace.app` | Same |

Silent uninstall does **not** auto-delete user data (no prompt). Manual deletion or interactive uninstall with “Yes” required.

---

## Artifact checksums (Gate A1 — P16.PF1)

After building the installer, publish a SHA-256 sidecar beside each setup.exe so downloads can be verified.

```powershell
pnpm installer:build
pnpm checksums:generate
```

Typical outputs:

```
target/release/bundle/nsis/Workspace_*_x64-setup.exe
target/release/bundle/nsis/Workspace_*_x64-setup.exe.sha256
```

Sidecar format (GNU-style):

```
<64-hex-sha256>  Workspace_0.1.0_x64-setup.exe
```

Verify tooling / existing sidecars:

```powershell
pnpm verify:artifact-checksums
```

A user (or release engineer) compares the published `.sha256` digest to a local hash of the downloaded setup.exe.

---

## Remaining work (not Slice 1 / A1)

- Authenticode code signing (SmartScreen) — Gate A2 (external cert)
- MSI parallel target (optional)
- Auto-updater — Gate B2 (blocked by A2)
- CI release job that publishes the setup.exe + checksums (Gate F1)

---

## Verification

```powershell
pnpm verify:installer-foundation
pnpm verify:artifact-checksums
pnpm verify:production-readiness
```
