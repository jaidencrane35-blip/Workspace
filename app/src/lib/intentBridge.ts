/**
 * Deterministic intent bridge — no model reasoning.
 * Maps short natural phrases to existing Product Proof surfaces / shell modes /
 * developer health / capability-evolution proposals.
 */

import {
  createProposal,
  formatBacklogReply,
  formatProposalReply,
  isEvolutionRequest,
  latestProposed,
  loadEvolutionState,
  parseApprovalIntent,
  saveEvolutionState,
  setProposalStatus,
  undoLastStatusChange,
  appendProposal,
} from "./capabilityEvolution";
import {
  companionGreetingReply,
  isVoiceCheckUtterance,
  resolveUnknownGuidance,
  softenUtterance,
  voiceCheckReply,
} from "./conversationGuidance";
import type { PilotPrimaryView } from "./pilotChrome";
import { suggestNearbyCapabilities } from "./capabilityRegistry";
import {
  resolveDesktopEntity,
  resolveSemanticIntent,
  resolveWindowQuery,
} from "./semanticIntentEngine";
import { applyGoalResolution } from "./goalResolution";
import {
  commitWorkspaceContext,
  resolveFromWorkspaceContext,
} from "./workspaceContext";
import {
  resolveCompoundOpen,
  type CompoundOpenTarget,
} from "./compoundOpen";
import { resolvePrepareCodingWorkspace } from "./prepareCodingWorkspace";
import { resolveIntelligenceRoute } from "./intelligenceRouting";

export type IntentAction =
  | { kind: "navigate"; view: PilotPrimaryView; reply: string }
  | {
      kind: "navigateNamed";
      view: "resume";
      nameQuery: string;
      reply: string;
    }
  | { kind: "saveAs"; name: string; reply: string }
  | { kind: "expand"; reply: string }
  | { kind: "collapse"; reply: string }
  | { kind: "settings"; reply: string }
  | { kind: "health"; reply: string }
  | { kind: "supportBundle"; reply: string }
  | { kind: "developer"; enabled: boolean; reply: string }
  | { kind: "proposal"; reply: string }
  | { kind: "clipboardRead"; reply: string }
  | { kind: "clipboardWrite"; text: string; reply: string }
  | { kind: "notifyStatus"; reply: string }
  | {
      kind: "notifyShow";
      title?: string;
      text: string;
      category?: string;
      priority?: string;
      duration?: string;
      reply: string;
    }
  | { kind: "notifyDismiss"; id?: string; reply: string }
  | { kind: "browserStatus"; reply: string }
  | { kind: "browserOpen"; url: string; reply: string }
  | {
      kind: "browserOpenFocus";
      url: string;
      focusQuery: string;
      reply: string;
    }
  | {
      kind: "browserOpenBeside";
      url: string;
      beside: string;
      reply: string;
    }
  | {
      kind: "compoundOpen";
      targets: CompoundOpenTarget[];
      encode: string;
      reply: string;
    }
  | {
      kind: "prepareCodingWorkspace";
      targets: CompoundOpenTarget[];
      encode: string;
      reply: string;
    }
  | { kind: "appOpenMaximize"; query: string; reply: string }
  | { kind: "browserExplain"; reply: string; suggestion?: string }
  | { kind: "capabilityExplain"; reply: string; suggestion?: string }
  | { kind: "winFocusMinimize"; query: string; reply: string }
  | { kind: "screenshotStatus"; reply: string }
  | { kind: "screenshotDesktop"; reply: string }
  | { kind: "screenshotWindow"; query: string; reply: string }
  | { kind: "screenshotMonitor"; monitorIndex: number; reply: string }
  | { kind: "screenshotSave"; reply: string }
  | { kind: "screenshotCopy"; path?: string; reply: string }
  | {
      kind: "screenshotCaptureAndCopy";
      query?: string;
      monitorIndex?: number;
      reply: string;
    }
  | { kind: "voiceStatus"; reply: string }
  | { kind: "voiceExplain"; reply: string; suggestion?: string }
  | { kind: "appOpen"; query: string; reply: string }
  | { kind: "appLaunch"; query: string; reply: string }
  | { kind: "appFocus"; query: string; reply: string }
  | { kind: "appClose"; query: string; reply: string }
  | { kind: "appMinimize"; query: string; reply: string }
  | { kind: "appRestore"; query: string; reply: string }
  | { kind: "appEnumerate"; reply: string }
  | { kind: "winEnumerate"; reply: string }
  | { kind: "winEnumerateControls"; query: string; reply: string }
  | { kind: "winFindControl"; control: string; query: string; reply: string }
  | { kind: "winClickControl"; control: string; query: string; reply: string }
  | {
      kind: "winTypeControl";
      control: string;
      query: string;
      text: string;
      reply: string;
    }
  | {
      kind: "winWaitCondition";
      condition:
        | "control_available"
        | "control_gone"
        | "window_available"
        | "window_active";
      control?: string;
      query: string;
      duration?: string;
      reply: string;
    }
  | { kind: "winActive"; reply: string }
  | { kind: "winMonitors"; reply: string }
  | { kind: "winBounds"; query: string; reply: string }
  | { kind: "winMaximize"; query: string; reply: string }
  | { kind: "winMinimize"; query: string; reply: string }
  | { kind: "winRestore"; query: string; reply: string }
  | { kind: "winSnap"; query: string; snap: string; reply: string }
  | { kind: "winCenter"; query: string; reply: string }
  | { kind: "winMoveMonitor"; query: string; monitorIndex: number; reply: string }
  | { kind: "winFocus"; query: string; reply: string }
  | {
      kind: "winResize";
      query: string;
      width: number;
      height: number;
      reply: string;
    }
  | { kind: "unknown"; reply: string; suggestion?: string };

function normalize(input: string): string {
  return input
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/,/g, " ")
    .replace(/\s+/g, " ");
}

/** Match against original or politeness-softened wording (deterministic). */
function matchFirst(
  raw: string,
  softRaw: string,
  pattern: RegExp,
): RegExpMatchArray | null {
  return raw.trim().match(pattern) ?? softRaw.trim().match(pattern);
}

function stripTrailingPunctuation(value: string): string {
  return value.trim().replace(/[.!?]+$/g, "").trim();
}

/** Map spoken monitor references to 1-based indices used by Window Provider. */
function parseMonitorIndex(token: string): number | null {
  const t = token.trim().toLowerCase();
  if (/^\d+$/.test(t)) {
    return Number(t);
  }
  const words: Record<string, number> = {
    one: 1,
    first: 1,
    two: 2,
    second: 2,
    three: 3,
    third: 3,
    four: 4,
    fourth: 4,
  };
  return words[t] ?? null;
}

function windowTarget(rawQuery: string | undefined): string {
  const q = stripTrailingPunctuation(rawQuery ?? "");
  if (!q || /^(this|it|the|active|current|foreground)(\s+window)?$/i.test(q)) {
    return "this";
  }
  const cleaned = q.replace(/^(the|my)\s+/i, "").trim();
  // Semantic focus queries (GPT → ChatGPT, Chrome → Chrome — not spaced .exe names).
  return resolveWindowQuery(cleaned) || cleaned || "this";
}

/**
 * Voice Input intents — status / help only (recognition is the mic button).
 * Voice never executes desktop operations itself.
 */
function resolveVoiceIntent(_raw: string, text: string): IntentAction | null {
  if (
    /\b(what can you do with voice|voice help|how do (i|you) use voice|voice input help|can i talk to you|can i speak to you)\b/.test(
      text,
    )
  ) {
    return {
      kind: "voiceExplain",
      reply:
        "You can speak to Workspace with the microphone beside the message box. I’ll put what I hear into Conversation — same as typing — then act on it.",
      suggestion:
        'Try the mic, then say “open chatgpt”, “take a screenshot”, or “bring chrome to the front”.',
    };
  }

  // Soften leaves “could you hear me?” → “hear me” — match both raw and softened (P16.25).
  if (
    /\b(can you (use |hear )?voice|is voice available|voice support|microphone (available|working)|((can|could|would|do) you hear me)|are you listening|do you support voice)\b/.test(
      text,
    ) ||
    text === "hear me" ||
    text === "voice?" ||
    text === "voice" ||
    text === "microphone?"
  ) {
    return {
      kind: "voiceStatus",
      reply: "Checking whether voice input is available.",
    };
  }

  return null;
}

/**
 * Screenshot intents (Conversation language → desktop capture operations).
 * Levels 1–2 only — no OCR / recording / annotation.
 */
function resolveScreenshotIntent(raw: string, text: string): IntentAction | null {
  // Leave capability-evolution phrasing alone (“add a screenshot button”).
  if (
    /\b(add|propose|implement|build)\b/.test(text) &&
    /\b(screenshot|button|feature|capability)\b/.test(text)
  ) {
    return null;
  }

  if (
    /\b(can you take screenshots?|are screenshots? available|screenshot support|do (you|we) support screenshots?|can you (capture|screenshot) (my )?(screen|desktop|window))\b/.test(
      text,
    ) ||
    text === "screenshots?" ||
    text === "screenshots" ||
    text === "screenshot?"
  ) {
    return {
      kind: "screenshotStatus",
      reply: "Checking whether I can take screenshots on this PC.",
    };
  }

  const andCopy =
    /\b(and copy( it| (to )?(the )?clipboard)?|copy (it|this|the screenshot))\b/.test(
      text,
    ) || /\bcopy this screenshot\b/.test(text);

  const monitorMatch = text.match(
    /\b(?:screenshot|capture|take a screenshot of)\s+(?:monitor|display)\s+(\w+)\b/,
  );
  const monitorAlt = text.match(
    /\b(?:monitor|display)\s+(\w+)\b.+\b(screenshot|capture)\b/,
  );
  const monitorToken = monitorMatch?.[1] ?? monitorAlt?.[1];
  if (monitorToken) {
    const idx = parseMonitorIndex(monitorToken);
    if (idx == null) {
      return {
        kind: "unknown",
        reply: `I couldn’t tell which monitor you meant by “${monitorToken}”. Try “screenshot monitor 1” or “screenshot monitor 2”.`,
      };
    }
    if (andCopy) {
      return {
        kind: "screenshotCaptureAndCopy",
        monitorIndex: idx,
        reply: `Capturing monitor ${idx} and copying it.`,
      };
    }
    return {
      kind: "screenshotMonitor",
      monitorIndex: idx,
      reply: `Capturing monitor ${idx}.`,
    };
  }

  if (
    /\b(copy (this |the )?(screenshot|capture)|copy (it|that) to (the )?clipboard)\b/.test(
      text,
    ) &&
    !/\b(take|capture|screenshot)\b.+\b(and copy|copy)\b/.test(text) &&
    !/\b(take|capture)\b/.test(text)
  ) {
    return {
      kind: "screenshotCopy",
      reply: "Copying the latest screenshot to the clipboard.",
    };
  }

  if (
    /\b(save (a |the )?screenshot|save (this |the )?(screen|capture))\b/.test(
      text,
    )
  ) {
    return {
      kind: "screenshotSave",
      reply: "Saving a screenshot as a PNG.",
    };
  }

  if (
    /\b(screenshot|capture|take a screenshot of)\s+(this|the|primary)(\s+)?monitor\b/.test(
      text,
    ) ||
    /\b(this|the|primary)\s+monitor\b/.test(text) &&
      /\b(screenshot|capture)\b/.test(text)
  ) {
    if (andCopy) {
      return {
        kind: "screenshotCaptureAndCopy",
        monitorIndex: 1,
        reply: "Capturing monitor 1 and copying it.",
      };
    }
    return {
      kind: "screenshotMonitor",
      monitorIndex: 1,
      reply: "Capturing monitor 1.",
    };
  }

  const thisWindow =
    /\b(screenshot|capture|take a screenshot of)\s+(this|the active|the current|the foreground)(\s+window)?\b/.test(
      text,
    ) ||
    (/\b(screenshot|capture)\s+this\b/.test(text) &&
      !/\bmonitor\b/.test(text)) ||
    text === "screenshot this" ||
    text === "screenshot this window" ||
    text === "capture this window" ||
    text === "capture this";

  if (thisWindow) {
    if (andCopy) {
      return {
        kind: "screenshotCaptureAndCopy",
        query: "this",
        reply: "Capturing this window and copying it.",
      };
    }
    return {
      kind: "screenshotWindow",
      query: "this",
      reply: "Capturing this window.",
    };
  }

  const namedWindow = text.match(
    /^(?:take a screenshot of|screenshot|capture(?: a screenshot of)?)\s+(.+)$/i,
  );
  if (namedWindow?.[1]) {
    let target = stripTrailingPunctuation(namedWindow[1]);
    target = target
      .replace(/\s+and copy( it| to( the)? clipboard)?$/i, "")
      .replace(/^(the|my)\s+/i, "")
      .trim();
    if (
      /^(desktop|screen|my desktop|my screen|the desktop|the screen)$/i.test(
        target,
      )
    ) {
      if (andCopy) {
        return {
          kind: "screenshotCaptureAndCopy",
          reply: "Capturing your desktop and copying it.",
        };
      }
      return {
        kind: "screenshotDesktop",
        reply: "Capturing your desktop.",
      };
    }
    if (/^(this monitor|the monitor|primary monitor)$/i.test(target)) {
      return {
        kind: "screenshotMonitor",
        monitorIndex: 1,
        reply: "Capturing monitor 1.",
      };
    }
    if (!target || /^(a screenshot|screenshot|screen)$/i.test(target)) {
      // fall through to desktop
    } else {
      if (andCopy) {
        return {
          kind: "screenshotCaptureAndCopy",
          query: target,
          reply: `Capturing “${target}” and copying it.`,
        };
      }
      return {
        kind: "screenshotWindow",
        query: target,
        reply: `Capturing “${target}”.`,
      };
    }
  }

  if (
    /^(take a screenshot|take screenshot|screenshot|grab a screenshot|snap a screenshot|capture my desktop|capture the desktop|capture my screen|capture the screen|capture screen|screen capture)$/i.test(
      text,
    ) ||
    /\b(take a screenshot|grab a screenshot|snap a screenshot|capture my (desktop|screen))\b/.test(
      text,
    )
  ) {
    if (andCopy) {
      return {
        kind: "screenshotCaptureAndCopy",
        reply: "Capturing your desktop and copying it.",
      };
    }
    return {
      kind: "screenshotDesktop",
      reply: "Capturing your desktop.",
    };
  }

  // Bare “screenshot …” leftovers with punctuation/spacing tolerance
  if (/^screenshot\b/.test(text) || /^capture\b/.test(text)) {
    const rest = stripTrailingPunctuation(
      raw.replace(/^(screenshot|capture)\s+/i, ""),
    ).trim();
    if (!rest || /^(please|now)$/i.test(rest)) {
      return {
        kind: "screenshotDesktop",
        reply: "Capturing your desktop.",
      };
    }
  }

  return null;
}

/**
 * Notifications intents (Conversation language → desktop notification operations).
 * Deferred “when X finishes” watching is out of Level 1–2 scope — clarify truthfully.
 */
function resolveNotificationIntent(raw: string, text: string): IntentAction | null {
  if (
    /\b(notify me when|tell me when|let me know when)\b/.test(text) ||
    /\bwhen\b.+\b(finishes|finished|completes|completed|is done|done)\b/.test(
      text,
    )
  ) {
    return {
      kind: "unknown",
      reply:
        "I can show a desktop notification now. Watching for when something finishes isn’t available yet.",
      suggestion: "Tell me the notification text if you want one now.",
    };
  }

  if (
    /\b(can you (send |show )?(me )?(a )?notifications?|are notifications available|notification support|do (you|we) support notifications)\b/.test(
      text,
    ) ||
    text === "notifications?" ||
    text === "notifications"
  ) {
    return {
      kind: "notifyStatus",
      reply: "Checking desktop notification support.",
    };
  }

  if (
    /\b(dismiss|clear|hide)\b.+\bnotification\b/.test(text) ||
    text === "dismiss notification" ||
    text === "clear notification"
  ) {
    return {
      kind: "notifyDismiss",
      reply: "Trying to dismiss that notification.",
    };
  }

  const notifyThat = raw
    .trim()
    .match(/^notify(?:\s+me)?\s+that\s+(.+)$/i);
  if (notifyThat?.[1]) {
    const body = stripTrailingPunctuation(notifyThat[1]);
    if (body) {
      return {
        kind: "notifyShow",
        title: "Workspace",
        text: body,
        reply: "Showing a desktop notification.",
      };
    }
  }

  const withMessage = raw
    .trim()
    .match(
      /^(?:show(?:\s+me)?(?:\s+a)?(?:\s+desktop)?\s+notification|send(?:\s+me)?(?:\s+a)?(?:\s+desktop)?\s+notification|notify(?:\s+me)?|desktop\s+notification)\s*[:\-~]\s*(.+)$/i,
    );
  if (withMessage?.[1]) {
    const body = stripTrailingPunctuation(withMessage[1]);
    if (body) {
      return {
        kind: "notifyShow",
        title: "Workspace",
        text: body,
        reply: "Showing a desktop notification.",
      };
    }
  }

  if (
    /^(show(?:\s+me)?(?:\s+a)?(?:\s+desktop)?\s+notification|send(?:\s+me)?(?:\s+a)?(?:\s+desktop)?\s+notification|notify(?:\s+me)?|desktop\s+notification)[.!]?$/i.test(
      raw.trim(),
    )
  ) {
    return {
      kind: "notifyShow",
      title: "Workspace",
      text: "Notification from Workspace.",
      reply: "Showing a desktop notification.",
    };
  }

  return null;
}

/** Deterministic phrase key — capitalization, punctuation, spacing collapsed. */
function normalizeAliasKey(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .replace(/[-_]+/g, " ")
    .replace(/[^a-z0-9\s./]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function stripOpenDeterminers(value: string): string {
  return value.replace(/^(my|the|our|a|an)\s+/i, "").trim();
}

/**
 * Collapse ordinary browser phrasing (“a new GPT tab”, “ChatGPT in a new tab”)
 * so site aliases match without memorized command forms.
 */
function canonicalizeOpenTarget(value: string): string {
  let t = stripTrailingPunctuation(value);
  t = t.replace(/\s+in\s+(a\s+)?new\s+tab$/i, "");
  t = t.replace(/\s+on\s+(a\s+)?new\s+tab$/i, "");
  t = t.replace(/^(a\s+|the\s+)?new\s+/i, "");
  t = t.replace(/\s+tab$/i, "");
  // “Chrome browser” / “Edge app” — desktop surface words, not executables (P16.18).
  t = t.replace(/\s+(web\s+)?browsers?$/i, "");
  t = t.replace(/\s+apps?$/i, "");
  t = stripOpenDeterminers(t);
  return t.trim();
}

/**
 * Semantic Alias Rule (P16.8) — Kernel Operator / Intent Layer only.
 * Providers never see these abbreviations; they receive expanded product names / URLs.
 */
const SEMANTIC_ALIASES: Record<string, string> = {
  gpt: "chatgpt",
  "g p t": "chatgpt",
  "chat g p t": "chatgpt",
  git: "github",
  yt: "youtube",
  "y t": "youtube",
  "you tube": "youtube",
  vscode: "visual studio code",
  "vs code": "visual studio code",
  vs: "visual studio code",
  "code editor": "visual studio code",
  edge: "microsoft edge",
  msedge: "microsoft edge",
  chrome: "google chrome",
  settings: "windows settings",
  cursor: "cursor",
  "cursor ide": "cursor",
  // Windows / Search / PowerToys-style desktop terminology (aliases only).
  explorer: "file explorer",
  "file explorer": "file explorer",
  "this pc": "file explorer",
  calculator: "calculator",
  calc: "calculator",
  notepad: "notepad",
};

const SITE_ALIASES: Record<string, string> = {
  chatgpt: "https://chatgpt.com",
  "chat gpt": "https://chatgpt.com",
  "chat g p t": "https://chatgpt.com",
  gpt: "https://chatgpt.com",
  "g p t": "https://chatgpt.com",
  openai: "https://chatgpt.com",
  "open ai": "https://chatgpt.com",
  "latest chat": "https://chatgpt.com",
  "recent chat": "https://chatgpt.com",
  "latest chatgpt": "https://chatgpt.com",
  "recent chatgpt": "https://chatgpt.com",
  "latest gpt": "https://chatgpt.com",
  "recent gpt": "https://chatgpt.com",
  google: "https://www.google.com",
  github: "https://github.com",
  "git hub": "https://github.com",
  git: "https://github.com",
  youtube: "https://www.youtube.com",
  "you tube": "https://www.youtube.com",
  yt: "https://www.youtube.com",
  "y t": "https://www.youtube.com",
  bing: "https://www.bing.com",
};

/** Expand Operator-owned semantic aliases before matching. */
function expandSemanticAlias(name: string): string {
  const key = normalizeAliasKey(canonicalizeOpenTarget(name));
  const expanded = SEMANTIC_ALIASES[key];
  return expanded ?? canonicalizeOpenTarget(name);
}

function resolveSiteAlias(name: string): string | null {
  const expanded = expandSemanticAlias(name);
  const key = normalizeAliasKey(expanded);
  return SITE_ALIASES[key] ?? SITE_ALIASES[normalizeAliasKey(canonicalizeOpenTarget(name))] ?? null;
}

/** Window-title hint for Operator composition (beside / close / focus). */
function windowMatchLabel(name: string): string {
  const cleaned = expandSemanticAlias(name) || stripTrailingPunctuation(name);
  const alias = resolveSiteAlias(cleaned) ?? resolveSiteAlias(name);
  if (alias) {
    if (/chatgpt\.com/i.test(alias)) return "ChatGPT";
    if (/youtube\.com/i.test(alias)) return "YouTube";
    if (/github\.com/i.test(alias)) return "GitHub";
    if (/google\.com/i.test(alias)) return "Google";
    if (/bing\.com/i.test(alias)) return "Bing";
  }
  const key = normalizeAliasKey(cleaned);
  if (key === "microsoft edge" || key === "edge" || key === "msedge") {
    return "Microsoft Edge";
  }
  if (key === "google chrome" || key === "chrome") return "Google Chrome";
  if (key === "visual studio code" || key === "vscode" || key === "vs" || key === "vs code") {
    return "Visual Studio Code";
  }
  if (key === "windows settings" || key === "settings") return "Windows Settings";
  if (key === "cursor") return "Cursor";
  if (key === "file explorer" || key === "explorer" || key === "this pc") {
    return "File Explorer";
  }
  if (
    key === "microsoft store" ||
    key === "ms store" ||
    key === "windows store" ||
    key === "store"
  ) {
    return "Microsoft Store";
  }
  if (key === "calculator" || key === "calc") return "Calculator";
  if (key === "notepad") return "Notepad";
  return cleaned || name;
}

const APP_OPEN_EXCLUSIONS =
  /^(notepad|calculator|calc|spotify|discord|slack|figma|cursor|code|vscode|visual studio code|word|excel|outlook|chrome|edge|firefox|brave|msedge|file explorer|explorer)$/i;

const BROWSER_WINDOW_NAMES =
  /^(chrome|google chrome|edge|microsoft edge|msedge|firefox|brave|browser|my browser|current browser|latest browser|recent browser|chrome tab)$/i;

const INVALID_WEBSITE_SUGGESTION =
  'Try “open google”, “open github”, “open youtube”, or provide a complete URL.';

function invalidWebsiteReply(): IntentAction {
  return {
    kind: "unknown",
    reply: "I couldn’t determine a valid website.",
    suggestion: INVALID_WEBSITE_SUGGESTION,
  };
}

/** Common public suffixes — deterministic allowlist (not a full PSL). */
const PLAUSIBLE_TLDS = new Set([
  "com",
  "org",
  "net",
  "edu",
  "gov",
  "mil",
  "int",
  "io",
  "ai",
  "app",
  "dev",
  "co",
  "uk",
  "au",
  "ca",
  "de",
  "fr",
  "jp",
  "us",
  "nz",
  "in",
  "info",
  "biz",
  "me",
  "tv",
  "cc",
  "tech",
  "online",
  "site",
  "store",
  "cloud",
  "gg",
  "so",
  "fm",
  "xyz",
  "pro",
  "name",
  "blog",
  "page",
  "shop",
]);

/** Deterministic hostname / URL plausibility — never invent a launch. */
function isPlausibleWebsite(raw: string): boolean {
  const trimmed = raw.trim();
  if (!trimmed || /\s/.test(trimmed)) {
    return false;
  }
  let candidate = trimmed;
  if (!/^https?:\/\//i.test(candidate)) {
    if (!candidate.includes(".")) {
      return false;
    }
    candidate = `https://${candidate}`;
  }
  let url: URL;
  try {
    url = new URL(candidate);
  } catch {
    return false;
  }
  if (url.protocol !== "http:" && url.protocol !== "https:") {
    return false;
  }
  const host = url.hostname.toLowerCase();
  if (!host || host.includes("..")) {
    return false;
  }
  if (host === "localhost") {
    return true;
  }
  if (/^\d{1,3}(\.\d{1,3}){3}$/.test(host)) {
    return host.split(".").every((part) => {
      const n = Number(part);
      return Number.isInteger(n) && n >= 0 && n <= 255;
    });
  }
  if (
    !/^[a-z0-9]([a-z0-9-]*[a-z0-9])?(\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)+$/i.test(
      host,
    )
  ) {
    return false;
  }
  const labels = host.split(".");
  if (labels.length < 2) {
    return false;
  }
  const tld = labels[labels.length - 1] ?? "";
  if (!PLAUSIBLE_TLDS.has(tld)) {
    return false;
  }
  const sld = labels[labels.length - 2] ?? "";
  if (sld.length < 2) {
    return false;
  }
  return true;
}

function resolveOpenWebsiteTarget(target: string): IntentAction | null {
  const rawClean = stripTrailingPunctuation(target);
  const cleaned = canonicalizeOpenTarget(target);
  if (!rawClean || /^(workspace|conversation)$/i.test(cleaned || rawClean)) {
    return null;
  }
  // Known desktop apps → leave for application intents (never invent a site).
  if (APP_OPEN_EXCLUSIONS.test(normalizeAliasKey(cleaned || rawClean))) {
    return null;
  }

  // open browser / my browser / current|latest|recent browser
  if (
    /^(my\s+)?(current\s+|latest\s+|recent\s+)?browsers?$/i.test(rawClean) ||
    /^(my\s+)?(current\s+|latest\s+|recent\s+)?browsers?$/i.test(cleaned) ||
    /^(current|latest|recent)\s+browser$/i.test(cleaned)
  ) {
    return {
      kind: "browserOpen",
      url: "https://www.google.com",
      reply: "Opening your browser.",
    };
  }

  const alias = resolveSiteAlias(rawClean) ?? resolveSiteAlias(cleaned);
  if (alias) {
    const label = cleaned || stripOpenDeterminers(rawClean);
    return {
      kind: "browserOpen",
      url: alias,
      reply: `Opening ${label}.`,
    };
  }

  const looksLikeUrl =
    /^https?:\/\//i.test(rawClean) ||
    /^www\./i.test(rawClean) ||
    rawClean.includes(".");

  if (looksLikeUrl) {
    if (!isPlausibleWebsite(rawClean)) {
      return invalidWebsiteReply();
    }
    const url = /^https?:\/\//i.test(rawClean)
      ? rawClean
      : `https://${rawClean}`;
    return {
      kind: "browserOpen",
      url,
      reply: "Opening that site.",
    };
  }

  return null;
}

/**
 * Browser intents — URL / site open via Kernel Operator (P14 / P14.5).
 * Must run before generic “open <app>” application intents.
 * Natural-language robustness lives here — not inside desktop browser adapters.
 */
function resolveBrowserIntent(raw: string, text: string): IntentAction | null {
  const utterance = stripTrailingPunctuation(raw);

  // Capability discovery — never route browser “what can you…” to Guide.
  if (
    /\b(what can you do with browsers?|what do you do with browsers?|browser help|help with browsers?|how do (i|you) use (the )?browser|browser capabilities|what browsers? can you)\b/.test(
      text,
    )
  ) {
    return {
      kind: "browserExplain",
      reply:
        "With browsers I can open websites, bring Chrome or Edge forward, open a site beside Cursor, move or restore browser windows, and check which browsers are available on this PC.",
      suggestion:
        'Try “open google”, “open chatgpt beside cursor”, “bring chrome to the front”, or “which browsers are available?”.',
    };
  }

  if (
    /\b(can you (use |open )?browsers?|are browsers? available|browser support|which browsers|supported browsers)\b/.test(
      text,
    ) ||
    text === "browsers?" ||
    text === "browsers"
  ) {
    return {
      kind: "browserStatus",
      reply: "Checking which browsers are available.",
    };
  }

  // Bare browser window names → bring that browser forward
  if (BROWSER_WINDOW_NAMES.test(utterance) || BROWSER_WINDOW_NAMES.test(text)) {
    const query = utterance.replace(/\s+tab$/i, "").trim() || "Chrome";
    const label = /browser/i.test(query) ? "your browser" : query;
    return {
      kind: "winFocus",
      query: /^(my|current|latest|recent)\s+browser$/i.test(query)
        ? "Chrome"
        : query,
      reply: `Bringing ${label} to the front.`,
    };
  }

  const browserFocus =
    utterance.match(
      /^(?:bring)\s+(.+?)\s+(?:to\s+(?:the\s+)?front|forward)$/i,
    ) ??
    utterance.match(/^(?:focus|activate)\s+(.+)$/i) ??
    utterance.match(/^(?:show)\s+(?!me\b)(.+)$/i) ??
    utterance.match(/^(?:switch\s+to)\s+(.+)$/i);
  if (browserFocus?.[1]) {
    const target = stripTrailingPunctuation(browserFocus[1]);
    const key = normalizeAliasKey(target);
    if (
      BROWSER_WINDOW_NAMES.test(target) ||
      BROWSER_WINDOW_NAMES.test(key) ||
      /^(chrome|edge|firefox|brave)(\s+tab)?$/i.test(key)
    ) {
      const query = key
        .replace(/\s+tab$/, "")
        .replace(/^google\s+/, "")
        .replace(/^microsoft\s+/, "");
      const focusQuery =
        query === "msedge" || query === "microsoft edge"
          ? "Edge"
          : query === "google chrome"
            ? "Chrome"
            : query === "current browser" ||
                query === "latest browser" ||
                query === "recent browser" ||
                query === "my browser" ||
                query === "browser"
              ? "Chrome"
              : query;
      return {
        kind: "winFocus",
        query: focusQuery,
        reply: `Bringing “${focusQuery}” to the front.`,
      };
    }
  }

  if (
    /^(open|go to|visit|browse)\s+this\s+website$/i.test(utterance) ||
    text === "open this website"
  ) {
    return {
      kind: "unknown",
      reply: "Which website should I open?",
      suggestion: INVALID_WEBSITE_SUGGESTION,
    };
  }

  // “Open a/another/new browser” — open default browser site surface, not an .exe guess.
  // Longer qualifiers first so “a new” is not consumed as bare “a”.
  if (
    /^(?:open|launch|start)\s+(?:a\s+new\s+|another\s+|new\s+|my\s+|the\s+|a\s+)?browsers?$/i.test(
      utterance,
    )
  ) {
    return {
      kind: "browserOpen",
      url: "https://www.google.com",
      reply: "Opening your browser.",
    };
  }

  // “Open a browser beside Cursor” — browser chrome beside an app, not an executable name.
  const browserBeside = utterance.match(
    /^open\s+(?:a\s+new\s+|another\s+|new\s+|my\s+|the\s+|a\s+)?browsers?\s+beside\s+(.+)$/i,
  );
  if (browserBeside?.[1]) {
    const besideLabel = windowMatchLabel(stripTrailingPunctuation(browserBeside[1]));
    return {
      kind: "browserOpenBeside",
      url: "https://www.google.com",
      beside: besideLabel,
      reply: `Opening a browser beside “${besideLabel}”.`,
    };
  }

  const beside = utterance.match(/^open\s+(.+?)\s+beside\s+(.+)$/i);
  if (beside?.[1] && beside[2]) {
    const left = stripTrailingPunctuation(beside[1]);
    const right = stripTrailingPunctuation(beside[2]);
    if (!right) {
      return null;
    }
    const besideLabel = windowMatchLabel(right);
    const alias = resolveSiteAlias(left);
    if (alias) {
      return {
        kind: "browserOpenBeside",
        url: alias,
        beside: besideLabel,
        reply: `Opening beside “${besideLabel}”.`,
      };
    }
    if (left.includes(".") || /^https?:\/\//i.test(left) || /^www\./i.test(left)) {
      if (!isPlausibleWebsite(left)) {
        return invalidWebsiteReply();
      }
      const url = /^https?:\/\//i.test(left) ? left : `https://${left}`;
      return {
        kind: "browserOpenBeside",
        url,
        beside: besideLabel,
        reply: `Opening beside “${besideLabel}”.`,
      };
    }
  }

  // “Open ChatGPT in another browser window / in a new tab”
  const inWindow = utterance.match(
    /^(?:open|go to|visit|browse)\s+(.+?)\s+in\s+(?:another|a\s+new|new)\s+(?:browser\s+)?(?:window|tab)$/i,
  );
  if (inWindow?.[1]) {
    const site = resolveOpenWebsiteTarget(inWindow[1]);
    if (site) {
      return site;
    }
  }

  const openUrl = utterance.match(/^(?:open|go to|visit|browse)\s+(.+)$/i);
  if (openUrl?.[1]) {
    return resolveOpenWebsiteTarget(openUrl[1]);
  }

  return null;
}

/**
 * Architecture-driven Window intents (Conversation language → Window operations).
 * Must run before capability-evolution proposals that also match move/resize/show.
 */
function resolveWindowControlDiscovery(text: string): IntentAction | null {
  // C-OBS-004 — locate named control (observation only). Must run before semantic
  // “find / where is” window-locate, which otherwise soft-misses control phrases.
  const findControl =
    text.match(
      /^(?:find|locate|where(?:'s| is)|is there)\s+(?:the\s+)?(.+?)\s+(?:button|menu|control|field|item)?\s*(?:in|inside|on|within)\s+(.+)$/i,
    ) ??
    text.match(
      /^(?:find|locate)\s+(?:the\s+)?(?:control\s+)?(?:named\s+|called\s+)?["']?(.+?)["']?\s+(?:in|inside|on)\s+(.+)$/i,
    ) ??
    text.match(
      /^does\s+(.+)\s+have\s+(?:a\s+|an\s+|the\s+)?(.+?)(?:\s+button|\s+menu|\s+control)?\??$/i,
    );
  if (!findControl) {
    return null;
  }
  const doesHave = /^does\s+/i.test(text);
  const controlRaw = (doesHave ? findControl[2] : findControl[1])
    .trim()
    .replace(/[.!?]+$/g, "")
    .replace(/^(the|a|an)\s+/i, "");
  const windowRaw = (doesHave ? findControl[1] : findControl[2])
    .trim()
    .replace(/[.!?]+$/g, "");
  const control = controlRaw
    .replace(/\s+(button|menu|control|field|item)$/i, "")
    .trim();
  const query = /^(this|it|the window|this window|active|current)?$/i.test(
    windowRaw,
  )
    ? "this"
    : windowRaw || "this";
  if (!control || /^(click|type|press|invoke)\b/i.test(control)) {
    return null;
  }
  return {
    kind: "winFindControl",
    control,
    query,
    reply:
      query === "this"
        ? `Looking for “${control}” in the active window.`
        : `Looking for “${control}” in “${query}”.`,
  };
}

/** C-VER-003 — bounded wait for an observable desktop condition. */
function resolveWindowWaitCondition(text: string): IntentAction | null {
  const gone =
    text.match(
      /^(?:wait\s+(?:for|until)\s+)(?:the\s+)?(.+?)\s+(?:to\s+)?(?:disappear(?:s|ed)?|go(?:es|ing)?\s+away|be\s+gone|vanish(?:es|ed)?)(?:\s+(?:in|inside|on|within)\s+(.+))?$/i,
    ) ??
    text.match(
      /^(?:wait\s+(?:until|for)\s+)(?:the\s+)?(.+?)\s+(?:is\s+)?gone(?:\s+(?:in|inside|on|within)\s+(.+))?$/i,
    );
  if (gone) {
    const control = gone[1]
      .trim()
      .replace(/[.!?]+$/g, "")
      .replace(/^(the|a|an)\s+/i, "")
      .replace(/\s+(button|menu|control|field|item|window)$/i, "")
      .trim();
    const windowRaw = (gone[2] ?? "this").trim().replace(/[.!?]+$/g, "");
    const query = /^(this|it|the window|this window|active|current)?$/i.test(
      windowRaw,
    )
      ? "this"
      : windowRaw || "this";
    if (control) {
      return {
        kind: "winWaitCondition",
        condition: "control_gone",
        control,
        query,
        reply:
          query === "this"
            ? `Waiting until “${control}” is gone in the active window.`
            : `Waiting until “${control}” is gone in “${query}”.`,
      };
    }
  }

  const active = text.match(
    /^(?:wait\s+(?:for|until)\s+)(.+?)\s+(?:is\s+)?(?:active|focused|in\s+front|foreground)(?:\s+window)?$/i,
  );
  if (active) {
    const windowRaw = active[1].trim().replace(/[.!?]+$/g, "");
    const query = /^(this|it|the window|this window|active|current)?$/i.test(
      windowRaw,
    )
      ? "this"
      : windowRaw;
    if (query) {
      return {
        kind: "winWaitCondition",
        condition: "window_active",
        query,
        reply: `Waiting until “${query}” is active.`,
      };
    }
  }

  const windowAvail = text.match(
    /^(?:wait\s+(?:for|until)\s+)(.+?)\s+(?:window\s+)?(?:is\s+)?(?:available|open|ready)$/i,
  );
  if (windowAvail) {
    const windowRaw = windowAvail[1]
      .trim()
      .replace(/[.!?]+$/g, "")
      .replace(/\s+window$/i, "");
    const query = /^(this|it|the window|this window|active|current)?$/i.test(
      windowRaw,
    )
      ? "this"
      : windowRaw;
    if (query && !/\b(button|menu|control|field|item)\b/i.test(query)) {
      return {
        kind: "winWaitCondition",
        condition: "window_available",
        query,
        reply: `Waiting until “${query}” is available.`,
      };
    }
  }

  if (/\b(disappear|vanish|go(?:es)?\s+away|be\s+gone)\b/i.test(text)) {
    return null;
  }

  const appear =
    text.match(
      /^(?:wait\s+(?:for|until)\s+)(?:the\s+)?(.+?)\s+(?:to\s+)?(?:appear|show\s+up|be\s+available|become\s+available)(?:\s+(?:in|inside|on|within)\s+(.+))?$/i,
    ) ??
    text.match(
      /^(?:wait\s+(?:for|until)\s+)(?:the\s+)?(.+?)(?:\s+(?:button|menu|control|field|item))?(?:\s+(?:in|inside|on|within)\s+(.+))$/i,
    );
  if (appear) {
    const control = appear[1]
      .trim()
      .replace(/[.!?]+$/g, "")
      .replace(/^(the|a|an)\s+/i, "")
      .replace(/\s+(button|menu|control|field|item)$/i, "")
      .trim();
    const windowRaw = (appear[2] ?? "this").trim().replace(/[.!?]+$/g, "");
    const query = /^(this|it|the window|this window|active|current)?$/i.test(
      windowRaw,
    )
      ? "this"
      : windowRaw || "this";
    if (control) {
      return {
        kind: "winWaitCondition",
        condition: "control_available",
        control,
        query,
        reply:
          query === "this"
            ? `Waiting for “${control}” in the active window.`
            : `Waiting for “${control}” in “${query}”.`,
      };
    }
  }

  return null;
}

/** C-ACT-004 / C-ACT-005 — click / type named controls (before semantic soft-miss). */
function resolveWindowControlInteraction(text: string): IntentAction | null {
  const click =
    text.match(
      /^(?:click|press|tap|invoke)\s+(?:the\s+)?(.+?)\s+(?:button|menu|control|item)?\s*(?:in|inside|on|within)\s+(.+)$/i,
    ) ??
    text.match(
      /^(?:click|press|tap)\s+(?:the\s+)?["']?(.+?)["']?\s+(?:in|inside|on)\s+(.+)$/i,
    );
  if (click) {
    const control = click[1]
      .trim()
      .replace(/[.!?]+$/g, "")
      .replace(/^(the|a|an)\s+/i, "")
      .replace(/\s+(button|menu|control|field|item)$/i, "")
      .trim();
    const windowRaw = click[2].trim().replace(/[.!?]+$/g, "");
    const query = /^(this|it|the window|this window|active|current)?$/i.test(
      windowRaw,
    )
      ? "this"
      : windowRaw || "this";
    if (control) {
      return {
        kind: "winClickControl",
        control,
        query,
        reply:
          query === "this"
            ? `Clicking “${control}” in the active window.`
            : `Clicking “${control}” in “${query}”.`,
      };
    }
  }

  const typeIn =
    text.match(
      /^(?:type|enter|write)\s+["'](.+?)["']\s+(?:into|in|to)\s+(?:the\s+)?(.+?)\s+(?:field|box|control)?\s*(?:in|inside|on|within)\s+(.+)$/i,
    ) ??
    text.match(
      /^(?:type|enter|write)\s+(.+?)\s+(?:into|in)\s+(?:the\s+)?(.+?)\s+(?:in|inside|on|within)\s+(.+)$/i,
    );
  if (typeIn) {
    const typed = typeIn[1].trim().replace(/[.!?]+$/g, "");
    const control = typeIn[2]
      .trim()
      .replace(/[.!?]+$/g, "")
      .replace(/^(the|a|an)\s+/i, "")
      .replace(/\s+(field|box|control|edit)$/i, "")
      .trim();
    const windowRaw = typeIn[3].trim().replace(/[.!?]+$/g, "");
    const query = /^(this|it|the window|this window|active|current)?$/i.test(
      windowRaw,
    )
      ? "this"
      : windowRaw || "this";
    if (typed && control) {
      return {
        kind: "winTypeControl",
        control,
        query,
        text: typed,
        reply:
          query === "this"
            ? `Typing into “${control}” in the active window.`
            : `Typing into “${control}” in “${query}”.`,
      };
    }
  }
  return null;
}

function resolveWindowIntent(raw: string, text: string): IntentAction | null {
  const utterance = stripTrailingPunctuation(raw);

  // Before generic “minimize <target>” — do not treat “all apps” as a window name.
  if (
    /\bminimize\s+all\b/.test(text) ||
    /\b(minimise|minimize)\s+(all\s+)?(apps|applications|windows)\b/.test(
      text,
    )
  ) {
    return {
      kind: "unknown",
      reply: "I can’t minimize every application at once yet.",
      suggestion: "Name a window and I can minimize that one.",
    };
  }

  // C-OBS-003 — Desktop UI Tree (control discovery; observation only).
  {
    const controlsMatch =
      text.match(
        /^(?:what|list|show)(?:\s+(?:are\s+the|the))?\s+controls(?:\s+(?:in|for|inside|on))\s+(.+)$/i,
      ) ??
      text.match(/^what controls (?:are )?(?:in|inside|on)\s+(.+)$/i) ??
      text.match(
        /^(?:list|show)\s+(?:ui\s+)?(?:controls|control tree|desktop ui tree)(?:\s+(?:in|for|inside|on)\s+(.+))?$/i,
      );
    if (controlsMatch) {
      const target = (controlsMatch[1] ?? "this").trim().replace(/[.!?]+$/g, "");
      const query = /^(this|it|the window|this window|active|current)?$/i.test(target)
        ? "this"
        : target || "this";
      return {
        kind: "winEnumerateControls",
        query,
        reply:
          query === "this"
            ? "Checking controls in the active window."
            : `Checking controls in “${query}”.`,
      };
    }
  }

  // C-OBS-004 — Window Control Discovery (locate named control; observation only).
  {
    const discovered = resolveWindowControlDiscovery(text);
    if (discovered) {
      return discovered;
    }
  }

  if (
    /\b(what windows are open|which windows are open|what windows do i have|show( me)?( my)?( open)? windows|list( (all|my|open))? windows|open windows|what('?s| is) open on (my |the )?desktop|what('?s| is) on (my |the )?screen)\b/.test(
      text,
    ) ||
    text === "windows" ||
    text === "what windows" ||
    text === "show windows" ||
    text === "show me what's open" ||
    text === "show me whats open"
  ) {
    return {
      kind: "winEnumerate",
      reply: "Checking which windows are open.",
    };
  }

  if (
    /\b(which window is active|what('?s| is) (the )?(active|focused|foreground) window|active window|foreground window|what('?s| is) focused)\b/.test(
      text,
    )
  ) {
    return {
      kind: "winActive",
      reply: "Checking the active window.",
    };
  }

  if (
    /\b(list (my )?monitors|list (my )?displays|what monitors|which monitors|how many (monitors|displays))\b/.test(
      text,
    ) ||
    text === "monitors" ||
    text === "displays"
  ) {
    return {
      kind: "winMonitors",
      reply: "Checking attached monitors.",
    };
  }

  const snapEdge =
    utterance.match(
      /^(?:snap|move)\s+(?:window\s+)?(.+?)\s+(?:to\s+the\s+)?(left|right|top|bottom)(?:\s+side)?$/i,
    ) ??
    utterance.match(
      /^(?:move|snap)\s+(?:this|the|my)?\s*window\s+(?:to\s+the\s+)?(left|right|top|bottom)(?:\s+side)?$/i,
    );
  if (snapEdge) {
    if (snapEdge.length === 3 && snapEdge[1] && snapEdge[2]) {
      const edge = snapEdge[2].toLowerCase();
      const query = windowTarget(snapEdge[1]);
      return {
        kind: "winSnap",
        query,
        snap: edge,
        reply: `Moving ${query === "this" ? "this window" : `“${query}”`} to the ${edge}.`,
      };
    }
    if (snapEdge.length === 2 && snapEdge[1]) {
      const edge = snapEdge[1].toLowerCase();
      return {
        kind: "winSnap",
        query: "this",
        snap: edge,
        reply: `Moving this window to the ${edge}.`,
      };
    }
  }

  const moveMonitor =
    utterance.match(
      /^(?:move|send)\s+(?:window\s+)?(.+?)\s+to\s+(?:monitor|display)\s+(\d+|one|two|three|four|first|second|third|fourth)$/i,
    ) ??
    utterance.match(
      /^(?:move|send)\s+(?:this|the|my)?\s*window\s+to\s+(?:monitor|display)\s+(\d+|one|two|three|four|first|second|third|fourth)$/i,
    );
  if (moveMonitor) {
    if (moveMonitor.length === 3 && moveMonitor[1] && moveMonitor[2]) {
      const monitorIndex = parseMonitorIndex(moveMonitor[2]);
      if (monitorIndex != null) {
        const query = windowTarget(moveMonitor[1]);
        return {
          kind: "winMoveMonitor",
          query,
          monitorIndex,
          reply: `Moving ${query === "this" ? "this window" : `“${query}”`} to monitor ${monitorIndex}.`,
        };
      }
    }
    if (moveMonitor.length === 2 && moveMonitor[1]) {
      const monitorIndex = parseMonitorIndex(moveMonitor[1]);
      if (monitorIndex != null) {
        return {
          kind: "winMoveMonitor",
          query: "this",
          monitorIndex,
          reply: `Moving this window to monitor ${monitorIndex}.`,
        };
      }
    }
  }

  const maximize = utterance.match(
    /^(?:maximize|maximise)(?:\s+(?:window\s+)?(.+))?$/i,
  );
  if (maximize) {
    const query = windowTarget(maximize[1]);
    return {
      kind: "winMaximize",
      query,
      reply:
        query === "this"
          ? "Maximizing this window."
          : `Maximizing “${query}”.`,
    };
  }

  const minimize = utterance.match(
    /^(?:minimize|minimise)(?:\s+(?:window\s+)?(.+))?$/i,
  );
  if (minimize) {
    const query = windowTarget(minimize[1]);
    return {
      kind: "winMinimize",
      query,
      reply:
        query === "this"
          ? "Minimizing this window."
          : `Minimizing “${query}”.`,
    };
  }

  const restoreWin = utterance.match(
    /^(?:restore|unminimize)(?:\s+(?:window\s+|app\s+)?(.+))?$/i,
  );
  if (restoreWin) {
    const candidate = stripTrailingPunctuation(restoreWin[1] ?? "");
    const momentOnly =
      /^(yesterday|workspace|history|saved work|moments?)$/i.test(candidate);
    if (!momentOnly) {
      const query = windowTarget(candidate || "this");
      return {
        kind: "winRestore",
        query,
        reply:
          query === "this"
            ? "Restoring this window."
            : `Restoring “${query}”.`,
      };
    }
  }

  const center = utterance.match(
    /^(?:center|centre)(?:\s+(?:window\s+)?(.+))?$/i,
  );
  if (center) {
    const query = windowTarget(center[1]);
    return {
      kind: "winCenter",
      query,
      reply:
        query === "this"
          ? "Centering this window."
          : `Centering “${query}”.`,
    };
  }

  const focusFront =
    utterance.match(
      /^(?:bring|put)\s+(.+?)\s+(?:to\s+(?:the\s+)?front|forward|in\s+front)$/i,
    ) ??
    utterance.match(/^(?:bring)\s+(.+?)\s+forward$/i) ??
    utterance.match(/^(?:focus(?:\s+window)?|activate)\s+(.+)$/i) ??
    utterance.match(/^(?:show)\s+(?!me\b)(.+)$/i) ??
    utterance.match(/^(?:switch\s+to)\s+(.+)$/i);
  if (focusFront?.[1]) {
    const query = windowTarget(focusFront[1]);
    return {
      kind: "winFocus",
      query,
      reply:
        query === "this"
          ? "Bringing this window to the front."
          : `Bringing “${query}” to the front.`,
    };
  }

  const bounds = utterance.match(
    /^(?:bounds|where is|window (?:info|size|position)(?: for)?)\s+(.+)$/i,
  );
  if (bounds?.[1]) {
    const query = windowTarget(bounds[1]);
    return {
      kind: "winBounds",
      query,
      reply:
        query === "this"
          ? "Reading this window’s size and position."
          : `Reading size and position for “${query}”.`,
    };
  }

  const resizeDims = utterance.match(
    /^(?:resize)\s+(?:window\s+)?(.+?)\s+to\s+(\d+)\s*[x×]\s*(\d+)$/i,
  );
  if (resizeDims?.[1] && resizeDims[2] && resizeDims[3]) {
    const query = windowTarget(resizeDims[1]);
    const width = Number(resizeDims[2]);
    const height = Number(resizeDims[3]);
    return {
      kind: "winResize",
      query,
      width,
      height,
      reply: `Resizing ${query === "this" ? "this window" : `“${query}”`} to ${width}×${height}.`,
    };
  }

  // Honest clarification — never invent a placement.
  if (
    /^(move|resize)(\s+(this|the|my))?(\s+window)?$/.test(text) ||
    /^move this window$/.test(text) ||
    /^resize this window$/.test(text)
  ) {
    if (text.startsWith("resize")) {
      return {
        kind: "unknown",
        reply:
          "I can resize a window when you give a size — for example “resize this window to 1280x720”.",
        suggestion: "Or try “snap this window left” / “center this window”.",
      };
    }
    return {
      kind: "unknown",
      reply:
        "I can move a window when you say where — for example “move this window to the left” or “move this window to monitor two”.",
      suggestion: "Try “center this window” or “maximize this window”.",
    };
  }

  return null;
}

/**
 * Resolve a user utterance to an existing Workspace capability.
 * Never invents desktop awareness or memory.
 * Raw transcripts never become executable names (Semantic Intent Engine — P16.31).
 */
function resolveIntentCore(raw: string): IntentAction {
  const text = normalize(raw);
  if (!text) {
    return {
      kind: "unknown",
      reply: "Waiting.",
    };
  }

  const softText = softenUtterance(text);
  const softRaw = softenUtterance(raw.trim());
  const matchText = softText || text;

  if (isVoiceCheckUtterance(text) || isVoiceCheckUtterance(matchText)) {
    return voiceCheckReply(raw);
  }

  // C-VER-003 before click/type so “wait for Save…” is not misread as click.
  const waitCondition =
    resolveWindowWaitCondition(text) ??
    (matchText !== text ? resolveWindowWaitCondition(matchText) : null);
  if (waitCondition) {
    return waitCondition;
  }

  // C-ACT-004/005 before C-OBS-004 / semantic soft-miss.
  const controlInteraction =
    resolveWindowControlInteraction(text) ??
    (matchText !== text ? resolveWindowControlInteraction(matchText) : null);
  if (controlInteraction) {
    return controlInteraction;
  }

  // C-OBS-004 before semantic “find / where is” window locate (which soft-misses controls).
  const controlDiscovery =
    resolveWindowControlDiscovery(text) ??
    (matchText !== text ? resolveWindowControlDiscovery(matchText) : null);
  if (controlDiscovery) {
    return controlDiscovery;
  }

  // Semantic Intent Engine — grammar + entity reasoning before app/exe fallthrough.
  // Situation Goals (Continue) run inside semantic and must win over prepare phrasing.
  const semantic =
    resolveSemanticIntent(raw.trim()) ??
    resolveSemanticIntent(softRaw) ??
    resolveSemanticIntent(matchText);
  if (semantic) {
    return semantic;
  }

  // C-PROC-002 — Prepare Coding Workspace (after Situation Goals Continue ownership).
  const prepare =
    resolvePrepareCodingWorkspace(raw.trim()) ??
    resolvePrepareCodingWorkspace(softRaw) ??
    resolvePrepareCodingWorkspace(matchText);
  if (prepare) {
    return prepare;
  }

  if (
    /^(hi|hello|hey)\b/.test(text) ||
    text === "hi" ||
    text === "hello"
  ) {
    return companionGreetingReply();
  }

  if (
    /\b(developer mode|engineering mode|dev mode)\b/.test(text) ||
    text === "toggle developer"
  ) {
    const enable = !/\b(off|disable|exit)\b/.test(text);
    return {
      kind: "developer",
      enabled: enable,
      reply: enable
        ? "Developer mode on. Repository health and evidence are available."
        : "Developer mode off. Engineering surfaces stay hidden.",
    };
  }

  const approval = parseApprovalIntent(raw);
  if (approval) {
    const state = loadEvolutionState();
    if (approval === "list") {
      return { kind: "proposal", reply: formatBacklogReply(state) };
    }
    const target = latestProposed(state) ?? state.proposals[0];
    if (!target) {
      return {
        kind: "proposal",
        reply: "There is no proposal to act on yet.",
      };
    }
    if (approval === "undo") {
      const next = undoLastStatusChange(state, target.id);
      if (!next) {
        return {
          kind: "proposal",
          reply: `Nothing to undo for ${target.id}.`,
        };
      }
      saveEvolutionState(next);
      return {
        kind: "proposal",
        reply: `Undid last status change on ${target.id}.`,
      };
    }
    const status = approval === "approve" ? "approved" : "rejected";
    const next = setProposalStatus(state, target.id, status, "conversation");
    saveEvolutionState(next);
    return {
      kind: "proposal",
      reply:
        status === "approved"
          ? `Approved ${target.id}. It is on the capability backlog. Implementation still requires a constitutional execution program — Workspace will not rewrite itself.`
          : `Rejected ${target.id}. Recorded in the audit trail.`,
    };
  }

  // Window operations before evolution (evolution also matches move/resize/show).
  // Softened wording covers “please / could you …” without rigid command memorization.
  const windowIntent =
    resolveWindowIntent(raw, text) ??
    (matchText !== text
      ? resolveWindowIntent(softRaw, matchText)
      : null);
  if (windowIntent) {
    return windowIntent;
  }

  const notificationIntent =
    resolveNotificationIntent(raw, text) ??
    (matchText !== text
      ? resolveNotificationIntent(softRaw, matchText)
      : null);
  if (notificationIntent) {
    return notificationIntent;
  }

  const browserIntent =
    resolveBrowserIntent(raw, text) ??
    (matchText !== text
      ? resolveBrowserIntent(softRaw, matchText)
      : null);
  if (browserIntent) {
    return browserIntent;
  }

  const screenshotIntent =
    resolveScreenshotIntent(raw, text) ??
    (matchText !== text
      ? resolveScreenshotIntent(softRaw, matchText)
      : null);
  if (screenshotIntent) {
    return screenshotIntent;
  }

  const voiceIntent =
    resolveVoiceIntent(raw, text) ??
    (matchText !== text ? resolveVoiceIntent(softRaw, matchText) : null);
  if (voiceIntent) {
    return voiceIntent;
  }

  if (isEvolutionRequest(raw)) {
    const proposal = createProposal(raw);
    const state = appendProposal(loadEvolutionState(), proposal);
    saveEvolutionState(state);
    return {
      kind: "proposal",
      reply: formatProposalReply(proposal),
    };
  }

  if (
    /\b(repo(sitory)?\s*health|engineering\s*health|project\s*health)\b/.test(
      text,
    )
  ) {
    return {
      kind: "health",
      reply:
        "Opening repository health (developer surface). If developer mode is off, it will be enabled for this view.",
    };
  }

  if (
    /\b((export|create|make|generate|save)\s+(a\s+)?(support|diagnostic|diagnostics)\s+(package|bundle|report|zip)|support\s+package|diagnostic\s+bundle|export\s+logs)\b/.test(
      text,
    )
  ) {
    return {
      kind: "supportBundle",
      reply: "Creating a local support package (logs and version info — not your Moments).",
    };
  }

  if (
    /\b(collapse|minimize|minimise|float|icon)\b/.test(text) ||
    text === "desktop operator"
  ) {
    return {
      kind: "collapse",
      reply:
        "Returning to the desktop operator. Click the floating W to open conversation again.",
    };
  }

  if (
    /\b(expand|open workspace|show workspace|workspace surfaces|mode 2)\b/.test(
      text,
    ) ||
    text === "open workspace"
  ) {
    return {
      kind: "expand",
      reply:
        "There’s no separate Expanded Workspace form. Tools open beside this conversation when you ask — try Save, Continue, or Guide.",
    };
  }

  const saveAs = raw
    .trim()
    .match(
      /^(?:save(?:\s+this)?\s+as|remember\s+(?:this\s+as|as))\s+(.+)$/i,
    );
  if (saveAs?.[1]) {
    const name = saveAs[1].trim().replace(/[.!?]+$/g, "");
    return {
      kind: "saveAs",
      name,
      reply: `Opening Save. Name it “${name}” when you approve capture — nothing is written until you consent.`,
    };
  }

  if (
    /\b(save( this| moment| work| context)?|remember (this|where)|put (this|it) down)\b/.test(
      text,
    ) ||
    text === "save" ||
    text === "save this"
  ) {
    return {
      kind: "navigate",
      view: "save",
      reply: "Opening Save. You approve capture before anything is written.",
    };
  }

  // Moments use Continue / Resume. Bare “restore <app>” is Window state (P12.5).
  const namedMoment = raw
    .trim()
    .match(/^(?:continue|resume|open\s+moment)\s+(.+)$/i);
  if (namedMoment?.[1]) {
    const nameQuery = stripTrailingPunctuation(namedMoment[1]);
    const blocked =
      /^(yesterday|workspace|history|saved work|moments?|settings|guide|help|check[- ]?in)$/i.test(
        nameQuery,
      );
    if (!blocked && nameQuery.length > 0) {
      return {
        kind: "navigateNamed",
        view: "resume",
        nameQuery,
        reply: `Looking for a saved Moment matching “${nameQuery}”.`,
      };
    }
  }

  if (
    /\b(continue|resume)\b/.test(text) ||
    /\byesterday\b/.test(text) ||
    text === "restore workspace" ||
    text === "restore yesterday" ||
    text === "continue yesterday"
  ) {
    return {
      kind: "navigate",
      view: "resume",
      reply:
        "Opening Continue. Restore only runs after you approve a plan — and only for windows still open in this session.",
    };
  }

  if (
    /\b(history|saved work|saved moments?|review saved|moments?|open history)\b/.test(
      text,
    )
  ) {
    return {
      kind: "navigate",
      view: "home",
      reply: "Opening your saved Moments.",
    };
  }

  if (/\b(check[- ]?in|pilot|measurement)\b/.test(text)) {
    return {
      kind: "navigate",
      view: "pilot",
      reply:
        "Opening Check-in (pilot measurement — separate from saved Moments).",
    };
  }

  // Guide chrome — explicit Guide/help navigation only (capability discovery is Semantic Engine).
  if (
    text === "guide" ||
    matchText === "guide" ||
    text === "help" ||
    matchText === "help" ||
    text === "open guide" ||
    matchText === "open guide" ||
    text === "show guide" ||
    matchText === "show guide" ||
    text === "show me the guide" ||
    matchText === "show me the guide" ||
    /\b(open (the )?guide|show (me )?(the )?guide|trust limits?|what are (your|the) limits|how does workspace work)\b/.test(
      matchText,
    )
  ) {
    return {
      kind: "navigate",
      view: "help",
      reply: "Opening Guide.",
    };
  }

  // Windows Settings (not an in-shell preferences panel).
  if (
    /^(open\s+)?(windows\s+)?settings$/i.test(matchText) ||
    /^(open\s+)?(system\s+)?preferences$/i.test(matchText) ||
    matchText === "open windows settings" ||
    matchText === "windows settings"
  ) {
    return {
      kind: "appLaunch",
      query: "ms-settings:",
      reply: "Opening Windows Settings.",
    };
  }

  if (/\b(workspace settings|in-app settings|shell options)\b/.test(text)) {
    return {
      kind: "settings",
      reply:
        "There’s no Settings surface in this shell. Collapse returns to the desktop operator; Exit Workspace quits. Ask Guide for trust limits.",
    };
  }

  // Capture this Conversation / chat window
  if (
    /\b(take a )?(capture|screenshot|screen\s*shot)\b.+\b(chat|conversation|this chat|our chat)\b/.test(
      text,
    ) ||
    /\b(capture|screenshot)\s+(our|this)\s+chat\b/.test(text) ||
    text === "take a capture of our chat" ||
    text === "capture this conversation"
  ) {
    return {
      kind: "screenshotWindow",
      query: "this",
      reply: "Capturing this window.",
    };
  }

  // Truthful unsupported — volume / tab close / minimize-all / live transcription
  if (
    /\b(speaker\s+)?volume\b/.test(text) ||
    /\b(set|change|mute|unmute)\b.+\b(volume|speaker|sound|audio)\b/.test(text)
  ) {
    return {
      kind: "unknown",
      reply: "I can’t change speaker volume yet.",
      suggestion: "I can open Windows Settings if you want to adjust it there.",
    };
  }

  if (
    /\b(close|quit)\b.+\b(browser\s+)?tab\b/.test(text) ||
    text === "close this browser tab" ||
    text === "close this tab"
  ) {
    return {
      kind: "unknown",
      reply:
        "I can’t close a single browser tab yet — only whole windows or apps.",
      suggestion: "Name the browser window if you want me to close that instead.",
    };
  }

  if (
    /\bminimize\s+all\b/.test(text) ||
    /\b(minimise|minimize)\s+(all\s+)?(apps|applications|windows)\b/.test(
      text,
    )
  ) {
    return {
      kind: "unknown",
      reply: "I can’t minimize every application at once yet.",
      suggestion: "Name a window and I can minimize that one.",
    };
  }

  if (
    /\btranscribe\b/.test(text) ||
    /\b(transcribe|transcription of)\b.+\b(conversation|chat|meeting|call)\b/.test(
      text,
    )
  ) {
    return {
      kind: "unknown",
      reply:
        "I don’t transcribe other conversations — the mic only puts what you say into Workspace Conversation.",
    };
  }

  const clipboardWrite = matchFirst(
    raw,
    softRaw,
    /^(?:copy(?:\s+to\s+clipboard)?|clipboard\s+write|put\s+on\s+clipboard)\s*[:\s]+(.+)$/i,
  );
  if (clipboardWrite?.[1]) {
    const payload = clipboardWrite[1].trim();
    if (payload.length > 0) {
      return {
        kind: "clipboardWrite",
        text: payload,
        reply: "Copying that to the clipboard.",
      };
    }
  }

  if (
    /\b(what'?s on my clipboard|read clipboard|show clipboard|clipboard)\b/.test(
      matchText,
    ) ||
    matchText === "clipboard?"
  ) {
    return {
      kind: "clipboardRead",
      reply: "Checking the clipboard.",
    };
  }

  if (
    /\b(list (running )?apps?|list applications|what('?s| is) running|running applications)\b/.test(
      matchText,
    )
  ) {
    return {
      kind: "appEnumerate",
      reply: "Listing running applications.",
    };
  }

  const appClose = matchFirst(
    raw,
    softRaw,
    /^(?:close|quit|exit)\s+(.+)$/i,
  );
  if (appClose?.[1]) {
    const rawQuery = stripTrailingPunctuation(appClose[1]);
    const query = windowMatchLabel(rawQuery);
    if (
      query &&
      !/^(workspace|conversation|this browser tab|this tab|browser tab)$/i.test(
        query,
      )
    ) {
      return {
        kind: "appClose",
        query,
        reply: `Closing “${query}”.`,
      };
    }
  }

  const appFocus = matchFirst(
    raw,
    softRaw,
    /^(?:switch\s+to|bring\s+up)\s+(.+)$/i,
  );
  if (appFocus?.[1]) {
    const query = stripTrailingPunctuation(appFocus[1]);
    if (query) {
      return {
        kind: "appFocus",
        query,
        reply: `Switching to “${query}”.`,
      };
    }
  }

  const appLaunchExplicit = matchFirst(
    raw,
    softRaw,
    /^(?:launch|start)\s+(.+)$/i,
  );
  if (appLaunchExplicit?.[1]) {
    const rawQuery = stripTrailingPunctuation(appLaunchExplicit[1]);
    const query = expandSemanticAlias(rawQuery) || rawQuery;
    // Bare “launch/start browser” — never invent browser.exe.
    if (/^(a\s+|my\s+|the\s+|another\s+|a\s+new\s+|new\s+)?browsers?$/i.test(query)) {
      return {
        kind: "browserOpen",
        url: "https://www.google.com",
        reply: "Opening your browser.",
      };
    }
    // Site abbreviations must never become executable launches (User Adaptation).
    if (resolveSiteAlias(rawQuery) || resolveSiteAlias(query)) {
      const site =
        resolveOpenWebsiteTarget(rawQuery) ?? resolveOpenWebsiteTarget(query);
      if (site) {
        return site;
      }
    }
    // P16.32: only resolved desktop entities may launch — never invent executables.
    const entity = resolveDesktopEntity(rawQuery) ?? resolveDesktopEntity(query);
    if (entity?.kind === "protocol" || entity?.kind === "shell") {
      return {
        kind: "appLaunch",
        query: entity.value,
        reply: `Launching ${entity.label}.`,
      };
    }
    if (entity?.kind === "browser" || entity?.kind === "application") {
      return {
        kind: "appOpen",
        query: entity.openQuery ?? entity.value,
        reply: `Opening “${entity.label}”.`,
      };
    }
    if (entity?.kind === "site") {
      return {
        kind: "browserOpen",
        url: entity.value,
        reply: `Opening ${entity.label}.`,
      };
    }
    if (rawQuery) {
      const nearby = suggestNearbyCapabilities(rawQuery);
      return {
        kind: "unknown",
        reply: `I don’t recognize “${rawQuery}” as something I can launch yet.`,
        suggestion: nearby
          ? `Closest I can try: “${nearby.split("”, “")[0]}”.`
          : "Name an app or site you already use and I’ll try that.",
      };
    }
  }

  const appOpen = matchFirst(raw, softRaw, /^open\s+(.+)$/i);
  if (appOpen?.[1]) {
    const rawQuery = stripTrailingPunctuation(appOpen[1]);
    // P21.S2 — resolvable “open A and B” → compound; unresolved compounds stay unknown.
    if (/\band\b/i.test(rawQuery) && !/\bbeside\b/i.test(rawQuery)) {
      const compound = resolveCompoundOpen(rawQuery);
      if (compound) {
        return compound;
      }
      return {
        kind: "unknown",
        reply: "I need a clearer desktop request before I can open that.",
        suggestion: "Name each app or site — I only open targets I already know.",
      };
    }
    // Safety net: compounds / layout modifiers must never become executable names.
    if (
      /\b(full\s*size|fullscreen|full\s*screen|maximized|maximised)\b/i.test(
        rawQuery,
      ) ||
      /\bbeside\b/i.test(rawQuery)
    ) {
      return {
        kind: "unknown",
        reply: "I need a clearer desktop request before I can open that.",
        suggestion: "Which app or site should I open?",
      };
    }
    const query = expandSemanticAlias(rawQuery) || rawQuery;
    // Never treat GPT / site aliases as executable names (User Adaptation).
    if (resolveSiteAlias(rawQuery) || resolveSiteAlias(query)) {
      const site =
        resolveOpenWebsiteTarget(rawQuery) ?? resolveOpenWebsiteTarget(query);
      if (site) {
        return site;
      }
    }
    // Browser-shaped phrasing without a site alias (e.g. “open a tab”) stays honest.
    if (/\b(tab|website|site|url|webpage|web page)\b/i.test(rawQuery)) {
      return {
        kind: "unknown",
        reply: "Which website should I open?",
        suggestion: INVALID_WEBSITE_SUGGESTION,
      };
    }
    const entity = resolveDesktopEntity(rawQuery) ?? resolveDesktopEntity(query);
    if (entity?.kind === "protocol" || entity?.kind === "shell") {
      return {
        kind: "appLaunch",
        query: entity.value,
        reply: `Opening ${entity.label}.`,
      };
    }
    if (entity?.kind === "browser" || entity?.kind === "application") {
      return {
        kind: "appOpen",
        query: entity.openQuery ?? entity.value,
        reply: `Opening “${entity.label}”.`,
      };
    }
    if (query && !/^(workspace|conversation)$/i.test(query)) {
      const nearby = suggestNearbyCapabilities(rawQuery);
      return {
        kind: "unknown",
        reply: `I don’t recognize “${rawQuery}” as something I can open yet.`,
        suggestion: nearby
          ? `Closest I can try: “${nearby.split("”, “")[0]}”.`
          : "Name an app or site you already use and I’ll try that.",
      };
    }
  }

  // P22.S1 — intelligence kind before desktop soft-miss refusal.
  const intelligence = resolveIntelligenceRoute(raw.trim()) ??
    resolveIntelligenceRoute(matchText);
  if (intelligence) {
    return intelligence;
  }

  return resolveUnknownGuidance(matchText);
}

/**
 * Intent resolution before Goal Resolution (evidence / audits only).
 * Ordinary Conversation uses `resolveIntent`.
 */
export function resolveIntentBeforeGoalResolution(raw: string): IntentAction {
  return resolveIntentCore(raw);
}

/**
 * Public Intent entry:
 * Workspace Context (continuity) → Intent core → Goal Resolution → commit context.
 * Context-owned actions are final (Goal Resolution must not re-unbind referents).
 */
export function resolveIntent(raw: string): IntentAction {
  const fromContext = resolveFromWorkspaceContext(raw);
  if (fromContext) {
    commitWorkspaceContext(raw, fromContext.action);
    return fromContext.action;
  }
  const action = applyGoalResolution(raw, resolveIntentCore(raw));
  commitWorkspaceContext(raw, action);
  return action;
}

/** Progressive reveal for reply text (not model streaming). */
export async function streamText(
  text: string,
  onChunk: (partial: string) => void,
  signal?: AbortSignal,
): Promise<void> {
  if (!text) {
    onChunk("");
    return;
  }
  const step = Math.max(1, Math.ceil(text.length / 28));
  let i = 0;
  while (i < text.length) {
    if (signal?.aborted) {
      return;
    }
    i = Math.min(text.length, i + step);
    onChunk(text.slice(0, i));
    await new Promise((r) => setTimeout(r, 16));
  }
}
