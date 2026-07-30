/**
 * Purpose: Presentational card grid of registered workspace applications.
 * Owner: Frontend product shell (Milestone A.1 + optimisation)
 * Inputs: ApplicationReference rows, work-mode density, selection/launch callbacks
 * Outputs: Selection and launch intent events
 * Dependencies: applicationsUi + productShellUi + workMode type
 * Non-responsibilities: IPC, PermissionGateway, process creation, OS discovery
 *
 * Problem: Applications surface ignored Flow/Focus density.
 * Why here: same chrome-density preference as Layouts stage.
 */

import {
  applicationIdentityLine,
  applicationStatusLabel,
  canLaunchApplication,
} from "../lib/applicationsUi";
import { monogramFromName } from "../lib/productShellUi";
import { partitionFocusApplications, type WorkMode } from "../lib/workMode";
import type { ApplicationReference } from "../types/domain";
import { FocusSupportingAppChips } from "./FocusSupportingAppChips";

interface ApplicationListProps {
  applications: ApplicationReference[];
  selectedId: string | null;
  workMode: WorkMode;
  busy: boolean;
  onSelect: (id: string) => void;
  onLaunch: (app: ApplicationReference) => void;
}

export function ApplicationList({
  applications,
  selectedId,
  workMode,
  busy,
  onSelect,
  onLaunch,
}: ApplicationListProps) {
  if (workMode === "focus") {
    const { primary, supporting } = partitionFocusApplications(
      applications,
      selectedId,
    );
    return (
      <div className="focus-stage-layout" aria-label="Registered applications">
        {primary ? (
          <article className="application-card selected stage-tile-primary">
            <p className="home-current-label">Primary</p>
            <button
              type="button"
              className="application-card-main"
              disabled={busy}
              aria-current="true"
              onClick={() => onSelect(primary.id)}
            >
              <span className="application-monogram large" aria-hidden="true">
                {monogramFromName(primary.name)}
              </span>
              <span className="application-card-copy">
                <span className="product-list-title">{primary.name}</span>
                <span className="product-list-meta muted">
                  {applicationIdentityLine(primary)}
                </span>
                <span className="application-status">
                  {applicationStatusLabel(primary)}
                </span>
              </span>
            </button>
            <button
              type="button"
              className="ghost application-launch"
              disabled={busy || !canLaunchApplication(primary)}
              onClick={() => onLaunch(primary)}
            >
              Launch
            </button>
          </article>
        ) : null}
        {supporting.length > 0 ? (
          <FocusSupportingAppChips
            apps={supporting}
            busy={busy}
            onSelect={onSelect}
            selectTitle="Emphasise in Focus"
          />
        ) : null}
      </div>
    );
  }

  return (
    <ul
      className="application-card-grid density-flow"
      aria-label="Registered applications"
    >
      {applications.map((app) => {
        const selected = app.id === selectedId;
        const launchable = canLaunchApplication(app);
        return (
          <li key={app.id}>
            <article
              className={
                selected ? "application-card selected" : "application-card"
              }
            >
              <button
                type="button"
                className="application-card-main"
                disabled={busy}
                aria-current={selected ? "true" : undefined}
                onClick={() => onSelect(app.id)}
              >
                <span className="application-monogram" aria-hidden="true">
                  {monogramFromName(app.name)}
                </span>
                <span className="application-card-copy">
                  <span className="product-list-title">{app.name}</span>
                  <span className="product-list-meta muted">
                    {applicationIdentityLine(app)}
                  </span>
                  <span className="application-status">
                    {applicationStatusLabel(app)}
                  </span>
                </span>
              </button>
              <button
                type="button"
                className="ghost application-launch"
                disabled={busy || !launchable}
                title={
                  launchable
                    ? "Launch this application"
                    : "Add an executable path before launching"
                }
                onClick={() => onLaunch(app)}
              >
                Launch
              </button>
            </article>
          </li>
        );
      })}
    </ul>
  );
}
