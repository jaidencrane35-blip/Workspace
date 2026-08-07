/**
 * Intent Grammar (P16.30) — permanent Intent Layer authority.
 *
 * Speech / typed text never reaches executable launching as a raw transcript.
 * Every desktop request is parsed into a structured DesktopIntent first.
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

/**
 * Parse ordinary desktop language into a structured intent.
 * Returns null when no desktop grammar matched (caller continues other resolvers).
 */
export function parseDesktopIntent(raw: string): DesktopIntent | null {
  const text = stripPolite(raw.trim());
  if (!text) {
    return null;
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

  // Locate / find / search for the application/window/app with|named X
  const locateApp = text.match(
    /^(?:locate|find|search\s+for)\s+(?:the\s+)?(?:application|app|window|program)?\s*(?:with|named|called|for)?\s*(.+)$/i,
  );
  if (locateApp?.[1]) {
    const target = locateApp[1]
      .replace(/^(the\s+)?(application|app|window|program)\s+/i, "")
      .trim();
    if (target) {
      return {
        action: "locate",
        target,
        modifier: "none",
        object: "",
        context: "window",
        confidence: 0.9,
      };
    }
  }

  // Open File Explorer and locate/show/find Pictures (or other folder)
  const explorerLocate = text.match(
    /^(?:open|launch|start)\s+(?:file\s+)?explorer\s+and\s+(?:locate|find|show|open)\s+(.+)$/i,
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

  // Open X beside Y (handled elsewhere too — grammar records structure)
  const beside = text.match(
    /^(?:open|launch)\s+(.+?)\s+beside\s+(.+)$/i,
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
