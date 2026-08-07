/**
 * Intent Grammar (P16.30 / P16.31) — permanent Intent Layer authority.
 *
 * Speech / typed text never reaches executable launching as a raw transcript.
 * Every desktop request is parsed into a structured DesktopIntent first.
 * Semantic Intent Engine then reasons over these components.
 *
 * Providers never see this structure. Kernel receives CapabilityIntent only.
 */

export type DesktopAction =
  | "open"
  | "launch"
  | "focus"
  | "maximize"
  | "minimize"
  | "restore"
  | "close"
  | "locate"
  | "capture"
  | "unknown";

export type DesktopModifier =
  | "foreground"
  | "fullscreen"
  | "beside"
  | "locate_object"
  | "none";

export type DesktopContext =
  | "browser"
  | "application"
  | "window"
  | "folder"
  | "screenshot"
  | "unknown";

export type DesktopFollowUp =
  | "minimize"
  | "maximize"
  | "restore"
  | "close"
  | "focus"
  | "none";

export interface DesktopIntent {
  action: DesktopAction;
  target: string;
  modifier: DesktopModifier;
  object: string;
  context: DesktopContext;
  /** Deterministic confidence 0–1 (grammar match strength, not ML). */
  confidence: number;
  /** Remaining clause for secondary routing (e.g. beside label). */
  secondaryTarget?: string;
  /** Composed follow-up after locate/focus (e.g. minimise it). */
  followUp?: DesktopFollowUp;
}

function stripPolite(text: string): string {
  return text
    .replace(/^(please|kindly)\s+/i, "")
    .replace(/^(could you|would you|can you|will you)\s+(please\s+)?/i, "")
    .replace(/\s+please[.!?]*$/i, "")
    .replace(/\s+for\s+me[.!?]*$/i, "")
    .replace(/[.!?]+$/g, "")
    .trim();
}

function parseFollowUp(raw: string): DesktopFollowUp | null {
  const t = raw.trim().toLowerCase();
  if (/^minimi[sz]e/.test(t)) return "minimize";
  if (/^maximi[sz]e/.test(t)) return "maximize";
  if (/^restore/.test(t)) return "restore";
  if (/^close/.test(t)) return "close";
  if (/^focus/.test(t)) return "focus";
  return null;
}

/**
 * Parse ordinary desktop language into a structured intent.
 * Returns null when no desktop grammar matched (caller continues other resolvers).
 */
export function parseDesktopIntent(raw: string): DesktopIntent | null {
  const text = stripPolite(raw.trim());
  if (!text) {
    return null;
  }

  // Locate X open and minimise/maximise/close/restore it
  const locateThen = text.match(
    /^(?:locate|find|search\s+for)\s+(?:the\s+)?(?:application|app|window|program|browser)?\s*(?:with|named|called)?\s*(.+?)\s+open\s+and\s+(minimi[sz]e|maximi[sz]e|restore|close|focus)\s*(?:it)?$/i,
  );
  if (locateThen?.[1] && locateThen[2]) {
    const followUp = parseFollowUp(locateThen[2]);
    const target = locateThen[1]
      .replace(/^(the\s+)?(application|app|window|program|browser)\s+/i, "")
      .trim();
    if (target && followUp) {
      return {
        action: "locate",
        target,
        modifier: "none",
        object: "",
        context: /browser/i.test(text) ? "browser" : "window",
        confidence: 0.96,
        followUp,
      };
    }
  }

  // Locate the browser with YouTube (open)
  const locateBrowser = text.match(
    /^(?:locate|find|search\s+for)\s+(?:the\s+)?browser\s+with\s+(.+?)(?:\s+open)?$/i,
  );
  if (locateBrowser?.[1]) {
    return {
      action: "locate",
      target: locateBrowser[1].trim(),
      modifier: "none",
      object: "",
      context: "browser",
      confidence: 0.94,
      followUp: "focus",
    };
  }

  // Open X and bring/put … to the front / forward
  const openAndFront = text.match(
    /^(?:open|launch|start)\s+(.+?)\s+and\s+(?:bring|put)(?:\s+it)?\s+(?:to\s+(?:the\s+)?front|forward)$/i,
  );
  if (openAndFront?.[1]) {
    return {
      action: "open",
      target: openAndFront[1].trim(),
      modifier: "foreground",
      object: "",
      context: "unknown",
      confidence: 0.95,
    };
  }

  // Open X to full size / maximized / maximize
  const openFull = text.match(
    /^(?:open|launch|start)\s+(.+?)\s+(?:to\s+)?(?:full\s*size|fullscreen|full\s*screen|maximized|maximised)$/i,
  );
  if (openFull?.[1]) {
    return {
      action: "open",
      target: openFull[1].trim(),
      modifier: "fullscreen",
      object: "",
      context: "application",
      confidence: 0.95,
    };
  }

  // Bring / put X to the front / forward / in front
  const bringFront = text.match(
    /^(?:bring|put)\s+(.+?)\s+(?:to\s+(?:the\s+)?front|forward|in\s+front)$/i,
  );
  if (bringFront?.[1]) {
    return {
      action: "focus",
      target: bringFront[1].trim(),
      modifier: "foreground",
      object: "",
      context: "window",
      confidence: 0.94,
    };
  }

  // Focus / activate / switch to X
  const focusOnly = text.match(/^(?:focus|activate|switch\s+to)\s+(.+)$/i);
  if (focusOnly?.[1]) {
    return {
      action: "focus",
      target: focusOnly[1].trim(),
      modifier: "none",
      object: "",
      context: "window",
      confidence: 0.93,
    };
  }

  // Maximise / Minimise / Restore X (single window — not bulk “all apps”)
  const stateVerb = text.match(
    /^(maximi[sz]e|minimi[sz]e|restore)\s+(.+)$/i,
  );
  if (stateVerb?.[1] && stateVerb[2]) {
    const target = stateVerb[2].trim();
    if (/^(all|every)\b/i.test(target) || /\ball\s+(apps?|applications?|windows?)\b/i.test(target)) {
      return null;
    }
    const follow = parseFollowUp(stateVerb[1]);
    const action: DesktopAction =
      follow === "minimize"
        ? "minimize"
        : follow === "maximize"
          ? "maximize"
          : follow === "restore"
            ? "restore"
            : "unknown";
    if (action !== "unknown") {
      return {
        action,
        target,
        modifier: "none",
        object: "",
        context: "window",
        confidence: 0.94,
      };
    }
  }

  // Locate / find / search for the application/window/app with|named X
  const locateApp = text.match(
    /^(?:locate|find|search\s+for)\s+(?:the\s+)?(?:application|app|window|program)?\s*(?:with|named|called|for)?\s*(.+)$/i,
  );
  if (locateApp?.[1]) {
    const target = locateApp[1]
      .replace(/^(the\s+)?(application|app|window|program)\s+/i, "")
      .replace(/\s+open$/i, "")
      .trim();
    if (target) {
      return {
        action: "locate",
        target,
        modifier: "none",
        object: "",
        context: "window",
        confidence: 0.9,
        followUp: "focus",
      };
    }
  }

  // Open File Explorer and locate/show/find Pictures — or “to Pictures”
  const explorerLocate = text.match(
    /^(?:open|launch|start)\s+(?:file\s+)?explorer\s+(?:and\s+(?:locate|find|show|open)|to)\s+(.+)$/i,
  );
  if (explorerLocate?.[1]) {
    return {
      action: "open",
      target: "File Explorer",
      modifier: "locate_object",
      object: explorerLocate[1].trim(),
      context: "folder",
      confidence: 0.92,
    };
  }

  // Locate / show / open known folder names (Downloads, Desktop, Pictures, …)
  const folderOnly = text.match(
    /^(?:locate|show|open|go\s+to|take\s+me\s+to)\s+(?:my\s+)?(pictures|documents|downloads|desktop|music|videos)(?:\s+folder)?$/i,
  );
  if (folderOnly?.[1]) {
    return {
      action: "open",
      target: "File Explorer",
      modifier: "locate_object",
      object: folderOnly[1].trim(),
      context: "folder",
      confidence: 0.93,
    };
  }

  // Open / put / place / I want X beside|next to|alongside Y — before bare “I want X”
  const beside = text.match(
    /^(?:open|launch|put|place|i\s+want)\s+(.+?)\s+(?:beside|next\s+to|alongside)\s+(.+)$/i,
  );
  if (beside?.[1] && beside[2]) {
    return {
      action: "open",
      target: beside[1].trim(),
      modifier: "beside",
      object: "",
      context: "unknown",
      confidence: 0.9,
      secondaryTarget: beside[2].trim(),
    };
  }

  // Goal phrasing for desktop entities only — do not steal “show me a notification / windows”.
  const goal = text.match(
    /^(?:take\s+me\s+to|go\s+to|i\s+want)\s+(.+)$/i,
  );
  if (goal?.[1]) {
    const target = goal[1].trim();
    if (
      !/\b(notification|windows?|screenshot|clipboard|capabilities|guide)\b/i.test(
        target,
      ) &&
      !/\b(beside|next\s+to|alongside)\b/i.test(target)
    ) {
      return {
        action: "open",
        target,
        modifier: "none",
        object: "",
        context: "unknown",
        confidence: 0.88,
      };
    }
  }

  // “show me X” only when X is a compact desktop target (not lists / notifications).
  const showMe = text.match(/^show\s+me\s+(.+)$/i);
  if (showMe?.[1]) {
    const target = showMe[1].trim();
    if (
      /^(my\s+)?(chatgpt|gpt|youtube|chrome|edge|cursor|store|settings|downloads|desktop|pictures|documents)(\s+folder)?$/i.test(
        target,
      )
    ) {
      return {
        action: "open",
        target,
        modifier: "none",
        object: "",
        context: "unknown",
        confidence: 0.9,
      };
    }
  }

  // Open / launch / start X (simple) — Semantic Engine resolves entity
  const openSimple = text.match(/^(?:open|launch|start)\s+(.+)$/i);
  if (openSimple?.[1]) {
    const target = openSimple[1].trim();
    // Leave compounds with "and" / "beside" to other patterns (already handled).
    if (!/\band\b|\bbeside\b/i.test(target)) {
      return {
        action: "open",
        target,
        modifier: "none",
        object: "",
        context: "unknown",
        confidence: 0.85,
      };
    }
  }

  return null;
}

/** Folders Windows Explorer can open via shell: URIs (deterministic). */
const SHELL_FOLDERS: Record<string, string> = {
  pictures: "shell:My Pictures",
  "my pictures": "shell:My Pictures",
  documents: "shell:Personal",
  "my documents": "shell:Personal",
  downloads: "shell:Downloads",
  desktop: "shell:Desktop",
  music: "shell:My Music",
  videos: "shell:My Video",
  home: "shell:UsersFilesFolder",
};

export function resolveShellFolder(object: string): string | null {
  const key = object.trim().toLowerCase().replace(/[.!?]+$/g, "");
  return SHELL_FOLDERS[key] ?? null;
}
