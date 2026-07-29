/**
 * Purpose: Spatial “application stage” strip on Layouts — presents registered
 *   workspace apps as stage assets beside the arrangements rail.
 * Owner: Frontend product shell (Cycle 1)
 * Inputs: workspace identity, application list, zone count, navigation callback
 * Outputs: Presentation + navigate-to-Applications intent
 * Dependencies: applicationsUi / layoutsStageUi / productShellUi helpers
 * Non-responsibilities: Window control, Flow/Focus modes, OS discovery,
 *   arrangement restore, Assistant, inventing live HWND tiles
 */

import {
  applicationIdentityLine,
  canLaunchApplication,
} from "../lib/applicationsUi";
import {
  layoutsStageCanvasNote,
  layoutsStageEmptyAppsCopy,
  layoutsStageEyebrow,
  layoutsStageLede,
  layoutsStageTitle,
} from "../lib/layoutsStageUi";
import { monogramFromName } from "../lib/productShellUi";
import type { ApplicationReference, Workspace } from "../types/domain";

interface WorkspaceApplicationStageProps {
  workspace: Workspace;
  applications: ApplicationReference[];
  appsLoading: boolean;
  zoneCount: number;
  onManageApplications: () => void;
}

export function WorkspaceApplicationStage({
  workspace,
  applications,
  appsLoading,
  zoneCount,
  onManageApplications,
}: WorkspaceApplicationStageProps) {
  const empty = layoutsStageEmptyAppsCopy();

  return (
    <section
      className="workspace-application-stage"
      aria-label="Workspace application stage"
    >
      <header className="stage-hero">
        <p className="arrangement-eyebrow">{layoutsStageEyebrow()}</p>
        <h2>{layoutsStageTitle(workspace.name)}</h2>
        <p className="lede">{layoutsStageLede()}</p>
      </header>

      <div className="stage-meta muted">{layoutsStageCanvasNote(zoneCount)}</div>

      {appsLoading ? (
        <p className="muted">Loading applications…</p>
      ) : applications.length === 0 ? (
        <div className="arrangement-empty stage-empty">
          <h3>{empty.title}</h3>
          <p className="muted">{empty.body}</p>
          <div className="stage-tile-grid ghost-preview" aria-hidden="true">
            {["Editor", "Browser", "Notes"].map((label) => (
              <div key={label} className="stage-tile preview">
                <span className="application-monogram">
                  {monogramFromName(label)}
                </span>
                <span className="stage-tile-name">{label}</span>
                <span className="muted">example</span>
              </div>
            ))}
          </div>
          <button type="button" onClick={onManageApplications}>
            Add applications
          </button>
        </div>
      ) : (
        <ul className="stage-tile-grid" aria-label="Applications on stage">
          {applications.map((app) => (
            <li key={app.id}>
              <article className="stage-tile">
                <span className="application-monogram" aria-hidden="true">
                  {monogramFromName(app.name)}
                </span>
                <span className="stage-tile-name">{app.name}</span>
                <span className="product-list-meta muted">
                  {applicationIdentityLine(app)}
                </span>
                <span className="stage-tile-status">
                  {canLaunchApplication(app)
                    ? "Launchable asset"
                    : "Registered asset"}
                </span>
              </article>
            </li>
          ))}
        </ul>
      )}

      {applications.length > 0 ? (
        <div className="row stage-actions">
          <button
            type="button"
            className="ghost"
            onClick={onManageApplications}
          >
            Manage applications
          </button>
          <p className="muted stage-hint">
            Desktop window save/restore stays in the arrangements rail →
          </p>
        </div>
      ) : null}
    </section>
  );
}
