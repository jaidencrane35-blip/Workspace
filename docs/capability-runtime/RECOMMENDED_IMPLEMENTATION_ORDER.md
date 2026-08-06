# Recommended Implementation Order
## Post–Owner approval only

UI Architecture remains frozen. Each phase is a **separate** constitutional execution program.

---

## Phase 0 — Contracts & ports (prerequisite)

- Rust traits / ports per domain (`ClipboardPort`, `CapturePort`, …)  
- IPC tier registration for product commands  
- Permission keys + audit event names  
- **No user-visible capability expansion yet**

## Phase 1 — Safe I/O verticals (V1 candidate)

1. **Clipboard** (WRAP `arboard`) — read/write with permission  
2. **Notifications** (WRAP Tauri plugin) — sparse system feedback  
3. **Browser open** (WRAP `webbrowser`) — URL open only  

*Why first:* High user value, limited blast radius, clear conversation entry.

## Phase 2 — Capture & Moments deepen

4. **Screenshots** (WRAP capture crate)  
5. **Desktop Observation** harden (existing pipeline + Save Moment)  
6. **Memory / Search** deepen SQLite FTS  

## Phase 3 — Application & windows

7. **Application Control** (ADAPT Win32 launch/focus)  
8. **Window Management** (placement / restore quality)

## Phase 4 — Higher trust

9. **File Operations** (scoped)  
10. **OCR** (STUDY → WRAP)  
11. **Terminal** (allowlisted recipes only)

## Phase 5 — Critical / optional

12. **Voice Input** (utterance → IntentEnvelope)  
13. **Automation** (synthetic input) — last among early domains  
14. **Browser CDP** / **Workflow** composer — only with proven Gateway discipline  

---

## Explicit non-goals for V1

- Embedding Kiro Crew  
- Ambient desktop watching  
- Capability launcher UI  
- Redesigning Desktop Operator or Conversation chrome  

---

## Exit criterion for each phase

- Domain contract tests  
- Permission denial paths  
- Audit coverage  
- UI Spec verifier still green  
- Owner review before next phase
