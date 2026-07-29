/**
 * Purpose: Presentational card grid of registered workspace applications.
 * Owner: Frontend product shell (Milestone A.1)
 * Inputs: ApplicationReference rows + selection/launch callbacks
 * Outputs: Selection and launch intent events
 * Dependencies: applicationsUi + productShellUi helpers
 * Non-responsibilities: IPC, PermissionGateway, process creation, OS discovery
 */

import {
  applicationIdentityLine,
  applicationStatusLabel,
  canLaunchApplication,
} from "../lib/applicationsUi";
import { monogramFromName } from "../lib/productShellUi";
import type { ApplicationReference } from "../types/domain";

interface ApplicationListProps {
  applications: ApplicationReference[];
  selectedId: string | null;
  busy: boolean;
  onSelect: (id: string) => void;
  onLaunch: (app: ApplicationReference) => void;
}

export function ApplicationList({
  applications,
  selectedId,
  busy,
  onSelect,
  onLaunch,
}: ApplicationListProps) {
  return (
    <ul className="application-card-grid" aria-label="Registered applications">
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
