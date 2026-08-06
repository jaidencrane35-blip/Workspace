import { describe, expect, it } from "vitest";
import { matchMomentByName } from "../app/src/lib/momentMatch";
import { resolveIntent } from "../app/src/lib/intentBridge";

const moments = [
  { id: "1", name: "Northwind deck · Client" },
  { id: "2", name: "Sprint board · Engineering" },
  { id: "3", name: "Pilot notes · Writing" },
];

describe("moment match + named intents", () => {
  it("matches unique name fragments", () => {
    expect(matchMomentByName(moments, "Northwind")?.id).toBe("1");
    expect(matchMomentByName(moments, "sprint board")?.id).toBe("2");
  });

  it("refuses ambiguous or missing names", () => {
    expect(matchMomentByName(moments, "deck")).toBeNull();
    expect(matchMomentByName(moments, "Atlantis")).toBeNull();
  });

  it("routes continue <name> and save as <name>", () => {
    // Moments use Continue/Resume; bare “Restore <app>” is Window state (P12.5).
    expect(resolveIntent("Continue Northwind")).toMatchObject({
      kind: "navigateNamed",
      nameQuery: "Northwind",
    });
    expect(resolveIntent("Restore Chrome")).toMatchObject({
      kind: "winRestore",
      query: "Chrome",
    });
    expect(resolveIntent("save this as Northwind")).toMatchObject({
      kind: "saveAs",
      name: "Northwind",
    });
  });

  it("routes open through Application Provider (P11)", () => {
    const launch = resolveIntent("open Cursor");
    expect(launch).toMatchObject({
      kind: "appOpen",
      query: "Cursor",
    });
  });
});
