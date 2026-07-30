/**
 * Purpose: Spatial “application stage” on Layouts — Flow (dense) vs Focus
 *   (primary emphasis) chrome presentation of registered workspace apps.
 * Owner: Frontend product shell (Milestone B chrome density)
 * Inputs: workspace, apps, zone count, work mode, navigation callback
 * Outputs: Presentation + navigate-to-Applications; primary selection in Focus
 * Dependencies: applicationsUi / layoutsStageUi / productShellUi / workMode
 * Non-responsibilities: OS window moves, arrangement restore apply, AI,
 *   WindowController, inventing live HWND tiles
 *
 * Problem: reference Flow/Focus density without geometry apply yet.
 * Why here: Layouts is the workspace stage host.
 * Why not in DesktopArrangement: this is chrome density, not OS restore.
 */

import { useEffect, useState } from "react";
import {
  applicationIdentityLine,
  canLaunchApplication,
} from "../lib/applicationsUi";
import {
  layoutsStageCanvasNote,
  layoutsStageEmptyAppsCopy,
  layoutsStageEyebrow,
  layoutsStageTitle,
} from "../lib/layoutsStageUi";
import { monogramFromName } from "../lib/productShellUi";
import {
  partitionFocusApplications,
  workModeStageLede,
  type WorkMode,
} from "../lib/workMode";
import type { ApplicationReference, Workspace } from "../types/domain";
import { WorkModeSwitch } from "./WorkModeSwitch";
import { FocusSupportingAppChips } from "./FocusSupportingAppChips";

interface WorkspaceApplicationStageProps {
  workspace: Workspace;
  applications: ApplicationReference[];
  appsLoading: boolean;
  zoneCount: number;
  workMode: WorkMode;
  onWorkModeChange: (mode: WorkMode) => void;
  onManageApplications: () => void;
}

export function WorkspaceApplicationStage({
  workspace,
  applications,
  appsLoading,
  zoneCount,
  workMode,
  onWorkModeChange,
  onManageApplications,
}: WorkspaceApplicationStageProps) {
  const empty = layoutsStageEmptyAppsCopy();
  const [primaryId, setPrimaryId] = useState<string | null>(null);

  useEffect(() => {
    if (applications.length === 0) {
      setPrimaryId(null);
      return;
    }
    setPrimaryId((prev) => {
      if (prev && applications.some((app) => app.id === prev)) {
        return prev;
      }
      return applications[0]?.id ?? null;
    });
  }, [applications]);

  const { primary, supporting } = partitionFocusApplications(
    applications,
    primaryId,
  );

  return (
    <section
      className={
        workMode === "focus"
          ? "workspace-application-stage mode-focus"
          : "workspace-application-stage mode-flow"
      }
      aria-label="Workspace application stage"
      data-work-mode={workMode}
    >
      <header className="stage-hero stage-hero-with-mode">
        <div>
          <p className="arrangement-eyebrow">{layoutsStageEyebrow()}</p>
          <h2>{layoutsStageTitle(workspace.name)}</h2>
          <p className="lede">{workModeStageLede(workMode)}</p>
        </div>
        <WorkModeSwitch
          mode={workMode}
          onChange={onWorkModeChange}
          density="stage"
        />
      </header>

      {workMode === "flow" ? (
        <div className="stage-meta muted">{layoutsStageCanvasNote(zoneCount)}</div>
      ) : (
        <div className="stage-meta muted">
          Focus chrome hides the companion canvas to reduce noise. Switch to Flow
          to edit zones. Desktop arrangements remain in the rail — OS apply is
          not part of this milestone.
        </div>
      )}

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
      ) : workMode === "flow" ? (
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
      ) : (
        <div className="focus-stage-layout" aria-label="Focus application stage">
          {primary ? (
            <article className="stage-tile stage-tile-primary">
              <p className="home-current-label">Primary</p>
              <span className="application-monogram large" aria-hidden="true">
                {monogramFromName(primary.name)}
              </span>
              <span className="stage-tile-name">{primary.name}</span>
              <span className="product-list-meta muted">
                {applicationIdentityLine(primary)}
              </span>
              <span className="stage-tile-status">
                Emphasised in Focus — still a registry asset, not an OS move
              </span>
            </article>
          ) : null}
          {supporting.length > 0 ? (
            <FocusSupportingAppChips
              apps={supporting}
              onSelect={setPrimaryId}
              selectTitle="Make primary in Focus"
            />
          ) : null}
        </div>
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
            {workMode === "flow"
              ? "Desktop window save/restore stays in the arrangements rail →"
              : "Supporting apps stay available — Focus does not quit them."}
          </p>
        </div>
      ) : null}
    </section>
  );
}
