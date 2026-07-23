import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type PointerEvent as ReactPointerEvent,
  type WheelEvent as ReactWheelEvent,
} from "react";
import {
  ensureLayout,
  mergeZoneNodes,
  saveLayout,
} from "../lib/layoutPersistence";
import type { Zone } from "../types/domain";
import type { Layout, LayoutNode, Viewport } from "../types/layout";
import { DEFAULT_VIEWPORT } from "../types/layout";

interface CanvasShellProps {
  workspaceId: string;
  workspaceName: string;
  zones: Zone[];
  busy?: boolean;
  onError: (message: string) => void;
  onSaved?: (layout: Layout) => void;
  onCreateWorkspace?: () => void;
  onAddZone?: () => void;
}

interface DragState {
  zoneId: string;
  startClientX: number;
  startClientY: number;
  originX: number;
  originY: number;
}

interface ResizeState {
  zoneId: string;
  startClientX: number;
  startClientY: number;
  originWidth: number;
  originHeight: number;
}

interface PanState {
  startClientX: number;
  startClientY: number;
  originX: number;
  originY: number;
}

const MIN_ZONE = 120;

export function CanvasShell({
  workspaceId,
  workspaceName,
  zones,
  busy = false,
  onError,
  onSaved,
  onCreateWorkspace,
  onAddZone,
}: CanvasShellProps) {
  const [layout, setLayout] = useState<Layout | null>(null);
  const [nodes, setNodes] = useState<LayoutNode[]>([]);
  const [viewport, setViewport] = useState<Viewport>(DEFAULT_VIEWPORT);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const dragRef = useRef<DragState | null>(null);
  const resizeRef = useRef<ResizeState | null>(null);
  const panRef = useRef<PanState | null>(null);
  const zoomSaveRef = useRef<number | null>(null);
  const nodesRef = useRef(nodes);
  const viewportRef = useRef(viewport);
  const layoutRef = useRef(layout);

  nodesRef.current = nodes;
  viewportRef.current = viewport;
  layoutRef.current = layout;

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    ensureLayout(workspaceId)
      .then((next) => {
        if (cancelled) {
          return;
        }
        setLayout(next);
        setViewport(next.viewport);
        setNodes(mergeZoneNodes(zones, next.nodes));
      })
      .catch((err: unknown) => {
        onError(err instanceof Error ? err.message : String(err));
      })
      .finally(() => {
        if (!cancelled) {
          setLoading(false);
        }
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [workspaceId, onError]);

  const zoneKey = zones.map((z) => z.id).join("|");

  useEffect(() => {
    if (!layout) {
      return;
    }
    setNodes((prev) =>
      mergeZoneNodes(zones, prev.length > 0 ? prev : layout.nodes),
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [zoneKey, layout?.id]);

  const persist = useCallback(
    async (nextNodes: LayoutNode[], nextViewport: Viewport) => {
      const current = layoutRef.current;
      if (!current) {
        return;
      }
      setSaving(true);
      try {
        const saved = await saveLayout({
          layoutId: current.id,
          viewport: nextViewport,
          nodes: nextNodes,
        });
        setLayout(saved);
        onSaved?.(saved);
      } catch (err: unknown) {
        onError(err instanceof Error ? err.message : String(err));
      } finally {
        setSaving(false);
      }
    },
    [onError, onSaved],
  );

  const onZonePointerDown = (event: ReactPointerEvent, zoneId: string) => {
    event.stopPropagation();
    const node = nodes.find((n) => n.resource_ref.id === zoneId);
    if (!node || node.locked) {
      return;
    }
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
    dragRef.current = {
      zoneId,
      startClientX: event.clientX,
      startClientY: event.clientY,
      originX: node.bounds.position.x,
      originY: node.bounds.position.y,
    };
  };

  const onResizePointerDown = (event: ReactPointerEvent, zoneId: string) => {
    event.stopPropagation();
    const node = nodes.find((n) => n.resource_ref.id === zoneId);
    if (!node || node.locked) {
      return;
    }
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
    resizeRef.current = {
      zoneId,
      startClientX: event.clientX,
      startClientY: event.clientY,
      originWidth: node.bounds.size.width,
      originHeight: node.bounds.size.height,
    };
  };

  const onSurfacePointerDown = (event: ReactPointerEvent) => {
    if (event.button !== 0 && event.button !== 1) {
      return;
    }
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
    panRef.current = {
      startClientX: event.clientX,
      startClientY: event.clientY,
      originX: viewport.origin.x,
      originY: viewport.origin.y,
    };
  };

  const onPointerMove = (event: ReactPointerEvent) => {
    const resize = resizeRef.current;
    if (resize) {
      const zoom = viewportRef.current.zoom || 1;
      const dx = (event.clientX - resize.startClientX) / zoom;
      const dy = (event.clientY - resize.startClientY) / zoom;
      setNodes((prev) =>
        prev.map((n) =>
          n.resource_ref.id === resize.zoneId
            ? {
                ...n,
                bounds: {
                  ...n.bounds,
                  size: {
                    width: Math.max(MIN_ZONE, resize.originWidth + dx),
                    height: Math.max(MIN_ZONE, resize.originHeight + dy),
                  },
                },
              }
            : n,
        ),
      );
      return;
    }

    const drag = dragRef.current;
    if (drag) {
      const zoom = viewportRef.current.zoom || 1;
      const dx = (event.clientX - drag.startClientX) / zoom;
      const dy = (event.clientY - drag.startClientY) / zoom;
      setNodes((prev) =>
        prev.map((n) =>
          n.resource_ref.id === drag.zoneId
            ? {
                ...n,
                bounds: {
                  ...n.bounds,
                  position: {
                    x: drag.originX + dx,
                    y: drag.originY + dy,
                  },
                },
              }
            : n,
        ),
      );
      return;
    }

    const pan = panRef.current;
    if (pan) {
      const zoom = viewportRef.current.zoom || 1;
      const dx = (event.clientX - pan.startClientX) / zoom;
      const dy = (event.clientY - pan.startClientY) / zoom;
      setViewport((prev) => ({
        ...prev,
        origin: {
          x: pan.originX - dx,
          y: pan.originY - dy,
        },
      }));
    }
  };

  const onPointerUp = () => {
    const changed =
      dragRef.current !== null ||
      resizeRef.current !== null ||
      panRef.current !== null;
    dragRef.current = null;
    resizeRef.current = null;
    panRef.current = null;
    if (changed) {
      void persist(nodesRef.current, viewportRef.current);
    }
  };

  const onWheel = (event: ReactWheelEvent) => {
    event.preventDefault();
    const factor = event.deltaY < 0 ? 1.08 : 1 / 1.08;
    setViewport((prev) => {
      const nextZoom = Math.min(2.5, Math.max(0.4, prev.zoom * factor));
      const next = { ...prev, zoom: nextZoom };
      viewportRef.current = next;
      return next;
    });
    if (zoomSaveRef.current !== null) {
      window.clearTimeout(zoomSaveRef.current);
    }
    zoomSaveRef.current = window.setTimeout(() => {
      void persist(nodesRef.current, viewportRef.current);
      zoomSaveRef.current = null;
    }, 400);
  };

  const zoneName = (id: string) =>
    zones.find((z) => z.id === id)?.name ?? id.slice(0, 8);

  if (loading) {
    return (
      <div className="canvas-shell">
        <p className="muted">Loading canvas…</p>
      </div>
    );
  }

  return (
    <div className="canvas-shell">
      <div className="canvas-chrome">
        <div>
          <strong>{workspaceName}</strong>
          <span className="muted"> · spatial canvas</span>
        </div>
        <div className="row canvas-actions">
          {onAddZone && (
            <button type="button" disabled={busy} onClick={onAddZone}>
              Add zone
            </button>
          )}
          {onCreateWorkspace && (
            <button type="button" disabled={busy} onClick={onCreateWorkspace}>
              New workspace
            </button>
          )}
          <span className="muted">
            {saving ? "Saving…" : layout ? `layout ${layout.id.slice(0, 8)}…` : ""}
            {" · "}
            drag · resize corner · pan · wheel zoom
          </span>
        </div>
      </div>
      <div
        className="canvas-viewport"
        onPointerDown={onSurfacePointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerUp}
        onWheel={onWheel}
      >
        <div
          className="canvas-world"
          style={{
            transform: `translate(${-viewport.origin.x * viewport.zoom}px, ${
              -viewport.origin.y * viewport.zoom
            }px) scale(${viewport.zoom})`,
          }}
        >
          {nodes
            .filter((n) => !n.hidden)
            .map((node) => (
              <div
                key={node.resource_ref.id}
                className="zone-node"
                style={{
                  left: node.bounds.position.x,
                  top: node.bounds.position.y,
                  width: node.bounds.size.width,
                  height: node.bounds.size.height,
                  zIndex: node.z_index,
                }}
                onPointerDown={(e) =>
                  onZonePointerDown(e, node.resource_ref.id)
                }
              >
                <div className="zone-node-title">
                  {zoneName(node.resource_ref.id)}
                </div>
                <div className="zone-node-meta mono">
                  {node.resource_ref.id.slice(0, 10)}…
                </div>
                <button
                  type="button"
                  className="zone-resize"
                  aria-label="Resize zone"
                  onPointerDown={(e) =>
                    onResizePointerDown(e, node.resource_ref.id)
                  }
                />
              </div>
            ))}
          {zones.length === 0 && (
            <div className="canvas-empty muted">
              No zones yet — use Add zone above.
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
