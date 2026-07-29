/**
 * Purpose: Presentational list of registered workspace applications.
 * Owner: Frontend product shell (Milestone A)
 * Inputs: ApplicationReference rows + selection/launch callbacks
 * Outputs: Selection and launch intent events
 * Dependencies: applicationsUi helpers
 * Non-responsibilities: IPC, PermissionGateway, process creation
 */

import {
  applicationIdentityLine,
  canLaunchApplication,
} from "../lib/applicationsUi";
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
    <ul className="product-list" aria-label="Registered applications">
      {applications.map((app) => {
        const selected = app.id === selectedId;
        const launchable = canLaunchApplication(app);
        return (
          <li key={app.id} className="application-list-row">
            <button
              type="button"
              className={
                selected ? "product-list-item active" : "product-list-item"
              }
              disabled={busy}
              aria-current={selected ? "true" : undefined}
              onClick={() => onSelect(app.id)}
            >
              <span className="product-list-title">{app.name}</span>
              <span className="product-list-meta muted">
                {applicationIdentityLine(app)}
              </span>
            </button>
            <button
              type="button"
              className="ghost application-launch"
              disabled={busy || !launchable}
              title={
                launchable
                  ? "Launch through governed application path"
                  : "Add an executable path before launching"
              }
              onClick={() => onLaunch(app)}
            >
              Launch
            </button>
          </li>
        );
      })}
    </ul>
  );
}
