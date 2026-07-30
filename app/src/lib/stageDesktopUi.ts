/**
 * Purpose: Pure desktop-object model for the Desktop Interaction Layer Stage.
 * Owner: Frontend product shell (Product Contract V8 / Product Foundation V15)
 * Inputs: WorkspaceState windows, window_groups, attention, semantics
 * Outputs: Runtime object tiles, relationship keys from groups + semantic links,
 *   Flow/Focus organisation, selection helpers, arrangement overlays,
 *   subtle awareness cues (attention primary, semantic roles)
 * Dependencies: None (pure)
 * Non-goals: Fake windows, WindowController ownership, OS geometry apply,
 *   inventing process/relationship groups, diagnostic runtime dumps
 */

import type { DesktopArrangementEntry } from "../types/desktopArrangement";
import type {
  DesktopAttentionProjection,
  DesktopObjectMemory,
  DesktopRuntimeMemory,
  DesktopSemanticProjection,
  DesktopWindowGroup,
  WorkspaceObservationDelta,
  WorkspaceStateMonitor,
  WorkspaceStateWindow,
} from "../types/domain";

/** First-class Stage desktop object projected from observation — not a card. */
export interface StageDesktopWindowTile {
  key: string;
  hwnd: string;
  stableWindowId: string | null;
  title: string;
  processLabel: string;
  processId: number;
  /** Display/process relationship key (name when known, else pid). */
  processKey: string;
  /** 0–5 hue bucket shared by windows of the same process. */
  relationIndex: number;
  visible: boolean;
  focused: boolean;
  minimized: boolean;
  zOrder: number | null;
  monitorIndex: number | null;
  monitorLabel: string | null;
  leftPct: number;
  topPct: number;
  widthPct: number;
  heightPct: number;
}

export type StageDesktopLoadState =
  | "loading"
  | "runtime_unavailable"
  | "error"
  | "ready";

export function stageDesktopWindowKey(window: WorkspaceStateWindow): string {
  const stable = window.stable_window_id?.trim();
  if (stable) {
    return stable;
  }
  return window.hwnd;
}

export function stageDesktopWindowTitle(window: WorkspaceStateWindow): string {
  const title = window.title.trim();
  if (title) {
    return title;
  }
  const process = window.process_name?.trim();
  if (process) {
    return process;
  }
  return `Window ${window.hwnd}`;
}

/** Display name for a spatial app object: prefer process, fall back to title. */
export function stageDesktopAppObjectLabel(
  window: WorkspaceStateWindow,
): string {
  const process = window.process_name?.trim();
  if (process) {
    return process;
  }
  return stageDesktopWindowTitle(window);
}

/** Prefer PID for true process siblings; name only as display fallback key. */
export function stageDesktopProcessKey(window: WorkspaceStateWindow): string {
  return `pid:${window.process_id}`;
}

export function stageDesktopProcessDisplayKey(
  window: WorkspaceStateWindow,
): string {
  const process = window.process_name?.trim().toLowerCase();
  if (process) {
    return process;
  }
  return stageDesktopProcessKey(window);
}

/** Deterministic 0–5 bucket so same process shares an accent without a grouping engine. */
export function stageDesktopRelationIndex(processKey: string): number {
  let hash = 0;
  for (let i = 0; i < processKey.length; i += 1) {
    hash = (hash * 31 + processKey.charCodeAt(i)) >>> 0;
  }
  return hash % 6;
}

/**
 * Map observed windows into a spatial stage using relative bounds.
 * When monitors are present, the plane is the union of observed displays.
 * Rectangles encode identity + geometry only — lightweight representations.
 */
export function layoutStageDesktopWindows(
  windows: WorkspaceStateWindow[],
  monitors: WorkspaceStateMonitor[] = [],
): StageDesktopWindowTile[] {
  if (windows.length === 0) {
    return [];
  }

  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;

  if (monitors.length > 0) {
    for (const monitor of monitors) {
      const w = Math.max(monitor.width, 1);
      const h = Math.max(monitor.height, 1);
      minX = Math.min(minX, monitor.x);
      minY = Math.min(minY, monitor.y);
      maxX = Math.max(maxX, monitor.x + w);
      maxY = Math.max(maxY, monitor.y + h);
    }
  } else {
    for (const window of windows) {
      const w = Math.max(window.width, 1);
      const h = Math.max(window.height, 1);
      minX = Math.min(minX, window.x);
      minY = Math.min(minY, window.y);
      maxX = Math.max(maxX, window.x + w);
      maxY = Math.max(maxY, window.y + h);
    }
  }

  const spanX = Math.max(maxX - minX, 1);
  const spanY = Math.max(maxY - minY, 1);

  return windows.map((window) => {
    const w = Math.max(window.width, 1);
    const h = Math.max(window.height, 1);
    const leftPct = ((window.x - minX) / spanX) * 100;
    const topPct = ((window.y - minY) / spanY) * 100;
    const widthPct = Math.max((w / spanX) * 100, 8);
    const heightPct = Math.max((h / spanY) * 100, 10);
    const monitorLabel =
      window.monitor_name?.trim() ||
      (window.monitor_index === null
        ? null
        : `Monitor ${window.monitor_index + 1}`);

    const appLabel = stageDesktopAppObjectLabel(window);
    const windowTitle = stageDesktopWindowTitle(window);
    const processKey = stageDesktopProcessKey(window);
    const displayKey = stageDesktopProcessDisplayKey(window);
    return {
      key: stageDesktopWindowKey(window),
      hwnd: window.hwnd,
      stableWindowId: window.stable_window_id?.trim() || null,
      title: appLabel,
      processLabel: windowTitle !== appLabel ? windowTitle : "",
      processId: window.process_id,
      processKey,
      relationIndex: stageDesktopRelationIndex(displayKey),
      visible: window.visible,
      focused: window.focused,
      minimized: window.minimized,
      zOrder: window.z_order,
      monitorIndex: window.monitor_index,
      monitorLabel,
      leftPct,
      topPct,
      widthPct: Math.min(widthPct, 100 - leftPct),
      heightPct: Math.min(heightPct, 100 - topPct),
    };
  });
}

export function stageDesktopPlaneMessage(state: StageDesktopLoadState): string {
  if (state === "runtime_unavailable") {
    return "Open the desktop app to see your windows.";
  }
  if (state === "error") {
    return "Could not read the desktop.";
  }
  if (state === "loading") {
    return "Reading desktop…";
  }
  return "No windows open.";
}

export function stageDesktopMetaLine(args: {
  windowCount: number;
  monitorCount: number;
  focusedTitle: string | null;
  selectedCount?: number;
  /** One short product awareness line from attention (not a diagnostic dump). */
  awarenessLine?: string | null;
}): string {
  const parts: string[] = [];
  parts.push(
    args.windowCount === 1
      ? "1 window"
      : `${args.windowCount} windows`,
  );
  if (args.monitorCount > 0) {
    parts.push(
      args.monitorCount === 1
        ? "1 monitor"
        : `${args.monitorCount} monitors`,
    );
  }
  if (args.selectedCount && args.selectedCount > 1) {
    parts.push(`${args.selectedCount} selected`);
  }
  if (args.focusedTitle) {
    parts.push(args.focusedTitle);
  }
  const awareness = args.awarenessLine?.trim();
  if (awareness) {
    parts.push(awareness);
  }
  return parts.join(" · ");
}

/** Primary attention entity keys (stable window ids) for Stage highlighting. */
export function stageAttentionPrimaryKeys(
  attention: DesktopAttentionProjection | null | undefined,
): Set<string> {
  const keys = new Set<string>();
  if (!attention?.primary_item_id) {
    return keys;
  }
  const primary = attention.items.find(
    (item) => item.id === attention.primary_item_id,
  );
  if (!primary) {
    return keys;
  }
  for (const id of primary.entity_ids) {
    const trimmed = id.trim();
    if (trimmed) {
      keys.add(trimmed);
    }
  }
  return keys;
}

/** Short product line for Stage meta — primary attention summary only. */
export function stageAttentionAwarenessLine(
  attention: DesktopAttentionProjection | null | undefined,
): string | null {
  if (!attention?.primary_item_id) {
    return null;
  }
  const primary = attention.items.find(
    (item) => item.id === attention.primary_item_id,
  );
  const summary = primary?.summary?.trim();
  return summary || null;
}

/** Semantic role by Stage object key (stable window id). */
export function stageSemanticRoleByKey(
  semantics: DesktopSemanticProjection | null | undefined,
): Map<string, string> {
  const roles = new Map<string, string>();
  if (!semantics?.objects?.length) {
    return roles;
  }
  for (const object of semantics.objects) {
    const id = object.stable_window_id?.trim();
    if (id && object.role) {
      roles.set(id, object.role);
    }
  }
  return roles;
}

/**
 * Focus preference order without selection: attention primary entities, then
 * semantic working objects. Empty when runtime planes are quiet.
 */
export function stageFocusPreferredKeys(
  attention: DesktopAttentionProjection | null | undefined,
  semantics: DesktopSemanticProjection | null | undefined,
): string[] {
  const keys: string[] = [];
  const seen = new Set<string>();
  for (const id of stageAttentionPrimaryKeys(attention)) {
    if (!seen.has(id)) {
      seen.add(id);
      keys.push(id);
    }
  }
  for (const object of semantics?.objects ?? []) {
    if (object.role !== "working") {
      continue;
    }
    const id = object.stable_window_id?.trim();
    if (id && !seen.has(id)) {
      seen.add(id);
      keys.push(id);
    }
  }
  return keys;
}

/** Continuity cues by Stage key from runtime_memory (presence + knowledge). */
export function stageContinuityByKey(
  memory: DesktopRuntimeMemory | null | undefined,
): Map<string, Pick<DesktopObjectMemory, "presence" | "knowledge">> {
  const map = new Map<
    string,
    Pick<DesktopObjectMemory, "presence" | "knowledge">
  >();
  for (const entity of memory?.entities ?? []) {
    const id = entity.stable_window_id?.trim();
    if (!id) {
      continue;
    }
    map.set(id, {
      presence: entity.presence,
      knowledge: entity.knowledge,
    });
  }
  return map;
}

/** Keys that opened in the latest observation delta. */
export function stageDeltaOpenedKeys(
  delta: WorkspaceObservationDelta | null | undefined,
): Set<string> {
  const keys = new Set<string>();
  for (const window of delta?.opened_windows ?? []) {
    const stable = window.stable_window_id?.trim();
    if (stable) {
      keys.add(stable);
      continue;
    }
    const hwnd = window.hwnd?.trim();
    if (hwnd) {
      keys.add(hwnd);
    }
  }
  return keys;
}

/** Quiet continuity line for Stage meta (not an Operator delta dump). */
export function stageContinuityAwarenessLine(
  memory: DesktopRuntimeMemory | null | undefined,
  delta: WorkspaceObservationDelta | null | undefined,
): string | null {
  const parts: string[] = [];
  if (delta?.has_changes) {
    const opened = delta.opened_windows.length;
    const closed = delta.closed_windows.length;
    if (opened > 0 || closed > 0) {
      const bits: string[] = [];
      if (opened > 0) {
        bits.push(opened === 1 ? "1 opened" : `${opened} opened`);
      }
      if (closed > 0) {
        bits.push(closed === 1 ? "1 closed" : `${closed} closed`);
      }
      parts.push(bits.join(", "));
    }
  }
  const returning = memory?.returning_count ?? 0;
  if (returning > 0) {
    parts.push(returning === 1 ? "1 returning" : `${returning} returning`);
  }
  return parts.length > 0 ? parts.join(" · ") : null;
}

const FLOW_SEMANTIC_RELATION_KINDS = new Set([
  "works_with",
  "commonly_accompanies",
  "frequently_alternates",
  "belongs_inside",
  "supports",
  "precedes",
  "follows",
]);

/**
 * Relationships from authoritative WorkspaceState.window_groups, plus Flow-mode
 * semantic relationships when provided. Focus: process_id groups only.
 * Falls back to empty when groups/links are absent (no inventing).
 */
export function relatedStageObjectKeys(
  tiles: StageDesktopWindowTile[],
  anchorKey: string | null,
  workMode: "flow" | "focus",
  windowGroups: DesktopWindowGroup[] = [],
  semantics: DesktopSemanticProjection | null | undefined = null,
): Set<string> {
  const related = new Set<string>();
  if (!anchorKey) {
    return related;
  }
  if (!tiles.some((tile) => tile.key === anchorKey)) {
    return related;
  }
  related.add(anchorKey);
  const criteria =
    workMode === "focus"
      ? new Set(["process_id"])
      : new Set(["process_id", "monitor_index", "arrangement_membership"]);
  for (const group of windowGroups) {
    if (!criteria.has(group.criterion)) {
      continue;
    }
    if (!group.member_ids.includes(anchorKey)) {
      continue;
    }
    for (const memberId of group.member_ids) {
      if (tiles.some((tile) => tile.key === memberId)) {
        related.add(memberId);
      }
    }
  }
  if (workMode === "flow" && semantics?.relationships?.length) {
    const tileKeys = new Set(tiles.map((tile) => tile.key));
    for (const link of semantics.relationships) {
      if (!FLOW_SEMANTIC_RELATION_KINDS.has(link.kind)) {
        continue;
      }
      const from = link.from_stable_window_id?.trim();
      const to = link.to_stable_window_id?.trim();
      if (!from || !to) {
        continue;
      }
      if (from === anchorKey && tileKeys.has(to)) {
        related.add(to);
      } else if (to === anchorKey && tileKeys.has(from)) {
        related.add(from);
      }
    }
  }
  return related;
}

/** Match arrangement entries to Stage object keys via stable id / hwnd. */
export function stageArrangementMemberKeys(
  tiles: StageDesktopWindowTile[],
  entries: DesktopArrangementEntry[] | null | undefined,
): Set<string> {
  const members = new Set<string>();
  if (!entries || entries.length === 0) {
    return members;
  }
  for (const entry of entries) {
    const stable = entry.stable_window_id?.trim();
    const hwnd = entry.hwnd?.trim();
    for (const tile of tiles) {
      if (stable && tile.stableWindowId === stable) {
        members.add(tile.key);
        continue;
      }
      if (hwnd && tile.hwnd === hwnd) {
        members.add(tile.key);
      }
    }
  }
  return members;
}

export function primaryStageSelectionKey(
  selectedKeys: readonly string[],
): string | null {
  if (selectedKeys.length === 0) {
    return null;
  }
  return selectedKeys[selectedKeys.length - 1] ?? null;
}

export function replaceStageSelection(key: string): string[] {
  return [key];
}

export function toggleStageSelection(
  selectedKeys: readonly string[],
  key: string,
): string[] {
  if (selectedKeys.includes(key)) {
    const next = selectedKeys.filter((item) => item !== key);
    return next;
  }
  return [...selectedKeys, key];
}

/** Spatial neighbour for keyboard selection among laid-out tiles. */
export function nextStageSelectionKey(
  tiles: StageDesktopWindowTile[],
  currentKey: string | null,
  direction: "next" | "previous" | "home" | "end",
): string | null {
  if (tiles.length === 0) {
    return null;
  }
  const ordered = [...tiles].sort((a, b) => {
    if (a.topPct !== b.topPct) {
      return a.topPct - b.topPct;
    }
    return a.leftPct - b.leftPct;
  });
  if (direction === "home") {
    return ordered[0]?.key ?? null;
  }
  if (direction === "end") {
    return ordered[ordered.length - 1]?.key ?? null;
  }
  const index = currentKey
    ? ordered.findIndex((tile) => tile.key === currentKey)
    : -1;
  if (direction === "next") {
    if (index < 0) {
      return ordered[0]?.key ?? null;
    }
    return ordered[Math.min(index + 1, ordered.length - 1)]?.key ?? null;
  }
  if (index < 0) {
    return ordered[ordered.length - 1]?.key ?? null;
  }
  return ordered[Math.max(index - 1, 0)]?.key ?? null;
}

/** One dock entry per other process while Focus mode keeps a primary app on the map. */
export interface StageProcessDockEntry {
  processKey: string;
  processId: number;
  label: string;
  hwnd: string;
  tileKey: string;
  windowCount: number;
}

export interface StageWorkModeOrganisation {
  /** Windows drawn on the spatial map (Focus: primary process only). */
  mapWindows: WorkspaceStateWindow[];
  /** Other processes collapsed to dock objects (Focus only). */
  dockEntries: StageProcessDockEntry[];
}

/**
 * Paint order for Stage tiles from observed stacking.
 * Lower z_order is closer to foreground (EnumWindows top-first); paint later.
 */
export function sortStageTilesByZOrder(
  tiles: StageDesktopWindowTile[],
): StageDesktopWindowTile[] {
  return [...tiles].sort((a, b) => {
    const aOrder = a.zOrder ?? Number.MAX_SAFE_INTEGER;
    const bOrder = b.zOrder ?? Number.MAX_SAFE_INTEGER;
    if (aOrder !== bOrder) {
      return bOrder - aOrder;
    }
    return a.key.localeCompare(b.key);
  });
}

/**
 * Flow keeps every observed window on the map (relationships exposed via accents).
 * Focus keeps one process on the map and docks the rest as process objects.
 * Primary membership and dock process buckets come from WorkspaceState
 * `process_id` window_groups only — never invent multi-window PID groups.
 * When no selection, prefer attention/semantic working keys before OS focus.
 */
export function organiseStageForWorkMode(
  windows: WorkspaceStateWindow[],
  workMode: "flow" | "focus",
  selectedKey: string | null,
  windowGroups: DesktopWindowGroup[] = [],
  monitors: WorkspaceStateMonitor[] = [],
  preferredKeys: ReadonlySet<string> | string[] = [],
): StageWorkModeOrganisation {
  if (workMode !== "focus" || windows.length === 0) {
    return { mapWindows: windows, dockEntries: [] };
  }

  const tiles = layoutStageDesktopWindows(windows, monitors);
  const tileByKey = new Map(tiles.map((tile) => [tile.key, tile]));
  const preferred =
    preferredKeys instanceof Set
      ? preferredKeys
      : new Set(preferredKeys);
  const anchor =
    tiles.find((tile) => tile.key === selectedKey) ??
    tiles.find((tile) => preferred.has(tile.key)) ??
    tiles.find((tile) => tile.focused) ??
    tiles[0] ??
    null;
  if (!anchor) {
    return { mapWindows: windows, dockEntries: [] };
  }

  const processGroups = windowGroups.filter(
    (group) => group.criterion === "process_id",
  );
  const primaryGroup = processGroups.find((group) =>
    group.member_ids.includes(anchor.key),
  );
  const primaryKeys = new Set(
    primaryGroup?.member_ids?.length
      ? primaryGroup.member_ids
      : [anchor.key],
  );

  const mapWindows = windows.filter((window) =>
    primaryKeys.has(stageDesktopWindowKey(window)),
  );

  const dockEntries: StageProcessDockEntry[] = [];
  const dockedKeys = new Set<string>();

  for (const group of processGroups) {
    if (primaryGroup && group.id === primaryGroup.id) {
      continue;
    }
    const members = group.member_ids
      .map((id) => tileByKey.get(id))
      .filter((tile): tile is StageDesktopWindowTile => Boolean(tile))
      .filter((tile) => !primaryKeys.has(tile.key));
    if (members.length === 0) {
      continue;
    }
    const representative = members[0]!;
    for (const member of members) {
      dockedKeys.add(member.key);
    }
    dockEntries.push({
      processKey: representative.processKey,
      processId: representative.processId,
      label: group.label || representative.title,
      hwnd: representative.hwnd,
      tileKey: representative.key,
      windowCount: members.length,
    });
  }

  // Ungrouped non-primary windows stay visible as single dock objects —
  // do not merge them into invented process buckets.
  for (const tile of tiles) {
    if (primaryKeys.has(tile.key) || dockedKeys.has(tile.key)) {
      continue;
    }
    dockEntries.push({
      processKey: tile.processKey,
      processId: tile.processId,
      label: tile.title,
      hwnd: tile.hwnd,
      tileKey: tile.key,
      windowCount: 1,
    });
  }

  return {
    mapWindows,
    dockEntries,
  };
}
