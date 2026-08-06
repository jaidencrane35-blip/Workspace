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
  | { kind: "unknown"; reply: string; suggestion?: string };

function normalize(input: string): string {
  return input
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/\s+/g, " ");
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

  const namedMoment = raw
    .trim()
    .match(/^(?:restore|continue|resume|open\s+moment)\s+(.+)$/i);
  if (namedMoment?.[1]) {
    const nameQuery = namedMoment[1].trim().replace(/[.!?]+$/g, "");
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
    /\b(continue|restore|resume)\b/.test(text) ||
    /\byesterday\b/.test(text) ||
    text === "restore workspace" ||
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
        reply: "Writing to the clipboard through Capability Runtime.",
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
      reply: "Reading the clipboard through Capability Runtime.",
    };
  }

  if (
    /\b(list (running )?apps?|list applications|what('?s| is) (running|open)|running applications)\b/.test(
      text,
    )
  ) {
    return {
      kind: "appEnumerate",
      reply: "Listing running application windows through Capability Runtime.",
    };
  }

  const appClose = raw
    .trim()
    .match(/^(?:close|quit|exit)\s+(.+)$/i);
  if (appClose?.[1]) {
    const query = appClose[1].trim().replace(/[.!?]+$/g, "");
    if (query && !/^(workspace|conversation)$/i.test(query)) {
      return {
        kind: "appClose",
        query,
        reply: `Closing “${query}” through Application Provider.`,
      };
    }
  }

  const appMinimize = raw.trim().match(/^minimize\s+(.+)$/i);
  if (appMinimize?.[1]) {
    const query = appMinimize[1].trim().replace(/[.!?]+$/g, "");
    if (query) {
      return {
        kind: "appMinimize",
        query,
        reply: `Minimizing “${query}” through Application Provider.`,
      };
    }
  }

  const appRestore = raw
    .trim()
    .match(/^(?:unminimize|restore\s+(?:window|app))\s+(.+)$/i);
  if (appRestore?.[1]) {
    const query = appRestore[1].trim().replace(/[.!?]+$/g, "");
    if (query) {
      return {
        kind: "appRestore",
        query,
        reply: `Restoring “${query}” through Application Provider.`,
      };
    }
  }

  const appFocus = raw
    .trim()
    .match(/^(?:switch\s+to|focus|bring\s+(?:up|to front))\s+(.+)$/i);
  if (appFocus?.[1]) {
    const query = appFocus[1].trim().replace(/[.!?]+$/g, "");
    if (query) {
      return {
        kind: "appFocus",
        query,
        reply: `Focusing “${query}” through Application Provider.`,
      };
    }
  }

  const appLaunchExplicit = raw
    .trim()
    .match(/^(?:launch|start)\s+(.+)$/i);
  if (appLaunchExplicit?.[1]) {
    const query = appLaunchExplicit[1].trim().replace(/[.!?]+$/g, "");
    if (query) {
      return {
        kind: "appLaunch",
        query,
        reply: `Launching “${query}” through Application Provider.`,
      };
    }
  }

  const appOpen = raw.trim().match(/^open\s+(.+)$/i);
  if (appOpen?.[1]) {
    const query = appOpen[1].trim().replace(/[.!?]+$/g, "");
    if (query && !/^(workspace|conversation)$/i.test(query)) {
      return {
        kind: "appOpen",
        query,
        reply: `Opening “${query}” through Application Provider (focus if running, else launch).`,
      };
    }
  }

  return {
    kind: "unknown",
    reply:
      "I don’t have that yet — and I won’t invent it. Closest available: Save, Continue, Moments, Check-in, Guide, open/launch apps, or propose a change.",
    suggestion:
      "Try “open notepad”, “list apps”, “switch to Chrome”, “save this”, or “continue”.",
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
