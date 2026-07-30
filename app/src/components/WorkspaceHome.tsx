/**
 * Purpose: Quiet Home — point back to Stage; profiles stay optional.
 * Owner: Frontend product shell
 * Inputs: Active profile summary, navigation
 * Outputs: Navigation intents to Stage / Applications / Profiles
 * Dependencies: productShellUi monogram (presentation only)
 * Non-goals: Dashboard cards, "coming soon" lists, setup-first gates
 */

import type { ApplicationReference, Workspace } from "../types/domain";
import { monogramFromName } from "../lib/productShellUi";
import { workModeLabel, type WorkMode } from "../lib/workMode";

export type ProductPrimaryView =
  | "home"
  | "workspaces"
  | "applications"
  | "layouts";

interface WorkspaceHomeProps {
  workspace: Workspace | null;
  applications: ApplicationReference[];
  appsLoading: boolean;
  bootstrapped: boolean;
  busy: boolean;
  workMode: WorkMode;
  onNavigate: (view: ProductPrimaryView) => void;
  onCreateWorkspace: () => void;
}

export function WorkspaceHome({
  workspace,
  applications,
  appsLoading,
  bootstrapped,
  busy,
  workMode,
  onNavigate,
  onCreateWorkspace,
}: WorkspaceHomeProps) {
  if (!bootstrapped) {
    return (
      <section className="product-panel product-home" aria-label="Home">
        <p className="muted">Loading…</p>
      </section>
    );
  }

  return (
    <section className="product-panel product-home" aria-label="Home">
      <header className="product-panel-hero">
        <p className="arrangement-eyebrow">Home</p>
        <h2>Desktop</h2>
        <p className="lede">
          Your existing desktop is the product. Open Stage to see it.
        </p>
      </header>

      <div className="home-stage-cta row">
        <button type="button" onClick={() => onNavigate("layouts")}>
          Open Stage
        </button>
        <button
          type="button"
          className="ghost"
          onClick={() => onNavigate("applications")}
        >
          Running apps
        </button>
      </div>

      {workspace ? (
        <div className="home-current" aria-live="polite">
          <p className="home-current-label">Named profile (optional)</p>
          <div className="home-current-identity">
            <span className="workspace-monogram" aria-hidden="true">
              {monogramFromName(workspace.name)}
            </span>
            <div>
              <p className="home-current-name">{workspace.name}</p>
              <p className="muted">
                {appsLoading
                  ? "loading library…"
                  : applications.length === 1
                    ? "1 library app"
                    : `${applications.length} library apps`}
                {" · "}
                {workModeLabel(workMode)}
              </p>
            </div>
          </div>
        </div>
      ) : (
        <div className="arrangement-empty">
          <p className="muted">
            No named profile needed for Stage. Create one only to save layouts.
          </p>
          <button
            type="button"
            className="ghost"
            disabled={busy}
            onClick={onCreateWorkspace}
          >
            Create optional profile
          </button>
        </div>
      )}
    </section>
  );
}
