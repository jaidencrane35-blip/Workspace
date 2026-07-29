/**
 * Purpose: User-facing Flow ↔ Focus chrome-density switch.
 * Owner: Frontend product shell (Milestone B)
 * Inputs: current WorkMode + change callback
 * Outputs: User mode selection events
 * Dependencies: workMode helpers only
 * Non-responsibilities: OS geometry, arrangements restore, Assistant control
 */

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

export function WorkModeSwitch({
  mode,
  onChange,
  density = "compact",
}: WorkModeSwitchProps) {
  return (
    <div
      className={
        density === "stage"
          ? "work-mode-switch stage-density"
          : "work-mode-switch"
      }
      role="group"
      aria-label="Workspace presentation mode"
    >
      <button
        type="button"
        className={mode === "flow" ? "work-mode-option active" : "work-mode-option"}
        aria-pressed={mode === "flow"}
        onClick={() => onChange("flow")}
      >
        {workModeLabel("flow")}
      </button>
      <button
        type="button"
        className={
          mode === "focus" ? "work-mode-option active" : "work-mode-option"
        }
        aria-pressed={mode === "focus"}
        onClick={() => onChange("focus")}
      >
        {workModeLabel("focus")}
      </button>
      {density === "stage" ? (
        <p className="muted work-mode-hint">{workModeDescription(mode)}</p>
      ) : (
        <span className="visually-hidden">{workModeDescription(mode)}</span>
      )}
    </div>
  );
}
