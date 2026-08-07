# Browser Provider — Product Proof (P14)

| Field | Value |
| --- | --- |
| **Program** | P14 Browser Provider |
| **Harness** | `browser-provider.proof.json` |
| **Composition audit** | `BROWSER_PROVIDER_COMPOSITION_AUDIT.md` |
| **Handoff** | `AWAITING_PROJECT_OWNER_BROWSER_PRODUCT_PROOF_REVIEW` |

---

## Owner checklist

- “Open ChatGPT.” / “Open Google.” / “Open GitHub.” / “Open YouTube.”
- “Open https://example.com”
- “Open ChatGPT beside Cursor.”
- “Which browsers are available?”
- “Open this website.” → asks which site
- “Bring Chrome to the front.” remains Window/Application (existing)

Replies never mention Browser Provider / Runtime / WRAP crates.

---

## Launch verification (2026-08-07)

| Check | Result |
| --- | --- |
| `pnpm dev` → `workspace-app.exe` | Kernel ready; `GetWorkspace` live |
| Conversation Intent harness | Pass (`browser-provider.proof.json`) |
| Runtime router (Memory port) | `browser_status_and_open_through_router` pass |
| Intentional termination | Complete — no leftover `workspace-app` / Vite on 1420 |
