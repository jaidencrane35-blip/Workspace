import type { ResourceRef } from "./domain";

export interface Position2D {
  x: number;
  y: number;
}

export interface Size2D {
  width: number;
  height: number;
}

export interface LayoutBounds {
  position: Position2D;
  size: Size2D;
}

export interface Viewport {
  origin: Position2D;
  size: Size2D;
  zoom: number;
}

export interface LayoutNode {
  resource_ref: ResourceRef;
  bounds: LayoutBounds;
  z_index: number;
  collapsed: boolean;
  hidden: boolean;
  locked: boolean;
  metadata: string | null;
}

export interface Layout {
  id: string;
  workspace_id: string;
  viewport: Viewport;
  nodes: LayoutNode[];
  metadata: string | null;
  created_at: string;
  updated_at: string;
}

export const DEFAULT_VIEWPORT: Viewport = {
  origin: { x: 0, y: 0 },
  size: { width: 1920, height: 1080 },
  zoom: 1,
};

export const DEFAULT_ZONE_SIZE: Size2D = { width: 220, height: 140 };
