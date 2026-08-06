import { useCallback, useEffect, useRef, useState } from "react";
import {
  emitShellModeEvent,
  saveShellMode,
} from "../../lib/shellRuntime";
import { SHELL_EXITS } from "../../lib/shellStateMachine";
import {
  applyShellMode,
  exitWorkspace,
  startOperatorDrag,
} from "../../lib/shellWindows";

interface DesktopOperatorProps {
  /** Browser/fallback: parent updates React form. */
  onOpenConversation?: () => void;
}

/**
 * Form A — Desktop Operator.
 * Single click → Conversation. Drag → move. Right click → Open / Exit.
 */
export function DesktopOperator({ onOpenConversation }: DesktopOperatorProps) {
  const dragRef = useRef<{ sx: number; sy: number; moved: boolean } | null>(
    null,
  );
  const [menu, setMenu] = useState<{ x: number; y: number } | null>(null);

  useEffect(() => {
    document.documentElement.dataset.shellMode = "0";
    document.title = "Workspace";
    return () => {
      delete document.documentElement.dataset.shellMode;
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

  const openConversation = useCallback(async () => {
    setMenu(null);
    saveShellMode(1);
    emitShellModeEvent();
    if (onOpenConversation) {
      onOpenConversation();
      return;
    }
    await applyShellMode(1);
  }, [onOpenConversation]);

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
          void openConversation();
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
                    void openConversation();
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
