import fs from "node:fs";
import path from "node:path";

const read = (file) => fs.readFileSync(file, "utf8");
const sorted = (values) => [...new Set(values)].sort();

function commandDefinitions(commandsDir) {
  const commands = [];
  for (const entry of fs.readdirSync(commandsDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue;
    const source = read(path.join(commandsDir, entry.name));
    const pattern =
      /#\[tauri::command\](?:\s*#\[[^\]]+\])*\s*pub(?:\(crate\))?\s+(?:async\s+)?fn\s+([a-z0-9_]+)/g;
    commands.push(...[...source.matchAll(pattern)].map((match) => match[1]));
  }
  return sorted(commands);
}

function commandEdgeErrorCodes(commandsDir) {
  const codes = [];
  for (const entry of fs.readdirSync(commandsDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue;
    const source = read(path.join(commandsDir, entry.name));
    const pattern = /CommandError::new\(\s*"([a-z0-9_]+)"/g;
    codes.push(...[...source.matchAll(pattern)].map((match) => match[1]));
  }
  return sorted(codes);
}

function registeredCommands(libFile) {
  const source = read(libFile);
  const block = source.match(
    /\.invoke_handler\(tauri::generate_handler!\[([\s\S]*?)\]\)/,
  );
  if (!block) throw new Error("Unable to locate Tauri generate_handler block");
  return sorted(
    [...block[1].matchAll(/^\s*([a-z][a-z0-9_]+),?\s*$/gm)].map(
      (match) => match[1],
    ),
  );
}

function frontendInvocations(frontendDir) {
  const commands = [];
  const visit = (directory) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const file = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        visit(file);
      } else if (/\.(?:ts|tsx)$/.test(entry.name)) {
        const source = read(file);
        const pattern =
          /invokeIpc(?:<[^;]*?>)?\s*\(\s*["']([a-z0-9_]+)["']/g;
        commands.push(...[...source.matchAll(pattern)].map((match) => match[1]));
      }
    }
  };
  visit(frontendDir);
  return sorted(commands);
}

function rustErrorCodes(errorFile) {
  return sorted(
    [...read(errorFile).matchAll(/code:\s*"([a-z0-9_]+)"\.into\(\)/g)].map(
      (match) => match[1],
    ),
  );
}

function typescriptErrorCodes(errorCodesFile) {
  const source = read(errorCodesFile);
  const block = source.match(
    /export const PUBLIC_ERROR_CODES = \[([\s\S]*?)\] as const;/,
  );
  if (!block) throw new Error("Unable to locate PUBLIC_ERROR_CODES");
  return sorted(
    [...block[1].matchAll(/"([a-z0-9_]+)"/g)].map((match) => match[1]),
  );
}

function rustStructFields(file, name) {
  const source = read(file);
  const block = source.match(
    new RegExp(`pub struct ${name} \\{([\\s\\S]*?)\\n\\}`),
  );
  if (!block) throw new Error(`Unable to locate Rust struct ${name}`);
  return sorted(
    [...block[1].matchAll(/^\s*pub\s+([a-z][a-z0-9_]*):/gm)].map(
      (match) => match[1],
    ),
  );
}

function typescriptInterfaceFields(file, name) {
  const source = read(file);
  const block = source.match(
    new RegExp(`export interface ${name} \\{([\\s\\S]*?)\\n\\}`),
  );
  if (!block) throw new Error(`Unable to locate TypeScript interface ${name}`);
  return sorted(
    [...block[1].matchAll(/^\s*([a-z][a-z0-9_]*)(?:\?)?:/gm)].map(
      (match) => match[1],
    ),
  );
}

const difference = (left, right) =>
  left.filter((value) => !new Set(right).has(value));

export function auditIpcContracts(rootDir) {
  const registered = registeredCommands(
    path.join(rootDir, "app/src-tauri/src/lib.rs"),
  );
  const defined = commandDefinitions(
    path.join(rootDir, "app/src-tauri/src/commands"),
  );
  const commandErrorCodes = commandEdgeErrorCodes(
    path.join(rootDir, "app/src-tauri/src/commands"),
  );
  const invoked = frontendInvocations(path.join(rootDir, "app/src"));
  const rustCodes = rustErrorCodes(
    path.join(rootDir, "packages/kernel/src/error.rs"),
  );
  const typescriptCodes = typescriptErrorCodes(
    path.join(rootDir, "app/src/types/ipc-errors.ts"),
  );
  const rustOutcomeFields = rustStructFields(
    path.join(rootDir, "packages/domain/src/action_proposal/mod.rs"),
    "RecommendationOutcome",
  );
  const typescriptOutcomeFields = typescriptInterfaceFields(
    path.join(rootDir, "app/src/types/domain.ts"),
    "RecommendationOutcome",
  );

  const violations = [
    ...difference(registered, defined).map(
      (name) => `Registered command has no definition: ${name}`,
    ),
    ...difference(defined, registered).map(
      (name) => `Tauri command is not registered: ${name}`,
    ),
    ...difference(invoked, registered).map(
      (name) => `Frontend invokes unregistered command: ${name}`,
    ),
    ...difference(rustCodes, typescriptCodes).map(
      (code) => `Kernel public error code missing from TypeScript: ${code}`,
    ),
    ...difference(typescriptCodes, rustCodes).map(
      (code) => `TypeScript public error code missing from kernel: ${code}`,
    ),
    ...difference(commandErrorCodes, rustCodes).map(
      (code) => `Command-edge error code missing from kernel catalog: ${code}`,
    ),
    ...difference(rustOutcomeFields, typescriptOutcomeFields).map(
      (field) => `RecommendationOutcome field missing from TypeScript: ${field}`,
    ),
    ...difference(typescriptOutcomeFields, rustOutcomeFields).map(
      (field) => `RecommendationOutcome field missing from Rust: ${field}`,
    ),
  ];

  return {
    violations,
    registered,
    defined,
    invoked,
    unused: difference(registered, invoked),
    publicErrorCodes: rustCodes,
    commandErrorCodes,
  };
}
