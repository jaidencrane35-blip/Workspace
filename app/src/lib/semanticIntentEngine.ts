/**
 * Semantic Intent Engine (P16.31) — Intent Layer authority.
 *
 * Grammar extracts Action / Target / Modifier / Object / Context.
 * This engine reasons over those components + a known desktop entity catalog
 * to produce IntentAction — never raw executable-name guessing.
 *
 * Deterministic (no ML). Providers never see this structure.
 */

import {
  generateCapabilityDiscovery,
  isCapabilityDiscoveryUtterance,
  resolveDiscoveryScope,
} from "./capabilityRegistry";
import {
  parseDesktopIntent,
  resolveShellFolder,
  type DesktopFollowUp,
  type DesktopIntent,
} from "./intentGrammar";
import type { IntentAction } from "./intentBridge"; // type-only — avoid runtime cycle

export type DesktopEntityKind =
  | "protocol"
  | "shell"
  | "site"
  | "browser"
  | "application"
  | "window_title";

export interface DesktopEntity {
  kind: DesktopEntityKind;
  /** Value passed to CapabilityIntent (URL, protocol, window title, app label). */
  value: string;
  /** Owner-facing label. */
  label: string;
  /** Window title hint for focus/locate (defaults to label). */
  focusQuery?: string;
  /** Application open/find query (defaults to value). */
  openQuery?: string;
  context: "browser" | "application" | "window" | "folder" | "unknown";
}

function normalizeKey(raw: string): string {
  return raw
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/,/g, " ")
    .replace(/\s+/g, " ")
    // Discourse tails — reason over the desktop goal, not filler words.
    .replace(/\s+(please|thanks|thank you|for me|just|now|today)$/i, "")
    .trim();
}

/** Known Windows / desktop entities — reasoned resolution, not phrase memorize-only. */
const KNOWN_ENTITIES: Array<{
  keys: string[];
  entity: DesktopEntity;
}> = [
  {
    keys: ["microsoft store", "ms store", "windows store", "store"],
    entity: {
      kind: "protocol",
      value: "ms-windows-store:",
      label: "Microsoft Store",
      context: "application",
    },
  },
  {
    keys: ["windows settings", "settings", "system settings"],
    entity: {
      kind: "protocol",
      value: "ms-settings:",
      label: "Windows Settings",
      context: "application",
    },
  },
  {
    keys: ["file explorer", "explorer", "this pc", "files"],
    entity: {
      kind: "application",
      value: "File Explorer",
      label: "File Explorer",
      context: "application",
    },
  },
  {
    keys: ["chatgpt", "chat gpt", "gpt", "g p t", "openai"],
    entity: {
      kind: "site",
      value: "https://chatgpt.com",
      label: "ChatGPT",
      context: "browser",
    },
  },
  {
    keys: ["youtube", "you tube", "yt", "y t"],
    entity: {
      kind: "site",
      value: "https://www.youtube.com",
      label: "YouTube",
      context: "browser",
    },
  },
  {
    keys: ["github", "git hub", "git"],
    entity: {
      kind: "site",
      value: "https://github.com",
      label: "GitHub",
      context: "browser",
    },
  },
  {
    keys: ["google"],
    entity: {
      kind: "site",
      value: "https://www.google.com",
      label: "Google",
      context: "browser",
    },
  },
  {
    keys: ["chrome", "google chrome"],
    entity: {
      kind: "browser",
      value: "Google Chrome",
      label: "Chrome",
      focusQuery: "Chrome",
      openQuery: "Google Chrome",
      context: "browser",
    },
  },
  {
    keys: ["edge", "microsoft edge", "msedge"],
    entity: {
      kind: "browser",
      value: "Microsoft Edge",
      label: "Edge",
      focusQuery: "Edge",
      openQuery: "Microsoft Edge",
      context: "browser",
    },
  },
  {
    keys: ["firefox"],
    entity: {
      kind: "browser",
      value: "Firefox",
      label: "Firefox",
      context: "browser",
    },
  },
  {
    keys: ["cursor", "cursor ide"],
    entity: {
      kind: "application",
      value: "Cursor",
      label: "Cursor",
      context: "application",
    },
  },
  {
    keys: ["vscode", "vs code", "visual studio code", "code editor"],
    entity: {
      kind: "application",
      value: "Visual Studio Code",
      label: "Visual Studio Code",
      context: "application",
    },
  },
  {
    keys: ["notepad"],
    entity: {
      kind: "application",
      value: "Notepad",
      label: "Notepad",
      context: "application",
    },
  },
  {
    keys: ["calculator", "calc"],
    entity: {
      kind: "application",
      value: "Calculator",
      label: "Calculator",
      context: "application",
    },
  },
  {
    keys: ["spotify"],
    entity: {
      kind: "application",
      value: "Spotify",
      label: "Spotify",
      openQuery: "Spotify",
      context: "application",
    },
  },
];

export function resolveDesktopEntity(raw: string): DesktopEntity | null {
  const key = normalizeKey(raw)
    .replace(/^(the|my|a|an)\s+/i, "")
    .replace(/\s+(app|application|window|program|browser|tab)$/i, "")
    .trim();
  if (!key) {
    return null;
  }
  for (const entry of KNOWN_ENTITIES) {
    if (entry.keys.includes(key)) {
      return entry.entity;
    }
  }
  const shell = resolveShellFolder(key);
  if (shell) {
    return {
      kind: "shell",
      value: shell,
      label: raw.trim(),
      context: "folder",
    };
  }
  return null;
}

/** Window-title hint for focus / maximize / restore (sites → product title). */
export function resolveWindowQuery(raw: string): string {
  const entity = resolveDesktopEntity(raw);
  if (!entity) {
    return raw
      .trim()
      .replace(/^(the|my|a|an)\s+/i, "")
      .replace(/\s+(app|application|window|program|browser|tab)$/i, "")
      .trim() || "this";
  }
  return entity.focusQuery ?? entity.label;
}

function followUpAction(
  followUp: DesktopFollowUp | undefined,
  query: string,
): IntentAction | null {
  switch (followUp) {
    case "minimize":
      return {
        kind: "winFocusMinimize",
        query,
        reply: `Looking for “${query}” and minimizing it.`,
      };
    case "maximize":
      return {
        kind: "winMaximize",
        query,
        reply: `Maximizing “${query}”.`,
      };
    case "restore":
      return {
        kind: "winRestore",
        query,
        reply: `Restoring “${query}”.`,
      };
    case "close":
      return {
        kind: "appClose",
        query,
        reply: `Closing “${query}”.`,
      };
    case "focus":
    case "none":
    case undefined:
      return {
        kind: "winFocus",
        query,
        reply: `Looking for “${query}”.`,
      };
    default:
      return null;
  }
}

function reasonFromGrammar(grammar: DesktopIntent): IntentAction | null {
  const targetRaw = grammar.target.trim();
  if (!targetRaw && grammar.action !== "unknown") {
    return null;
  }

  const entity = targetRaw ? resolveDesktopEntity(targetRaw) : null;
  const windowQuery = targetRaw ? resolveWindowQuery(targetRaw) : "this";

  // Locate / find windows (optionally composed with minimize/maximize/…)
  if (grammar.action === "locate") {
    if (
      grammar.followUp &&
      grammar.followUp !== "none" &&
      grammar.followUp !== "focus"
    ) {
      return followUpAction(grammar.followUp, windowQuery);
    }
    return {
      kind: "winFocus",
      query: windowQuery,
      reply:
        grammar.context === "browser"
          ? `Looking for a browser window with “${windowQuery}”.`
          : `Looking for “${windowQuery}”.`,
    };
  }

  if (grammar.action === "focus") {
    return {
      kind: "winFocus",
      query: windowQuery,
      reply: `Bringing “${windowQuery}” to the front.`,
    };
  }

  if (grammar.action === "maximize") {
    return {
      kind: "winMaximize",
      query: windowQuery,
      reply: `Maximizing “${windowQuery}”.`,
    };
  }

  if (grammar.action === "minimize") {
    return {
      kind: "winMinimize",
      query: windowQuery,
      reply: `Minimizing “${windowQuery}”.`,
    };
  }

  if (grammar.action === "restore") {
    return {
      kind: "winRestore",
      query: windowQuery,
      reply: `Restoring “${windowQuery}”.`,
    };
  }

  if (grammar.action === "close") {
    return {
      kind: "appClose",
      query: windowQuery,
      reply: `Closing “${windowQuery}”.`,
    };
  }

  if (grammar.modifier === "locate_object" && grammar.context === "folder") {
    const shell = resolveShellFolder(grammar.object);
    if (shell) {
      return {
        kind: "appLaunch",
        query: shell,
        reply: `Opening ${grammar.object} in File Explorer.`,
      };
    }
  }

  if (grammar.modifier === "beside" && grammar.secondaryTarget) {
    if (entity?.kind === "site") {
      return {
        kind: "browserOpenBeside",
        url: entity.value,
        beside: resolveWindowQuery(grammar.secondaryTarget),
        reply: `Opening ${entity.label} beside “${resolveWindowQuery(grammar.secondaryTarget)}”.`,
      };
    }
  }

  if (grammar.modifier === "foreground" && (grammar.action === "open" || grammar.action === "launch")) {
    if (entity?.kind === "site") {
      return {
        kind: "browserOpenFocus",
        url: entity.value,
        focusQuery: entity.label,
        reply: `Opening ${entity.label} and bringing it to the front.`,
      };
    }
    return {
      kind: "appOpen",
      query: windowQuery,
      reply: `Opening “${windowQuery}” and bringing it to the front.`,
    };
  }

  if (grammar.modifier === "fullscreen" && (grammar.action === "open" || grammar.action === "launch")) {
    if (entity?.kind === "site") {
      return {
        kind: "browserOpenFocus",
        url: entity.value,
        focusQuery: entity.label,
        reply: `Opening ${entity.label}.`,
      };
    }
    return {
      kind: "appOpenMaximize",
      query: windowQuery,
      reply: `Opening “${windowQuery}” full size.`,
    };
  }

  if (grammar.action === "open" || grammar.action === "launch") {
    if (!entity) {
      // Defer rich phrasing (tabs, notifications, window lists) to specialized resolvers.
      // Executable invent is blocked at appOpen/appLaunch fallthrough + Kernel launch_alias.
      return null;
    }
    if (entity.kind === "protocol" || entity.kind === "shell") {
      return {
        kind: "appLaunch",
        query: entity.value,
        reply: `Opening ${entity.label}.`,
      };
    }
    if (entity.kind === "site") {
      return {
        kind: "browserOpen",
        url: entity.value,
        reply: `Opening ${entity.label}.`,
      };
    }
    if (entity.kind === "browser" || entity.kind === "application") {
      return {
        kind: "appOpen",
        query: entity.openQuery ?? entity.value,
        reply: `Opening “${entity.label}”.`,
      };
    }
  }

  return null;
}

/**
 * Semantic Intent Engine entry — capability discovery + grammar reasoning.
 * Returns null when another Intent Layer resolver should continue.
 */
export function resolveSemanticIntent(raw: string): IntentAction | null {
  const text = normalizeKey(raw);
  if (!text) {
    return null;
  }

  if (isCapabilityDiscoveryUtterance(text)) {
    const discovery = generateCapabilityDiscovery(resolveDiscoveryScope(text));
    return {
      kind: "capabilityExplain",
      reply: discovery.reply,
      suggestion: discovery.suggestion,
    };
  }

  const grammar =
    parseDesktopIntent(raw.trim()) ?? parseDesktopIntent(text);
  if (grammar) {
    const reasoned = reasonFromGrammar(grammar);
    if (reasoned) {
      return reasoned;
    }
  }

  // Simple window verbs without compound grammar (Restore Cursor / Maximise Cursor / Focus Chrome)
  const simpleWindow =
    text.match(/^(maximi[sz]e|minimi[sz]e|restore|focus|bring)\s+(.+)$/i) ??
    text.match(
      /^(bring|put)\s+(.+?)\s+(to\s+(the\s+)?front|forward|in\s+front)$/i,
    );
  if (simpleWindow) {
    const verb = simpleWindow[1]?.toLowerCase() ?? "";
    let target = simpleWindow[2] ?? "";
    if (/^(bring|put)$/i.test(verb)) {
      target = target
        .replace(/\s+(to\s+(the\s+)?front|forward|in\s+front)$/i, "")
        .trim();
    }
    if (
      /^(all|every)\b/i.test(target) ||
      /\ball\s+(apps?|applications?|windows?)\b/i.test(target)
    ) {
      return null;
    }
    const query = resolveWindowQuery(target);
    if (/^maximi/i.test(verb)) {
      return {
        kind: "winMaximize",
        query,
        reply: `Maximizing “${query}”.`,
      };
    }
    if (/^minimi/i.test(verb)) {
      return {
        kind: "winMinimize",
        query,
        reply: `Minimizing “${query}”.`,
      };
    }
    if (/^restore/i.test(verb)) {
      return {
        kind: "winRestore",
        query,
        reply: `Restoring “${query}”.`,
      };
    }
    if (/^(focus|bring|put)$/i.test(verb)) {
      return {
        kind: "winFocus",
        query,
        reply: `Bringing “${query}” to the front.`,
      };
    }
  }

  // Open / launch / start known entities (Microsoft Store, etc.)
  const openKnown = text.match(/^(open|launch|start)\s+(.+)$/i);
  if (openKnown?.[2]) {
    const entity = resolveDesktopEntity(openKnown[2]);
    if (entity) {
      const reasoned = reasonFromGrammar({
        action: "open",
        target: openKnown[2],
        modifier: "none",
        object: "",
        context: entity.context,
        confidence: 0.93,
      });
      if (reasoned) {
        return reasoned;
      }
    }
  }

  return null;
}
