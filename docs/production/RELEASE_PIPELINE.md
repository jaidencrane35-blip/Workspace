# Release Pipeline (F1)

| Field | Value |
| --- | --- |
| **Gate** | F1-ci-automation |
| **Channel** | Unsigned engineering artifacts |
| **Not included** | Code signing (A2) · Updater (B2) · UX changes |
| **Repository posture** | **Release Hold** — `docs/production/R2_RELEASE_HOLD.md` |

This document is the Stage 3 release-pipeline authority for **engineering confidence**.  
It does not claim Release Ready. Under R2 Release Hold, do not begin A2 until Owner Acceptance and Authenticode certificate (trigger T3).

---

## Pipeline architecture

```text
ci-pr.yml (main + v2-dev)
  version consistency
  typecheck · frontend build
  cargo check/build/test
  pnpm test (verifiers)
  verify:release-pipeline

ci-release.yml (v2-dev push · tags v* · workflow_dispatch)
  verify:release
  installer:build          → Workspace_*_x64-setup.exe
  checksums:generate       → *.sha256
  release:manifest         → Workspace_*_release-manifest.json
  upload-artifact          → workspace-unsigned-nsis
```

---

## Validation stages

| Stage | Command / job | Fail condition |
| --- | --- | --- |
| Version consistency | `pnpm verify:version-consistency` | Drift across package.json / tauri.conf / Cargo |
| Typecheck | `pnpm typecheck` | TS errors |
| Frontend build | `pnpm build` | Vite/tsc build fail |
| Tests + verifiers | `pnpm test` | Any verifier fail |
| Cargo check | `cargo check -p workspace-kernel/app` | Compile fail |
| Project health | `verify-project-health` | Health/handoff drift |
| Installer foundation | `verify-installer-foundation` | NSIS config/hooks missing |
| Checksum tooling | `verify-artifact-checksums` | Tooling/docs/gate drift |
| Support bundle wiring | `verify-support-bundle` | Export path broken |
| Release pipeline bundle | `pnpm verify:release-pipeline` | Any stage above |
| F1 gate wiring | `pnpm verify:f1-ci-automation` | CI/workflows incomplete |
| Full release validate | `pnpm verify:release` | Any of the above + cargo focused tests |
| Installer generation | `pnpm installer:build` (ci-release) | No setup.exe |
| Checksum generation | `pnpm checksums:generate` | No sidecar |
| Artifact naming | `Workspace_*_x64-setup.exe` assert | Name mismatch |
| Release metadata | `pnpm release:manifest` | Manifest missing |
| CI status | GitHub Actions | Red workflow |

---

## Local release engineer flow (unsigned)

```text
pnpm verify:release
pnpm installer:build
pnpm checksums:generate
pnpm release:manifest
```

Artifacts land under `target/release/bundle/nsis/` or `app/src-tauri/target/release/bundle/nsis/`.

---

## Remaining Stage 3 dependencies

| Next | Blocker |
| --- | --- |
| A2 code signing | Authenticode certificate (external) |
| F2 signed release | A2 |
| B2 updater | A2 |

F1 Complete ≠ Release Ready.
