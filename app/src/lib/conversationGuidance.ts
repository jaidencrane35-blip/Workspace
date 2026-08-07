/**
 * Conversation quality after transcription — deterministic, no AI guessing.
 * Unsupported requests get truthful, varied guidance (never identical churn).
 */

export type UnknownGuidance = {
  kind: "unknown";
  reply: string;
  suggestion?: string;
};

export type HeardGuidance = {
  kind: "unknown";
  reply: string;
  suggestion?: string;
};

/** Strip polite wrappers so ordinary phrasing still matches deterministic intents. */
export function softenUtterance(text: string): string {
  return text
    .replace(/^(please|kindly)\s+/i, "")
    .replace(
      /^(could you|would you|can you|will you)\s+(please\s+)?/i,
      "",
    )
    .replace(/^(i\s+want\s+you\s+to|i\s+need\s+you\s+to|i\s+need\s+to)\s+/i, "")
    .replace(/^(just\s+)?(go\s+ahead\s+and)\s+/i, "")
    .replace(/\s+please$/i, "")
    .replace(/\s+for\s+me$/i, "")
    .trim();
}

/** Voice / mic check phrases — acknowledge hearing, do not treat as desktop failure. */
export function isVoiceCheckUtterance(text: string): boolean {
  if (
    /^(testing|test)(\s+(testing|test|\d+))*$/i.test(text) ||
    /^(mic|microphone)\s*check$/i.test(text) ||
    /^(check\s+check|one two three|1 2 3)$/i.test(text) ||
    /\bsally sells seashells\b/i.test(text) ||
    /\bshe sells seashells\b/i.test(text) ||
    /\bpeter piper\b/i.test(text) ||
    /\bhow much wood\b/i.test(text) ||
    /\bred leather yellow leather\b/i.test(text)
  ) {
    return true;
  }
  // Short digit / letter drills after transcription (e.g. “testing testing 123”).
  if (/^(testing|test|check)(\s+\w+){0,6}$/i.test(text) && text.length <= 48) {
    const words = text.split(/\s+/);
    return words.every((w) =>
      /^(testing|test|check|mic|one|two|three|four|five|six|seven|eight|nine|zero|\d+)$/i.test(
        w,
      ),
    );
  }
  return false;
}

export function voiceCheckReply(raw: string): HeardGuidance {
  const heard = raw.trim().replace(/\s+/g, " ");
  const preview =
    heard.length > 72 ? `${heard.slice(0, 69).trimEnd()}…` : heard;
  return {
    kind: "unknown",
    reply: `Heard you: “${preview}”.`,
    suggestion:
      "Say what you need on the desktop — for example “open ChatGPT”, “take a screenshot”, or “what windows are open?”.",
  };
}

const GENERIC_REPLIES = [
  "I can’t do that on the desktop yet — and I won’t invent it.",
  "That’s outside what I can operate right now.",
  "I stay with desktop work I can actually complete.",
  "I’m not set up for that request yet.",
] as const;

const GENERIC_SUGGESTIONS = [
  'Try “what windows are open?”, “open ChatGPT”, “take a screenshot”, or “bring Chrome to the front”.',
  'Nearby: open a site or app, arrange windows, capture the screen, or Save / Continue.',
  'You can ask me to open apps, manage windows, take screenshots, or open sites in your browser.',
  'If you want desktop help, try “open my browser”, “list apps”, or “save this”.',
] as const;

type NearMiss = {
  test: (text: string) => boolean;
  reply: string;
  suggestion: string;
};

const NEAR_MISSES: NearMiss[] = [
  {
    test: (t) =>
      /\b(file|files|folder|folders|directory|directories|document|documents)\b/.test(
        t,
      ),
    reply:
      "I can’t work with files and folders yet — and I won’t pretend I can.",
    suggestion:
      'For now I can open apps and sites, arrange windows, take screenshots, or Save / Continue. Try “open Explorer” or “what windows are open?”.',
  },
  {
    test: (t) =>
      /\b(terminal|command prompt|powershell|shell|console|cmd)\b/.test(t),
    reply: "I can’t run terminal commands yet — and I won’t invent output.",
    suggestion:
      'I can open apps, arrange windows, or take screenshots. Try “open Windows Terminal” or “what windows are open?”.',
  },
  {
    test: (t) =>
      /\b(weather|joke|story|poem|recipe|homework|translate|define|meaning of)\b/.test(
        t,
      ) ||
      /\b(who is|what is the capital|tell me a)\b/.test(t),
    reply:
      "I’m here to operate your desktop — not general chat or look-ups.",
    suggestion:
      'Ask me to open something, arrange windows, take a screenshot, or Save / Continue.',
  },
  {
    test: (t) =>
      /\b(screenshot|screen shot|capture|screengrab)\b/.test(t),
    reply: "I can capture the screen when you ask directly.",
    suggestion:
      'Try “take a screenshot”, “screenshot this window”, or “take a screenshot and copy it”.',
  },
  {
    test: (t) =>
      /\b(window|windows|monitor|snap|maximize|minimise|minimize)\b/.test(t),
    reply: "I can work with open windows when the ask is clear.",
    suggestion:
      'Try “what windows are open?”, “bring Chrome to the front”, or “snap this window left”.',
  },
  {
    test: (t) =>
      /\b(browser|chrome|edge|firefox|brave|website|web page|url)\b/.test(t),
    reply: "I can open sites and bring browsers forward.",
    suggestion:
      'Try “open my browser”, “open GitHub”, “open ChatGPT beside Cursor”, or “bring Chrome to the front”.',
  },
  {
    test: (t) =>
      /\b(notif|notify|toast|alert me|remind)\b/.test(t),
    reply: "I can show a desktop notification when you ask for one.",
    suggestion:
      'Try “show me a notification” or “notify me that the build finished.” Watching for when something finishes isn’t available yet.',
  },
  {
    test: (t) => /\b(clipboard|paste|copy that)\b/.test(t),
    reply: "I can read or write the clipboard when you ask.",
    suggestion:
      'Try “what’s on my clipboard?” or “copy to clipboard: hello”.',
  },
  {
    test: (t) =>
      /\b(voice|microphone|mic|speak|talk|listen)\b/.test(t),
    reply:
      "Use the microphone beside the message box to speak — I’ll put what I hear into Conversation, same as typing.",
    suggestion:
      'Or ask “can you hear me?” / “what can you do with voice?”',
  },
];

let lastUnknownReply = "";
let genericCursor = 0;

/** Test helper — clears rotation so cases stay deterministic. */
export function resetConversationGuidanceState(): void {
  lastUnknownReply = "";
  genericCursor = 0;
}

function pickGeneric(text: string): UnknownGuidance {
  const start = genericCursor % GENERIC_REPLIES.length;
  for (let offset = 0; offset < GENERIC_REPLIES.length; offset += 1) {
    const idx = (start + offset) % GENERIC_REPLIES.length;
    const reply = GENERIC_REPLIES[idx]!;
    if (reply !== lastUnknownReply) {
      genericCursor = idx + 1;
      lastUnknownReply = reply;
      return {
        kind: "unknown",
        reply,
        suggestion: GENERIC_SUGGESTIONS[idx]!,
      };
    }
  }
  const reply = `Still can’t help with “${text.slice(0, 40)}${text.length > 40 ? "…" : ""}” — I won’t invent a desktop action.`;
  lastUnknownReply = reply;
  return {
    kind: "unknown",
    reply,
    suggestion: GENERIC_SUGGESTIONS[start]!,
  };
}

/**
 * Truthful unsupported guidance — varies wording, suggests nearby desktop work.
 */
export function resolveUnknownGuidance(text: string): UnknownGuidance {
  for (const miss of NEAR_MISSES) {
    if (miss.test(text)) {
      if (miss.reply !== lastUnknownReply) {
        lastUnknownReply = miss.reply;
        return {
          kind: "unknown",
          reply: miss.reply,
          suggestion: miss.suggestion,
        };
      }
      // Same near-miss twice → rotate into a sibling generic with the same suggestion.
      const rotated = pickGeneric(text);
      return {
        ...rotated,
        suggestion: miss.suggestion,
      };
    }
  }
  return pickGeneric(text);
}
