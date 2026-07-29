import type { DesktopArrangementRestoreResult } from "../types/desktopArrangement";
import {
  applyStatusLabel,
  availabilityLabel,
  gapDiagnostics,
  restoreSummaryCopy,
} from "../lib/desktopArrangementUi";

interface RestoreDiagnosticsViewProps {
  result: DesktopArrangementRestoreResult;
}

export function RestoreDiagnosticsView({ result }: RestoreDiagnosticsViewProps) {
  const gaps = gapDiagnostics(result.diagnostics);

  return (
    <section
      className="restore-diagnostics"
      aria-label="Restore diagnostics"
    >
      <h3>Restore result</h3>
      <p className="restore-diagnostics-summary">{restoreSummaryCopy(result)}</p>

      {gaps.length > 0 ? (
        <div className="restore-diagnostics-block">
          <h4>Unavailable or incomplete</h4>
          <ul className="list">
            {gaps.map((gap) => (
              <li key={gap.entry_id}>
                <strong>{gap.label || gap.entry_id}</strong>
                <span className="badge">{availabilityLabel(gap.availability)}</span>
                <span className="muted">{gap.detail}</span>
              </li>
            ))}
          </ul>
        </div>
      ) : (
        <p className="muted">No restore gaps reported.</p>
      )}

      {result.outcomes.length > 0 ? (
        <div className="restore-diagnostics-block">
          <h4>Apply steps</h4>
          <ul className="list">
            {result.outcomes.map((outcome) => (
              <li key={`${outcome.entry_id}-${outcome.status}`}>
                <strong>{outcome.label || outcome.entry_id}</strong>
                <span className="badge">{applyStatusLabel(outcome.status)}</span>
                <span className="muted">{outcome.detail}</span>
              </li>
            ))}
          </ul>
        </div>
      ) : null}
    </section>
  );
}
