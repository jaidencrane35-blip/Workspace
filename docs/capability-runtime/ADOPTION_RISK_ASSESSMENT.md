# Adoption Risk Assessment — Capability Runtime

| Risk | Severity | Mitigation |
| --- | --- | --- |
| External platform becomes product identity (Kiro, AHK, n8n, PowerToys) | **Critical** | REJECT platform ADOPT; WRAP crates only behind ports |
| Ambient observation / always-on capture | **Critical** | Law XII; default off; consented scopes only |
| License contamination (GPL into core) | **High** | Prefer MIT/Apache; isolate GPL tools out-of-process if ever used |
| Unmaintained capture/input crates | **High** | Pin versions; thin ports; replaceability tests |
| Privilege / UIPI blocking window control | **High** | Honest failures; never silent elevation |
| Clipboard secret exfiltration via logs | **High** | Audit lengths/hashes; not full payloads |
| Terminal arbitrary execution | **Critical** | Allowlist + ApprovalRequired; no free agent shell |
| Voice/OCR model supply chain | **Medium** | STUDY; vendor models as optional WRAP; local-first |
| CDP browser control abuse | **High** | STUDY; Critical permission; delay to late phase |
| Duplicate OS authority paths | **Critical** | All effects via `workspace-windows-integration` |
| UI regression from capability satellites | **Medium** | UI Spec frozen; satellites only; verifier:ui-architecture |
| Premature multi-domain build | **High** | This research gate; V1 = thin vertical slice only |

## Legal posture

- Prefer MIT / Apache-2.0 crates.  
- Document each WRAP license in ADR when code lands.  
- Do not vendor GPL into the Tauri binary without Owner legal review.

## Security posture

- Gateway + audit non-negotiable for consequential domains.  
- ADAPT Kiro principle: authorization checks outside the model.  
- No dependency may bypass CommandPipeline.
