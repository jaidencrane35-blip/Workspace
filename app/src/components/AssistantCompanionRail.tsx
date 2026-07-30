/**
 * Purpose: Quiet right-side Assistant companion rail.
 * Owner: Frontend product shell (Product Contract V3)
 * Inputs: active workspace, busy/error/message callbacks, work mode, collapse
 * Outputs: Companion ask/answer only
 * Dependencies: AssistantIntelligencePanel (compose surface)
 * Non-responsibilities: Mode switching, OS windows, advanced operator workflow
 */

import { useEffect, useId, useRef } from "react";
import type { WorkMode } from "../lib/workMode";
import type { Workspace } from "../types/domain";
import { AssistantIntelligencePanel } from "./AssistantIntelligencePanel";
import { ASSISTANT_COMPANION_RAIL_ID } from "../lib/assistantRail";

interface AssistantCompanionRailProps {
  workspace: Workspace | null;
  workMode: WorkMode;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  onEnsureWorkspace?: () => Promise<Workspace>;
  onCollapse: () => void;
}

export function AssistantCompanionRail({
  workspace,
  workMode,
  busy,
  onBusy,
  onError,
  onMessage,
  onEnsureWorkspace,
  onCollapse,
}: AssistantCompanionRailProps) {
  const titleId = useId();
  const railRef = useRef<HTMLElement | null>(null);

  useEffect(() => {
    const node = railRef.current;
    if (!node) {
      return;
    }
    node.focus();
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onCollapse();
      }
    };
    node.addEventListener("keydown", onKeyDown);
    return () => node.removeEventListener("keydown", onKeyDown);
  }, [onCollapse]);

  return (
    <aside
      ref={railRef}
      id={ASSISTANT_COMPANION_RAIL_ID}
      className={
        workMode === "focus"
          ? "assistant-companion-rail density-focus"
          : "assistant-companion-rail density-flow"
      }
      aria-labelledby={titleId}
      data-work-mode={workMode}
      tabIndex={-1}
    >
      <header className="assistant-companion-header">
        <div className="assistant-companion-header-row">
          <h2 id={titleId}>Ask</h2>
          <button
            type="button"
            className="ghost assistant-rail-collapse"
            onClick={onCollapse}
            aria-label="Hide Assistant companion"
            title="Hide Assistant companion (Esc)"
          >
            Hide
          </button>
        </div>
      </header>

      <div className="assistant-companion-body">
        <AssistantIntelligencePanel
          workspace={workspace}
          busy={busy}
          onBusy={onBusy}
          onError={onError}
          onMessage={onMessage}
          onEnsureWorkspace={onEnsureWorkspace}
          presentation="rail"
        />
      </div>
    </aside>
  );
}
