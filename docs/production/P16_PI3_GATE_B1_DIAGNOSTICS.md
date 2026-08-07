# P16.PI3 — Gate B1 Diagnostics & Support Bundle

| Field | Value |
| --- | --- |
| **Authority** | `PRODUCTION_DEPENDENCY_AUTHORITY.md` |
| **Unit** | B1-diagnostics-support-bundle |
| **Class at start** | Ready Now |
| **Status** | **Complete** (Engineering Complete for B1; not Release Ready) |
| **P17** | Not begun |
| **Commit** | `3c86adb` (`v2-dev`) |

---

## Before (four questions)

| Q | A |
| --- | --- |
| Highest Ready Now risk? | Operators cannot produce evidence when Voice/desktop fails — trust & recoverability collapse |
| Why now? | Canonical order #1; unblocked; signing/updater correctly deferred |
| User problem gone? | “I can’t get Workspace help without a developer on the machine” |
| Success metric | File logs rotate; Conversation “export support package” returns a local folder path; Moments DB never included; verifiers green |

---

## Implementation

| Piece | Path |
| --- | --- |
| Rotating file log | `app/src-tauri/src/file_log.rs` |
| Setup attach | `lib.rs` setup |
| `export_support_bundle` | `commands/support_bundle.rs` |
| Conversation | `intentBridge` + `intelligence.ts` |
| Diagnostic IPC tier | `ipc-tiers.json` |
| Verifiers | `verify-support-bundle`, `verify-production-dependency-authority` |

Privacy: manifest sets `databaseContentsIncluded: false`; no network upload.

---

## After

| Q | A |
| --- | --- |
| Readiness improved? | Yes — diagnostics/supportPackage/logging toward Production Ready |
| Verified? | `pnpm verify:support-bundle` + dependency authority verifier |
| Ship? | Yes in next app build (Engineering Complete); not Release Ready |
| Complexity vs value? | Local FS + one IPC + NL path — proportional |

---

## Stop

Next Ready Now per authority: **A1-artifact-checksums**.  
Do not start A1/D1/E1/updater until Owner reviews PI3.
