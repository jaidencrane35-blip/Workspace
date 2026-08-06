import { describe, expect, it } from "vitest";
import {
  classifyEvolutionRequest,
  createProposal,
  isEvolutionRequest,
  parseApprovalIntent,
} from "../app/src/lib/capabilityEvolution";
import { resolveIntent } from "../app/src/lib/intentBridge";

describe("capability evolution foundation", () => {
  it("classifies common owner phrases", () => {
    expect(classifyEvolutionRequest("Add a screenshot button").category).toBe(
      "capability",
    );
    expect(classifyEvolutionRequest("Move the chat").category).toBe(
      "user_preference",
    );
    expect(
      classifyEvolutionRequest("Make the background transparent").category,
    ).toBe("user_preference");
    expect(classifyEvolutionRequest("Resize the operator").category).toBe(
      "user_preference",
    );
  });

  it("creates proposals without executing changes", () => {
    const proposal = createProposal("Add a screenshot button.");
    expect(proposal.status).toBe("proposed");
    expect(proposal.implementationPlan.length).toBeGreaterThan(0);
    expect(proposal.audit[0]?.event).toBe("proposed");
  });

  it("detects evolution requests and approval phrases", () => {
    expect(isEvolutionRequest("Add a screenshot button")).toBe(true);
    expect(isEvolutionRequest("save this")).toBe(false);
    expect(isEvolutionRequest("Move this window to the left.")).toBe(false);
    expect(isEvolutionRequest("Show me my open windows.")).toBe(false);
    expect(parseApprovalIntent("approve proposal")).toBe("approve");
    expect(parseApprovalIntent("reject proposal")).toBe("reject");
  });

  it("routes evolution through the intent bridge as proposals", () => {
    const action = resolveIntent("Add a screenshot button");
    expect(action.kind).toBe("proposal");
    expect(action.reply.toLowerCase()).toMatch(/will not change the product/);
  });
});
