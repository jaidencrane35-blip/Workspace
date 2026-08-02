/** Workspace Design System — single source of visual truth. */

export const space = {
  1: "4px",
  2: "8px",
  3: "12px",
  4: "16px",
  5: "24px",
  6: "32px",
  7: "48px",
  8: "64px",
} as const;

export const type = {
  display: "clamp(2.2rem, 4.4vw, 3.25rem)",
  title: "clamp(1.5rem, 2.5vw, 2.05rem)",
  section: "1.2rem",
  body: "0.98rem",
  label: "0.68rem",
  meta: "0.82rem",
} as const;

export const elevation = {
  surface: 1,
  floating: 2,
  overlay: 3,
} as const;

export const radius = {
  sm: "10px",
  md: "18px",
  lg: "26px",
  xl: "32px",
  pill: "999px",
} as const;

export const blur = {
  surface: "18px",
  floating: "26px",
  overlay: "34px",
  dock: "30px",
} as const;

export const opacity = {
  mute: 0.55,
  soft: 0.72,
  glass: 0.62,
  strong: 0.88,
  full: 1,
} as const;

export const duration = {
  instant: 0.01,
  fast: 0.14,
  base: 0.28,
  slow: 0.4,
  ambient: 72,
} as const;

export const spring = {
  snappy: { type: "spring" as const, stiffness: 480, damping: 32, mass: 0.45 },
  soft: { type: "spring" as const, stiffness: 300, damping: 34, mass: 0.85 },
  lush: { type: "spring" as const, stiffness: 240, damping: 30, mass: 1 },
  dock: { type: "spring" as const, stiffness: 420, damping: 28, mass: 0.4 },
  layout: { type: "spring" as const, stiffness: 340, damping: 36 },
} as const;

export const lighting = {
  ambient: "rgba(95, 208, 216, 0.12)",
  warm: "rgba(232, 184, 122, 0.1)",
  focus: "rgba(142, 236, 240, 0.18)",
  hover: "rgba(255, 255, 255, 0.06)",
  dim: "rgba(0, 0, 0, 0.45)",
} as const;

export const interaction = {
  hoverLift: -3,
  pressScale: 0.97,
  magneticMax: 6,
  magneticFactor: 0.22,
} as const;

export const color = {
  ink: "#05070c",
  bg: "#070b12",
  text: "#f2f6fb",
  muted: "#8fa3b8",
  accent: "#5fd0d8",
  accentStrong: "#8eecf0",
  accentWarm: "#e8b87a",
  ok: "#86efac",
  danger: "#f87171",
} as const;

export const spatial = {
  max: "76rem",
  gutter: "clamp(1.25rem, 3vw, 2rem)",
  dockH: "5rem",
} as const;
