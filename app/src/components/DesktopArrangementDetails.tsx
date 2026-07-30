import type { DesktopArrangement } from "../types/desktopArrangement";
import type { WorkspaceStateWindow } from "../types/domain";
import {
  arrangementStatusLabel,
  entryIdentitySummary,
} from "../lib/desktopArrangementUi";
import { deriveArrangementProductMeta } from "../lib/arrangementProductUi";

interface DesktopArrangementDetailsProps {
  arrangement: DesktopArrangement;
  observedWindows?: readonly WorkspaceStateWindow[];
}

export function DesktopArrangementDetails({
  arrangement,
  observedWindows = [],
}: DesktopArrangementDetailsProps) {
  const meta = deriveArrangementProductMeta(arrangement, observedWindows);

  return (
    <div className="arrangement-details" aria-label="Arrangement details">
      <header className="arrangement-details-header">
        <h3>{arrangement.name}</h3>
        <span className="badge">{arrangementStatusLabel(arrangement.status)}</span>
      </header>
      {arrangement.description.trim() ? (
        <p className="muted arrangement-details-desc">{arrangement.description}</p>
      ) : null}
      <p className="muted arrangement-details-meta">{meta.summaryLine}</p>
      <p className="muted arrangement-details-meta">{meta.readinessLine}</p>
      {arrangement.entries.length === 0 ? (
        <p className="muted">This Arrangement has no windows yet.</p>
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
