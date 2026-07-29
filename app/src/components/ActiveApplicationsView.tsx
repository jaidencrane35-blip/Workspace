/**
 * Purpose: Show currently observed desktop applications from WorkspaceState.
 * Owner: Frontend product shell (Milestone A.1)
 * Inputs: WorkspaceActiveApplication rows from get_workspace_state
 * Outputs: Read-only presentation
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
    return <p className="muted">Loading observed applications…</p>;
  }
  if (applications.length === 0) {
    return (
      <p className="muted">
        No running desktop applications in the latest observation. Refresh after
        the desktop app has captured windows, or after you launch apps.
      </p>
    );
  }
  return (
    <ul className="product-list" aria-label="Active desktop applications">
      {applications.map((app) => (
        <li key={`${app.process_id}-${app.process_name ?? "unknown"}`}>
          <div className="product-list-item static">
            <span className="product-list-title">
              {activeApplicationLabel(app)}
            </span>
            <span className="product-list-meta muted">
              Observed process · PID {app.process_id}
            </span>
          </div>
        </li>
      ))}
    </ul>
  );
}
