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
import type { PilotPrimaryView } from "./pilotChrome";

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
  | { kind: "developer"; enabled: boolean; reply: string }
  | { kind: "proposal"; reply: string }
  | { kind: "clipboardRead"; reply: string }
  | { kind: "clipboardWrite"; text: string; reply: string }
  | { kind: "appOpen"; query: string; reply: string }
  | { kind: "appLaunch"; query: string; reply: string }
  | { kind: "appFocus"; query: string; reply: string }
  | { kind: "appClose"; query: string; reply: string }
  | { kind: "appMinimize"; query: string; reply: string }
  | { kind: "appRestore"; query: string; reply: string }
  | { kind: "appEnumerate"; reply: string }
  | { kind: "winEnumerate"; reply: string }
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
    .replace(/\s+/g, " ");
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
  return q.replace(/^(the|my)\s+/i, "").trim() || "this";
}

/**
 * Architecture-driven Window intents (Conversation language → Window operations).
 * Must run before capability-evolution proposals that also match move/resize/show.
 */
function resolveWindowIntent(raw: string, text: string): IntentAction | null {
  const utterance = stripTrailingPunctuation(raw);

  if (
    /\b(what windows are open|which windows are open|show( me)?( my)?( open)? windows|list( (all|my|open))? windows|open windows|what('?s| is) open on (my |the )?desktop)\b/.test(
      text,
    ) ||
    text === "windows" ||
    text === "what windows" ||
    text === "show windows"
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
    utterance.match(/^(?:bring)\s+(.+?)\s+to\s+(?:the\s+)?front$/i) ??
    utterance.match(/^(?:focus(?:\s+window)?|activate)\s+(.+)$/i);
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
 */
export function resolveIntent(raw: string): IntentAction {
  const text = normalize(raw);
  if (!text) {
    return {
      kind: "unknown",
      reply: "Waiting.",
    };
  }

  if (
    /^(hi|hello|hey)\b/.test(text) ||
    text === "hi" ||
    text === "hello"
  ) {
    return {
      kind: "unknown",
      reply: "Here when you need the desktop.",
    };
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
  const windowIntent = resolveWindowIntent(raw, text);
  if (windowIntent) {
    return windowIntent;
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

  if (
    /\b(guide|help|how (does|do)|what (do you|can you)|limits?|trust)\b/.test(
      text,
    )
  ) {
    return {
      kind: "navigate",
      view: "help",
      reply: "Opening Guide.",
    };
  }

  if (/\b(settings|preferences|options)\b/.test(text)) {
    return {
      kind: "settings",
      reply:
        "There’s no Settings surface in this shell. Collapse returns to the desktop operator; Exit Workspace quits. Ask Guide for trust limits.",
    };
  }

  const clipboardWrite = raw
    .trim()
    .match(
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
      text,
    ) ||
    text === "clipboard?"
  ) {
    return {
      kind: "clipboardRead",
      reply: "Checking the clipboard.",
    };
  }

  if (
    /\b(list (running )?apps?|list applications|what('?s| is) running|running applications)\b/.test(
      text,
    )
  ) {
    return {
      kind: "appEnumerate",
      reply: "Listing running applications.",
    };
  }

  const appClose = raw
    .trim()
    .match(/^(?:close|quit|exit)\s+(.+)$/i);
  if (appClose?.[1]) {
    const query = stripTrailingPunctuation(appClose[1]);
    if (query && !/^(workspace|conversation)$/i.test(query)) {
      return {
        kind: "appClose",
        query,
        reply: `Closing “${query}”.`,
      };
    }
  }

  const appFocus = raw
    .trim()
    .match(/^(?:switch\s+to|bring\s+up)\s+(.+)$/i);
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

  const appLaunchExplicit = raw
    .trim()
    .match(/^(?:launch|start)\s+(.+)$/i);
  if (appLaunchExplicit?.[1]) {
    const query = stripTrailingPunctuation(appLaunchExplicit[1]);
    if (query) {
      return {
        kind: "appLaunch",
        query,
        reply: `Launching “${query}”.`,
      };
    }
  }

  const appOpen = raw.trim().match(/^open\s+(.+)$/i);
  if (appOpen?.[1]) {
    const query = stripTrailingPunctuation(appOpen[1]);
    if (query && !/^(workspace|conversation)$/i.test(query)) {
      return {
        kind: "appOpen",
        query,
        reply: `Opening “${query}”.`,
      };
    }
  }

  return {
    kind: "unknown",
    reply:
      "I don’t have that yet — and I won’t invent it. Closest available: window control, open/launch apps, Save, Continue, Moments, Check-in, or Guide.",
    suggestion:
      "Try “what windows are open?”, “maximize Cursor”, “move this window to the left”, “open notepad”, or “save this”.",
  };
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
