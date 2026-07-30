/**
 * Purpose: Spatial application stage on Layouts — apps as the product centrepiece.
 * Owner: Frontend product shell (Milestone D — Workspace Stage)
 * Inputs: workspace, apps, zone count, work mode, launch + navigate callbacks
 * Outputs: Stage presentation, Focus primary selection, launch / manage intents
 * Dependencies: applicationsUi / applicationLaunch / layoutsStageUi / workMode
 * Non-responsibilities: OS window moves, arrangement editing, grouping, AI,
 *   WindowController, inventing live HWND thumbnails
 *
 * Milestone D: make registered applications the hero of the workspace UI.
 */

import { useEffect, useState } from "react";
import {
  applicationIdentityLine,
  applicationStatusLabel,
  canLaunchApplication,
} from "../lib/applicationsUi";
import {
  layoutsStageCanvasNote,
  layoutsStageEmptyAppsCopy,
  layoutsStageEyebrow,
  layoutsStageFocusHint,
  layoutsStageFocusNote,
  layoutsStageFlowHint,
  layoutsStageTitle,
} from "../lib/layoutsStageUi";
import { monogramFromName } from "../lib/productShellUi";
import {
  partitionFocusApplications,
  workModeStageLede,
  type WorkMode,
} from "../lib/workMode";
import type { ApplicationReference, Workspace } from "../types/domain";
import { FocusSupportingAppChips } from "./FocusSupportingAppChips";
import { WorkModeSwitch } from "./WorkModeSwitch";

interface WorkspaceApplicationStageProps {
  workspace: Workspace;
  applications: ApplicationReference[];
  appsLoading: boolean;
  zoneCount: number;
  workMode: WorkMode;
  busy: boolean;
  onWorkModeChange: (mode: WorkMode) => void;
  onManageApplications: () => void;
  onLaunchApplication: (app: ApplicationReference) => void;
}

export function WorkspaceApplicationStage({
  workspace,
  applications,
  appsLoading,
  zoneCount,
  workMode,
  busy,
  onWorkModeChange,
  onManageApplications,
  onLaunchApplication,
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
        <p className="stage-meta muted">{layoutsStageCanvasNote(zoneCount)}</p>
      ) : (
        <p className="stage-meta muted">{layoutsStageFocusNote()}</p>
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
                <span className="application-monogram large">
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
          {applications.map((app) => {
            const launchable = canLaunchApplication(app);
            return (
              <li key={app.id}>
                <article className="stage-tile">
                  <span className="application-monogram large" aria-hidden="true">
                    {monogramFromName(app.name)}
                  </span>
                  <span className="stage-tile-name">{app.name}</span>
                  <span className="product-list-meta muted">
                    {applicationIdentityLine(app)}
                  </span>
                  <span className="stage-tile-status">
                    {applicationStatusLabel(app)}
                  </span>
                  <button
                    type="button"
                    className="stage-tile-launch"
                    disabled={busy || !launchable}
                    title={
                      launchable
                        ? "Launch this application"
                        : "Add an executable path under Applications first"
                    }
                    onClick={() => onLaunchApplication(app)}
                  >
                    Launch
                  </button>
                </article>
              </li>
            );
          })}
        </ul>
      ) : (
        <div className="focus-stage-layout" aria-label="Focus application stage">
          {primary ? (
            <article className="stage-tile stage-tile-primary">
              <p className="home-current-label">Primary on stage</p>
              <span className="application-monogram large" aria-hidden="true">
                {monogramFromName(primary.name)}
              </span>
              <span className="stage-tile-name">{primary.name}</span>
              <span className="product-list-meta muted">
                {applicationIdentityLine(primary)}
              </span>
              <span className="stage-tile-status">
                {applicationStatusLabel(primary)} · emphasised in Focus
              </span>
              <button
                type="button"
                className="stage-tile-launch"
                disabled={busy || !canLaunchApplication(primary)}
                onClick={() => onLaunchApplication(primary)}
              >
                Launch
              </button>
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
              ? layoutsStageFlowHint()
              : layoutsStageFocusHint()}
          </p>
        </div>
      ) : null}
    </section>
  );
}
