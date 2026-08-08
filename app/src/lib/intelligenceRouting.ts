/**
 * P22.S1 — Intelligence Kind Routing (Intent / Conversation).
 *
 * Deterministic classification before desktop soft-miss refusal.
 * Local reasoning answers only when Workspace can compute them truthfully.
 * Broad knowledge hands off to ChatGPT via Browser Provider — never invents.
 */

import {
  generateCapabilityDiscovery,
  isCapabilityDiscoveryUtterance,
  resolveDiscoveryScope,
} from "./capabilityRegistry";
import type { IntentAction } from "./intentBridge";
import { resolveDesktopEntity } from "./semanticIntentEngine";

export type IntelligenceKind =
  | "REASONING_LOCAL"
  | "REASONING_PROVIDER"
  | "DESKTOP"
  | "HYBRID"
  | "CLARIFICATION"
  | "OWNER_AUTHORIZATION"
  | "CAPABILITY_LIMIT";

const CHATGPT_ORIGIN = "https://chatgpt.com";

/** ChatGPT accepts `?q=` as a query handoff on the product surface. */
export function chatgptReasoningUrl(query: string): string {
  const q = query.trim();
  if (!q) {
    return CHATGPT_ORIGIN;
  }
  return `${CHATGPT_ORIGIN}/?q=${encodeURIComponent(q)}`;
}

function normalize(text: string): string {
  return text
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

function isCapabilityLimit(text: string): boolean {
  return (
    /\b(file|files|folder|folders|directory|directories|document|documents)\b/.test(
      text,
    ) ||
    /\b(terminal|command prompt|powershell|shell|console|cmd)\b/.test(text) ||
    /\b(mute|volume|sound level)\b/.test(text)
  );
}

function isOwnerAuthorization(text: string): boolean {
  return (
    /\b(restore|continue)\b/.test(text) &&
    /\b(moment|workspace|everything)\b/.test(text)
  );
}

function isConversationalLocal(text: string): boolean {
  return /^(thanks|thank you|thx|ty|ok|okay|cool|great|cheers|bye|goodbye|good night|good morning|good afternoon|nm|never ?mind|all good)$/i.test(
    text,
  );
}

function conversationalReply(text: string): string {
  if (/^(thanks|thank you|thx|ty|cheers)$/i.test(text)) {
    return "You’re welcome.";
  }
  if (/^(bye|goodbye|good night)$/i.test(text)) {
    return "I’m here when you need the desktop again.";
  }
  if (/^(good morning|good afternoon)$/i.test(text)) {
    return "Hi — I’m here with you on the desktop.";
  }
  if (/^(nm|never ?mind|all good)$/i.test(text)) {
    return "Okay.";
  }
  return "Okay.";
}

/** Safe arithmetic — digits, + − * / % parentheses, and unicode minus. */
function evaluateArithmetic(expr: string): number | null {
  const cleaned = expr
    .replace(/×/g, "*")
    .replace(/÷/g, "/")
    .replace(/−/g, "-")
    .replace(/\s+/g, "");
  if (!cleaned || !/^[\d.+\-*/%()]+$/.test(cleaned)) {
    return null;
  }
  try {
    let i = 0;
    const peek = () => cleaned[i] ?? "";
    const consume = () => {
      const c = cleaned[i];
      i += 1;
      return c;
    };

    const parseExpr = (): number => {
      let v = parseTerm();
      while (peek() === "+" || peek() === "-") {
        const op = consume();
        const r = parseTerm();
        v = op === "+" ? v + r : v - r;
      }
      return v;
    };

    const parseTerm = (): number => {
      let v = parseFactor();
      while (peek() === "*" || peek() === "/" || peek() === "%") {
        const op = consume();
        const r = parseFactor();
        if (op === "*") v *= r;
        else if (op === "/") {
          if (r === 0) throw new Error("div0");
          v /= r;
        } else {
          if (r === 0) throw new Error("div0");
          v %= r;
        }
      }
      return v;
    };

    const parseFactor = (): number => {
      if (peek() === "+") {
        consume();
        return parseFactor();
      }
      if (peek() === "-") {
        consume();
        return -parseFactor();
      }
      if (peek() === "(") {
        consume();
        const v = parseExpr();
        if (peek() !== ")") throw new Error("paren");
        consume();
        return v;
      }
      let start = i;
      if (peek() === ".") {
        /* fraction */
      }
      while (/\d/.test(peek()) || peek() === ".") {
        consume();
      }
      if (start === i) throw new Error("num");
      const n = Number(cleaned.slice(start, i));
      if (!Number.isFinite(n)) throw new Error("nan");
      return n;
    };

    const value = parseExpr();
    if (i !== cleaned.length) return null;
    if (!Number.isFinite(value)) return null;
    return value;
  } catch {
    return null;
  }
}

function formatNumber(n: number): string {
  if (Number.isInteger(n)) return String(n);
  const rounded = Math.round(n * 1e6) / 1e6;
  return String(rounded);
}

function tryLocalArithmetic(text: string): string | null {
  const stripped = text
    .replace(/^(?:what(?:'s| is)|calculate|compute)\s+/i, "")
    .trim();

  const percentOf =
    stripped.match(/^(\d+(?:\.\d+)?)\s*%\s*(?:of\s+)?(\d+(?:\.\d+)?)$/i) ??
    stripped.match(/^(\d+(?:\.\d+)?)\s*percent\s+of\s+(\d+(?:\.\d+)?)$/i);
  if (percentOf) {
    const a = Number(percentOf[1]);
    const b = Number(percentOf[2]);
    if (!Number.isFinite(a) || !Number.isFinite(b)) return null;
    return formatNumber((a / 100) * b);
  }

  const expr = stripped;
  // Bare expression or “2 + 2”
  if (!/^[\d.+\-*/%()\s×÷−]+$/.test(expr)) {
    return null;
  }
  // Require an operator so bare numbers don't become "answers".
  if (!/[+\-*/%×÷]/.test(expr)) {
    return null;
  }
  const value = evaluateArithmetic(expr);
  if (value === null) return null;
  return formatNumber(value);
}

type UnitPair = {
  test: RegExp;
  convert: (n: number) => number;
  label: string;
};

const UNIT_CONVERSIONS: UnitPair[] = [
  {
    test: /^(-?\d+(?:\.\d+)?)\s*(?:°\s*)?c(?:elsius)?\s*(?:to|in|as)\s*f(?:ahrenheit)?$/i,
    convert: (c) => (c * 9) / 5 + 32,
    label: "°F",
  },
  {
    test: /^(-?\d+(?:\.\d+)?)\s*(?:°\s*)?f(?:ahrenheit)?\s*(?:to|in|as)\s*c(?:elsius)?$/i,
    convert: (f) => ((f - 32) * 5) / 9,
    label: "°C",
  },
  {
    test: /^(-?\d+(?:\.\d+)?)\s*(?:km|kilometers?|kilometres?)\s*(?:to|in)\s*(?:mi|miles?)$/i,
    convert: (km) => km * 0.621371,
    label: "miles",
  },
  {
    test: /^(-?\d+(?:\.\d+)?)\s*(?:mi|miles?)\s*(?:to|in)\s*(?:km|kilometers?|kilometres?)$/i,
    convert: (mi) => mi * 1.60934,
    label: "km",
  },
  {
    test: /^(-?\d+(?:\.\d+)?)\s*(?:kg|kilograms?)\s*(?:to|in)\s*(?:lb|lbs|pounds?)$/i,
    convert: (kg) => kg * 2.20462,
    label: "lb",
  },
  {
    test: /^(-?\d+(?:\.\d+)?)\s*(?:lb|lbs|pounds?)\s*(?:to|in)\s*(?:kg|kilograms?)$/i,
    convert: (lb) => lb / 2.20462,
    label: "kg",
  },
];

function tryLocalUnitConversion(text: string): string | null {
  const stripped = text
    .replace(/^(?:what(?:'s| is)|convert|how much is)\s+/i, "")
    .replace(/\?+$/g, "")
    .trim();
  for (const unit of UNIT_CONVERSIONS) {
    const m = stripped.match(unit.test);
    if (!m) continue;
    const n = Number(m[1]);
    if (!Number.isFinite(n)) continue;
    return `${formatNumber(unit.convert(n))} ${unit.label}`;
  }
  return null;
}

const TIMEZONE_ALIASES: { test: RegExp; zone: string; label: string }[] = [
  { test: /\b(utc|gmt)\b/i, zone: "UTC", label: "UTC" },
  { test: /\b(perth)\b/i, zone: "Australia/Perth", label: "Perth" },
  {
    test: /\b(western australia|wa time)\b/i,
    zone: "Australia/Perth",
    label: "Western Australia",
  },
  { test: /\b(sydney|nsw)\b/i, zone: "Australia/Sydney", label: "Sydney" },
  { test: /\b(brisbane|qld)\b/i, zone: "Australia/Brisbane", label: "Brisbane" },
  { test: /\b(melbourne|vic)\b/i, zone: "Australia/Melbourne", label: "Melbourne" },
  { test: /\b(adelaide|sa)\b/i, zone: "Australia/Adelaide", label: "Adelaide" },
  { test: /\b(tokyo|japan)\b/i, zone: "Asia/Tokyo", label: "Tokyo" },
  { test: /\b(london|uk)\b/i, zone: "Europe/London", label: "London" },
  { test: /\b(new york|nyc|et)\b/i, zone: "America/New_York", label: "New York" },
  {
    test: /\b(los angeles|la|pt|pacific)\b/i,
    zone: "America/Los_Angeles",
    label: "Los Angeles",
  },
];

function formatInZone(zone: string): string {
  return new Intl.DateTimeFormat(undefined, {
    timeZone: zone,
    weekday: "short",
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
    timeZoneName: "short",
  }).format(new Date());
}

function tryLocalDateTime(
  text: string,
): { kind: "answer" | "clarify"; reply: string } | null {
  // Ambiguous WA — ask once (Western Australia vs Washington).
  if (
    /\bwhat(?:'s| is)?\s+the\s+time\b/i.test(text) ||
    /\bwhat\s+time\b/i.test(text)
  ) {
    if (/\bin\s+wa\b/i.test(text) && !/\bwestern australia\b/i.test(text)) {
      return {
        kind: "clarify",
        reply:
          "Did you mean Western Australia or Washington state? I can answer either.",
      };
    }
    for (const alias of TIMEZONE_ALIASES) {
      if (alias.test.test(text)) {
        return {
          kind: "answer",
          reply: `In ${alias.label} it’s ${formatInZone(alias.zone)}.`,
        };
      }
    }
    if (
      /^(what(?:'s| is) the time|what time is it|what(?:'s| is) the current time)$/i.test(
        text,
      )
    ) {
      return {
        kind: "answer",
        reply: `It’s ${formatInZone(Intl.DateTimeFormat().resolvedOptions().timeZone)}.`,
      };
    }
  }

  if (
    /^(what(?:'s| is) (?:today'?s )?date|what day is it|what(?:'s| is) (?:the )?day)$/i.test(
      text,
    )
  ) {
    const formatted = new Intl.DateTimeFormat(undefined, {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    }).format(new Date());
    return { kind: "answer", reply: `Today is ${formatted}.` };
  }

  return null;
}

function isDesktopCapabilityMeta(text: string): boolean {
  if (isCapabilityDiscoveryUtterance(text)) return true;
  return (
    /\b(can you|do you|are you able to)\b/.test(text) &&
    /\b(manage|control|help with|handle|support|work with)\b/.test(text) &&
    /\b(windows?|desktop|screenshots?|clipboard|notifications?|apps?|applications?|browsers?)\b/.test(
      text,
    )
  );
}

function isReasoningProviderAsk(text: string): boolean {
  if (isCapabilityLimit(text)) return false;
  if (isDesktopCapabilityMeta(text)) return false;

  // Distance / travel / shipping / weather / open knowledge
  if (
    /\b(how (long|far)|distance|drive|flight|shipping|postage|freight|weather|forecast)\b/i.test(
      text,
    )
  ) {
    return true;
  }
  if (
    /\b(who (is|was|wrote|invented)|what is the capital|tell me a |joke|poem|recipe|homework|translate|define|meaning of)\b/i.test(
      text,
    )
  ) {
    return true;
  }
  if (
    /\b(explain|how (do|does|can|should) (i|you|we)|why (is|do|does)|plan|advice|suggest|recommend|help me (write|code|debug|design))\b/i.test(
      text,
    )
  ) {
    return true;
  }
  if (
    /\b(who|what|where|when|why|how)\b/i.test(text) &&
    text.length >= 12 &&
    !/\b(window|screenshot|clipboard|notification|open|launch|focus|minimi|maximi|snap|beside)\b/i.test(
      text,
    )
  ) {
    return true;
  }
  return false;
}

function splitHybrid(text: string): { reason: string; openTarget: string } | null {
  const m = text.match(
    /^(.+?)\s+(?:and|then)\s+(?:please\s+)?(?:open|launch|show|bring up)\s+(.+)$/i,
  );
  if (!m) return null;
  const reason = m[1]!.trim();
  const openTarget = m[2]!.trim();
  if (!reason || !openTarget) return null;
  // Avoid stealing compound opens: "open A and open B"
  if (/^(open|launch)\b/i.test(reason)) return null;
  return { reason, openTarget };
}

/**
 * Classify intelligence kind for an utterance that did not already match a
 * desktop IntentAction. Returns null when the existing fallthrough should run
 * unchanged (true DESKTOP misses already handled earlier in the pipeline).
 */
export function classifyIntelligenceKind(raw: string): IntelligenceKind | null {
  const text = normalize(raw);
  if (!text) return null;

  if (isCapabilityLimit(text)) return "CAPABILITY_LIMIT";
  if (isOwnerAuthorization(text)) return "OWNER_AUTHORIZATION";

  const hybrid = splitHybrid(text);
  if (hybrid) {
    const entity = resolveDesktopEntity(hybrid.openTarget);
    if (entity) return "HYBRID";
  }

  if (isDesktopCapabilityMeta(text)) return "REASONING_LOCAL";
  if (isConversationalLocal(text)) return "REASONING_LOCAL";

  const dt = tryLocalDateTime(text);
  if (dt?.kind === "clarify") return "CLARIFICATION";
  if (dt?.kind === "answer") return "REASONING_LOCAL";
  if (tryLocalArithmetic(text) !== null) return "REASONING_LOCAL";
  if (tryLocalUnitConversion(text) !== null) return "REASONING_LOCAL";

  if (isReasoningProviderAsk(text)) return "REASONING_PROVIDER";

  return null;
}

function openTargetAction(
  openTarget: string,
  reply: string,
): IntentAction | null {
  const entity = resolveDesktopEntity(openTarget);
  if (!entity) return null;
  if (entity.kind === "site") {
    return {
      kind: "browserOpen",
      url: entity.value,
      reply,
    };
  }
  if (
    entity.kind === "application" ||
    entity.kind === "browser" ||
    entity.kind === "protocol" ||
    entity.kind === "shell"
  ) {
    return {
      kind: "appOpen",
      query: entity.openQuery ?? entity.value,
      reply,
    };
  }
  return null;
}

/**
 * Resolve a non-desktop IntentAction for intelligence routing.
 * Returns null to preserve existing CAPABILITY_LIMIT / unknown walls.
 */
export function resolveIntelligenceRoute(raw: string): IntentAction | null {
  const original = raw.trim();
  const text = normalize(original);
  if (!text) return null;

  const kind = classifyIntelligenceKind(original);
  if (!kind) return null;

  // Preserve existing truthful walls / Moments paths.
  if (kind === "CAPABILITY_LIMIT" || kind === "OWNER_AUTHORIZATION") {
    return null;
  }

  if (kind === "CLARIFICATION") {
    const dt = tryLocalDateTime(text);
    if (dt?.kind === "clarify") {
      return { kind: "unknown", reply: dt.reply };
    }
  }

  if (kind === "HYBRID") {
    const hybrid = splitHybrid(text);
    if (!hybrid) return null;
    const local =
      tryLocalArithmetic(normalize(hybrid.reason)) ??
      tryLocalUnitConversion(normalize(hybrid.reason));
    if (local) {
      const action = openTargetAction(
        hybrid.openTarget,
        `${local}. Opening ${hybrid.openTarget.replace(/[.!?]+$/g, "")}.`,
      );
      if (action) return action;
    }
    // Honest hybrid when local can't answer (e.g. live CPU): open the tool.
    if (
      /\b(cpu|memory|ram|disk|storage|performance|task manager)\b/i.test(
        hybrid.reason,
      )
    ) {
      const action = openTargetAction(
        hybrid.openTarget,
        `I can’t read live system stats from Conversation — opening ${hybrid.openTarget.replace(/[.!?]+$/g, "")} so you can see them there.`,
      );
      if (action) return action;
    }
    // Reasoning + open ChatGPT / known site
    const entity = resolveDesktopEntity(hybrid.openTarget);
    if (entity) {
      const action = openTargetAction(
        hybrid.openTarget,
        `Opening ${entity.label}. I won’t invent the answer here — continue the question there if you need more.`,
      );
      if (action) return action;
    }
    return null;
  }

  if (kind === "REASONING_LOCAL") {
    if (isDesktopCapabilityMeta(text) || isCapabilityDiscoveryUtterance(text)) {
      const discovery = generateCapabilityDiscovery(resolveDiscoveryScope(text));
      return {
        kind: "capabilityExplain",
        reply: discovery.reply,
        suggestion: discovery.suggestion,
      };
    }
    if (isConversationalLocal(text)) {
      return { kind: "unknown", reply: conversationalReply(text) };
    }
    const dt = tryLocalDateTime(text);
    if (dt?.kind === "answer") {
      return { kind: "unknown", reply: dt.reply };
    }
    const arithmetic = tryLocalArithmetic(text);
    if (arithmetic !== null) {
      return { kind: "unknown", reply: arithmetic };
    }
    const units = tryLocalUnitConversion(text);
    if (units !== null) {
      return { kind: "unknown", reply: units };
    }
  }

  if (kind === "REASONING_PROVIDER") {
    const url = chatgptReasoningUrl(original);
    return {
      kind: "browserOpen",
      url,
      reply:
        "Opening ChatGPT with your question — reasoning continues there. I won’t invent an answer here.",
      // P23.S2: this route, not the comprehended outcome, selected the handoff.
      informationHandoff: true,
    };
  }

  return null;
}
