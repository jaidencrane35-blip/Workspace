import type { DesktopArrangement } from "../types/desktopArrangement";
import { arrangementSummaryLine } from "../lib/desktopArrangementUi";

interface DesktopArrangementListProps {
  arrangements: DesktopArrangement[];
  selectedId: string | null;
  busy: boolean;
  onSelect: (id: string) => void;
}

export function DesktopArrangementList({
  arrangements,
  selectedId,
  busy,
  onSelect,
}: DesktopArrangementListProps) {
  if (arrangements.length === 0) {
    return null;
  }

  return (
    <ul className="arrangement-list" aria-label="Saved desktop arrangements">
      {arrangements.map((arrangement) => {
        const selected = arrangement.id === selectedId;
        return (
          <li key={arrangement.id}>
            <button
              type="button"
              className={
                selected
                  ? "arrangement-list-item selected"
                  : "arrangement-list-item"
              }
              disabled={busy}
              aria-pressed={selected}
              onClick={() => onSelect(arrangement.id)}
            >
              <span className="arrangement-list-name">{arrangement.name}</span>
              <span className="arrangement-list-meta muted">
                {arrangementSummaryLine(arrangement)}
              </span>
            </button>
          </li>
        );
      })}
    </ul>
  );
}
