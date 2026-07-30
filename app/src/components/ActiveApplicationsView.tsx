/**
 * Purpose: Observed running desktop applications as objects (process-first).
 * Owner: Frontend product shell (Product Contract V4)
 * Inputs: WorkspaceActiveApplication rows from get_workspace_state
 * Outputs: Read-only presentation grouped by process when multiple windows
 * Dependencies: applicationsUi helpers
 * Non-responsibilities: Observation capture, window control, registry mutation
 */

import { activeApplicationLabel } from "../lib/applicationsUi";
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
    return (
      <p className="muted">
        Nothing observed yet. Open Stage once the desktop app is running.
      </p>
    );
  }
  return (
    <ul className="product-list app-object-list" aria-label="Running applications">
      {applications.map((app) => (
        <li key={`${app.process_id}-${app.process_name ?? "unknown"}`}>
          <div className="product-list-item static app-object-row">
            <span className="product-list-title">
              {activeApplicationLabel(app)}
            </span>
          </div>
        </li>
      ))}
    </ul>
  );
}
