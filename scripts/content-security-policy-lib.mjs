/**
 * Content Security Policy rules for the Tauri shell (PP-B03).
 *
 * The policy is the app's only defence against injected script inside the
 * WebView, which runs with full IPC reach into the kernel. These rules pin it
 * so it cannot be disabled or widened without an explicit edit here.
 */
import fs from "node:fs";
import path from "node:path";

/**
 * Sources Tauri's custom-protocol IPC dials.
 * `http://ipc.localhost` is the Windows/Android form, `ipc:` the others.
 * Without them the IPC `fetch` is refused and Tauri silently degrades to the
 * slower postMessage fallback, so this is a functional requirement too.
 */
export const REQUIRED_IPC_SOURCES = ["ipc:", "http://ipc.localhost"];

/** Tokens that reopen the injection surface the policy exists to close. */
export const FORBIDDEN_SOURCES = [
  "'unsafe-inline'",
  "'unsafe-eval'",
  "'unsafe-hashes'",
  "'wasm-unsafe-eval'",
  "*",
  "data:",
  "blob:",
  "filesystem:",
];

/**
 * Every directive the policy must declare, mapped to the widest source set it
 * may declare. `default-src` does not fall back to `base-uri`, `form-action`
 * or `frame-ancestors`, so those are named explicitly.
 */
export const REQUIRED_DIRECTIVES = {
  "default-src": ["'self'"],
  "script-src": ["'self'"],
  "style-src": ["'self'"],
  "img-src": ["'self'"],
  "font-src": ["'self'"],
  "connect-src": ["'self'", ...REQUIRED_IPC_SOURCES],
  "frame-src": ["'none'"],
  "worker-src": ["'none'"],
  "object-src": ["'none'"],
  "base-uri": ["'self'"],
  "form-action": ["'none'"],
  "frame-ancestors": ["'none'"],
};

const INLINE_SCRIPT = /<script\b(?![^>]*\bsrc\s*=)[^>]*>/i;
const INLINE_STYLE_ELEMENT = /<style\b/i;
const EVENT_HANDLER_ATTRIBUTE = /\son[a-z]+\s*=\s*["']/i;
const JAVASCRIPT_URL = /(?:src|href)\s*=\s*["']\s*javascript:/i;
const REMOTE_RESOURCE = /(?:src|href)\s*=\s*["']\s*(?:https?:)?\/\//i;

/** Split a policy string into directive name -> source list. */
export function parseCsp(policy) {
  const directives = new Map();
  for (const segment of policy.split(";")) {
    const tokens = segment.trim().split(/\s+/).filter(Boolean);
    if (tokens.length === 0) continue;
    directives.set(tokens[0].toLowerCase(), tokens.slice(1));
  }
  return directives;
}

export function auditCspPolicy(policy) {
  const violations = [];

  if (typeof policy !== "string" || policy.trim() === "") {
    violations.push(
      "csp: no Content Security Policy is set — the WebView would run unrestricted with full IPC reach",
    );
    return violations;
  }

  const directives = parseCsp(policy);

  for (const [directive, allowed] of Object.entries(REQUIRED_DIRECTIVES)) {
    const sources = directives.get(directive);
    if (!sources) {
      violations.push(
        `csp: missing required directive '${directive}' (expected ${allowed.join(" ")})`,
      );
      continue;
    }
    for (const source of sources) {
      if (FORBIDDEN_SOURCES.includes(source)) {
        violations.push(`csp: '${directive}' declares forbidden source ${source}`);
      } else if (!allowed.includes(source)) {
        violations.push(
          `csp: '${directive}' declares unapproved source ${source} (allowed: ${allowed.join(" ")})`,
        );
      }
    }
  }

  for (const required of REQUIRED_IPC_SOURCES) {
    if (!(directives.get("connect-src") ?? []).includes(required)) {
      violations.push(
        `csp: 'connect-src' omits ${required} — Tauri IPC would be refused and fall back to postMessage`,
      );
    }
  }

  for (const [directive, sources] of directives) {
    if (directive in REQUIRED_DIRECTIVES) continue;
    violations.push(
      `csp: undeclared directive '${directive} ${sources.join(" ")}' — add it to REQUIRED_DIRECTIVES with a reviewed source set`,
    );
  }

  return violations;
}

export function auditSecurityConfig(security) {
  const violations = auditCspPolicy(security?.csp);

  if (security?.devCsp !== undefined) {
    violations.push(
      "security: 'devCsp' is set — development must not run under a weaker policy than production",
    );
  }

  const disabled = security?.dangerousDisableAssetCspModification;
  if (disabled !== undefined && disabled !== false) {
    violations.push(
      `security: 'dangerousDisableAssetCspModification' is ${JSON.stringify(disabled)} — Tauri must keep injecting its nonce and hash sources`,
    );
  }

  return violations;
}

export function auditHtmlDocument(name, html) {
  const violations = [];
  const checks = [
    [INLINE_SCRIPT, "inline <script> element — blocked by script-src 'self'"],
    [INLINE_STYLE_ELEMENT, "inline <style> element — blocked by style-src 'self'"],
    [EVENT_HANDLER_ATTRIBUTE, "inline event handler attribute — blocked by script-src 'self'"],
    [JAVASCRIPT_URL, "javascript: URL — blocked by script-src 'self'"],
    [REMOTE_RESOURCE, "remote resource reference — blocked by default-src 'self'"],
  ];
  for (const [pattern, reason] of checks) {
    if (pattern.test(html)) {
      violations.push(`${name}: ${reason}`);
    }
  }
  return violations;
}

/**
 * Audit the shipped configuration and the HTML documents it serves.
 * `htmlPaths` that do not exist are skipped so the build output is checked
 * when present without requiring a build.
 */
export function auditContentSecurityPolicy({ configPath, htmlPaths }) {
  const config = JSON.parse(fs.readFileSync(configPath, "utf8"));
  const violations = auditSecurityConfig(config?.app?.security);

  for (const htmlPath of htmlPaths) {
    if (!fs.existsSync(htmlPath)) continue;
    violations.push(
      ...auditHtmlDocument(
        path.basename(path.dirname(htmlPath)) + "/" + path.basename(htmlPath),
        fs.readFileSync(htmlPath, "utf8"),
      ),
    );
  }

  return violations;
}
