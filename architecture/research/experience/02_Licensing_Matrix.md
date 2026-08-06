# Experience Licensing Matrix

Research ID: EXP-001 · Stage 4
Status: Research complete; no dependency approved
Date: 2026-08-02
Evidence date: 2026-08-02 (public package/registry pages)

Policy preference (task + registry practice): MIT, Apache-2.0, BSD, ISC, MPL.
Avoid GPL unless Open Source Registry explicitly accepts. Exact transitive
review required before adoption.

---

## A. Candidate licences (summary)

| Licence | Commercial use | Attribution | Modification | Redistribution | Copyleft risk | Workspace fit |
|---|---|---|---|---|---|---|
| MIT | Yes | Notice required | Yes | Yes | None (permissive) | **Preferred** |
| Apache-2.0 | Yes | Notice + NOTICE | Yes | Yes | Patent grant; permissive | **Preferred** |
| BSD-2/3 | Yes | Notice required | Yes | Yes | None | **Preferred** |
| ISC | Yes | Notice required | Yes | Yes | None (MIT-like) | **Preferred** |
| MPL-2.0 | Yes | File-level | Yes | Yes | File-level copyleft | Acceptable for isolated files; prefer avoid for UI kit core |
| LGPL-3.0 | Yes (dynamic) | Yes | Yes | Yes | Library copyleft | Generally avoid for JS bundled into app |
| GPL-3.0 | Contested when static-linked/bundled | Yes | Yes | Yes | Strong copyleft | **Reject** for Experience UI stack unless registry override |

---

## B. Evaluated projects

| Project | Repo / distribution | Licence (claimed public) | Maintenance | Quality | A11y | React | TS | Tailwind | Tauri/WebView | Maturity | Verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|
| React 18 | facebook/react | MIT | Active | High | N/A | Native | Yes | N/A | Yes (current) | Production | **Already in use** |
| Vite 6 | vitejs/vite | MIT | Active | High | N/A | Via plugin | Yes | Compatible | Yes (current) | Production | **Already in use** |
| Tailwind CSS 4.x | tailwindlabs/tailwindcss | MIT | Active | High | N/A | Yes | Yes | Native | Yes if built CSS (no CDN; CSP OK) | Production | **Recommend adopt** |
| Radix Primitives | radix-ui/primitives | MIT | Active (WorkOS) | High | Excellent | Yes | Yes | Optional | Yes | Production | **Recommend (via or under shadcn)** |
| shadcn/ui | shadcn-ui/ui (registry) | MIT (code you copy) | Active | High | Inherits primitives | Yes | Yes | Required | Yes with Vite | Production pattern | **Recommend preferred kit** |
| Base UI | mui/base-ui | MIT | Active | High | Strong | Yes | Yes | Optional | Yes | Rising 2026 | Alternative primitive base |
| React Aria / React Spectrum | adobe/react-spectrum | Apache-2.0 | Active | High | Excellent | Yes | Yes | Optional | Yes | Production | Strong a11y alternative |
| Headless UI | tailwindlabs/headlessui | MIT | Active | High | Strong | Yes | Yes | Designed for | Yes | Production | Narrower set; OK subset |
| Ark UI | chakra-ui/ark | MIT | Active | High | Strong | Yes (+others) | Yes | Optional | Yes | Growing | Alternate headless |
| Lucide React | lucide-icons/lucide | ISC | Active | High | Icons | Yes | Yes | Yes | Yes | Production | **Recommend icons** |
| Motion (Framer Motion) | motiondivision/motion | MIT | Active | High | Motion careful | Yes | Yes | Yes | Yes | Production | **Recommend Phase 4; optional Phase 2** |
| cmdk | pacocoursey/cmdk | MIT | Active | High | Good | Yes | Yes | Common | Yes | Production | Phase 3+ command palette |
| Sonner | emilkowalski/sonner | MIT | Active | High | Good | Yes | Yes | Common | Yes | Production | Optional toasts |
| Vaul | emilkowalski/vaul | MIT | Active | High | Good | Yes | Yes | Common | Yes | Production | Drawer for Inspect |
| react-virtuoso (core) | petyosi/react-virtuoso | MIT (core list/grid/masonry) | Active | High | Good | Yes | Yes | Yes | Yes | Production | Phase 3 long lists; avoid Message List commercial SKU |
| class-variance-authority | joe-bell/cva | Apache-2.0 | Active | High | N/A | Yes | Yes | Common w/ shadcn | Yes | Production | **Recommend with shadcn** |
| clsx / tailwind-merge | lukeed/clsx; dcastil/tailwind-merge | MIT | Active | High | N/A | Yes | Yes | Yes | Yes | Production | **Recommend with Tailwind** |
| date-fns | date-fns/date-fns | MIT | Active | High | N/A | Yes | Yes | N/A | Yes | Production | Relative times (“14 min ago”) |
| MUI Material | mui/material-ui | MIT | Active | High | Strong | Yes | Yes | Poor fit | Yes | Production | **Reject for shell** (opinionated Material look fights concepts) |
| Chakra UI v2/v3 | chakra-ui/chakra-ui | MIT | Active | High | Strong | Yes | Yes | Different model | Yes | Production | Reject as full kit; Ark OK as headless |
| Ant Design | ant-design/ant-design | MIT | Active | High | Mixed | Yes | Yes | Poor fit | Yes | Production | **Reject** (enterprise form aesthetic) |
| Fluent UI React | microsoft/fluentui | MIT | Active | High | Strong | Yes | Yes | Possible | Yes | Production | Pattern reference; heavy for wedge |
| GPL-licensed UI kits (various) | — | GPL | — | — | — | — | — | — | — | — | **Reject** |

Notes:

1. shadcn/ui is a **code registry**, not a traditional npm runtime dependency of
   components; you own copied files. Primitive packages (Radix/Base UI) remain
   npm dependencies and need licence/notice compliance.
2. Lucide is **ISC** (MIT-like permissive) — acceptable under preferred set.
3. react-virtuoso **Message List** is commercially licensed — do not use that
   SKU; core Virtuoso/Masonry remain MIT.
4. No GPL UI dependency is recommended.

---

## C. Compatibility with Workspace packaging

| Concern | Guidance |
|---|---|
| CSP | Build CSS/JS into the app; no CDN scripts/styles. Tailwind JIT output is fine. |
| Tauri 2 | Vite React path already proven; avoid Node-only packages in frontend. |
| Attribution | Retain NOTICE/licence files for bundled npm deps; document copied shadcn files. |
| Trust | Prefer headless + owned styles so Product Proof chrome stays Workspace-branded. |

---

## D. Rejection list (Experience UI)

- GPL / AGPL component kits
- Ant Design / heavy Material as the product shell
- Commercial-only icon packs without clear redistribution rights
- react-virtuoso Message List commercial SKU
- Any library requiring network telemetry at runtime
