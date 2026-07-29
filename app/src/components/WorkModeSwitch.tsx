/**
 * Purpose: User-facing Flow ↔ Focus chrome-density switch with keyboard support.
 * Owner: Frontend product shell (Milestone B + optimisation)
 * Inputs: current WorkMode + change callback
 * Outputs: User mode selection events (click / arrow keys / Home / End)
 * Dependencies: workMode helpers only
 * Non-responsibilities: OS geometry, arrangements restore, Assistant control
 *
 * Problem: mode switch was mouse-only; commercial UX needs keyboard parity.
 * Why here: presentation control owned by product shell chrome.
 */

import { useRef, type KeyboardEvent } from "react";
import {
  workModeDescription,
  workModeLabel,
  type WorkMode,
} from "../lib/workMode";

interface WorkModeSwitchProps {
  mode: WorkMode;
  onChange: (mode: WorkMode) => void;
  /** compact = chrome strip; stage = larger Layouts control */
  density?: "compact" | "stage";
}

const MODE_ORDER: WorkMode[] = ["flow", "focus"];

export function WorkModeSwitch({
  mode,
  onChange,
  density = "compact",
}: WorkModeSwitchProps) {
  const flowRef = useRef<HTMLButtonElement>(null);
  const focusRef = useRef<HTMLButtonElement>(null);

  const focusButton = (next: WorkMode) => {
    if (next === "flow") {
      flowRef.current?.focus();
    } else {
      focusRef.current?.focus();
    }
  };

  const select = (next: WorkMode) => {
    onChange(next);
    focusButton(next);
  };

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const index = MODE_ORDER.indexOf(mode);
    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      select(MODE_ORDER[(index + 1) % MODE_ORDER.length] ?? "flow");
      return;
    }
    if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      select(
        MODE_ORDER[(index - 1 + MODE_ORDER.length) % MODE_ORDER.length] ??
          "flow",
      );
      return;
    }
    if (event.key === "Home") {
      event.preventDefault();
      select("flow");
      return;
    }
    if (event.key === "End") {
      event.preventDefault();
      select("focus");
    }
  };

  return (
    <div
      className={
        density === "stage"
          ? "work-mode-switch stage-density"
          : "work-mode-switch"
      }
      role="radiogroup"
      aria-label="Workspace presentation mode"
      onKeyDown={onKeyDown}
    >
      <button
        ref={flowRef}
        type="button"
        role="radio"
        className={mode === "flow" ? "work-mode-option active" : "work-mode-option"}
        aria-checked={mode === "flow"}
        tabIndex={mode === "flow" ? 0 : -1}
        onClick={() => select("flow")}
      >
        {workModeLabel("flow")}
      </button>
      <button
        ref={focusRef}
        type="button"
        role="radio"
        className={
          mode === "focus" ? "work-mode-option active" : "work-mode-option"
        }
        aria-checked={mode === "focus"}
        tabIndex={mode === "focus" ? 0 : -1}
        onClick={() => select("focus")}
      >
        {workModeLabel("focus")}
      </button>
      {density === "stage" ? (
        <p className="muted work-mode-hint" id="work-mode-hint">
          {workModeDescription(mode)}
        </p>
      ) : (
        <span className="visually-hidden">{workModeDescription(mode)}</span>
      )}
    </div>
  );
}
