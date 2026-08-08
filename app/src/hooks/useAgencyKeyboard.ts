import { useEffect, useRef, type RefObject } from "react";

/** Conversation composer — Product Gravity focus return (P17.S3). */
export const CONVERSATION_INPUT_ID = "workspace-conversation-input";

export function focusConversationInput(): void {
  const el = document.getElementById(CONVERSATION_INPUT_ID);
  if (el instanceof HTMLElement) {
    el.focus({ preventScroll: true });
  }
}

export function isTextEntryTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) {
    return false;
  }
  if (target.isContentEditable) {
    return true;
  }
  if (target instanceof HTMLTextAreaElement) {
    return true;
  }
  if (target instanceof HTMLSelectElement) {
    return true;
  }
  if (target instanceof HTMLInputElement) {
    const type = (target.type || "text").toLowerCase();
    return ![
      "button",
      "submit",
      "checkbox",
      "radio",
      "file",
      "reset",
      "image",
      "hidden",
      "color",
      "range",
    ].includes(type);
  }
  return false;
}

function focusPrimaryControl(root: HTMLElement): void {
  const primary =
    root.querySelector<HTMLElement>(
      'button.exp-btn.primary:not([disabled]), button.primary:not([disabled])',
    ) ?? root.querySelector<HTMLElement>("button:not([disabled])");
  primary?.focus({ preventScroll: true });
}

/**
 * P17.S3 — Moments agency keyboard continuity.
 * Esc = dismiss · Enter = primary · focus in on open · Conversation on close.
 * Single handler; skips text-entry targets and summary toggles.
 */
export function useAgencyKeyboard(options: {
  active: boolean;
  busy?: boolean;
  onPrimary: () => void;
  onDismiss: () => void;
}): RefObject<HTMLDivElement> {
  const rootRef = useRef<HTMLDivElement>(null!);
  const onPrimaryRef = useRef(options.onPrimary);
  const onDismissRef = useRef(options.onDismiss);
  const busyRef = useRef(options.busy ?? false);
  onPrimaryRef.current = options.onPrimary;
  onDismissRef.current = options.onDismiss;
  busyRef.current = options.busy ?? false;

  const { active } = options;

  useEffect(() => {
    if (!active) {
      return;
    }

    const focusTimer = window.setTimeout(() => {
      if (rootRef.current) {
        focusPrimaryControl(rootRef.current);
      }
    }, 0);

    const onKeyDown = (event: KeyboardEvent) => {
      if (busyRef.current || event.isComposing) {
        return;
      }
      const target = event.target;
      if (isTextEntryTarget(target)) {
        return;
      }
      if (target instanceof Element && target.closest("summary")) {
        return;
      }

      if (event.key === "Escape") {
        event.preventDefault();
        event.stopPropagation();
        onDismissRef.current();
        return;
      }
      if (event.key === "Enter") {
        event.preventDefault();
        event.stopPropagation();
        onPrimaryRef.current();
      }
    };

    // Bubble phase so voice capture (capture-phase) keeps ownership while listening.
    window.addEventListener("keydown", onKeyDown);
    return () => {
      window.clearTimeout(focusTimer);
      window.removeEventListener("keydown", onKeyDown);
      focusConversationInput();
    };
  }, [active]);

  return rootRef;
}
