/**
 * Purpose: Persistent right-side Assistant companion rail (Milestone C).
 * Owner: Frontend product shell
 * Inputs: active workspace, busy/error/message callbacks, work mode, collapse
 * Outputs: Companion presentation only — explain / help affordances
 * Dependencies: AssistantIntelligencePanel, AssistantPanel (existing IPC surfaces)
 * Non-responsibilities: Mode switching, OS windows, PermissionGateway, new AI
 *
 * Problem: concept references keep Assistant as a stable side rail, not a peer tab.
 * Why here: chrome placement only — reuses existing assistant panels.
 */

import { useEffect, useId, useRef, useState } from "react";
import type { WorkMode } from "../lib/workMode";
import type { Workspace } from "../types/domain";
import { AssistantIntelligencePanel } from "./AssistantIntelligencePanel";
import { AssistantPanel } from "./AssistantPanel";

/** Stable DOM id for chrome aria-controls → companion rail. */
export const ASSISTANT_COMPANION_RAIL_ID = "workspace-assistant-companion-rail";

interface AssistantCompanionRailProps {
  workspace: Workspace | null;
  workMode: WorkMode;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  onCollapse: () => void;
}

export function AssistantCompanionRail({
  workspace,
  workMode,
  busy,
  onBusy,
  onError,
  onMessage,
  onCollapse,
}: AssistantCompanionRailProps) {
  const titleId = useId();
  const [advancedOpen, setAdvancedOpen] = useState(false);
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
          <div>
            <p className="arrangement-eyebrow">Assistant</p>
            <h2 id={titleId}>Companion</h2>
          </div>
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
        <p className="lede assistant-companion-lede">
          Supporting help for your desktop workspace. Workspace, apps, and
          layouts stay primary.
        </p>
      </header>

      <div className="assistant-companion-body">
        <AssistantIntelligencePanel
          workspace={workspace}
          busy={busy}
          onBusy={onBusy}
          onError={onError}
          onMessage={onMessage}
          presentation="rail"
        />

        <section
          className="assistant-legacy-section"
          aria-label="Advanced governed workflow"
        >
          <button
            type="button"
            className="ghost assistant-advanced-toggle"
            aria-expanded={advancedOpen}
            onClick={() => setAdvancedOpen((open) => !open)}
          >
            {advancedOpen ? "Hide advanced workflow" : "Advanced workflow"}
          </button>
          {advancedOpen ? (
            <>
              <p className="muted">
                Optional plan → permission path. Prefer Home, Applications, and
                Layouts for daily use.
              </p>
              <AssistantPanel
                workspace={workspace}
                busy={busy}
                onBusy={onBusy}
                onError={onError}
                onMessage={onMessage}
              />
            </>
          ) : null}
        </section>
      </div>
    </aside>
  );
}
