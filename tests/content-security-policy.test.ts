/**
 * Content Security Policy tests (PP-B03).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  auditContentSecurityPolicy,
  auditCspPolicy,
  auditHtmlDocument,
  auditSecurityConfig,
  parseCsp,
} from "../scripts/content-security-policy-lib.mjs";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const configPath = path.join(root, "app/src-tauri/tauri.conf.json");
const htmlPaths = [
  path.join(root, "app/index.html"),
  path.join(root, "app/dist/index.html"),
];

const shippedPolicy = (): string =>
  JSON.parse(fs.readFileSync(configPath, "utf8")).app.security.csp;

describe("Content Security Policy", () => {
  it("the shipped configuration and served documents satisfy the policy rules", () => {
    expect(auditContentSecurityPolicy({ configPath, htmlPaths })).toEqual([]);
  });

  it("the shipped policy keeps Tauri IPC reachable", () => {
    const directives = parseCsp(shippedPolicy());
    expect(directives.get("connect-src")).toContain("ipc:");
    expect(directives.get("connect-src")).toContain("http://ipc.localhost");
  });

  it("rejects a disabled policy", () => {
    expect(auditCspPolicy(null)).toEqual([
      expect.stringContaining("no Content Security Policy is set"),
    ]);
  });

  it("rejects a policy that permits inline or evaluated script", () => {
    const weakened = shippedPolicy().replace(
      "script-src 'self'",
      "script-src 'self' 'unsafe-inline' 'unsafe-eval'",
    );
    expect(auditCspPolicy(weakened)).toEqual([
      expect.stringContaining("forbidden source 'unsafe-inline'"),
      expect.stringContaining("forbidden source 'unsafe-eval'"),
    ]);
  });

  it("rejects a policy that reaches a remote origin", () => {
    const weakened = shippedPolicy().replace(
      "default-src 'self'",
      "default-src 'self' https://cdn.example.com",
    );
    expect(auditCspPolicy(weakened)).toEqual([
      expect.stringContaining("unapproved source https://cdn.example.com"),
    ]);
  });

  it("rejects a policy that drops a required directive", () => {
    const weakened = shippedPolicy().replace("; object-src 'none'", "");
    expect(auditCspPolicy(weakened)).toEqual([
      expect.stringContaining("missing required directive 'object-src'"),
    ]);
  });

  it("rejects a development-only relaxation of the policy", () => {
    const security = {
      csp: shippedPolicy(),
      devCsp: "default-src *",
    };
    expect(auditSecurityConfig(security)).toEqual([
      expect.stringContaining("'devCsp' is set"),
    ]);
  });

  it("rejects disabling Tauri's own nonce and hash injection", () => {
    const security = {
      csp: shippedPolicy(),
      dangerousDisableAssetCspModification: true,
    };
    expect(auditSecurityConfig(security)).toEqual([
      expect.stringContaining("dangerousDisableAssetCspModification"),
    ]);
  });

  it("flags HTML the policy would block", () => {
    expect(auditHtmlDocument("probe.html", "<script>boot()</script>")).toEqual([
      expect.stringContaining("inline <script>"),
    ]);
    expect(
      auditHtmlDocument("probe.html", '<div onclick="boot()"></div>'),
    ).toEqual([expect.stringContaining("inline event handler")]);
    expect(
      auditHtmlDocument("probe.html", '<script src="https://cdn.example.com/a.js"></script>'),
    ).toEqual([expect.stringContaining("remote resource")]);
  });

  it("accepts the same-origin module script Vite emits", () => {
    expect(
      auditHtmlDocument(
        "probe.html",
        '<script type="module" crossorigin src="/assets/index-AQV_uPFL.js"></script>',
      ),
    ).toEqual([]);
  });
});
