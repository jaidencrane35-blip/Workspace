/** Relative time from owned timestamps — no invented activity. */

export function formatRelativeTime(iso: string, now = Date.now()): string {
  const at = new Date(iso).getTime();
  if (Number.isNaN(at)) {
    return iso;
  }
  const deltaSec = Math.round((at - now) / 1000);
  const abs = Math.abs(deltaSec);
  const rtf = new Intl.RelativeTimeFormat("en", { numeric: "auto" });

  if (abs < 60) {
    return rtf.format(deltaSec, "second");
  }
  const deltaMin = Math.round(deltaSec / 60);
  if (Math.abs(deltaMin) < 60) {
    return rtf.format(deltaMin, "minute");
  }
  const deltaHour = Math.round(deltaMin / 60);
  if (Math.abs(deltaHour) < 48) {
    return rtf.format(deltaHour, "hour");
  }
  const deltaDay = Math.round(deltaHour / 24);
  return rtf.format(deltaDay, "day");
}

export function formatAbsoluteTime(iso: string): string {
  const at = new Date(iso);
  return Number.isNaN(at.getTime()) ? iso : at.toLocaleString();
}
