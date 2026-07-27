import type { AttentionReason } from "../../app/src/types/domain";

export const attentionReason = (
  explanationKey: string,
  weight = 40,
): AttentionReason => ({
  source: "task_graph",
  signal: "blocked_task",
  weight,
  explanation_key: explanationKey,
});
