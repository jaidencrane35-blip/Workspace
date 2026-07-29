/**
 * Purpose: Workspace Home — first viewport: identity, belonging apps, actions,
 *   and honest future-capability notes.
 * Owner: Frontend product shell (Milestone A.1)
 * Inputs: Active workspace, zone/app summaries, navigation callbacks
 * Outputs: Navigation intents to Workspaces / Applications / Layouts
 * Dependencies: None beyond props (no IPC in this component)
 * Non-responsibilities: IPC mutations, Assistant, window control, fake features
 */

import type { ApplicationReference, Workspace } from "../types/domain";
import { monogramFromName } from "../lib/productShellUi";

export type ProductPrimaryView =
  | "home"
  | "workspaces"
  | "applications"
  | "layouts";

interface WorkspaceHomeProps {
  workspace: Workspace | null;
  zoneCount: number;
  applications: ApplicationReference[];
  appsLoading: boolean;
  bootstrapped: boolean;
  busy: boolean;
  onNavigate: (view: ProductPrimaryView) => void;
  onCreateWorkspace: () => void;
}

export function WorkspaceHome({
  workspace,
  zoneCount,
  applications,
  appsLoading,
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
        <h2>Manage your digital workspace</h2>
        <p className="lede">
          Switch environments, keep applications with each workspace, and
          arrange your desktop. Assistant stays a supporting tool — not the
          product itself.
        </p>
      </header>

      {workspace ? (
        <div className="home-current" aria-live="polite">
          <p className="home-current-label">Current workspace</p>
          <div className="home-current-identity">
            <span className="workspace-monogram" aria-hidden="true">
              {monogramFromName(workspace.name)}
            </span>
            <div>
              <p className="home-current-name">{workspace.name}</p>
              <p className="muted">
                {zoneCount === 1
                  ? "1 canvas zone"
                  : `${zoneCount} canvas zones`}
                {" · "}
                {appsLoading
                  ? "loading apps…"
                  : applications.length === 1
                    ? "1 registered application"
                    : `${applications.length} registered applications`}
              </p>
            </div>
          </div>
        </div>
      ) : (
        <div className="arrangement-empty">
          <h3>No active workspace</h3>
          <p className="muted">
            Create a workspace to start organising applications and desktop
            arrangements.
          </p>
          <button type="button" disabled={busy} onClick={onCreateWorkspace}>
            Create workspace
          </button>
        </div>
      )}

      {workspace ? (
        <section aria-label="Applications in this workspace">
          <h3>Applications in this workspace</h3>
          {appsLoading ? (
            <p className="muted">Loading applications…</p>
          ) : applications.length === 0 ? (
            <p className="muted">
              None registered yet. Add apps under Applications — they stay with
              this workspace.
            </p>
          ) : (
            <ul className="home-app-chip-row">
              {applications.slice(0, 8).map((app) => (
                <li key={app.id}>
                  <button
                    type="button"
                    className="home-app-chip"
                    onClick={() => onNavigate("applications")}
                  >
                    <span className="application-monogram compact" aria-hidden="true">
                      {monogramFromName(app.name)}
                    </span>
                    <span>{app.name}</span>
                  </button>
                </li>
              ))}
            </ul>
          )}
          <button
            type="button"
            className="ghost"
            onClick={() => onNavigate("applications")}
          >
            Manage applications
          </button>
        </section>
      ) : null}

      <section aria-label="Available actions">
        <h3>Available now</h3>
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

      <section className="home-future" aria-label="Coming later">
        <h3>Not available yet</h3>
        <ul className="home-future-list muted">
          <li>Flow ↔ Focus work modes (density switching)</li>
          <li>Automatic OS app discovery</li>
          <li>Assistant as a persistent side companion</li>
        </ul>
        <p className="muted">
          These are planned product capabilities — they are not hidden behind
          this screen.
        </p>
      </section>
    </section>
  );
}
