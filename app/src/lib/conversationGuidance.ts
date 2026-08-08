/**
 * Conversation quality after transcription — deterministic, no AI guessing.
 * Companion voice: truthful limits without Help-catalogue churn (T1).
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
    .replace(/^(i\s+think)\s+/i, "")
    .replace(/^(i\s+want\s+you\s+to|i\s+need\s+you\s+to|i\s+need\s+to)\s+/i, "")
    .replace(/^(just\s+)?(go\s+ahead\s+and)\s+/i, "")
    .replace(/^(try\s+to|help\s+me)\s+/i, "")
    .replace(/\s+please[.!?]*$/i, "")
    .replace(/\s+for\s+me[.!?]*$/i, "")
    .replace(/\s+now[.!?]*$/i, "")
    .replace(/\s+thanks[.!?]*$/i, "")
    .replace(/[.!?]+$/g, "")
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
    reply: `Heard you: “${preview}”. What do you need on the desktop?`,
  };
}

/** Calm companion greeting — invite conversation, never a feature catalogue. */
export function companionGreetingReply(): UnknownGuidance {
  return {
    kind: "unknown",
    reply: "Hi — I’m here with you on the desktop.",
  };
}

const GENERIC_REPLIES = [
  "I’m not sure how to help with that yet — what are you trying to finish?",
  "I don’t have a path for that yet. Want to try a desktop step, or ask another way?",
  "I can help with desktop work I can complete, or hand broader questions to ChatGPT.",
  "I can’t take that on yet.",
] as const;

type NearMiss = {
  test: (text: string) => boolean;
  reply: string;
  /** Single collaborative recovery line — never a command catalogue. */
  recovery?: string;
};

const NEAR_MISSES: NearMiss[] = [
  {
    test: (t) =>
      /\b(file|files|folder|folders|directory|directories|document|documents)\b/.test(
        t,
      ),
    reply: "I can’t work with files and folders yet.",
    recovery:
      "If something should be open on the desktop, name the app or window and I’ll take it from there.",
  },
  {
    test: (t) =>
      /\b(terminal|command prompt|powershell|shell|console|cmd)\b/.test(t),
    reply: "I can’t run terminal commands yet.",
    recovery:
      "I can open Terminal as an app if that helps — or we can keep working with windows you already have open.",
  },
  {
    test: (t) =>
      /\b(screenshot|screen shot|capture|screengrab)\b/.test(t),
    reply: "I can capture the screen when the ask is clear.",
    recovery: "Want the whole desktop, this window, or a copy to the clipboard?",
  },
  {
    test: (t) =>
      /\b(window|windows|monitor|snap|maximize|minimise|minimize)\b/.test(t),
    reply: "I can work with open windows when I know which one you mean.",
    recovery: "Name the window, or ask what’s open and we’ll pick from there.",
  },
  {
    test: (t) =>
      /\b(browser|chrome|edge|firefox|brave|website|web page|url)\b/.test(t),
    reply: "I can open sites and bring browsers forward.",
    recovery: "Which site or browser should we use?",
  },
  {
    test: (t) =>
      /\b(notif|notify|toast|alert me|remind)\b/.test(t),
    reply: "I can show a desktop notification when you ask for one.",
    recovery:
      "Tell me the message to show — watching for when something finishes isn’t available yet.",
  },
  {
    test: (t) => /\b(clipboard|paste|copy that)\b/.test(t),
    reply: "I can read or write the clipboard when you ask.",
    recovery: "Want me to read what’s there, or copy something specific?",
  },
  {
    test: (t) =>
      /\b(voice|microphone|mic|speak|talk|listen)\b/.test(t),
    reply:
      "Use the microphone beside the message box to speak — I’ll put what I hear into Conversation, same as typing.",
  },
];

let lastUnknownReply = "";
let lastSuggestion = "";
let genericCursor = 0;

/** Test helper — clears rotation so cases stay deterministic. */
export function resetConversationGuidanceState(): void {
  lastUnknownReply = "";
  lastSuggestion = "";
  genericCursor = 0;
}

/** Avoid repeating the same recovery line on consecutive soft misses. */
function maybeRecovery(recovery?: string): string | undefined {
  if (!recovery) {
    return undefined;
  }
  if (recovery === lastSuggestion) {
    return undefined;
  }
  lastSuggestion = recovery;
  return recovery;
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
      };
    }
  }
  const reply = `Still can’t help with “${text.slice(0, 40)}${text.length > 40 ? "…" : ""}”.`;
  lastUnknownReply = reply;
  return {
    kind: "unknown",
    reply,
  };
}

/**
 * Truthful unsupported guidance — companion voice, recovery only when it helps.
 */
export function resolveUnknownGuidance(text: string): UnknownGuidance {
  for (const miss of NEAR_MISSES) {
    if (miss.test(text)) {
      if (miss.reply !== lastUnknownReply) {
        lastUnknownReply = miss.reply;
        return {
          kind: "unknown",
          reply: miss.reply,
          suggestion: maybeRecovery(miss.recovery),
        };
      }
      // Same near-miss twice → rotate generic; still no command catalogue.
      return pickGeneric(text);
    }
  }
  return pickGeneric(text);
}
