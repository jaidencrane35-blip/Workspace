import {
  blur,
  color,
  duration,
  lighting,
  opacity,
  radius,
  space,
  spatial,
  type,
} from "./tokens";

/** Serialize design tokens into :root CSS custom properties. */
export function designSystemCssVars(): string {
  return `
:root {
  --ws-space-1: ${space[1]};
  --ws-space-2: ${space[2]};
  --ws-space-3: ${space[3]};
  --ws-space-4: ${space[4]};
  --ws-space-5: ${space[5]};
  --ws-space-6: ${space[6]};
  --ws-space-7: ${space[7]};
  --ws-space-8: ${space[8]};
  --ws-type-display: ${type.display};
  --ws-type-title: ${type.title};
  --ws-type-section: ${type.section};
  --ws-type-body: ${type.body};
  --ws-type-label: ${type.label};
  --ws-type-meta: ${type.meta};
  --ws-radius-sm: ${radius.sm};
  --ws-radius-md: ${radius.md};
  --ws-radius-lg: ${radius.lg};
  --ws-radius-xl: ${radius.xl};
  --ws-radius-pill: ${radius.pill};
  --ws-blur-surface: ${blur.surface};
  --ws-blur-floating: ${blur.floating};
  --ws-blur-overlay: ${blur.overlay};
  --ws-blur-dock: ${blur.dock};
  --ws-opacity-mute: ${opacity.mute};
  --ws-opacity-soft: ${opacity.soft};
  --ws-opacity-glass: ${opacity.glass};
  --ws-dur-fast: ${duration.fast}s;
  --ws-dur-base: ${duration.base}s;
  --ws-dur-slow: ${duration.slow}s;
  --ws-light-ambient: ${lighting.ambient};
  --ws-light-warm: ${lighting.warm};
  --ws-light-focus: ${lighting.focus};
  --ws-light-hover: ${lighting.hover};
  --ws-light-dim: ${lighting.dim};
  --ws-color-ink: ${color.ink};
  --ws-color-bg: ${color.bg};
  --ws-color-text: ${color.text};
  --ws-color-muted: ${color.muted};
  --ws-color-accent: ${color.accent};
  --ws-color-accent-strong: ${color.accentStrong};
  --ws-color-accent-warm: ${color.accentWarm};
  --ws-color-ok: ${color.ok};
  --ws-color-danger: ${color.danger};
  --ws-spatial-max: ${spatial.max};
  --ws-spatial-gutter: ${spatial.gutter};
  --ws-dock-h: ${spatial.dockH};

  --space-1: var(--ws-space-1);
  --space-2: var(--ws-space-2);
  --space-3: var(--ws-space-3);
  --space-4: var(--ws-space-4);
  --space-5: var(--ws-space-5);
  --space-6: var(--ws-space-6);
  --space-7: var(--ws-space-7);
  --type-display: var(--ws-type-display);
  --type-title: var(--ws-type-title);
  --type-section: var(--ws-type-section);
  --type-body: var(--ws-type-body);
  --type-label: var(--ws-type-label);
  --radius-sm: var(--ws-radius-sm);
  --radius-md: var(--ws-radius-md);
  --radius-lg: var(--ws-radius-lg);
  --radius-pill: var(--ws-radius-pill);
  --dock-h: var(--ws-dock-h);
  --spatial-max: var(--ws-spatial-max);
  --spatial-gutter: var(--ws-spatial-gutter);
  --color-ink: var(--ws-color-ink);
  --color-bg: var(--ws-color-bg);
  --color-text: var(--ws-color-text);
  --color-text-muted: var(--ws-color-muted);
  --color-accent: var(--ws-color-accent);
  --color-accent-strong: var(--ws-color-accent-strong);
  --color-accent-warm: var(--ws-color-accent-warm);
  --color-ok: var(--ws-color-ok);
  --color-danger: var(--ws-color-danger);
  --exp-text: var(--ws-color-text);
  --exp-muted: var(--ws-color-muted);
  --exp-accent: var(--ws-color-accent);
  --exp-accent-strong: var(--ws-color-accent-strong);
  --exp-ok: var(--ws-color-ok);
  --exp-danger: var(--ws-color-danger);
  --exp-font: "Segoe UI Variable Text", "Segoe UI", Candara, "Gill Sans", sans-serif;
  --exp-display: "Segoe UI Variable Display", "Segoe UI", Candara, sans-serif;
  --motion-fast: calc(var(--ws-dur-fast) * 1000ms);
  --motion-base: calc(var(--ws-dur-base) * 1000ms);
  --motion-slow: calc(var(--ws-dur-slow) * 1000ms);
  --ease: cubic-bezier(0.22, 0.82, 0.2, 1);
  --shadow-1: 0 10px 36px rgba(0, 0, 0, 0.38);
  --shadow-2: 0 22px 64px rgba(0, 0, 0, 0.48);
  --shadow-3: 0 32px 90px rgba(0, 0, 0, 0.55);
  --shadow-4: 0 40px 120px rgba(0, 0, 0, 0.6);
  --shadow-glow: 0 0 56px var(--ws-light-ambient);
  --edge-light: inset 0 1px 0 rgba(255, 255, 255, 0.1),
    inset 0 0 0 1px rgba(255, 255, 255, 0.035);
  --color-border: rgba(160, 190, 220, 0.1);
  --color-border-strong: rgba(130, 210, 230, 0.28);
  --color-surface: rgba(18, 26, 38, 0.62);
  --color-surface-hero: rgba(22, 34, 50, 0.72);
  --color-bg-elevated: rgba(14, 20, 30, 0.72);
  --exp-bg: var(--ws-color-bg);
  --exp-bg-elevated: var(--color-bg-elevated);
  --exp-bg-card: var(--color-surface);
  --exp-border: var(--color-border);
  --exp-radius: var(--ws-radius-lg);
  --exp-shadow: var(--shadow-1);
}
`.trim();
}
