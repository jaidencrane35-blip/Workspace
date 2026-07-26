/**
 * Experience catalog contract tests (Sprint 132).
 * Validates sync guard semantics and fixture resolution via shared lib.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  lookupCatalogEntry,
  parseGeneratedCatalogTs,
  resolveFixture,
  stableStringify,
} from "../scripts/explanation-catalog-lib.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const jsonPath = path.join(
  root,
  "packages/kernel/resources/explanation-catalog.json",
);
const tsPath = path.join(root, "app/src/generated/explanationCatalog.ts");

const sourceCatalog = JSON.parse(fs.readFileSync(jsonPath, "utf8"));
const generatedCatalog = parseGeneratedCatalogTs(
  fs.readFileSync(tsPath, "utf8"),
);

describe("explanation catalog contract", () => {
  it("generated artifact matches JSON source", () => {
    expect(stableStringify(generatedCatalog)).toBe(
      stableStringify(sourceCatalog),
    );
  });

  it("resolves exact keys", () => {
    const resolved = lookupCatalogEntry(sourceCatalog, "decision.base.outstanding");
    expect(resolved?.tier).toBe("exact");
    expect(resolved?.entry.title).toBe("Outstanding decision needs attention");
  });

  it("resolves pattern keys before prefix fallback", () => {
    const pattern = lookupCatalogEntry(
      sourceCatalog,
      "purpose.obstacle.composition:missing_application",
    );
    expect(pattern?.tier).toBe("prefix_pattern");

    const fallback = lookupCatalogEntry(
      sourceCatalog,
      "purpose.obstacle.unlisted_kind",
    );
    expect(fallback?.tier).toBe("prefix_fallback");
  });

  it("contract fixtures resolve deterministically", () => {
    for (const fixture of sourceCatalog.contract_fixtures ?? []) {
      const resolved = resolveFixture(sourceCatalog, fixture);
      expect(resolved.known).toBe(fixture.known);
      expect(resolved.title).toBe(fixture.title);
      expect(resolved.description).toBe(fixture.description);
    }
  });

  it("unknown keys degrade safely", () => {
    const resolved = resolveFixture(sourceCatalog, {
      key: "future.signal.unknown",
      signal: "blocked_task",
      source: "task_graph",
      weight: 10,
      known: false,
      title: "Blocked task needs attention",
      description:
        "No Experience translation for 'future.signal.unknown' yet (signal blocked_task from task_graph, weight 10).",
    });
    expect(resolved.known).toBe(false);
    expect(resolved.description).toContain("future.signal.unknown");
  });
});
