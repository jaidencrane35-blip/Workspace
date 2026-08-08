# F1 — Release Pipeline Hardening (CI Automation)

| Field | Value |
| --- | --- |
| **Gate** | F1-ci-automation |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Status** | Engineering Complete (unsigned pipeline) |
| **Prior** | R1 Release Engineering Readiness |
| **Authority** | `docs/production/RELEASE_PIPELINE.md` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `.github/workflows/ci-pr.yml` | Cover `main` + `v2-dev`; version + release-pipeline stages |
| `.github/workflows/ci-release.yml` | Unsigned installer build, checksums, manifest, artifact upload |
| `scripts/verify-version-consistency.mjs` | Fail on version drift |
| `scripts/verify-release-pipeline.mjs` | Bundled release validation stages |
| `scripts/verify-f1-ci-automation.mjs` | Gate verifier |
| `scripts/emit-release-manifest.mjs` | Unsigned release metadata |
| `package.json` | Scripts + `verify:release` / `pnpm test` wiring |
| `docs/production/RELEASE_PIPELINE.md` | Pipeline authority |
| Gate JSON / readiness / catalog / handoff | F1 Complete stamped |

**Unchanged:** Product runtime, UX, signing, updater.

---

## 2. Pipeline architecture

See `RELEASE_PIPELINE.md` — `ci-pr` validates continuously; `ci-release` produces unsigned NSIS + checksums + manifest.

---

## 3. Validation stages

Documented in `RELEASE_PIPELINE.md` (version → verify:release → installer → checksums → naming → manifest → CI).

---

## 4. CI flow

| Workflow | Trigger | Purpose |
| --- | --- | --- |
| `ci-pr` | PR/push `main`, `v2-dev` | Continuous verification |
| `ci-release` | push `v2-dev`, tags `v*`, manual | Unsigned release artifacts |

---

## 5. Remaining dependencies

- **A2** Authenticode certificate (external)  
- **F2** signed publish (after A2)  
- **B2** updater (after A2)  
- Owner OA on F1 checkboxes  

---

## 6. Stop

F1 complete. Do not begin A2 in this program.
