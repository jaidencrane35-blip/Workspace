/**
 * Purpose: Workspace Home — orientation; Stage remains the product landing.
 * Owner: Frontend product shell (Milestone R — Desktop Reality Stage)
 * Inputs: Active workspace profile, zone/app summaries, work mode, navigation
 * Outputs: Navigation intents to Stage / Applications / Workspaces
 * Dependencies: workMode labels, productShellUi monogram (presentation only)
 * Non-goals: Requiring create-workspace before value; AI dashboard; fake apps
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
        <h2>Your desktop, represented</h2>
        <p className="lede">
          Workspace shows your existing computing environment. Open the Stage to
          see observed applications — no setup required first. Named profiles and
          library registration are optional. Assistant stays a companion.
        </p>
      </header>

      <div className="home-stage-cta row">
        <button type="button" onClick={() => onNavigate("layouts")}>
          Open desktop stage
        </button>
        <button
          type="button"
          className="ghost"
          onClick={() => onNavigate("applications")}
        >
          Applications library
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
                {workModeLabel(workMode)} presentation
                {" · "}
                {zoneCount === 1
                  ? "1 companion canvas zone"
                  : `${zoneCount} companion canvas zones`}
              </p>
              <p className="muted">{workModeDescription(workMode)}</p>
            </div>
          </div>
        </div>
      ) : (
        <div className="arrangement-empty">
          <h3>No named profile selected</h3>
          <p className="muted">
            You can still open the Stage to see your desktop when the desktop
            app is running. Create a profile only if you want saved arrangements
            or a library tied to a name.
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

      {workspace ? (
        <section
          className="home-stage-preview"
          aria-label="Optional library preview"
        >
          <h3>Library preview</h3>
          {appsLoading ? (
            <p className="muted">Loading library…</p>
          ) : applications.length === 0 ? (
            <p className="muted">
              No library apps yet. Your observed desktop still appears on the
              Stage without registration.
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
                    <span className="muted">Library app</span>
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
            <span className="home-action-title">Desktop stage</span>
            <span className="muted">
              Observed windows · Flow/Focus · arrangements
            </span>
          </button>
          <button
            type="button"
            className="home-action"
            onClick={() => onNavigate("applications")}
          >
            <span className="home-action-title">Applications</span>
            <span className="muted">Optional library · inspect and launch</span>
          </button>
          <button
            type="button"
            className="home-action"
            onClick={() => onNavigate("workspaces")}
          >
            <span className="home-action-title">Profiles</span>
            <span className="muted">Optional names for saved layouts</span>
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
