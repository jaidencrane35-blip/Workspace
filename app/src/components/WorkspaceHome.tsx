/**
 * Purpose: Workspace Home — first viewport: apps-first identity and entry to
 *   the Workspace stage (Layouts).
 * Owner: Frontend product shell (Milestone D — Workspace Stage)
 * Inputs: Active workspace, zone/app summaries, work mode, navigation callbacks
 * Outputs: Navigation intents to Workspaces / Applications / Layouts
 * Dependencies: workMode labels, productShellUi monogram (presentation only)
 * Non-responsibilities: IPC mutations, Assistant, OS window control, fake features
 */

import type { ApplicationReference, Workspace } from "../types/domain";
import { monogramFromName } from "../lib/productShellUi";
import {
  workModeDescription,
  workModeLabel,
  type WorkMode,
} from "../lib/workMode";

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
  workMode: WorkMode;
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
        <h2>Your applications, organised</h2>
        <p className="lede">
          Workspace is where you organise and work with the apps that belong to
          each environment. Open the workspace stage to see them front and
          centre. Assistant stays a companion — not the product.
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
                {appsLoading
                  ? "loading apps…"
                  : applications.length === 1
                    ? "1 application on stage"
                    : `${applications.length} applications on stage`}
                {" · "}
                {workModeLabel(workMode)} presentation
                {" · "}
                {zoneCount === 1
                  ? "1 companion canvas zone"
                  : `${zoneCount} companion canvas zones`}
              </p>
              <p className="muted">{workModeDescription(workMode)}</p>
            </div>
          </div>
          <div className="home-stage-cta row">
            <button type="button" onClick={() => onNavigate("layouts")}>
              Open workspace stage
            </button>
            <button
              type="button"
              className="ghost"
              onClick={() => onNavigate("applications")}
            >
              Manage applications
            </button>
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
        <section
          className="home-stage-preview"
          aria-label="Applications in this workspace"
        >
          <h3>Applications on stage</h3>
          {appsLoading ? (
            <p className="muted">Loading applications…</p>
          ) : applications.length === 0 ? (
            <p className="muted">
              None registered yet. Add apps under Applications — they become the
              centre of this workspace.
            </p>
          ) : (
            <ul className="stage-tile-grid home-stage-grid" aria-label="App preview">
              {applications.slice(0, 6).map((app) => (
                <li key={app.id}>
                  <button
                    type="button"
                    className="stage-tile home-stage-tile"
                    onClick={() => onNavigate("layouts")}
                  >
                    <span
                      className="application-monogram large"
                      aria-hidden="true"
                    >
                      {monogramFromName(app.name)}
                    </span>
                    <span className="stage-tile-name">{app.name}</span>
                    <span className="muted">On workspace stage</span>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </section>
      ) : null}

      <section aria-label="Available actions">
        <h3>Go to</h3>
        <nav className="home-actions" aria-label="Primary workspace actions">
          <button
            type="button"
            className="home-action home-action-primary"
            onClick={() => onNavigate("layouts")}
          >
            <span className="home-action-title">Workspace stage</span>
            <span className="muted">
              Applications front and centre · Flow/Focus · arrangements
            </span>
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
            onClick={() => onNavigate("workspaces")}
          >
            <span className="home-action-title">Workspaces</span>
            <span className="muted">Switch saved environments</span>
          </button>
        </nav>
      </section>

      <section className="home-future" aria-label="Coming later">
        <h3>Not available yet</h3>
        <ul className="home-future-list muted">
          <li>OS window move/resize when switching Flow ↔ Focus</li>
          <li>Automatic OS app discovery</li>
          <li>Window grouping and lock</li>
        </ul>
        <p className="muted">
          Flow/Focus chrome density and the Assistant companion rail are
          available now. Geometry apply on mode switch remains planned.
        </p>
      </section>
    </section>
  );
}
