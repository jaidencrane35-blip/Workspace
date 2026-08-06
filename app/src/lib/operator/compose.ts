import type { IntentAction } from "../intentBridge";
import type { OperatorPlan, ProviderStepResult } from "./types";

const JARGON =
  /\b(capability runtime|window provider|application provider|provider registry|capability router)\b/gi;

/** Truthfulness: strip infrastructure jargon from user-facing text. */
export function sanitizeUserText(text: string): string {
  return text.replace(JARGON, "Workspace").replace(/\s{2,}/g, " ").trim();
}

export function composeStepReply(
  action: IntentAction,
  plan: OperatorPlan,
  results: ProviderStepResult[],
): string {
  const last = results[results.length - 1];
  if (!last) {
    return "Nothing happened.";
  }
  if (!last.ok) {
    return sanitizeUserText(
      last.message ?? "That didn’t work — and I won’t pretend it did.",
    );
  }

  switch (action.kind) {
    case "clipboardRead": {
      if (!last.text) {
        return "Clipboard is empty (or has no text).";
      }
      const preview = last.preview ?? last.text;
      return `Clipboard (${last.format ?? "text"}, ${last.bytes ?? 0} bytes):\n${preview}`;
    }
    case "clipboardWrite": {
      const preview = last.preview ?? "";
      return sanitizeUserText(
        `${last.message ?? "Copied."}${last.bytes != null ? ` (${last.bytes} bytes)` : ""}${preview ? `. Preview: ${preview}` : ""}`,
      );
    }
    case "winEnumerate": {
      const titles =
        last.items
          ?.slice(0, 12)
          .map((item) => `• ${item.title}`)
          .join("\n") ?? "(none)";
      const count = last.items?.length ?? 0;
      return count === 0
        ? "No open windows found."
        : `Open windows (${count}):\n${titles}`;
    }
    case "winMonitors": {
      const lines =
        last.monitors
          ?.map(
            (m) =>
              `• Monitor ${m.index}: ${m.name}${m.isPrimary ? " (primary)" : ""}`,
          )
          .join("\n") ?? "(none)";
      return `${last.message ?? "Monitors attached."}\n${lines}`;
    }
    case "appEnumerate": {
      const titles =
        last.items
          ?.slice(0, 12)
          .map((item) => `• ${item.title}`)
          .join("\n") ?? "(none)";
      return `${last.message ?? "Applications listed."}\n${titles}`;
    }
    case "appOpen": {
      if (plan.compositionId === "app.open_or_focus") {
        return sanitizeUserText(
          last.message ??
            (last.status === "focused" || last.ok
              ? "Ready."
              : "Couldn’t open that."),
        );
      }
      break;
    }
    default:
      break;
  }

  return sanitizeUserText(
    last.message ??
      last.text ??
      (last.ok ? "Done." : "That action didn’t succeed."),
  );
}
