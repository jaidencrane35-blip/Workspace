import { IpcCommandError, invokeIpc } from "./ipc";
import type { Zone } from "../types/domain";
import type {
  Layout,
  LayoutNode,
  Viewport,
} from "../types/layout";
import { DEFAULT_VIEWPORT, DEFAULT_ZONE_SIZE } from "../types/layout";

export async function ensureLayout(workspaceId: string): Promise<Layout> {
  try {
    return await invokeIpc<Layout>("get_layout", { workspaceId });
  } catch (err: unknown) {
    if (err instanceof IpcCommandError && err.code === "layout_not_found") {
      return invokeIpc<Layout>("create_layout", { workspaceId });
    }
    throw err;
  }
}

export function defaultNodeForZone(zone: Zone, index: number): LayoutNode {
  const col = index % 3;
  const row = Math.floor(index / 3);
  return {
    resource_ref: { kind: "zone", id: zone.id },
    bounds: {
      position: {
        x: 48 + col * 260,
        y: 48 + row * 180,
      },
      size: { ...DEFAULT_ZONE_SIZE },
    },
    z_index: index,
    collapsed: false,
    hidden: false,
    locked: false,
    metadata: null,
  };
}

/** Merge graph zones with layout nodes; new zones get grid defaults. */
export function mergeZoneNodes(
  zones: Zone[],
  existingNodes: LayoutNode[],
): LayoutNode[] {
  const byId = new Map(
    existingNodes
      .filter((n) => n.resource_ref.kind === "zone")
      .map((n) => [n.resource_ref.id, n]),
  );

  return zones.map((zone, index) => {
    const existing = byId.get(zone.id);
    if (existing) {
      return existing;
    }
    return defaultNodeForZone(zone, index);
  });
}

export async function saveLayout(args: {
  layoutId: string;
  viewport: Viewport;
  nodes: LayoutNode[];
}): Promise<Layout> {
  return invokeIpc<Layout>("update_layout", {
    layoutId: args.layoutId,
    viewport: args.viewport ?? DEFAULT_VIEWPORT,
    nodes: args.nodes,
    metadata: null,
  });
}
