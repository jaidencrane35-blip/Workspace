/**
 * Capability Evolution Foundation — proposals only.
 * Workspace must NOT rewrite itself. Approved items enter the engineering backlog
 * and execute only through constitutional engineering.
 */

export type ProposalCategory =
  | "user_preference"
  | "workflow"
  | "bug"
  | "capability"
  | "product_enhancement";

export type ProposalStatus =
  | "draft"
  | "proposed"
  | "approved"
  | "implementing"
  | "shipped"
  | "rejected"
  | "rolled_back";

export interface CapabilityProposal {
  id: string;
  createdAt: string;
  updatedAt: string;
  utterance: string;
  category: ProposalCategory;
  title: string;
  summary: string;
  impact: string;
  implementationPlan: string[];
  status: ProposalStatus;
  version: number;
  audit: Array<{ at: string; event: string; detail?: string }>;
}

export interface CapabilityEvolutionState {
  schemaVersion: 1;
  proposals: CapabilityProposal[];
  approvedBacklog: string[];
}

const STORAGE_KEY = "workspace.capabilityEvolution.v1";

const EMPTY: CapabilityEvolutionState = {
  schemaVersion: 1,
  proposals: [],
  approvedBacklog: [],
};

/** In-memory fallback for non-browser / test environments. */
const memoryStore = new Map<string, string>();

function storageGet(key: string): string | null {
  try {
    if (typeof localStorage !== "undefined") {
      return localStorage.getItem(key);
    }
  } catch {
    /* ignore */
  }
  return memoryStore.get(key) ?? null;
}

function storageSet(key: string, value: string): void {
  try {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(key, value);
      return;
    }
  } catch {
    /* fall through */
  }
  memoryStore.set(key, value);
}

function nowIso(): string {
  return new Date().toISOString();
}

function newId(): string {
  return `prop-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 7)}`;
}

export function loadEvolutionState(): CapabilityEvolutionState {
  try {
    const raw = storageGet(STORAGE_KEY);
    if (!raw) {
      return { ...EMPTY, proposals: [], approvedBacklog: [] };
    }
    const parsed = JSON.parse(raw) as CapabilityEvolutionState;
    if (parsed.schemaVersion !== 1 || !Array.isArray(parsed.proposals)) {
      return { ...EMPTY, proposals: [], approvedBacklog: [] };
    }
    return {
      schemaVersion: 1,
      proposals: parsed.proposals,
      approvedBacklog: Array.isArray(parsed.approvedBacklog)
        ? parsed.approvedBacklog
        : [],
    };
  } catch {
    return { ...EMPTY, proposals: [], approvedBacklog: [] };
  }
}

export function saveEvolutionState(state: CapabilityEvolutionState): void {
  storageSet(STORAGE_KEY, JSON.stringify(state));
}

function normalize(input: string): string {
  return input
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/\s+/g, " ");
}

/** Detect whether an utterance is a product-change request vs an operational intent. */
export function isEvolutionRequest(raw: string): boolean {
  const text = normalize(raw);
  if (!text) {
    return false;
  }
  return (
    /^(add|move|make|resize|change|remove|hide|show|enable|disable|put|relocate)\b/.test(
      text,
    ) ||
    /\b(add a |make the |move the |resize the |change the )\b/.test(text) ||
    /\b(screenshot button|transparent|background|chat (to|on|left|right)|operator)\b/.test(
      text,
    )
  );
}

export function classifyEvolutionRequest(raw: string): {
  category: ProposalCategory;
  title: string;
  summary: string;
  impact: string;
  implementationPlan: string[];
} {
  const text = normalize(raw);

  if (
    /\b(bug|broken|crash|error|fix|doesn't work|does not work)\b/.test(text)
  ) {
    return {
      category: "bug",
      title: shortenTitle(raw, "Reported defect"),
      summary: `Treat as a defect report: “${raw.trim()}”.`,
      impact:
        "May affect reliability or trust. No automatic code change — enters engineering triage after approval.",
      implementationPlan: [
        "Reproduce under constitutional permission boundaries",
        "Add or extend a verifier/test if feasible",
        "Ship fix via a dedicated execution program",
      ],
    };
  }

  if (
    /\b(workflow|every time|always|automate|shortcut)\b/.test(text) ||
    /\b(when i|whenever)\b/.test(text)
  ) {
    return {
      category: "workflow",
      title: shortenTitle(raw, "Workflow adjustment"),
      summary: `Workflow request: “${raw.trim()}”.`,
      impact:
        "Changes how repeated work is expressed. Preferences may apply immediately later; new automation requires approval + engineering.",
      implementationPlan: [
        "Map to existing Product Proof / intent bridge surfaces",
        "Prefer preference or intent routing over new architecture",
        "Implement only through an approved execution program",
      ],
    };
  }

  if (
    /\b(transparent|opacity|theme|colour|color|font|size|resize|move the chat|move chat|background)\b/.test(
      text,
    ) ||
    /^(make|move|resize)\b/.test(text)
  ) {
    return {
      category: "user_preference",
      title: shortenTitle(raw, "Operator preference"),
      summary: `Presentation preference: “${raw.trim()}”.`,
      impact:
        "Local presentation only. Must not bypass permission model or invent desktop capabilities.",
      implementationPlan: [
        "Store as operator preference when a setting exists",
        "Otherwise keep as approved backlog item for a UX execution program",
        "No privileged desktop action without consent",
      ],
    };
  }

  if (
    /\b(button|feature|capability|screenshot|launch|integrate|support)\b/.test(
      text,
    ) ||
    /^add\b/.test(text)
  ) {
    return {
      category: "capability",
      title: shortenTitle(raw, "New capability"),
      summary: `Capability request: “${raw.trim()}”.`,
      impact:
        "Would expand product surface. Requires constitutional review, ownership classification, and verification — Workspace will not rewrite itself.",
      implementationPlan: [
        "Classify Owns / Wraps / Adapts / Studies / Rejects",
        "Propose IPC tier + permission path if desktop mutation is involved",
        "Implement only via constitutional execution after approval",
      ],
    };
  }

  return {
    category: "product_enhancement",
    title: shortenTitle(raw, "Product enhancement"),
    summary: `Enhancement request: “${raw.trim()}”.`,
    impact:
      "Product-direction change. Tracked for owner approval; execution remains human-gated engineering.",
    implementationPlan: [
      "Record proposal + audit event",
      "Await explicit approval",
      "Schedule as a single execution program if approved",
    ],
  };
}

function shortenTitle(raw: string, fallback: string): string {
  const t = raw.trim().replace(/\s+/g, " ");
  if (!t) {
    return fallback;
  }
  return t.length > 72 ? `${t.slice(0, 69)}…` : t;
}

export function createProposal(utterance: string): CapabilityProposal {
  const classified = classifyEvolutionRequest(utterance);
  const at = nowIso();
  return {
    id: newId(),
    createdAt: at,
    updatedAt: at,
    utterance: utterance.trim(),
    category: classified.category,
    title: classified.title,
    summary: classified.summary,
    impact: classified.impact,
    implementationPlan: classified.implementationPlan,
    status: "proposed",
    version: 1,
    audit: [{ at, event: "proposed", detail: classified.category }],
  };
}

export function formatProposalReply(proposal: CapabilityProposal): string {
  const plan = proposal.implementationPlan
    .map((step, i) => `${i + 1}. ${step}`)
    .join("\n");
  return [
    `Proposal ${proposal.id} (${proposal.category.replace(/_/g, " ")}).`,
    proposal.summary,
    `Impact: ${proposal.impact}`,
    "Plan:",
    plan,
    "I will not change the product until you approve. Say “approve proposal” or “reject proposal”.",
  ].join("\n");
}

export function appendProposal(
  state: CapabilityEvolutionState,
  proposal: CapabilityProposal,
): CapabilityEvolutionState {
  return {
    ...state,
    proposals: [proposal, ...state.proposals].slice(0, 100),
  };
}

export function latestProposed(
  state: CapabilityEvolutionState,
): CapabilityProposal | undefined {
  return state.proposals.find((p) => p.status === "proposed");
}

export function setProposalStatus(
  state: CapabilityEvolutionState,
  id: string,
  status: ProposalStatus,
  detail?: string,
): CapabilityEvolutionState {
  const at = nowIso();
  const proposals = state.proposals.map((p) => {
    if (p.id !== id) {
      return p;
    }
    return {
      ...p,
      status,
      updatedAt: at,
      version: p.version + 1,
      audit: [...p.audit, { at, event: status, detail }],
    };
  });
  let approvedBacklog = state.approvedBacklog.filter((x) => x !== id);
  if (status === "approved" || status === "implementing") {
    approvedBacklog = [id, ...approvedBacklog];
  }
  if (status === "rejected" || status === "rolled_back" || status === "shipped") {
    approvedBacklog = approvedBacklog.filter((x) => x !== id);
  }
  return { schemaVersion: 1, proposals, approvedBacklog };
}

export function undoLastStatusChange(
  state: CapabilityEvolutionState,
  id: string,
): CapabilityEvolutionState | null {
  const proposal = state.proposals.find((p) => p.id === id);
  if (!proposal || proposal.audit.length < 2) {
    return null;
  }
  const previous = proposal.audit[proposal.audit.length - 2]?.event as
    | ProposalStatus
    | undefined;
  if (!previous) {
    return null;
  }
  return setProposalStatus(state, id, previous, "undo");
}

export function parseApprovalIntent(
  raw: string,
): "approve" | "reject" | "undo" | "list" | null {
  const text = normalize(raw);
  if (
    /^(approve( proposal)?|yes|lgtm|accept proposal)$/.test(text) ||
    text === "approve"
  ) {
    return "approve";
  }
  if (/^(reject( proposal)?|no|decline proposal)$/.test(text)) {
    return "reject";
  }
  if (/^(undo proposal|rollback proposal|roll back proposal)$/.test(text)) {
    return "undo";
  }
  if (
    /\b(list proposals|proposal backlog|capability backlog|evolution backlog)\b/.test(
      text,
    )
  ) {
    return "list";
  }
  return null;
}

export function formatBacklogReply(state: CapabilityEvolutionState): string {
  const open = state.proposals.filter((p) =>
    ["proposed", "approved", "implementing"].includes(p.status),
  );
  if (open.length === 0) {
    return "No open capability proposals. Describe a change (for example “Add a screenshot button”) and I will propose — not implement — it.";
  }
  return [
    "Open proposals:",
    ...open.slice(0, 8).map(
      (p) => `· ${p.id} [${p.status}/${p.category}] ${p.title}`,
    ),
    "Execution still requires a constitutional engineering program after approval.",
  ].join("\n");
}
