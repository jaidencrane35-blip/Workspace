/**
 * Purpose: Lightweight Assistant companion — history + desktop-fact enrichment.
 * Owner: Frontend product shell (Product Contract V6)
 * Inputs: Compose surface + sessionStorage + WorkspaceState / delta / arrangements
 * Outputs: Chat history, answer extraction, desktop-aware ask enrichment,
 *   deterministic local answers for common desktop questions
 * Dependencies: domain projection + arrangement types only
 * Non-responsibilities: AI engines, durable history IPC, PermissionGateway, restore/focus
 */

import type { DesktopArrangement } from "../types/desktopArrangement";
import type {
  WorkspaceAssistantSurfaceProjection,
  WorkspaceObservationDelta,
  WorkspaceState,
} from "../types/domain";

export const ASSISTANT_COMPANION_RECENT_KEY =
  "workspace.assistant.companion.recent.v1";

/** Chat-style history bound for the companion rail session. */
export const ASSISTANT_COMPANION_RECENT_LIMIT = 24;

export interface AssistantCompanionTurn {
  id: string;
  ask: string;
  answer: string;
  at: string;
}

export interface AssistantDesktopFacts {
  state: WorkspaceState | null;
  delta: WorkspaceObservationDelta | null;
  arrangements: DesktopArrangement[];
}

export function companionAnswerFromSurface(
  projection: WorkspaceAssistantSurfaceProjection | null | undefined,
): string {
  const body = projection?.current?.utterance?.body?.trim();
  if (body) {
    return body;
  }
  const summary = projection?.current?.narrative_summary?.trim();
  if (summary) {
    return summary;
  }
  return "No answer text was available from the existing assistant surface.";
}

export function loadCompanionRecentTurns(): AssistantCompanionTurn[] {
  try {
    const raw = sessionStorage.getItem(ASSISTANT_COMPANION_RECENT_KEY);
    if (!raw) {
      return [];
    }
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) {
      return [];
    }
    return parsed
      .filter((item): item is AssistantCompanionTurn => {
        if (!item || typeof item !== "object") {
          return false;
        }
        const turn = item as AssistantCompanionTurn;
        return (
          typeof turn.id === "string" &&
          typeof turn.ask === "string" &&
          typeof turn.answer === "string" &&
          typeof turn.at === "string"
        );
      })
      .slice(0, ASSISTANT_COMPANION_RECENT_LIMIT);
  } catch {
    return [];
  }
}

export function appendCompanionRecentTurn(
  turn: AssistantCompanionTurn,
): AssistantCompanionTurn[] {
  const next = [
    turn,
    ...loadCompanionRecentTurns().filter((item) => item.id !== turn.id),
  ].slice(0, ASSISTANT_COMPANION_RECENT_LIMIT);
  try {
    sessionStorage.setItem(ASSISTANT_COMPANION_RECENT_KEY, JSON.stringify(next));
  } catch {
    // sessionStorage may be unavailable; keep in-memory return value only
  }
  return next;
}

/** Newest-first storage → oldest-first thread for ChatGPT-like reading. */
export function companionThreadTurns(
  recent: AssistantCompanionTurn[],
): AssistantCompanionTurn[] {
  return [...recent].reverse();
}

function summariseOpen(state: WorkspaceState): string {
  const apps = state.active_applications
    .map((app) => {
      const name = app.process_name?.trim() || `pid ${app.process_id}`;
      return `${name} (${app.window_count})`;
    })
    .slice(0, 12);
  const focused = state.focused_window?.title?.trim();
  const parts = [
    `${state.windows.length} window${state.windows.length === 1 ? "" : "s"} open`,
  ];
  const monitors = state.monitors ?? [];
  if (monitors.length > 0) {
    parts.push(
      `${monitors.length} monitor${monitors.length === 1 ? "" : "s"} (${monitors
        .map((monitor) => monitor.name || `display ${monitor.monitor_index}`)
        .slice(0, 4)
        .join(", ")})`,
    );
  }
  if (focused) {
    parts.push(`focused on ${focused}`);
  }
  if (apps.length > 0) {
    parts.push(`apps: ${apps.join(", ")}`);
  }
  return parts.join(". ") + ".";
}

function summariseWorkingOn(state: WorkspaceState): string {
  const focused = state.focused_window;
  if (!focused) {
    return "No focused window is recorded in the latest observation.";
  }
  const focusedMemberId =
    focused.stable_window_id?.trim() || focused.hwnd;
  const semantic =
    state.semantics?.objects.find(
      (object) =>
        object.stable_window_id === focused.stable_window_id ||
        object.hwnd === focused.hwnd,
    ) ??
    state.semantics?.objects.find((object) => object.role === "working");
  const activity = state.semantics?.activities?.[0];
  const processGroup = (state.window_groups ?? []).find(
    (group) =>
      group.criterion === "process_id" &&
      group.member_ids.includes(focusedMemberId),
  );
  const siblingCount = processGroup?.member_ids.length
    ?? state.windows.filter((window) => window.process_id === focused.process_id)
        .length;
  const name =
    processGroup?.label?.trim() ||
    state.windows.find((window) => window.hwnd === focused.hwnd)?.process_name?.trim() ||
    focused.title.trim() ||
    `hwnd ${focused.hwnd}`;
  const semanticBit = semantic
    ? `; semantic role ${semantic.role} (${semantic.importance}, ${semantic.confidence})`
    : "";
  const activityBit = activity ? `; activity ${activity.kind}` : "";
  return `You appear to be working in ${name} (${focused.title || "untitled"}${
    siblingCount > 1 ? `; ${siblingCount} windows in that process` : ""
  }${semanticBit}${activityBit}).`;
}

function summariseBelongsTogether(
  state: WorkspaceState,
  arrangements: DesktopArrangement[],
): string {
  const processGroups = (state.window_groups ?? []).filter(
    (group) => group.criterion === "process_id" && group.member_ids.length > 1,
  );
  const monitorGroups = (state.window_groups ?? []).filter(
    (group) =>
      group.criterion === "monitor_index" && group.member_ids.length > 1,
  );
  const arrangementGroups = (state.window_groups ?? []).filter(
    (group) =>
      group.criterion === "arrangement_membership" &&
      group.member_ids.length > 0,
  );
  const parts: string[] = [];
  if (processGroups.length > 0) {
    parts.push(
      `process groups: ${processGroups
        .slice(0, 5)
        .map(
          (group) =>
            `${group.label} (${group.member_ids.length}${
              group.confidence && group.confidence !== "structural"
                ? `, ${group.confidence}`
                : ""
            })`,
        )
        .join(", ")}`,
    );
  }
  if (monitorGroups.length > 0) {
    parts.push(
      `monitor co-location: ${monitorGroups
        .slice(0, 3)
        .map((group) => `${group.label} (${group.member_ids.length})`)
        .join(", ")}`,
    );
  }
  if (arrangementGroups.length > 0) {
    parts.push(
      `arrangement membership: ${arrangementGroups
        .slice(0, 5)
        .map((group) => `${group.label} (${group.member_ids.length})`)
        .join(", ")}`,
    );
  } else if (arrangements.length > 0) {
    parts.push(
      `saved arrangements: ${arrangements
        .slice(0, 5)
        .map((item) => `${item.name} (${item.entries.length})`)
        .join(", ")}`,
    );
  }
  if (parts.length === 0) {
    return "No multi-window process or monitor groups, and no saved arrangements yet.";
  }
  return `${parts.join(". ")}.`;
}

function summariseChanged(delta: WorkspaceObservationDelta | null): string {
  if (!delta || !delta.has_changes) {
    return "No desktop changes between the latest observation passes.";
  }
  const parts: string[] = [];
  if (delta.opened_windows.length > 0) {
    parts.push(
      `opened ${delta.opened_windows
        .map((window) => window.title || window.hwnd)
        .slice(0, 5)
        .join(", ")}`,
    );
  }
  if (delta.closed_windows.length > 0) {
    parts.push(
      `closed ${delta.closed_windows
        .map((window) => window.title || window.hwnd)
        .slice(0, 5)
        .join(", ")}`,
    );
  }
  if (delta.focused_window_changed?.current) {
    parts.push(
      `focus → ${delta.focused_window_changed.current.title || delta.focused_window_changed.current.hwnd}`,
    );
  }
  if (delta.minimized_changes.length > 0) {
    parts.push(`${delta.minimized_changes.length} minimize changes`);
  }
  return parts.length > 0
    ? `Since last observation: ${parts.join("; ")}.`
    : "Observation reports changes, but no window open/close/focus details.";
}

function summariseReopen(
  delta: WorkspaceObservationDelta | null,
  arrangements: DesktopArrangement[],
): string {
  const closed = delta?.closed_windows ?? [];
  if (closed.length > 0) {
    return `Recently closed: ${closed
      .map((window) => window.title || window.hwnd)
      .slice(0, 6)
      .join(", ")}. Restore a saved arrangement under Stage if you want those layouts back — Assistant cannot move windows.`;
  }
  if (arrangements.length > 0) {
    return `Nothing closed in the latest delta. Saved arrangements you can restore from Stage: ${arrangements
      .map((item) => item.name)
      .slice(0, 5)
      .join(", ")}.`;
  }
  return "Nothing closed in the latest delta, and there are no saved arrangements to reopen yet.";
}

function summariseContinuity(state: WorkspaceState): string {
  const memory = state.runtime_memory;
  if (memory && memory.entities.length > 0) {
    const parts: string[] = [
      `runtime memory: ${memory.present_count} present, ${memory.returning_count} returning, ${memory.absent_count} absent`,
    ];
    const focusedId = state.windows.find((window) => window.focused)?.stable_window_id;
    const focusedMemory = focusedId
      ? memory.entities.find((entity) => entity.stable_window_id === focusedId)
      : undefined;
    if (focusedMemory) {
      parts.push(
        `focused ${focusedMemory.title || focusedMemory.hwnd} first observed ${focusedMemory.first_observed_at} (${focusedMemory.stability}, continuity ${focusedMemory.continuity_confidence})`,
      );
    }
    const persistent = memory.entities
      .filter((entity) => entity.stability === "persistent")
      .slice(0, 4);
    if (persistent.length > 0) {
      parts.push(
        `persistent: ${persistent
          .map((entity) => entity.title || entity.hwnd)
          .join(", ")}`,
      );
    }
    const returning = memory.entities
      .filter((entity) => entity.presence === "returning")
      .slice(0, 4);
    if (returning.length > 0) {
      parts.push(
        `returning: ${returning
          .map((entity) => entity.title || entity.hwnd)
          .join(", ")}`,
      );
    }
    return `${parts.join(". ")}.`;
  }

  const withIdentity = state.windows.filter((window) => window.first_seen_at);
  if (withIdentity.length === 0) {
    return "No identity continuity facts are available for the current windows yet.";
  }
  const focused =
    state.windows.find((window) => window.focused) ??
    state.windows.find((window) => window.hwnd === state.focused_window?.hwnd);
  const parts: string[] = [];
  if (focused?.first_seen_at) {
    parts.push(
      `focused ${focused.title || focused.hwnd} first seen ${focused.first_seen_at}${
        focused.identity_confidence
          ? ` (confidence ${focused.identity_confidence})`
          : ""
      }`,
    );
  }
  const oldest = [...withIdentity].sort((a, b) =>
    (a.first_seen_at ?? "").localeCompare(b.first_seen_at ?? ""),
  )[0];
  const newest = [...withIdentity].sort((a, b) =>
    (b.first_seen_at ?? "").localeCompare(a.first_seen_at ?? ""),
  )[0];
  if (oldest?.first_seen_at) {
    parts.push(
      `longest-running: ${oldest.title || oldest.hwnd} since ${oldest.first_seen_at}`,
    );
  }
  if (
    newest?.first_seen_at &&
    newest.stable_window_id !== oldest?.stable_window_id
  ) {
    parts.push(
      `most recently identified: ${newest.title || newest.hwnd} since ${newest.first_seen_at}`,
    );
  }
  return `${parts.join(". ")}.`;
}

function summariseRuntimeMemory(state: WorkspaceState): string | null {
  const memory = state.runtime_memory;
  if (!memory || memory.entities.length === 0) {
    return null;
  }
  const absent = memory.entities
    .filter((entity) => entity.presence === "absent")
    .slice(0, 6);
  const returning = memory.entities
    .filter((entity) => entity.presence === "returning")
    .slice(0, 6);
  const persistent = memory.entities
    .filter((entity) => entity.knowledge === "persistent" || entity.stability === "persistent")
    .slice(0, 6);
  const rising = memory.entities
    .filter((entity) => entity.knowledge === "rising")
    .slice(0, 6);
  const fading = memory.entities
    .filter((entity) => entity.knowledge === "fading" || entity.knowledge === "interrupted")
    .slice(0, 6);
  const parts: string[] = [
    `${memory.entities.length} remembered desktop object${
      memory.entities.length === 1 ? "" : "s"
    }`,
  ];
  if (returning.length > 0) {
    parts.push(
      `returned: ${returning.map((entity) => entity.title || entity.hwnd).join(", ")}`,
    );
  }
  if (rising.length > 0) {
    parts.push(
      `becoming important: ${rising
        .map((entity) => entity.title || entity.hwnd)
        .join(", ")}`,
    );
  }
  if (absent.length > 0) {
    parts.push(
      `absent but known: ${absent
        .map((entity) => `${entity.title || entity.hwnd} (${entity.knowledge})`)
        .join(", ")}`,
    );
  }
  if (fading.length > 0) {
    parts.push(
      `interrupted/fading: ${fading
        .map((entity) => entity.title || entity.hwnd)
        .join(", ")}`,
    );
  }
  if (persistent.length > 0) {
    parts.push(
      `persistent: ${persistent
        .map((entity) => entity.title || entity.hwnd)
        .join(", ")}`,
    );
  }
  return `${parts.join(". ")}.`;
}

function summariseBehaviour(state: WorkspaceState): string {
  const behaviour = state.behaviour;
  if (!behaviour || behaviour.sample_count <= 0) {
    return "No retained observation samples are available for desktop behaviour yet.";
  }
  const parts: string[] = [
    `${behaviour.sample_count} observation sample${
      behaviour.sample_count === 1 ? "" : "s"
    }`,
  ];
  if (behaviour.coverage_started_at && behaviour.coverage_ended_at) {
    parts.push(
      `coverage ${behaviour.coverage_started_at} → ${behaviour.coverage_ended_at}`,
    );
  }
  if (behaviour.focus_transitions.length > 0) {
    const recent = behaviour.focus_transitions.slice(-5).map((transition) => {
      const from = transition.previous?.title || transition.previous?.hwnd || "none";
      const to = transition.current?.title || transition.current?.hwnd || "none";
      return `${from} → ${to}`;
    });
    parts.push(`focus switches: ${recent.join("; ")}`);
  } else {
    parts.push("no focus switches in the sample window");
  }
  if (behaviour.window_revisits.length > 0) {
    parts.push(
      `revisits: ${behaviour.window_revisits
        .slice(0, 5)
        .map(
          (revisit) =>
            `${revisit.window.title || revisit.window.hwnd} (${revisit.focus_count})`,
        )
        .join(", ")}`,
    );
  }
  if (behaviour.current_focus) {
    const span =
      behaviour.current_focus_sample_span_seconds != null
        ? ` for ~${behaviour.current_focus_sample_span_seconds}s of samples`
        : "";
    parts.push(
      `current focus ${behaviour.current_focus.title || behaviour.current_focus.hwnd}${span}`,
    );
  }
  if (behaviour.coverage_gaps.length > 0) {
    parts.push(
      `${behaviour.coverage_gaps.length} coverage gap${
        behaviour.coverage_gaps.length === 1 ? "" : "s"
      } (≥30m between samples)`,
    );
  }
  if (behaviour.sessions && behaviour.sessions.length > 0) {
    const current =
      behaviour.sessions.find((session) => session.kind === "active") ??
      behaviour.sessions.find((session) => session.kind === "returning") ??
      behaviour.sessions[behaviour.sessions.length - 1];
    if (current) {
      const dominant =
        current.dominant_focus?.title ||
        current.dominant_focus?.hwnd ||
        "unknown focus";
      parts.push(
        `${behaviour.sessions.length} observation session${
          behaviour.sessions.length === 1 ? "" : "s"
        }; current ${current.kind} around ${dominant} (${current.sample_count} samples, ${current.confidence})`,
      );
    }
  }
  return `${parts.join(". ")}.`;
}

function summariseStopped(state: WorkspaceState): string {
  const behaviour = state.behaviour;
  if (!behaviour || behaviour.recent_focus_spans.length === 0) {
    return "No completed focus spans are recorded in the retained samples yet.";
  }
  const recent = behaviour.recent_focus_spans.slice(-5).reverse();
  return `Recently left focus (sample-based): ${recent
    .map((span) => {
      const seconds =
        span.sample_span_seconds != null ? ` ~${span.sample_span_seconds}s` : "";
      return `${span.window.title || span.window.hwnd}${seconds}`;
    })
    .join("; ")}.`;
}

function summariseAffinities(state: WorkspaceState): string {
  const behaviour = state.behaviour;
  if (!behaviour || behaviour.sample_count <= 0) {
    return "No behavioural affinity samples are available yet.";
  }
  const parts: string[] = [];
  if (behaviour.co_presence.length > 0) {
    parts.push(
      `open together: ${behaviour.co_presence
        .slice(0, 5)
        .map(
          (pair) =>
            `${pair.left.title || pair.left.hwnd} + ${pair.right.title || pair.right.hwnd} (${pair.sample_count}x, ${pair.session_count} sessions, ${pair.confidence})`,
        )
        .join("; ")}`,
    );
  }
  if (behaviour.focus_follows.length > 0) {
    parts.push(
      `usually next: ${behaviour.focus_follows
        .slice(0, 5)
        .map(
          (follow) =>
            `${follow.from.title || follow.from.hwnd} → ${follow.to.title || follow.to.hwnd} (${follow.transition_count}x, ${follow.session_count} sessions, ${follow.confidence})`,
        )
        .join("; ")}`,
    );
  }
  const strengthened = (state.window_groups ?? []).filter(
    (group) => group.confidence && group.confidence !== "structural",
  );
  if (strengthened.length > 0) {
    parts.push(
      `strengthened groups: ${strengthened
        .slice(0, 5)
        .map(
          (group) =>
            `${group.label} (${group.confidence}, evidence ${group.evidence_count})`,
        )
        .join(", ")}`,
    );
  }
  if (parts.length === 0) {
    return "No recurring co-presence or focus-follow patterns in the retained samples yet.";
  }
  return `${parts.join(". ")}.`;
}



function summariseDecisions(state: WorkspaceState): string | null {
  const projection = state.decisions;
  if (!projection) {
    return null;
  }
  const parts: string[] = [];
  if (projection.decisions.length > 0) {
    parts.push(
      `decisions: ${projection.decisions
        .slice(0, 5)
        .map(
          (decision) =>
            `${decision.summary} [${decision.kind}, ${decision.confidence}] — ${decision.explanation}`,
        )
        .join(" | ")}`,
    );
  }
  if (projection.recommendations.length > 0) {
    parts.push(
      `recommendations: ${projection.recommendations
        .slice(0, 5)
        .map((item) => `${item.summary} (${item.confidence})`)
        .join("; ")}`,
    );
  }
  if (projection.consistency_issues.length > 0) {
    parts.push(
      `uncertainty: ${projection.consistency_issues
        .slice(0, 4)
        .map((issue) => `${issue.summary} — ${issue.explanation}`)
        .join(" | ")}`,
    );
  }
  if (parts.length === 0) {
    return "No deterministic desktop decisions are available yet.";
  }
  return `Desktop decision support — ${parts.join(". ")}.`;
}

function summariseSemantics(state: WorkspaceState): string | null {
  const semantics = state.semantics;
  if (!semantics || semantics.objects.length === 0) {
    return null;
  }
  const parts: string[] = [];
  const byRole = new Map<string, string[]>();
  for (const object of semantics.objects.slice(0, 24)) {
    const list = byRole.get(object.role) ?? [];
    list.push(object.title || object.hwnd);
    byRole.set(object.role, list);
  }
  const important = semantics.objects
    .filter((object) => object.importance === "important" || object.importance === "emerging")
    .slice(0, 4)
    .map((object) => `${object.title || object.hwnd} (${object.importance})`);
  for (const [role, titles] of byRole) {
    parts.push(`${role}: ${titles.slice(0, 4).join(", ")}`);
  }
  if (important.length > 0) {
    parts.push(`importance: ${important.join(", ")}`);
  }
  if (semantics.relationships.length > 0) {
    parts.push(
      `relationships: ${semantics.relationships
        .slice(0, 5)
        .map(
          (rel) =>
            `${rel.kind} ${rel.from_stable_window_id}→${rel.to_stable_window_id} (${rel.confidence})`,
        )
        .join("; ")}`,
    );
  }
  if (semantics.activities.length > 0) {
    parts.push(
      `activities: ${semantics.activities
        .slice(0, 4)
        .map((activity) => `${activity.kind} (${activity.confidence})`)
        .join("; ")}`,
    );
  }
  if (semantics.graph?.nodes?.length) {
    parts.push(
      `graph: ${semantics.graph.nodes.length} nodes / ${semantics.graph.edges.length} edges`,
    );
  }
  return `Desktop semantics — ${parts.join(". ")}.`;
}

function summariseLifecycle(state: WorkspaceState): string | null {
  const lifecycles = state.behaviour?.window_lifecycles ?? [];
  if (lifecycles.length === 0) {
    return null;
  }
  return `Window lifecycle across samples: ${lifecycles
    .slice(0, 6)
    .map((item) => {
      const title = item.window.title || item.window.hwnd;
      return `${title} (opened ${item.opened_count}, closed ${item.closed_count})`;
    })
    .join("; ")}.`;
}

/**
 * Answer common desktop questions from observed facts without calling compose.
 * Returns null when the ask needs the broader assistant surface.
 */
export function answerDesktopQuestionLocally(
  ask: string,
  facts: AssistantDesktopFacts,
): string | null {
  const trimmed = ask.trim().toLowerCase();
  if (!trimmed) {
    return null;
  }
  const state = facts.state;
  if (!state) {
    return null;
  }
  const delta = facts.delta ?? state.latest_delta ?? null;

  if (
    /what('s| is) open|what windows|what apps|what applications/.test(trimmed)
  ) {
    return summariseOpen(state);
  }
  if (
    /what am i working on|what('s| is) focused|current (focus|work)/.test(
      trimmed,
    )
  ) {
    return summariseWorkingOn(state);
  }
  if (
    /what have i been working|been working on|focus history|behaviour|behavior|task switch|revisit|session/.test(
      trimmed,
    )
  ) {
    return summariseBehaviour(state);
  }
  if (/what did i stop|stopped doing|left focus|interrupted/.test(trimmed)) {
    return summariseStopped(state);
  }
  if (
    /keep(s)? opening together|open(ed)? together|co-?presence|usually come(s)? next|what follows|follow/.test(
      trimmed,
    )
  ) {
    return summariseAffinities(state);
  }
  if (
    /what should|recommend|decision|next|resume|interrupted|uncertain|consistency|why (do|does|is)|explain/.test(
      trimmed,
    )
  ) {
    const decisions = summariseDecisions(state);
    if (decisions) {
      return decisions;
    }
  }
  if (
    /semantic|what (is|are) (this|these|my)|working object|companion|background object|role|activity|researching|coding|comparing|monitoring|knowledge graph/.test(
      trimmed,
    )
  ) {
    const semantics = summariseSemantics(state);
    if (semantics) {
      return semantics;
    }
  }
  if (/belong|related|together|group/.test(trimmed)) {
    const semantics = summariseSemantics(state);
    if (semantics && /belong|related/.test(trimmed)) {
      return semantics;
    }
    return summariseBelongsTogether(state, facts.arrangements);
  }
  if (
    /how long|been open|first seen|continuity|longest.?running|focus duration/.test(
      trimmed,
    )
  ) {
    if (state.behaviour?.current_focus_sample_span_seconds != null) {
      const title =
        state.behaviour.current_focus?.title ||
        state.focused_window?.title ||
        "current focus";
      return `Current focus ${title} has an observed sample span of ~${state.behaviour.current_focus_sample_span_seconds}s (not OS active time). ${summariseContinuity(state)}`;
    }
    return summariseContinuity(state);
  }
  if (
    /came back|returned|returning|missing|disappeared|absent|gone|persistent|temporary|remember|important|fading|interrupted/.test(
      trimmed,
    )
  ) {
    const memory = summariseRuntimeMemory(state);
    if (memory) {
      return memory;
    }
  }
  if (/what changed|what('s| is) new|delta|recent change/.test(trimmed)) {
    return summariseChanged(delta);
  }
  if (/reopen|restore|bring back|closed|keep(s)? (re)?opening|lifecycle/.test(trimmed)) {
    const lifecycle = summariseLifecycle(state);
    if (lifecycle) {
      return lifecycle;
    }
    return summariseReopen(delta, facts.arrangements);
  }
  return null;
}

/**
 * Prefix the compose ask with observed desktop facts when available.
 * Displayed user ask stays unprefixed; enrichment is compose-only.
 */
export function enrichAskWithDesktopObservation(
  ask: string,
  facts: AssistantDesktopFacts | WorkspaceState | null | undefined,
): string {
  const trimmed = ask.trim();
  if (!trimmed) {
    return trimmed;
  }

  const normalized: AssistantDesktopFacts =
    facts && "state" in (facts as AssistantDesktopFacts)
      ? (facts as AssistantDesktopFacts)
      : {
          state: (facts as WorkspaceState | null | undefined) ?? null,
          delta: null,
          arrangements: [],
        };

  const state = normalized.state;
  if (!state || state.windows.length === 0) {
    return trimmed;
  }
  const delta = normalized.delta ?? state.latest_delta ?? null;

  const focused = state.focused_window?.title?.trim() || null;
  const processes: string[] = [];
  const seen = new Set<string>();
  for (const window of state.windows) {
    const label =
      window.process_name?.trim() || window.title.trim() || `hwnd ${window.hwnd}`;
    const key = `${window.process_id}:${label.toLowerCase()}`;
    if (seen.has(key)) {
      continue;
    }
    seen.add(key);
    processes.push(label);
    if (processes.length >= 8) {
      break;
    }
  }

  const parts = [
    `${state.windows.length} window${state.windows.length === 1 ? "" : "s"}`,
  ];
  if (focused) {
    parts.push(`focused: ${focused}`);
  }
  if (processes.length > 0) {
    parts.push(`apps: ${processes.join(", ")}`);
  }
  if (delta?.has_changes) {
    parts.push(
      `changed: +${delta.opened_windows.length}/-${delta.closed_windows.length}`,
    );
  }
  if (normalized.arrangements.length > 0) {
    parts.push(
      `arrangements: ${normalized.arrangements
        .map((item) => `${item.name}(${item.entries.length})`)
        .slice(0, 4)
        .join(", ")}`,
    );
  }
  const groups = state.window_groups ?? [];
  if (groups.length > 0) {
    parts.push(
      `groups: ${groups
        .slice(0, 6)
        .map((group) => `${group.criterion}:${group.label}(${group.member_ids.length})`)
        .join(", ")}`,
    );
  }
  const behaviour = state.behaviour;
  if (behaviour && behaviour.sample_count > 0) {
    parts.push(
      `behaviour: ${behaviour.sample_count} samples, ${behaviour.focus_transitions.length} focus switches`,
    );
  }
  const memory = state.runtime_memory;
  if (memory && memory.entities.length > 0) {
    parts.push(
      `memory: ${memory.present_count} present / ${memory.returning_count} returning / ${memory.absent_count} absent`,
    );
  }
  const semantics = state.semantics;
  if (semantics && semantics.objects.length > 0) {
    const working = semantics.objects
      .filter((object) => object.role === "working")
      .slice(0, 3)
      .map((object) => object.title || object.hwnd);
    parts.push(
      `semantics: ${semantics.objects.length} roles / ${semantics.relationships.length} relations / ${semantics.activities.length} activities`,
    );
    if (working.length > 0) {
      parts.push(`working: ${working.join(", ")}`);
    }
  }
  const decisions = state.decisions;
  if (decisions && decisions.decisions.length > 0) {
    parts.push(
      `decisions: ${decisions.decisions.length} / recommendations: ${decisions.recommendations.length} / issues: ${decisions.consistency_issues.length}`,
    );
  }

  return `Observed desktop (${parts.join("; ")}).\n\n${trimmed}`;
}
