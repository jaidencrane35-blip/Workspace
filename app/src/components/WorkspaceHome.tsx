/**
 * Purpose: Workspace Home — first viewport summary of current environment.
 * Owner: Frontend product shell (Milestone A)
 * Inputs: Active workspace, zone count, navigation callbacks
 * Outputs: Navigation intents to Workspaces / Applications / Layouts
 * Dependencies: None beyond props
 * Non-responsibilities: IPC mutations, Assistant, window control
 */

import type { Workspace } from "../types/domain";

export type ProductPrimaryView =
  | "home"
  | "workspaces"
  | "applications"
  | "layouts";

interface WorkspaceHomeProps {
  workspace: Workspace | null;
  zoneCount: number;
  bootstrapped: boolean;
  busy: boolean;
  onNavigate: (view: ProductPrimaryView) => void;
  onCreateWorkspace: () => void;
}

export function WorkspaceHome({
  workspace,
  zoneCount,
  bootstrapped,
  busy,
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
        <h2>Your workspace</h2>
        <p className="lede">
          Manage applications, switch saved work environments, and arrange your
          desktop. Assistant stays available as a supporting tool.
        </p>
      </header>

      {workspace ? (
        <div className="home-current" aria-live="polite">
          <p className="home-current-label">Current workspace</p>
          <p className="home-current-name">{workspace.name}</p>
          <p className="muted">
            {zoneCount === 1 ? "1 canvas zone" : `${zoneCount} canvas zones`} ·
            use Layouts for zones and desktop arrangements
          </p>
        </div>
      ) : (
        <div className="arrangement-empty">
          <h3>No active workspace</h3>
          <p className="muted">
            Create a workspace to start registering applications and saving
            desktop arrangements.
          </p>
          <button type="button" disabled={busy} onClick={onCreateWorkspace}>
            Create workspace
          </button>
        </div>
      )}

      <nav className="home-actions" aria-label="Primary workspace actions">
        <button
          type="button"
          className="home-action"
          onClick={() => onNavigate("workspaces")}
        >
          <span className="home-action-title">Workspaces</span>
          <span className="muted">Switch saved environments</span>
        </button>
        <button
          type="button"
          className="home-action"
          onClick={() => onNavigate("applications")}
        >
          <span className="home-action-title">Applications</span>
          <span className="muted">Register, inspect, and launch</span>
        </button>
        <button
          type="button"
          className="home-action"
          onClick={() => onNavigate("layouts")}
        >
          <span className="home-action-title">Layouts</span>
          <span className="muted">Canvas zones and desktop arrangements</span>
        </button>
      </nav>
    </section>
  );
}
