/**
 * IPC tier classification helpers (Constitution §4.8).
 *
 * Tiers: product | developer | diagnostic | experimental
 * Quarantine: non-product parity registrations unused by React.
 */
import fs from "node:fs";
import path from "node:path";

export const TIER_NAMES = [
  "product",
  "developer",
  "diagnostic",
  "experimental",
];

/**
 * @param {string} libRs
 * @returns {string[]}
 */
export function parseGenerateHandlerCommands(libRs) {
  const marker = "tauri::generate_handler![";
  const start = libRs.indexOf(marker);
  if (start < 0) {
    throw new Error("generate_handler! not found in lib.rs");
  }
  const after = libRs.slice(start + marker.length);
  const end = after.indexOf("]");
  if (end < 0) {
    throw new Error("generate_handler! closing ] not found");
  }
  const body = after.slice(0, end);
  const commands = [];
  for (const line of body.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("//")) continue;
    const match = trimmed.match(/^([a-z][a-z0-9_]*)\s*,?\s*$/);
    if (match) {
      commands.push(match[1]);
    }
  }
  if (commands.length === 0) {
    throw new Error("no commands parsed from generate_handler!");
  }
  return commands;
}

/**
 * @param {string} catalogTs
 * @returns {string[]}
 */
export function parseExperienceIpcCommands(catalogTs) {
  const marker = "export const EXPERIENCE_IPC_COMMANDS";
  const start = catalogTs.indexOf(marker);
  if (start < 0) {
    throw new Error("EXPERIENCE_IPC_COMMANDS not found");
  }
  const from = catalogTs.indexOf("[", start);
  const to = catalogTs.indexOf("] as const", from);
  if (from < 0 || to < 0) {
    throw new Error("EXPERIENCE_IPC_COMMANDS array bounds not found");
  }
  const body = catalogTs.slice(from, to + 1);
  const commands = [...body.matchAll(/"([a-z][a-z0-9_]*)"/g)].map((m) => m[1]);
  if (commands.length === 0) {
    throw new Error("no experience IPC commands parsed");
  }
  return commands;
}

/**
 * Extract command string literals from invokeIpc call sites (incl. generics).
 * @param {string} text
 * @returns {string[]}
 */
export function extractInvokeIpcCommandsFromSource(text) {
  const found = new Set();
  let i = 0;
  while (i < text.length) {
    const idx = text.indexOf("invokeIpc", i);
    if (idx < 0) break;
    let j = idx + "invokeIpc".length;
    while (j < text.length && /\s/.test(text[j])) j++;
    if (text[j] === "<") {
      let depth = 0;
      for (; j < text.length; j++) {
        const ch = text[j];
        if (ch === "<") depth++;
        else if (ch === ">") {
          depth--;
          if (depth === 0) {
            j++;
            break;
          }
        }
      }
    }
    while (j < text.length && /\s/.test(text[j])) j++;
    if (text[j] !== "(") {
      i = idx + 8;
      continue;
    }
    j++;
    while (j < text.length && /\s/.test(text[j])) j++;
    const quote = text[j];
    if (quote !== '"' && quote !== "'") {
      i = idx + 8;
      continue;
    }
    j++;
    const end = text.indexOf(quote, j);
    if (end < 0) {
      i = idx + 8;
      continue;
    }
    const cmd = text.slice(j, end);
    if (/^[a-z][a-z0-9_]*$/.test(cmd)) {
      found.add(cmd);
    }
    i = end + 1;
  }
  return [...found].sort();
}

/**
 * Collect string literals passed to invokeIpc(...) across app/src.
 * @param {string} appSrcDir
 * @returns {string[]}
 */
export function parseReactInvokeCommands(appSrcDir) {
  const found = new Set();

  /** @param {string} dir */
  function walk(dir) {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(full);
        continue;
      }
      if (!/\.(ts|tsx)$/.test(entry.name)) continue;
      if (entry.name.endsWith(".d.ts")) continue;
      const text = fs.readFileSync(full, "utf8");
      for (const cmd of extractInvokeIpcCommandsFromSource(text)) {
        found.add(cmd);
      }
    }
  }

  walk(appSrcDir);
  return [...found].sort();
}

/**
 * @param {string[]} items
 * @returns {string[]}
 */
export function uniqueSorted(items) {
  return [...new Set(items)].sort();
}

/**
 * @param {object} input
 * @param {string[]} input.registered
 * @param {string[]} input.product
 * @param {string[]} input.diagnostic
 * @param {string[]} input.reactUsed
 * @param {string[]} input.quarantine
 */
export function computeIpcTiers(input) {
  const registered = uniqueSorted(input.registered);
  const registeredSet = new Set(registered);
  const product = uniqueSorted(input.product);
  const diagnosticAuth = uniqueSorted(input.diagnostic);
  const quarantine = uniqueSorted(input.quarantine);
  const reactUsed = uniqueSorted(input.reactUsed);

  const errors = [];

  for (const cmd of product) {
    if (!registeredSet.has(cmd)) {
      errors.push(`product command not registered: ${cmd}`);
    }
  }
  for (const cmd of diagnosticAuth) {
    if (!registeredSet.has(cmd)) {
      errors.push(`diagnostic command not registered: ${cmd}`);
    }
    if (product.includes(cmd)) {
      errors.push(`diagnostic overlaps product: ${cmd}`);
    }
  }
  for (const cmd of quarantine) {
    if (!registeredSet.has(cmd)) {
      errors.push(`quarantine command not registered: ${cmd}`);
    }
    if (product.includes(cmd)) {
      errors.push(`quarantine overlaps product: ${cmd}`);
    }
  }

  const productSet = new Set(product);
  const diagnosticSet = new Set(diagnosticAuth);

  const developer = [];
  for (const cmd of reactUsed) {
    if (!registeredSet.has(cmd)) {
      errors.push(`React invokeIpc target not registered: ${cmd}`);
      continue;
    }
    if (productSet.has(cmd) || diagnosticSet.has(cmd)) continue;
    developer.push(cmd);
  }
  developer.sort();

  const assigned = new Set([...product, ...diagnosticAuth, ...developer]);
  const experimental = registered.filter((cmd) => !assigned.has(cmd));

  const experimentalSet = new Set(experimental);
  for (const cmd of quarantine) {
    if (!experimentalSet.has(cmd) && !diagnosticSet.has(cmd)) {
      // Quarantine may sit in diagnostic (read-only inspection unused by React)
      // or experimental. Must not be developer (React-used).
      if (developer.includes(cmd)) {
        errors.push(
          `quarantine command is React-used (developer): ${cmd} — remove from quarantine or stop calling it`,
        );
      } else if (!diagnosticSet.has(cmd)) {
        errors.push(
          `quarantine command not in experimental/diagnostic: ${cmd}`,
        );
      }
    }
  }

  const reactNonProduct = reactUsed.filter((c) => !productSet.has(c));
  for (const cmd of quarantine) {
    if (reactNonProduct.includes(cmd) && !diagnosticSet.has(cmd)) {
      // already covered
    }
  }

  return {
    registered,
    product,
    developer,
    diagnostic: diagnosticAuth,
    experimental,
    quarantine,
    reactUsed,
    errors,
    counts: {
      registered: registered.length,
      product: product.length,
      developer: developer.length,
      diagnostic: diagnosticAuth.length,
      experimental: experimental.length,
      quarantine: quarantine.length,
      reactUsed: reactUsed.length,
    },
  };
}

/**
 * @param {ReturnType<typeof computeIpcTiers>} tiers
 * @returns {string}
 */
export function renderIpcTiersTs(tiers) {
  const header = `/** AUTO-GENERATED — do not edit.
 * Source: app/src-tauri/src/lib.rs + app/src/demo/experienceIpcCatalog.ts
 *         + docs/03-Engineering/ipc-tiers.json + React invokeIpc usage
 * Constitution tiers: Product | Developer | Diagnostic | Experimental
 * Regenerate: node scripts/sync-ipc-tiers.mjs
 * Verify: node scripts/verify-ipc-tiers.mjs
 */
`;

  const asConstArray = (name, items) =>
    `export const ${name} = [\n${items
      .map((c) => `  "${c}",\n`)
      .join("")}] as const;\n`;

  const body = `${asConstArray("PRODUCT_IPC_COMMANDS", tiers.product)}
${asConstArray("DEVELOPER_IPC_COMMANDS", tiers.developer)}
${asConstArray("DIAGNOSTIC_IPC_COMMANDS", tiers.diagnostic)}
${asConstArray("EXPERIMENTAL_IPC_COMMANDS", tiers.experimental)}
${asConstArray("QUARANTINED_IPC_COMMANDS", tiers.quarantine)}
${asConstArray("REGISTERED_IPC_COMMANDS", tiers.registered)}

export type IpcTier = "product" | "developer" | "diagnostic" | "experimental";

export const IPC_TIER_COUNTS = ${JSON.stringify(tiers.counts, null, 2)} as const;

const productSet = new Set<string>(PRODUCT_IPC_COMMANDS);
const developerSet = new Set<string>(DEVELOPER_IPC_COMMANDS);
const diagnosticSet = new Set<string>(DIAGNOSTIC_IPC_COMMANDS);
const experimentalSet = new Set<string>(EXPERIMENTAL_IPC_COMMANDS);
const quarantineSet = new Set<string>(QUARANTINED_IPC_COMMANDS);

export function ipcTierOf(command: string): IpcTier | null {
  if (productSet.has(command)) return "product";
  if (diagnosticSet.has(command)) return "diagnostic";
  if (developerSet.has(command)) return "developer";
  if (experimentalSet.has(command)) return "experimental";
  return null;
}

export function isQuarantinedIpcCommand(command: string): boolean {
  return quarantineSet.has(command);
}

export function isProductIpcCommand(command: string): boolean {
  return productSet.has(command);
}
`;

  return header + body;
}

/**
 * Stable JSON for snapshot compare.
 * @param {unknown} value
 */
export function stableStringify(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}
