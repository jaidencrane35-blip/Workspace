/**
 * Purpose: Flow ↔ Focus density switch (same desktop, different spatial weight).
 * Owner: Frontend product shell
 * Inputs: current WorkMode + change callback
 * Outputs: User mode selection events (click / arrow keys / Home / End)
 * Dependencies: workMode helpers only
 * Non-responsibilities: OS geometry, arrangements restore, Assistant control
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
}

const MODE_ORDER: WorkMode[] = ["flow", "focus"];

export function WorkModeSwitch({ mode, onChange }: WorkModeSwitchProps) {
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
      className="work-mode-switch"
      role="radiogroup"
      aria-label="Flow or Focus presentation"
      onKeyDown={onKeyDown}
    >
      <button
        ref={flowRef}
        type="button"
        role="radio"
        className={mode === "flow" ? "work-mode-option active" : "work-mode-option"}
        aria-checked={mode === "flow"}
        title={workModeDescription("flow")}
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
        title={workModeDescription("focus")}
        onClick={() => select("focus")}
      >
        {workModeLabel("focus")}
      </button>
      <p className="work-mode-hint visually-hidden" aria-live="polite">
        {workModeDescription(mode)}
      </p>
    </div>
  );
}
