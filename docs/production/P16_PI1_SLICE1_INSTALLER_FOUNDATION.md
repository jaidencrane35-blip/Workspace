# P16.PI1 Slice 1 — Installer Foundation

| Field | Value |
| --- | --- |
| **Status** | **Complete** — config/hooks/docs/verifier + `Workspace_0.1.0_x64-setup.exe` built locally |
| **Max layer** | Production |
| **Next slice** | 2 — Update Infrastructure (do not start until this slice accepted) |
| **P17** | Not begun |

---

## Problem

PR1: no production-quality installer path (default Tauri bundle only; no NSIS policy, hooks, docs, or verification).

## Repository evidence (pre-slice)

- `tauri.conf.json` had `bundle.targets: "all"` without Windows NSIS customization
- No `windows/hooks.nsh`
- Health debt: `installer-signing`

## Implementation

| Deliverable | Path |
| --- | --- |
| NSIS-focused bundle + WebView2 prerequisites | `app/src-tauri/tauri.conf.json` |
| Install / upgrade / uninstall hooks | `app/src-tauri/windows/hooks.nsh` |
| Operator/docs | `docs/production/INSTALLER.md` |
| Verifier | `scripts/verify-installer-foundation.mjs` |
| Build script | `pnpm installer:build` |

### Behaviours

- **Install:** current-user NSIS; embedded WebView2 bootstrapper; minimum WebView2 version gate  
- **Upgrade:** re-run setup; running process closed; downgrades blocked (`allowDowngrades: false`)  
- **Repair:** reinstall same/newer setup over existing install  
- **Uninstall:** close app; remove install dir; optional interactive user-data purge  
- **Version detection:** `$INSTDIR/install-manifest.json`  
- **Verification:** `pnpm verify:installer-foundation`

## Verification

```
pnpm verify:installer-foundation
pnpm verify:production-readiness
pnpm installer:build   # Windows — produces setup.exe
```

## Risk

| Risk | Mitigation |
| --- | --- |
| Unsigned setup → SmartScreen | Remaining work (signing / Slice 8) |
| `${VERSION}` define depends on Tauri NSIS template | Standard Tauri bundler define; validate on first build |
| User data purge accidental | Default preserve; prompt only when not silent |

## Remaining work

- Code signing Authenticode  
- Slice 2 updater  
- CI artifact publish  
- Optional MSI target  

## Quality gate

| Gate | Result |
| --- | --- |
| Verifier | Pass (`verify-installer-foundation`) |
| Package | Pass — `target/release/bundle/nsis/Workspace_0.1.0_x64-setup.exe` (not committed; rebuild via `pnpm installer:build`) |
| Signing | Remaining (not Slice 1) |

**STOP before Slice 2** until Product Owner accepts Slice 1.
