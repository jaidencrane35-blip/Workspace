import type { DesktopArrangement } from "../types/desktopArrangement";
import {
  arrangementStatusLabel,
  entryIdentitySummary,
} from "../lib/desktopArrangementUi";

interface DesktopArrangementDetailsProps {
  arrangement: DesktopArrangement;
}

export function DesktopArrangementDetails({
  arrangement,
}: DesktopArrangementDetailsProps) {
  return (
    <div className="arrangement-details" aria-label="Arrangement details">
      <header className="arrangement-details-header">
        <h3>{arrangement.name}</h3>
        <span className="badge">{arrangementStatusLabel(arrangement.status)}</span>
      </header>
      {arrangement.description.trim() ? (
        <p className="muted arrangement-details-desc">{arrangement.description}</p>
      ) : null}
      <p className="muted mono arrangement-details-id">{arrangement.id}</p>
      {arrangement.entries.length === 0 ? (
        <p className="muted">This arrangement has no window membership yet.</p>
      ) : (
        <ul className="arrangement-entry-list">
          {arrangement.entries.map((entry) => (
            <li key={entry.id}>
              <strong>{entry.label || "Untitled window"}</strong>
              <span className="muted">{entryIdentitySummary(entry)}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
