# Frontend UX Audit

## Executive summary

The `app/src` frontend is functional and uses a coherent dark theme, but it is dominated by large, monolithic components and duplicated UI patterns. The product `Work` and `Assistant` views are generally clear, while the diagnostic `OperatorConsole` surface is especially dense and inconsistent. The current implementation would benefit from reusable presentation wrappers, more consistent empty/loading states, and clearer local feedback.

## Current strengths

- Shared theme and typography are applied consistently through `app/src/App.css`.
- The tabbed navigation in `app/src/App.tsx` is simple and easy to understand.
- Utility classes such as `.row`, `.muted`, `.mono`, `.badge`, and `.list` are reused across components.
- Buttons are consistently styled and most include `type="button"`.
- The product-facing `WorkspaceIntelligencePanel.tsx` and `AssistantPanel.tsx` clearly separate informative sections.
- `DisplayReasonList.tsx` is a small reusable rendering component for reason lists.

## Issues found

### 1. Component structure

- `app/src/components/WorkspaceIntelligencePanel.tsx` is oversized and contains many repeated `section` blocks with near-identical refresh/compare patterns.
- `app/src/components/OperatorConsole.tsx` is extremely large and spans many unrelated diagnostics, making it difficult to reason about and maintain.
- `app/src/components/AssistantPanel.tsx` is also large and repeats patterns for section headers, button rows, and status summaries.
- `app/src/components/RecommendationExplanationView.tsx` includes repeated explanation block markup and could use smaller reusable detail-block components.
- `app/src/App.tsx` contains repeated wrapper logic for the four main views; the layout could be more declarative with reusable page-shell components.
- Heading hierarchy is inconsistent across views: `OperatorConsole` relies on `h2` section headings while `WorkspaceIntelligencePanel` uses `h3` for equivalent sections.

### 2. Visual consistency

- Spacing across sections is uneven: some areas use `section` default spacing, others use inline margins or additional wrappers.
- Empty/loading/error states vary widely: some show plain `p.muted` text, others render `ul.muted`, `<details>`, or no visual separator.
- Buttons are mostly consistent, but button groups in `OperatorConsole` are dense and can feel cluttered.
- Panels are not visually separated into cards or containers; most content is raw `section` markup, which reduces scannability.
- The diagnostic `OperatorConsole` presentation is visually noisy because many subsections are displayed in one long page.

### 3. User experience

- The `OperatorConsole` flow is confusing due to its sheer density and many similarly labeled actions.
- Labels like "Refresh workspace understanding", "Inspect decision graph", and "Evaluate approved contracts now" are functional but could be clearer about the exact effect and scope.
- Local loading and success/failure feedback are centralized through App-level banners; individual sections lack dedicated loading indicators.
- There is no explicit `aria-live` region for top-level status messages, which may reduce accessibility for screen-reader users.
- Many form inputs rely on `aria-label` rather than visible labels; this is acceptable but reduces discoverability for some users.
- Collapsible summary sections (`<details>`) are used inconsistently and may hide important information without a consistent affordance.

### 4. Code quality

- `WorkspaceIntelligencePanel.tsx` and `OperatorConsole.tsx` both contain long lists of `useState` hooks, which increases maintenance risk.
- The `run()` wrapper pattern is repeated across multiple components; this is good for consistency, but it also suggests a shared hook or utility is needed.
- `OperatorConsole.tsx` contains a long list of typed props and state variables, increasing the risk of stale or unused code.
- `RecommendationExplanationView.tsx` repeats many subcomponents and relies on repeated `muted`/`explain-block` wrapper styling.
- `CanvasShell.tsx` uses inline styles for dynamic positioning and transform, which is necessary for the canvas but represents a localized inline styling pattern.

## Priority ranking

### P0 critical

- Break up `OperatorConsole.tsx` and `WorkspaceIntelligencePanel.tsx` into smaller, reusable presentation components to reduce maintenance risk and improve clarity.
- Add consistent local loading/error states for section-level refresh and compare actions so users can tell which section is waiting.
- Add an `aria-live` region for top-level status messages to improve accessibility.

### P1 important

- Standardize empty/loading panel presentation across the product and diagnostic surfaces.
- Introduce reusable section/card wrappers for headings, content, and action rows.
- Clarify ambiguous button labels and action descriptions in the diagnostic console.

### P2 polish

- Harmonize heading hierarchy and section title styling across `WorkspaceIntelligencePanel`, `AssistantPanel`, and `OperatorConsole`.
- Reduce visual density in `OperatorConsole` by grouping related actions and adding spacing or separators.
- Replace repeated explanation-block markup in `RecommendationExplanationView` with smaller reusable display components.

## Recommended future UI tasks

1. Extract a shared `SectionPanel` or `DetailSection` component for repeated `section`/`h3`/`row` structures.
2. Create a unified `EmptyState` / `LoadingState` component for snapshot and no-data states.
3. Refactor `OperatorConsole.tsx` into smaller diagnostic subcomponents with clear boundaries.
4. Add a shared status region with `aria-live` for `onMessage` and `onError` updates.
5. Audit button labels and field labels in the diagnostic and intelligence views for clarity.
6. Consider a lightweight card or panel visual treatment to improve scannability of long pages.

---

Audit scope: `app/src/**` only. No code changes were made as part of this report.
