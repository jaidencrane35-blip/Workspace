/**
 * Sprint 49 — craftsmanship: tokens, motion purposes, material consolidation.
 */
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { duration, spring } from "../app/src/design-system/tokens";
import { motionPrimitive } from "../app/src/lib/motion";

const root = path.resolve(__dirname, "..");
const tokensCss = readFileSync(
  path.join(root, "app/src/design-system/tokens.css"),
  "utf8",
);
const appCss = readFileSync(path.join(root, "app/src/App.css"), "utf8");

describe("craftsmanship refinement", () => {
  it("defines four motion purpose tokens", () => {
    expect(tokensCss).toContain("--motion-attention");
    expect(tokensCss).toContain("--motion-continuity");
    expect(tokensCss).toContain("--motion-memory");
    expect(tokensCss).toContain("--motion-reconstruction");
    expect(duration.attention).toBe(0.14);
    expect(duration.memory).toBe(1.1);
    expect(duration.reconstruction).toBe(0.9);
  });

  it("maps primitives onto the four purpose springs", () => {
    expect(motionPrimitive("focus").transition).toEqual(spring.attention);
    expect(motionPrimitive("reveal").transition).toEqual(spring.continuity);
    expect(motionPrimitive("orbit").transition).toEqual(spring.memory);
    expect(motionPrimitive("restore").transition).toEqual(
      spring.reconstruction,
    );
  });

  it("uses a single material blur/saturate recipe for object surfaces", () => {
    expect(tokensCss).toContain("--mat-blur");
    expect(tokensCss).toContain("--mat-saturate");
    expect(appCss).toContain("blur(var(--mat-blur))");
    expect(appCss).toContain("saturate(var(--mat-saturate))");
    expect(appCss).not.toMatch(/backdrop-filter:\s*blur\(22px\)/);
    expect(appCss).not.toMatch(/backdrop-filter:\s*blur\(28px\)/);
  });

  it("routes semantic field motion through memory purpose tokens", () => {
    expect(appCss).toContain("var(--motion-memory)");
    expect(appCss).toContain("var(--ease-memory)");
    expect(appCss).not.toMatch(/left 1\.25s/);
    expect(appCss).not.toMatch(/left 1\.15s/);
  });

  it("keeps --motion-lush as an alias of memory (no orphan timing)", () => {
    expect(tokensCss).toMatch(
      /--motion-lush:\s*var\(--motion-memory\)/,
    );
  });
});
