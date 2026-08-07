/**
 * Product Intelligence Boundary (P16.38) — Intent / Conversation surface.
 *
 * Classifies what belongs inside Workspace vs outside.
 * Enforces production trust: Owner-facing text must not expose engineering substrate.
 * Deterministic only — no ML, no hidden AI.
 */

/** Permanent ownership buckets for remaining product gaps. */
export type IntelligenceOwner =
  | "P16"
  | "TrackA"
  | "P17"
  | "Conversation"
  | "CapabilityRegistry"
  | "WorkspaceContext"
  | "DesktopOperator"
  | "OperatingSystem"
  | "OutsideWorkspace";

export interface IntelligenceGap {
  id: string;
  topic: string;
  owner: IntelligenceOwner;
  note: string;
}

/**
 * Authoritative gap ownership for premium desktop operator maturity.
 * Evidence-backed classifications — not engineering preference.
 */
export const PRODUCT_INTELLIGENCE_GAPS: IntelligenceGap[] = [
  {
    id: "intention-routing",
    topic: "Goal-oriented situation phrasing (setup / lost / screenshots / back)",
    owner: "P16",
    note: "Situation Goals + Goal Resolution + Context — not alias tables",
  },
  {
    id: "registry-discovery",
    topic: "Can / cannot / why / required / similar / recovery from Registry",
    owner: "CapabilityRegistry",
    note: "Discovery must generate from CAPABILITY_GRAPH only",
  },
  {
    id: "session-continuity",
    topic: "Again / Back / Previous / Continue / Resume referents",
    owner: "WorkspaceContext",
    note: "Session continuity only — not full desktop layout memory",
  },
  {
    id: "window-enumeration",
    topic: "What windows are open right now",
    owner: "DesktopOperator",
    note: "winEnumerate through Kernel — truthful live list",
  },
  {
    id: "saved-moments",
    topic: "Full workspace / monitor / app-group restore",
    owner: "DesktopOperator",
    note: "Continue + Owner-approved Moments — never invent layouts",
  },
  {
    id: "fuzzy-launch-index",
    topic: "Fuzzy app/file index like PowerToys / Windows Search",
    owner: "OutsideWorkspace",
    note: "Commodity OS search — Workspace stays governed Conversation",
  },
  {
    id: "llm-memory",
    topic: "Long semantic chat memory (Claude / ChatGPT Desktop)",
    owner: "OutsideWorkspace",
    note: "Not a chat agent; Continuity is deterministic Context + Moments",
  },
  {
    id: "agent-loops",
    topic: "Autonomous multi-step agent IDE loops (Kiro-class)",
    owner: "OutsideWorkspace",
    note: "Kernel composition only; no hidden planner AI",
  },
  {
    id: "file-provider",
    topic: "Arbitrary filesystem browse / open / search",
    owner: "P17",
    note: "Blocked until P16 Owner acceptance",
  },
  {
    id: "tray-native-polish",
    topic: "System tray and deeper native chrome",
    owner: "TrackA",
    note: "Presentation debt — not Voice / Intent cognition",
  },
  {
    id: "os-layout-apis",
    topic: "Persistent OS virtual-desktop / full monitor layout APIs",
    owner: "OperatingSystem",
    note: "Study / wrap later — do not reinvent Windows shell",
  },
];

/** Engineering substrate that must never appear in Owner-facing replies. */
export const ENGINEERING_LEAK_PATTERN =
  /Provider|Registry|Kernel|WinRT|HRESULT|Intent Layer|grammar|parser|Capability graph|execution engine|Capability Runtime|IPC/i;

export function hasEngineeringLeak(text: string): boolean {
  return ENGINEERING_LEAK_PATTERN.test(text);
}

/** Goal-oriented families Owner expects (product experience, not command tables). */
export type ProductExperienceFamily =
  | "resume_intention"
  | "locate_clarify"
  | "screenshots_intention"
  | "discovery"
  | "continuity"
  | "awareness"
  | "recovery"
  | "open_action";

export function classifyProductExperience(
  utterance: string,
  kind: string,
): {
  family: ProductExperienceFamily;
  goalOriented: boolean;
} {
  const t = utterance
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "");

  if (
    /what can you|what can('|’)t|capabilities|everything you (can|know)|what else can you|what are my options|what('?s| is) similar/.test(
      t,
    )
  ) {
    return {
      family: "discovery",
      goalOriented: kind === "capabilityExplain",
    };
  }
  if (/screenshot/.test(t) && /looking|find|show|where|need/.test(t)) {
    return {
      family: "screenshots_intention",
      goalOriented: kind === "appLaunch" || kind === "appOpen",
    };
  }
  if (
    /development setup|working on something|everything back|where was i|take me where|i was coding|need my workspace/.test(
      t,
    )
  ) {
    return {
      family: "resume_intention",
      goalOriented: kind === "navigate",
    };
  }
  if (/lost it|looking for something|find it|where did it go/.test(t)) {
    return {
      family: "locate_clarify",
      goalOriented: kind === "unknown" || kind === "winFocus",
    };
  }
  if (
    /^(continue|resume|again|back|previous|go back|do that again|i'?m still working)$/.test(
      t,
    ) ||
    /continue what i was doing|bring everything back/.test(t)
  ) {
    return {
      family: "continuity",
      goalOriented:
        kind === "navigate" ||
        kind === "navigateNamed" ||
        kind === "appOpen" ||
        kind === "appLaunch" ||
        kind === "browserOpenBeside" ||
        kind === "browserOpen" ||
        kind === "winFocus" ||
        kind === "appClose" ||
        kind === "unknown",
    };
  }
  if (/what windows are open|what'?s on (my |the )?screen|what'?s open/.test(t)) {
    return {
      family: "awareness",
      goalOriented: kind === "winEnumerate",
    };
  }
  if (/can'?t|won'?t|not supported|invent/.test(t) || kind === "unknown") {
    return {
      family: "recovery",
      goalOriented: kind === "unknown" || kind === "capabilityExplain",
    };
  }
  return {
    family: "open_action",
    goalOriented: kind !== "proposal",
  };
}
