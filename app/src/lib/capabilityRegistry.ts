/**
 * Capability Registry (P16.31) — Intent Layer authority for discovery.
 *
 * Every Conversation-facing desktop capability advertises verbs, aliases,
 * objects, modifiers, requirements, and examples. Discovery replies are
 * generated from this graph — never hard-coded marketing copy.
 *
 * Providers never see this registry. Kernel receives CapabilityIntent only.
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
  requirements: string[];
  examples: string[];
}

/** Live capability graph for desktop operation through Conversation. */
export const CAPABILITY_GRAPH: CapabilityNode[] = [
  {
    id: "open-app",
    domain: "Applications",
    summary: "Open or bring forward desktop apps on this PC",
    verbs: ["open", "launch", "start"],
    aliases: ["run", "start up"],
    objects: ["apps", "Microsoft Store", "Cursor", "Notepad", "File Explorer"],
    modifiers: ["to the front", "full size", "beside"],
    requirements: ["Windows can find or launch the app"],
    examples: [
      "Open Microsoft Store",
      "Open Cursor to full size",
      "Open Notepad",
    ],
  },
  {
    id: "focus-window",
    domain: "Windows",
    summary: "Find windows and bring them forward",
    verbs: ["focus", "bring forward", "bring to the front", "locate", "show"],
    aliases: ["activate", "switch to", "put in front"],
    objects: ["Chrome", "Edge", "Cursor", "ChatGPT", "YouTube"],
    modifiers: ["browser with …", "application with …"],
    requirements: ["A matching window is open"],
    examples: [
      "Bring GPT to the front",
      "Focus Chrome",
      "Focus Edge",
      "Locate the browser with YouTube open",
    ],
  },
  {
    id: "window-state",
    domain: "Windows",
    summary: "Maximize, minimize, restore, snap, or move windows",
    verbs: ["maximize", "maximise", "minimize", "minimise", "restore", "snap", "center"],
    aliases: ["full size", "unminimize"],
    objects: ["Cursor", "Chrome", "this window"],
    modifiers: ["left", "right", "to monitor"],
    requirements: ["A matching window is open"],
    examples: [
      "Maximise Cursor",
      "Restore Cursor",
      "Minimize ChatGPT",
      "Snap Chrome left",
    ],
  },
  {
    id: "browser",
    domain: "Browser",
    summary: "Open websites and place them on the desktop",
    verbs: ["open", "visit", "go to"],
    aliases: ["browse", "launch site"],
    objects: ["ChatGPT", "YouTube", "GitHub", "Google"],
    modifiers: ["beside", "in a new tab", "and bring to the front"],
    requirements: ["A browser is available on this PC"],
    examples: [
      "Open ChatGPT",
      "Open YouTube beside Cursor",
      "Open GPT and bring it to the front",
    ],
  },
  {
    id: "screenshots",
    domain: "Screenshots",
    summary: "Capture the desktop, a window, or a monitor",
    verbs: ["capture", "take screenshot", "screenshot"],
    aliases: ["snap a picture of"],
    objects: ["desktop", "window", "monitor"],
    modifiers: ["and copy"],
    requirements: ["Screenshot capability available"],
    examples: ["Take a screenshot", "Capture this window"],
  },
  {
    id: "clipboard",
    domain: "Clipboard",
    summary: "Read or write the clipboard when you ask",
    verbs: ["read clipboard", "copy", "paste text"],
    aliases: ["what’s on the clipboard"],
    objects: ["clipboard text"],
    modifiers: [],
    requirements: ["Clipboard access allowed"],
    examples: ["What’s on my clipboard?", "Copy this text"],
  },
  {
    id: "notifications",
    domain: "Notifications",
    summary: "Show or dismiss desktop notifications",
    verbs: ["notify", "show notification", "dismiss"],
    aliases: ["toast", "alert me"],
    objects: ["notification"],
    modifiers: [],
    requirements: ["Notifications available"],
    examples: ["Show me a notification: Done"],
  },
  {
    id: "voice",
    domain: "Voice",
    summary: "Speak into Conversation with the microphone",
    verbs: ["speak", "dictate", "listen"],
    aliases: ["talk", "voice"],
    objects: ["microphone"],
    modifiers: ["review then Send"],
    requirements: ["Microphone and speech privacy allowed"],
    examples: ["Can you hear me?", "What can you do with voice?"],
  },
  {
    id: "folders",
    domain: "Folders",
    summary: "Open common folders in File Explorer",
    verbs: ["open", "locate", "show"],
    aliases: ["go to folder"],
    objects: ["Pictures", "Documents", "Downloads", "Desktop"],
    modifiers: ["in File Explorer"],
    requirements: ["File Explorer available"],
    examples: ["Open File Explorer and locate Pictures"],
  },
];

/** Discovery / meta utterances that should answer from the live graph. */
export function isCapabilityDiscoveryUtterance(text: string): boolean {
  const t = text.trim().toLowerCase().replace(/[.!?]+$/g, "");
  return (
    /^(what can you (do|help with)|what do you (do|help with))$/i.test(t) ||
    /^(show me your capabilities|list (your )?capabilities|list desktop actions|what are your capabilities|capabilities)$/i.test(
      t,
    ) ||
    /^(what can you do on (the )?desktop|desktop (help|capabilities)|help with (the )?desktop)$/i.test(
      t,
    )
  );
}

/**
 * Generate a Conversation reply from the live capability graph.
 * Ordinary language only — no Provider / Registry jargon.
 */
export function generateCapabilityDiscovery(): {
  reply: string;
  suggestion: string;
} {
  const lines = CAPABILITY_GRAPH.map(
    (node) => `${node.domain}: ${node.summary}. Example — “${node.examples[0]}”.`,
  );
  const reply = [
    "Here’s what I can do on this desktop through Conversation:",
    ...lines.map((line) => `• ${line}`),
    "Say what you want in ordinary words — I’ll turn it into desktop actions.",
  ].join("\n");

  const examples = CAPABILITY_GRAPH.flatMap((n) => n.examples).slice(0, 4);
  const suggestion = `Try “${examples.join("”, “")}”.`;

  return { reply, suggestion };
}
