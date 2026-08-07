/**
 * Capability Registry (P16.31–P16.34) — single source of truth for discovery.
 *
 * If a Conversation-facing desktop capability exists, it is declared here.
 * If it is not declared here, discovery must not claim it.
 *
 * Discovery / recovery / related guidance are generated only from this graph.
 * Providers never see this registry.
 */

export interface CapabilityNode {
  id: string;
  /** User-facing domain label (not Provider jargon). */
  domain: string;
  summary: string;
  verbs: string[];
  aliases: string[];
  objects: string[];
  modifiers: string[];
  /** Supported argument shapes (Owner-facing). */
  arguments: string[];
  requirements: string[];
  limitations: string[];
  examples: string[];
  /** How Conversation recovers when this capability cannot complete. */
  failureRecovery: string;
  /** Related capability ids in this graph (relationships). */
  related: string[];
  /** Similar capability ids (overlapping purpose). */
  similar: string[];
  /** Alternative capability ids when this one cannot apply. */
  alternatives: string[];
  /** How an ordinary user discovers this without menus. */
  discoverability: string;
  /** Short Owner-facing note used only when generating discovery text. */
  documentation: string;
}

/** Live capability graph for desktop operation through Conversation. */
export const CAPABILITY_GRAPH: CapabilityNode[] = [
  {
    id: "open-app",
    domain: "Applications",
    summary: "Open or bring forward desktop apps on this PC",
    verbs: ["open", "launch", "start"],
    aliases: ["run", "start up", "take me to"],
    objects: ["apps", "Microsoft Store", "Cursor", "Notepad", "File Explorer"],
    modifiers: ["to the front", "full size", "beside"],
    arguments: ["app name", "optional layout (full size / beside)"],
    requirements: ["Windows can find or launch the app"],
    limitations: [
      "Unknown app names are never invented as programs",
      "Store / Settings use Windows protocol handlers",
    ],
    examples: [
      "Open Microsoft Store",
      "Open Cursor to full size",
      "Open Notepad",
    ],
    failureRecovery:
      "Say a known app name, or ask what applications I can control — I won’t invent a program.",
    related: ["focus-window", "browser", "folders"],
    similar: ["focus-window", "browser"],
    alternatives: ["focus-window", "folders"],
    discoverability: "Ask to open a named app, or ask what applications I can control.",
    documentation:
      "Resolved apps go through Find → Focus or Launch — never raw transcripts.",
  },
  {
    id: "focus-window",
    domain: "Windows",
    summary: "Find windows and bring them forward",
    verbs: ["focus", "bring forward", "bring to the front", "locate", "show", "switch to"],
    aliases: ["activate", "put in front", "take me to"],
    objects: ["Chrome", "Edge", "Cursor", "ChatGPT", "YouTube"],
    modifiers: ["browser with …", "application with …"],
    arguments: ["window title hint", "optional follow-up (minimize)"],
    requirements: ["A matching window is open"],
    limitations: ["Matches window titles — not deep browser-tab APIs"],
    examples: [
      "Bring GPT to the front",
      "Focus Chrome",
      "Focus Edge",
      "Locate the browser with YouTube open",
    ],
    failureRecovery:
      "Ask what windows are open, or name the app/site window you want — I won’t invent a window.",
    related: ["window-state", "browser", "open-app"],
    similar: ["open-app", "window-state"],
    alternatives: ["open-app", "browser"],
    discoverability: "Ask what windows are open, or say you’ve got an app somewhere.",
    documentation: "Locate/focus compose to Window focus (and optional minimise).",
  },
  {
    id: "window-state",
    domain: "Windows",
    summary: "Maximize, minimize, restore, snap, or move windows",
    verbs: ["maximize", "maximise", "minimize", "minimise", "restore", "snap", "center"],
    aliases: ["full size", "unminimize"],
    objects: ["Cursor", "Chrome", "this window"],
    modifiers: ["left", "right", "to monitor"],
    arguments: ["window target", "state or edge", "optional monitor number"],
    requirements: ["A matching window is open"],
    limitations: ["Bulk “minimise all apps” is not supported"],
    examples: [
      "Maximise Cursor",
      "Restore Cursor",
      "Minimize ChatGPT",
      "Snap Chrome left",
    ],
    failureRecovery:
      "Name the window and the change (maximize, snap left, other monitor) — or ask what windows are open.",
    related: ["focus-window", "browser"],
    similar: ["focus-window"],
    alternatives: ["focus-window"],
    discoverability: "Name a window and a change — maximize, snap, or other monitor.",
    documentation: "Window state changes apply only to matching open windows.",
  },
  {
    id: "browser",
    domain: "Browser",
    summary: "Open websites and place them on the desktop",
    verbs: ["open", "visit", "go to"],
    aliases: ["browse", "launch site", "take me to"],
    objects: ["ChatGPT", "YouTube", "GitHub", "Google"],
    modifiers: ["beside", "in a new tab", "and bring to the front"],
    arguments: ["site name or URL", "optional beside target", "optional focus"],
    requirements: ["A browser is available on this PC"],
    limitations: ["Site names resolve to known URLs — not arbitrary search"],
    examples: [
      "Open ChatGPT",
      "Open YouTube beside Cursor",
      "Open GPT and bring it to the front",
    ],
    failureRecovery:
      "Name a known site (ChatGPT, YouTube, GitHub) or ask what I can do with browsers.",
    related: ["focus-window", "window-state", "open-app"],
    similar: ["open-app", "focus-window"],
    alternatives: ["focus-window", "open-app"],
    discoverability: "Ask to open a site, or put a site beside an app.",
    documentation: "Browser open may compose with Window focus or snap.",
  },
  {
    id: "screenshots",
    domain: "Screenshots",
    summary: "Capture the desktop, a window, or a monitor",
    verbs: ["capture", "take screenshot", "screenshot"],
    aliases: ["snap a picture of"],
    objects: ["desktop", "window", "monitor"],
    modifiers: ["and copy"],
    arguments: ["capture target (desktop / window / monitor index)"],
    requirements: ["Screenshot capability available"],
    limitations: ["Does not edit images after capture"],
    examples: ["Take a screenshot", "Capture this window"],
    failureRecovery:
      "Try “take a screenshot” or “capture this window”. Finding old screenshot files uses Folders (Pictures).",
    related: ["folders", "clipboard"],
    similar: ["folders", "clipboard"],
    alternatives: ["folders"],
    discoverability: "Ask to take a screenshot, or to find your screenshots folder.",
    documentation: "Screenshot capture can compose with copy when you ask.",
  },
  {
    id: "clipboard",
    domain: "Clipboard",
    summary: "Read or write the clipboard when you ask",
    verbs: ["read clipboard", "copy", "paste text"],
    aliases: ["what’s on the clipboard"],
    objects: ["clipboard text"],
    modifiers: [],
    arguments: ["optional text to copy"],
    requirements: ["Clipboard access allowed"],
    limitations: ["Does not scrape arbitrary app UIs"],
    examples: ["What’s on my clipboard?", "Copy this text"],
    failureRecovery: "Ask what’s on the clipboard, or tell me the text to copy.",
    related: ["screenshots"],
    similar: ["screenshots"],
    alternatives: ["screenshots"],
    discoverability: "Ask what’s on the clipboard.",
    documentation: "Clipboard is a first-class capability domain.",
  },
  {
    id: "notifications",
    domain: "Notifications",
    summary: "Show or dismiss desktop notifications",
    verbs: ["notify", "show notification", "dismiss"],
    aliases: ["toast", "alert me"],
    objects: ["notification"],
    modifiers: [],
    arguments: ["notification text", "optional title"],
    requirements: ["Notifications available"],
    limitations: ["Does not replace the Windows Action Center"],
    examples: ["Show me a notification: Done"],
    failureRecovery: "Say “show me a notification: …” with the message text.",
    related: [],
    similar: [],
    alternatives: [],
    discoverability: "Ask to show a notification with your message text.",
    documentation: "Notifications are shown under Workspace governance.",
  },
  {
    id: "voice",
    domain: "Voice",
    summary: "Speak into Conversation with the microphone",
    verbs: ["speak", "dictate", "listen"],
    aliases: ["talk", "voice"],
    objects: ["microphone"],
    modifiers: ["review then Send"],
    arguments: ["spoken words → same Intent path as typing"],
    requirements: ["Microphone and speech privacy allowed"],
    limitations: ["Recognition quality follows Windows dictation"],
    examples: ["Can you hear me?", "What can you do with voice?"],
    failureRecovery:
      "Use the microphone, review the transcript, then Send — or type the same request.",
    related: [],
    similar: [],
    alternatives: [],
    discoverability: "Use the microphone beside Conversation, then Send.",
    documentation:
      "Voice is an input device — transcript follows the same Intent path as typing.",
  },
  {
    id: "folders",
    domain: "Folders",
    summary: "Open common folders in File Explorer",
    verbs: ["open", "locate", "show"],
    aliases: ["go to folder", "take me to"],
    objects: ["Pictures", "Documents", "Downloads", "Desktop"],
    modifiers: ["in File Explorer", "to folder"],
    arguments: ["known folder name (Pictures, Downloads, …)"],
    requirements: ["File Explorer available"],
    limitations: [
      "Uses known shell folders — not arbitrary paths",
      "Cannot filter folder contents by date from Conversation",
    ],
    examples: [
      "Open File Explorer and locate Pictures",
      "Locate Downloads",
      "Show my Desktop",
    ],
    failureRecovery:
      "Name a common folder (Downloads, Pictures, Desktop, Documents) — I won’t invent paths.",
    related: ["open-app", "screenshots"],
    similar: ["open-app", "screenshots"],
    alternatives: ["open-app"],
    discoverability: "Ask for Downloads, Pictures, Desktop, or Documents.",
    documentation: "Folders open via Windows shell: URIs.",
  },
];

export type DiscoveryScope =
  | "all"
  | "windows"
  | "applications"
  | "browser"
  | "desktop";

function normalizeDiscoveryText(text: string): string {
  return text.trim().toLowerCase().replace(/[.!?]+$/g, "");
}

/** Discovery / meta utterances that must answer from the live graph. */
export function isCapabilityDiscoveryUtterance(text: string): boolean {
  const t = normalizeDiscoveryText(text);
  // Voice / browser product-scoped explain stay on specialized Intent paths.
  if (/\bwith (voice|browsers?)\b/i.test(t)) {
    return false;
  }
  return (
    /^(what can you (do|help with)|what do you (do|help with)|how can you help( me)?)$/i.test(
      t,
    ) ||
    /^(show( me)?( your)? capabilities|list (your )?capabilities|list desktop (actions|commands)|what are your capabilities|capabilities)$/i.test(
      t,
    ) ||
    /^(show me everything you can (control|do)|show me everything you know how to control|what can you control|everything you can (control|do)|everything you know how to control)$/i.test(
      t,
    ) ||
    /^(tell me everything you (can|know how to) (control|do))$/i.test(t) ||
    /^(what (desktop )?tasks can you perform|what can you do on (the )?desktop|desktop (help|capabilities)|help with (the )?desktop)$/i.test(
      t,
    ) ||
    /^(what do you know about windows?|what (can you do|do you do) with windows?)$/i.test(
      t,
    ) ||
    /^(what applications can you (control|open|launch)|what apps can you (control|open|launch))$/i.test(
      t,
    ) ||
    /^(what (can you do|do you do) with (folders?|screenshots?|clipboard|notifications?))$/i.test(
      t,
    ) ||
    /^(what can('|’)t you do|what can you not do|what are your limits)$/i.test(t)
  );
}

export function resolveDiscoveryScope(text: string): DiscoveryScope {
  const t = normalizeDiscoveryText(text);
  if (/\bwindows?\b/.test(t) && !/\b(desktop|application)/.test(t)) {
    return "windows";
  }
  if (/\b(applications?|apps?)\b/.test(t)) {
    return "applications";
  }
  if (/\bbrowsers?\b/.test(t)) {
    return "browser";
  }
  if (/\bdesktop\b/.test(t)) {
    return "desktop";
  }
  return "all";
}

function nodesForScope(scope: DiscoveryScope): CapabilityNode[] {
  switch (scope) {
    case "windows":
      return CAPABILITY_GRAPH.filter((n) => n.domain === "Windows");
    case "applications":
      return CAPABILITY_GRAPH.filter((n) => n.domain === "Applications");
    case "browser":
      return CAPABILITY_GRAPH.filter((n) => n.domain === "Browser");
    case "desktop":
      return CAPABILITY_GRAPH.filter((n) =>
        ["Applications", "Windows", "Browser", "Folders", "Screenshots"].includes(
          n.domain,
        ),
      );
    default:
      return CAPABILITY_GRAPH;
  }
}

export function getCapabilityById(id: string): CapabilityNode | undefined {
  return CAPABILITY_GRAPH.find((n) => n.id === id);
}

/** Full self-description of one capability — Registry only. */
export function describeCapability(id: string): string | null {
  const node = getCapabilityById(id);
  if (!node) {
    return null;
  }
  const related = node.related
    .map((rid) => getCapabilityById(rid)?.domain)
    .filter(Boolean)
    .join(", ");
  const similar = node.similar
    .map((rid) => getCapabilityById(rid)?.domain)
    .filter(Boolean)
    .join(", ");
  const alternatives = node.alternatives
    .map((rid) => getCapabilityById(rid)?.domain)
    .filter(Boolean)
    .join(", ");
  return [
    `${node.domain}: ${node.summary}`,
    `Purpose: ${node.summary}`,
    `Does: ${node.documentation}`,
    `Arguments: ${node.arguments.join("; ") || "none"}`,
    `Needs: ${node.requirements.join("; ")}`,
    `Won’t: ${node.limitations.join("; ")}`,
    `If it fails: ${node.failureRecovery}`,
    `Discover: ${node.discoverability}`,
    related ? `Related: ${related}` : "",
    similar ? `Similar: ${similar}` : "",
    alternatives ? `Alternatives: ${alternatives}` : "",
    `Example — “${node.examples[0] ?? node.summary}”`,
  ]
    .filter(Boolean)
    .join(" ");
}

/**
 * Generate a Conversation reply from the live capability graph only.
 * Self-describing: can / cannot / why / arguments / recovery / related.
 */
export function generateCapabilityDiscovery(scope: DiscoveryScope = "all"): {
  reply: string;
  suggestion: string;
} {
  const nodes = nodesForScope(scope);
  if (nodes.length === 0) {
    return {
      reply: "I don’t have a matching capability to describe for that yet.",
      suggestion:
        nodesForScope("all")[0]?.examples[0]
          ? `Try “${nodesForScope("all")[0]!.examples[0]}”.`
          : "Ask for a desktop action in ordinary words.",
    };
  }

  const canLines = nodes.map((node) => {
    const example = node.examples[0] ?? node.summary;
    const args = node.arguments[0] ? ` Args — ${node.arguments[0]}.` : "";
    const need = node.requirements[0] ? ` Needs — ${node.requirements[0]}.` : "";
    return `• ${node.domain}: ${node.summary}. Example — “${example}”.${args}${need}`;
  });

  const cannotLines = nodes.flatMap((node) =>
    node.limitations.map((limit) => `• ${node.domain}: ${limit}`),
  );

  const recoveryLines = nodes
    .slice(0, 4)
    .map((node) => `• ${node.domain}: ${node.failureRecovery}`);

  const why =
    "I only claim actions the Capability graph declares — I won’t invent apps, folders, or success.";

  const reply = [
    scope === "all"
      ? "Here’s what I can control on this desktop through Conversation:"
      : `Here’s what I can control for ${scope}:`,
    ...canLines,
    cannotLines.length > 0 ? "What I won’t overclaim:" : "",
    ...cannotLines.slice(0, 8),
    recoveryLines.length > 0 ? "If something doesn’t work:" : "",
    ...recoveryLines,
    `Why: ${why}`,
    "Say what you want in ordinary words — I’ll plan desktop actions before I run them.",
  ]
    .filter((line) => line !== "")
    .join("\n");

  const examples = nodes.flatMap((n) => n.examples).slice(0, 4);
  const suggestion =
    examples.length > 0
      ? `Try “${examples.join("”, “")}”.`
      : "Try asking for a desktop action in ordinary words.";

  return { reply, suggestion };
}

/** Nearby supported examples from the live graph (truthful recovery). */
export function suggestNearbyCapabilities(seed: string, limit = 3): string {
  const key = seed.trim().toLowerCase();
  const scored = CAPABILITY_GRAPH.flatMap((node) =>
    node.examples.map((example) => {
      const e = example.toLowerCase();
      let score = 0;
      if (key && e.includes(key.slice(0, Math.min(6, key.length)))) score += 2;
      if (node.objects.some((o) => o.toLowerCase().includes(key))) score += 3;
      if (node.verbs.some((v) => key.includes(v))) score += 1;
      return { example, score, node };
    }),
  )
    .sort((a, b) => b.score - a.score);

  const unique = [...new Set(scored.map((x) => x.example))].slice(0, limit);
  if (unique.length === 0) {
    return CAPABILITY_GRAPH.flatMap((n) => n.examples).slice(0, limit).join("”, “");
  }
  return unique.join("”, “");
}

function bestMatchingNode(seed: string): CapabilityNode | null {
  const key = seed.trim().toLowerCase();
  if (!key) {
    return null;
  }
  let best: { node: CapabilityNode; score: number } | null = null;
  for (const node of CAPABILITY_GRAPH) {
    let score = 0;
    if (node.objects.some((o) => key.includes(o.toLowerCase()) || o.toLowerCase().includes(key))) {
      score += 3;
    }
    if (node.verbs.some((v) => key.includes(v))) score += 2;
    if (node.domain.toLowerCase().includes(key) || key.includes(node.domain.toLowerCase())) {
      score += 2;
    }
    if (score > 0 && (!best || score > best.score)) {
      best = { node, score };
    }
  }
  return best?.node ?? null;
}

/** Truthful recovery reply built only from registry recovery + nearby examples. */
export function generateRecoveryGuidance(seed: string): {
  reply: string;
  suggestion: string;
} {
  const nearby = suggestNearbyCapabilities(seed, 3);
  const matched = bestMatchingNode(seed);
  const recovery =
    matched?.failureRecovery ??
    "Ask what I can do, or name a desktop action in ordinary words.";
  const related = matched?.related
    .map((id) => getCapabilityById(id)?.examples[0])
    .filter(Boolean)
    .slice(0, 2)
    .join("”, “");

  const reply = [
    `I can’t do that exactly as asked — and I won’t invent a desktop action.`,
    `Recovery: ${recovery}`,
    related ? `Related I can try: “${related}”.` : "",
    nearby && !related ? `Closest things I can try: “${nearby}”.` : "",
  ]
    .filter(Boolean)
    .join(" ");
  return {
    reply,
    suggestion: nearby ? `Try “${nearby}”.` : "Ask “what can you do?”",
  };
}
