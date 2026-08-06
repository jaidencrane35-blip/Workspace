# Recommended OSS Dependencies (Experience)

Research ID: EXP-001 · Stage 2/4/8
Status: Recommendation only — **not approved for install**
Date: 2026-08-02

Exact package versions and transitive licence review are mandatory before
`pnpm add`. CSP and Tauri constraints: build-time CSS/JS only.

---

## A. Preferred stack (Experience presentation infrastructure)

| Layer | Recommendation | Licence | Rationale |
|---|---|---|---|
| Styling | Tailwind CSS 4.x | MIT | Tokenized utilities; CSP-safe when built; dominant with shadcn |
| Primitives + styled kit | shadcn/ui (copy) on Radix **or** Base UI | MIT + MIT | Own the chrome; excellent a11y; Vite-compatible |
| Variant helper | `class-variance-authority` | Apache-2.0 | Card/button variants |
| Class merge | `clsx` + `tailwind-merge` | MIT | Conditional classes |
| Icons | `lucide-react` | ISC | Clean stroke icons; commercial-OK |
| Relative time | `date-fns` | MIT | “14 minutes ago” without inventing activity |
| Dialogs/drawers | Radix Dialog / AlertDialog; optional Vaul | MIT | Delete confirm; Inspect drawer |
| Motion (Phase 4; light Phase 2 optional) | `motion` (Framer Motion line) | MIT | View transitions later |

### Why this stack over alternatives

| Alternative | Why not primary |
|---|---|
| MUI / Ant Design / Chakra full kits | Fight companion brand; “form framework” gravity |
| React Aria only | Excellent a11y but more styling work; keep as fallback if Radix trajectory concerns |
| Headless UI only | Too small a set for dashboard + dialogs + menus |
| CSS-only forever | Blocks accessible dialogs/focus traps; slows Phase 2–4 |

---

## B. Phase-mapped adoption

| Phase | Adopt | Avoid |
|---|---|---|
| 2 | Tailwind + tokens; shadcn Button/Input/Textarea/Tabs/Card/Dialog; Lucide; date-fns | cmdk, heavy motion, virtuoso |
| 3 | cmdk; scroll/virtuoso core if lists grow; Vaul drawer | Message List commercial SKU |
| 4 | Motion transitions; toast (Sonner) if banners insufficient | New product capabilities |

---

## C. Already present (keep)

| Package | Role |
|---|---|
| `react` / `react-dom` 18 | UI runtime |
| `@tauri-apps/api` 2 | IPC |
| `vite` 6 | Bundler |

---

## D. Supply-chain / trust notes

1. Prefer pinning exact versions at adoption time; refresh licence evidence.
2. shadcn copied files become Workspace code — review for unwanted analytics
   snippets (none expected) and strip demo cruft.
3. No runtime phone-home from UI kits.
4. Document attribution in repo NOTICE or Open Source Registry upon adoption.
5. Open Source Registry (`06`) should gain rows only when Approved=true after
   legal/security pass — this research does not set Approved.

---

## E. Rejected for Experience shell

- GPL UI kits
- Ant Design / Material as product chrome
- Icon fonts requiring external CDN
- Libraries that require ambient screen capture or analytics SDKs
