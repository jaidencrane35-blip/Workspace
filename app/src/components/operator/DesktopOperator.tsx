import { useCallback, useEffect, useRef } from "react";
import { emitShellModeEvent, saveShellMode } from "../../lib/shellRuntime";
import {
  restoreConversationShell,
  startOperatorDrag,
} from "../../lib/shellWindows";

interface DesktopOperatorProps {
  /** Browser/fallback: parent updates React form. */
  onOpenConversation?: () => void;
}

/**
 * Form A — Desktop Operator (independent shell mode, not a resized conversation).
 * Click / double-click → restore Conversation immediately.
 * Drag past threshold → move companion (drag never steals the click).
 */
export function DesktopOperator({ onOpenConversation }: DesktopOperatorProps) {
  const gestureRef = useRef<{
    sx: number;
    sy: number;
    moved: boolean;
    dragging: boolean;
  } | null>(null);

  useEffect(() => {
    document.documentElement.dataset.shellMode = "0";
    document.documentElement.dataset.shellForm = "operator";
    document.title = "Workspace";
    return () => {
      delete document.documentElement.dataset.shellMode;
      delete document.documentElement.dataset.shellForm;
    };
  }, []);

  const openConversation = useCallback(async () => {
    // Browser fallback: parent owns React mode; still persist durable ShellMode.
    if (onOpenConversation) {
      saveShellMode(1);
      emitShellModeEvent();
      onOpenConversation();
      return;
    }
    // Tauri Form A: same restore path as tray Show Conversation (P19.S1).
    await restoreConversationShell();
  }, [onOpenConversation]);

  return (
    <div className="op-desktop-root" data-shell-form="operator">
      <button
        type="button"
        className="op-desktop"
        aria-label="Workspace — open conversation"
        onPointerDown={(event) => {
          if (event.button !== 0) {
            return;
          }
          gestureRef.current = {
            sx: event.clientX,
            sy: event.clientY,
            moved: false,
            dragging: false,
          };
          try {
            event.currentTarget.setPointerCapture(event.pointerId);
          } catch {
            /* ignore */
          }
        }}
        onPointerMove={(event) => {
          const gesture = gestureRef.current;
          if (!gesture || gesture.dragging) {
            return;
          }
          if (
            Math.hypot(event.clientX - gesture.sx, event.clientY - gesture.sy) >
            5
          ) {
            gesture.moved = true;
            gesture.dragging = true;
            void startOperatorDrag();
          }
        }}
        onPointerUp={(event) => {
          if (event.button !== 0) {
            return;
          }
          const gesture = gestureRef.current;
          gestureRef.current = null;
          try {
            event.currentTarget.releasePointerCapture(event.pointerId);
          } catch {
            /* ignore */
          }
          if (!gesture || gesture.moved) {
            return;
          }
          void openConversation();
        }}
        onPointerCancel={() => {
          gestureRef.current = null;
        }}
        onDoubleClick={(event) => {
          event.preventDefault();
          void openConversation();
        }}
      >
        <span className="op-desktop__mark" aria-hidden="true">
          W
        </span>
      </button>
    </div>
  );
}
