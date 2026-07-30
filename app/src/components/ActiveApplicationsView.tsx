/**
 * Purpose: Observed running desktop applications as visual objects.
 * Owner: Frontend product shell (Product Contract V4)
 * Inputs: WorkspaceActiveApplication rows from get_workspace_state
 * Outputs: Object grid presentation
 * Dependencies: applicationsUi + productShellUi
 * Non-responsibilities: Observation capture, window control, registry mutation
 */

import {
  activeApplicationName,
  activeApplicationWindowLine,
} from "../lib/applicationsUi";
import { monogramFromName } from "../lib/productShellUi";
import type { WorkspaceActiveApplication } from "../types/domain";

interface ActiveApplicationsViewProps {
  applications: WorkspaceActiveApplication[];
  loading: boolean;
}

export function ActiveApplicationsView({
  applications,
  loading,
}: ActiveApplicationsViewProps) {
  if (loading && applications.length === 0) {
    return <p className="muted">Loading…</p>;
  }
  if (applications.length === 0) {
    return <p className="muted">Nothing observed yet.</p>;
  }
  return (
    <ul className="app-object-grid" aria-label="Running applications">
      {applications.map((app) => {
        const name = activeApplicationName(app);
        return (
          <li key={`${app.process_id}-${app.process_name ?? "unknown"}`}>
            <article className="app-object-card">
              <span className="application-monogram large" aria-hidden="true">
                {monogramFromName(name)}
              </span>
              <span className="app-object-name">{name}</span>
              <span className="muted app-object-meta">
                {activeApplicationWindowLine(app)}
              </span>
            </article>
          </li>
        );
      })}
    </ul>
  );
}
