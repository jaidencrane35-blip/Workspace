import { useCallback, useEffect, useRef, useState } from "react";
import {
  SHELL_MODE_EVENT,
  emitShellModeEvent,
  saveShellMode,
  saveSpecializedTarget,
} from "../../lib/shellRuntime";
import { SHELL_EXITS } from "../../lib/shellStateMachine";
import {
  applyShellMode,
  exitWorkspace,
  hideShellToTaskbar,
  startOperatorDrag,
} from "../../lib/shellWindows";

export { SHELL_MODE_EVENT };

interface DesktopOperatorProps {
  /** Browser/fallback: parent updates React mode. */
  onOpenCompact?: () => void;
  onExpand?: () => void;
  onOpenSettings?: () => void;
}

/**
 * Mode 0 — Floating Desktop Operator.
 * Left click → Compact. Right click → recovery menu. Double click → Expand.
 */
export function DesktopOperator({
  onOpenCompact,
  onExpand,
  onOpenSettings,
}: DesktopOperatorProps) {
  const dragRef = useRef<{ sx: number; sy: number; moved: boolean } | null>(
    null,
  );
  const lastClickRef = useRef(0);
  const clickTimerRef = useRef<number | null>(null);
  const [menu, setMenu] = useState<{ x: number; y: number } | null>(null);

  useEffect(() => {
    document.documentElement.dataset.shellMode = "0";
    document.title = "Workspace";
    return () => {
      delete document.documentElement.dataset.shellMode;
      if (clickTimerRef.current !== null) {
        window.clearTimeout(clickTimerRef.current);
      }
    };
  }, []);

  useEffect(() => {
    if (!menu) {
      return;
    }
    const close = () => setMenu(null);
    window.addEventListener("click", close);
    window.addEventListener("blur", close);
    return () => {
      window.removeEventListener("click", close);
      window.removeEventListener("blur", close);
    };
  }, [menu]);

  const openCompact = useCallback(async () => {
    setMenu(null);
    saveSpecializedTarget("none");
    saveShellMode(1);
    emitShellModeEvent();
    if (onOpenCompact) {
      onOpenCompact();
      return;
    }
    await applyShellMode(1);
  }, [onOpenCompact]);

  const openExpanded = useCallback(async () => {
    setMenu(null);
    saveSpecializedTarget("none");
    saveShellMode(2);
    emitShellModeEvent();
    if (onExpand) {
      onExpand();
      return;
    }
    await applyShellMode(2);
  }, [onExpand]);

  const openSettings = useCallback(async () => {
    setMenu(null);
    saveSpecializedTarget("settings");
    saveShellMode(3);
    emitShellModeEvent();
    if (onOpenSettings) {
      onOpenSettings();
      return;
    }
    await applyShellMode(3);
  }, [onOpenSettings]);

  const onHide = useCallback(async () => {
    setMenu(null);
    await hideShellToTaskbar();
  }, []);

  const onExit = useCallback(async () => {
    setMenu(null);
    await exitWorkspace();
  }, []);

  return (
    <div className="op-desktop-root">
      <button
        type="button"
        className="op-desktop"
        aria-label="Workspace desktop operator"
        aria-haspopup="menu"
        onContextMenu={(event) => {
          event.preventDefault();
          setMenu({ x: event.clientX, y: event.clientY });
        }}
        onPointerDown={(event) => {
          if (event.button !== 0) {
            return;
          }
          dragRef.current = {
            sx: event.clientX,
            sy: event.clientY,
            moved: false,
          };
          void startOperatorDrag();
        }}
        onPointerMove={(event) => {
          const started = dragRef.current;
          if (!started) {
            return;
          }
          if (
            Math.hypot(event.clientX - started.sx, event.clientY - started.sy) >
            4
          ) {
            started.moved = true;
          }
        }}
        onPointerUp={(event) => {
          if (event.button !== 0) {
            return;
          }
          const started = dragRef.current;
          dragRef.current = null;
          if (!started || started.moved) {
            return;
          }
          const now = Date.now();
          if (now - lastClickRef.current < 350) {
            if (clickTimerRef.current !== null) {
              window.clearTimeout(clickTimerRef.current);
              clickTimerRef.current = null;
            }
            lastClickRef.current = 0;
            void openExpanded();
            return;
          }
          lastClickRef.current = now;
          if (clickTimerRef.current !== null) {
            window.clearTimeout(clickTimerRef.current);
          }
          clickTimerRef.current = window.setTimeout(() => {
            clickTimerRef.current = null;
            void openCompact();
          }, 280);
        }}
      >
        <span className="op-desktop__mark" aria-hidden="true">
          W
        </span>
      </button>

      {menu && (
        <ul
          className="op-desktop-menu"
          role="menu"
          style={{ left: menu.x, top: menu.y }}
          onClick={(e) => e.stopPropagation()}
        >
          {SHELL_EXITS[0].map((action) => (
            <li key={action.id} role="none">
              <button
                type="button"
                role="menuitem"
                className="op-desktop-menu__item"
                onClick={() => {
                  if (action.to === 1) {
                    void openCompact();
                  } else if (action.to === 2) {
                    void openExpanded();
                  } else if (action.to === "settings") {
                    void openSettings();
                  } else if (action.to === "hide") {
                    void onHide();
                  } else if (action.to === "exit") {
                    void onExit();
                  }
                }}
              >
                {action.label}
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
