/**
 * UI ↔ Experience import boundary rules (Sprint 134).
 */
import fs from "node:fs";
import path from "node:path";

/** Files allowed to import resolver/catalog internals. */
export const RESOLVER_ALLOWLIST = new Set([
  "experienceTranslation.ts",
  "explanationResolver.ts",
  "DisplayReasonList.tsx",
]);

/** Files allowed to name Domain reasoning types for translation input. */
export const REASON_TYPE_ALLOWLIST = new Set([
  ...RESOLVER_ALLOWLIST,
  "domain.ts",
]);

/** Operator is an explicit diagnostic surface — exempt from user-surface rules. */
export const DIAGNOSTIC_COMPONENTS = new Set(["OperatorConsole.tsx"]);

const FORBIDDEN_IMPORT_PATTERNS = [
  /from\s+["'][^"']*explanationResolver["']/,
  /from\s+["'][^"']*generated\/explanationCatalog["']/,
  /from\s+["'][^"']*explanation-catalog-lib["']/,
];

const DOMAIN_REASON_TYPE_PATTERN =
  /import\s+(?:type\s+)?\{[^}]*\b(?:AttentionReason|DecisionReason)\b[^}]*\}\s+from\s+["'][^"']*types\/domain["']/;

export function listComponentSources(componentsDir) {
  return fs
    .readdirSync(componentsDir)
    .filter((name) => name.endsWith(".tsx"))
    .map((name) => ({
      name,
      path: path.join(componentsDir, name),
      source: fs.readFileSync(path.join(componentsDir, name), "utf8"),
    }));
}

export function auditUiExperienceBoundary(componentsDir) {
  const violations = [];
  for (const file of listComponentSources(componentsDir)) {
    const isDiagnostic = DIAGNOSTIC_COMPONENTS.has(file.name);
    const allowResolver = RESOLVER_ALLOWLIST.has(file.name);
    const allowReasonTypes = REASON_TYPE_ALLOWLIST.has(file.name);

    if (!allowResolver) {
      for (const pattern of FORBIDDEN_IMPORT_PATTERNS) {
        if (pattern.test(file.source)) {
          violations.push(
            `${file.name}: forbidden import matching ${pattern} (use DisplayReasonList / experienceTranslation boundary)`,
          );
        }
      }
    }

    if (!allowReasonTypes && !isDiagnostic) {
      if (DOMAIN_REASON_TYPE_PATTERN.test(file.source)) {
        violations.push(
          `${file.name}: imports AttentionReason/DecisionReason from domain — pass opaque arrays to DisplayReasonList instead`,
        );
      }
    }

    if (!isDiagnostic && !allowResolver) {
      if (/resolveAttentionReason|resolveDecisionReason|lookupCatalogEntry/.test(file.source)) {
        violations.push(
          `${file.name}: calls Experience resolver directly — use DisplayReasonList`,
        );
      }
    }
  }
  return violations;
}
