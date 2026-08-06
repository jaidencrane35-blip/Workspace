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

  it("routes restore <name> and save as <name>", () => {
    expect(resolveIntent("Restore Northwind")).toMatchObject({
      kind: "navigateNamed",
      nameQuery: "Northwind",
    });
    expect(resolveIntent("save this as Northwind")).toMatchObject({
      kind: "saveAs",
      name: "Northwind",
    });
  });

  it("keeps launch honesty without fabricating opens", () => {
    const launch = resolveIntent("open Cursor");
    expect(launch.kind).toBe("unknown");
    expect(launch.reply.toLowerCase()).toMatch(/won.t pretend/);
  });
});
