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
    /\b(collapse|minimize|minimise|hide|float|icon)\b/.test(text) ||
    text === "mode 1"
  ) {
    return {
      kind: "collapse",
      reply:
        "Collapsing to the desktop operator. The conversation window closes — click the floating W when you need Workspace again.",
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
        "Expanding around this conversation. Ask when you need Save, restore, or other tools — I won’t show a feature dashboard.",
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
        "Opening Settings. Preferences that aren’t wired yet stay listed as not available — I won’t invent controls.",
    };
  }

  const launch = raw.trim().match(/^(?:open|launch|start)\s+(.+)$/i);
  if (launch?.[1]) {
    const target = launch[1].trim().replace(/[.!?]+$/g, "");
    return {
      kind: "unknown",
      reply: `I can’t launch “${target}” from conversation yet — and I won’t pretend it opened. Desktop launch isn’t wired through this shell.`,
      suggestion:
        "Closest available: “save this”, “continue”, or “restore <Moment name>” for interruption recovery.",
    };
  }

  return {
    kind: "unknown",
    reply:
      "I don’t have that yet — and I won’t invent it. Closest available: Save, Continue, Moments, Check-in, Guide, expand Workspace, or propose a change.",
    suggestion: "Try “save this”, “continue”, “expand”, or “Add a screenshot button”.",
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
