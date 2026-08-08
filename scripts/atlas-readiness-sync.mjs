#!/usr/bin/env node
/**
 * One-shot Atlas v3 readiness repair for Capability Execution Loop.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const atlasPath = path.join(
  root,
  "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md",
);
let t = fs.readFileSync(atlasPath, "utf8");

t = t.replace(
  "`Capability ID · Name · Layer · Status · Lifecycle · Dependencies · Verification · Product Proof · Priority · Engineering Notes`",
  "`Capability ID · Name · Layer · Status · Lifecycle · Dependencies · Verification · Product Proof · Priority · Engineering Notes · Capability Readiness Score`",
);

if (!t.includes("## 1.1 Capability Readiness Score")) {
  const needle =
    "Required metadata on every capability:\n\n`Capability ID · Name · Layer · Status · Lifecycle · Dependencies · Verification · Product Proof · Priority · Engineering Notes · Capability Readiness Score`\n\n---";
  const insert = `Required metadata on every capability:

\`Capability ID · Name · Layer · Status · Lifecycle · Dependencies · Verification · Product Proof · Priority · Engineering Notes · Capability Readiness Score\`

### 1.1 Capability Readiness Score

Seven equally weighted dimensions (0–100%). **Overall** = arithmetic mean.

| Dimension | 100% means |
| --- | --- |
| Architecture | Fits frozen Kernel Operator / Intent / Runtime path |
| Dependencies | All prerequisite Atlas capabilities eng-complete |
| Implementation | Vertical slice live in repo |
| Verification | Atlas verifiers + regression green |
| Product Proof | Owner live path accepted for this capability |
| Trusted | Owner declared Trusted |
| Production | Trusted + production-gate ready |

Record format:

\`\`\`text
Architecture ............ N%
Dependencies ............ N%
Implementation .......... N%
Verification ............ N%
Product Proof ........... N%
Trusted ................. N%
Production .............. N%
Overall ................. N%
\`\`\`

Default guidance: eng-complete without Owner PP → Architecture/Dependencies/Implementation/Verification = 100; Product Proof/Trusted/Production = 0; **Overall ≈ 57%**.

---`;
  if (!t.includes(needle)) {
    console.error("atlas-readiness-sync: required-metadata needle not found");
    process.exit(1);
  }
  t = t.replace(needle, insert);
}

function readinessBlock({
  arch = 100,
  deps = 100,
  impl = 0,
  ver = 0,
  pp = 0,
  trusted = 0,
  prod = 0,
}) {
  const overall = Math.round((arch + deps + impl + ver + pp + trusted + prod) / 7);
  return `| **Readiness** | Overall **${overall}%** |
| | \`\`\`text
Architecture ............ ${String(arch).padStart(3)}%
Dependencies ............ ${String(deps).padStart(3)}%
Implementation .......... ${String(impl).padStart(3)}%
Verification ............ ${String(ver).padStart(3)}%
Product Proof ........... ${String(pp).padStart(3)}%
Trusted ................. ${String(trusted).padStart(3)}%
Production .............. ${String(prod).padStart(3)}%
Overall ................. ${String(overall).padStart(3)}%
\`\`\` |`;
}

// Patch C-OBS-003 / C-OBS-004 records thoroughly.
const obs003Old = `#### C-OBS-003 Desktop UI Tree
| | |
| --- | --- |
| **Name** | Desktop UI Tree |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** |
| **Dependencies** | C-OBS-001 / C-OBS-002; Window Provider; Windows UIA |
| **Verification** | \`verify-desktop-ui-tree\`, \`tests/desktop-ui-tree.test.ts\`, kernel \`window_enumerate_controls_through_router\` |
| **Product Proof** | **Required** — \`P22_S2_DESKTOP_UI_TREE.md\`; Owner live session |
| **Priority** | P0 (this slice complete eng) |
| **Engineering Notes** | Observation only. \`UiAutomationPort\` + \`enumerate_controls\`. No click/type. Custom-drawn UI may return honest empty. Owner Execution Loop authorized under R2. Cleared B-API-001 for observation surface. |`;

const obs003New = `#### C-OBS-003 Desktop UI Tree
| | |
| --- | --- |
| **Name** | Desktop UI Tree |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** |
| **Dependencies** | C-OBS-001 / C-OBS-002; Window Provider; Windows UIA |
| **Verification** | \`verify-desktop-ui-tree\`, \`tests/desktop-ui-tree.test.ts\`, kernel \`window_enumerate_controls_through_router\` |
| **Product Proof** | **Required** — \`P22_S2_DESKTOP_UI_TREE.md\`; Owner live session |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Observation only. \`UiAutomationPort\` + \`enumerate_controls\`. No click/type. Custom-drawn UI may return honest empty. Owner Execution Loop authorized under R2. Cleared B-API-001 for observation surface. |
${readinessBlock({ arch: 100, deps: 100, impl: 100, ver: 100, pp: 0, trusted: 0, prod: 0 })}`;

const obs004Old = `#### C-OBS-004 Window Control Discovery
| | |
| --- | --- |
| **Name** | Window Control Discovery |
| **Layer** | L4 Observation |
| **Status** | **PLANNED** |
| **Lifecycle** | Planned |
| **Dependencies** | **C-OBS-003** |
| **Verification** | Planned \`verify\` + Vitest for locate-by-name |
| **Product Proof** | Required after eng |
| **Priority** | **P0** (next observation slice) |
| **Engineering Notes** | Find/resolve a named control (not full tree dump). Distinct from C-OBS-003 list. |`;

const obs004New = `#### C-OBS-004 Window Control Discovery
| | |
| --- | --- |
| **Name** | Window Control Discovery |
| **Layer** | L4 Observation |
| **Status** | **IMPLEMENTED** (engineering) |
| **Lifecycle** | Verifying → **Product Proof required** |
| **Dependencies** | **C-OBS-003** (eng complete) |
| **Verification** | \`verify-window-control-discovery\`, \`tests/window-control-discovery.test.ts\`, kernel \`window_find_control_through_router\` |
| **Product Proof** | **Required** — \`P22_S3_WINDOW_CONTROL_DISCOVERY.md\`; Owner live session |
| **Priority** | P0 (PP pending) |
| **Engineering Notes** | Locate-by-name via \`find_control\` / \`match_control\`. Observation only. Honest \`control_not_found\`. Does not click/type (C-ACT-004/005). |
${readinessBlock({ arch: 100, deps: 100, impl: 100, ver: 100, pp: 0, trusted: 0, prod: 0 })}`;

if (!t.includes(obs003Old)) {
  console.error("atlas-readiness-sync: C-OBS-003 block not found");
  process.exit(1);
}
if (!t.includes(obs004Old)) {
  console.error("atlas-readiness-sync: C-OBS-004 block not found");
  process.exit(1);
}
t = t.replace(obs003Old, obs003New).replace(obs004Old, obs004New);

// Inject readiness matrix section before Priority Queue if missing.
if (!t.includes("## 7.5 Capability Readiness Matrix")) {
  const matrix = `## 7.5 Capability Readiness Matrix

Overall = mean of seven dimensions. Eng-complete without Owner PP defaults to **≈57%**. Trusted/Production remain Owner gates.

| ID | Name | Status | Overall |
| --- | --- | --- | --- |
| C-CON-001 | Companion Greeting | IMPLEMENTED | ~71% (PP partial) |
| C-CON-002 | Voice Input | IMPLEMENTED (eng) | ~57% (PP pending Accept) |
| C-CON-003 | Soft Send Continuity | IMPLEMENTED | ~71% |
| C-CON-004 | First-Session Cue | IMPLEMENTED | ~71% |
| C-REA-001 | Intelligence Routing | IMPLEMENTED | ~57% |
| C-REA-002 | Local Reasoning | IMPLEMENTED | ~57% |
| C-REA-003 | Provider Handoff | IMPLEMENTED | ~57% |
| C-REA-004 | In-Conversation Model | FUTURE | ~14% |
| C-ITL-001..005 | Intent stack | IMPLEMENTED | ~71% |
| C-OBS-001 | Enumerate Windows | IMPLEMENTED | ~71% |
| C-OBS-002 | Active Window | IMPLEMENTED | ~71% |
| **C-OBS-003** | **Desktop UI Tree** | **IMPLEMENTED (eng)** | **57%** |
| **C-OBS-004** | **Window Control Discovery** | **IMPLEMENTED (eng)** | **57%** |
| C-OBS-005 | Screenshot | IMPLEMENTED | ~71% |
| C-OBS-006 | Monitors | IMPLEMENTED | ~71% |
| C-OBS-007 | OCR Perception | FUTURE | ~14% |
| C-ACT-001..003,006..010 | Core actions | IMPLEMENTED | ~71% |
| C-ACT-004 | Mouse Click | PLANNED | ~29% (deps partial) |
| C-ACT-005 | Keyboard Input | PLANNED | ~29% |
| C-ACT-011 | File Provider | BLOCKED | ~14% |
| C-ACT-012 | Terminal | FUTURE | ~14% |
| C-ACT-013 | Agent Loop | REJECTED | 0% |
| C-VER-001 / C-CMP-001 | Completion | IMPLEMENTED | ~71% |
| C-VER-002 / C-VER-003 | Retry / Wait | PLANNED | ~29% |
| C-CMP-002..004 | Composition | IMPLEMENTED | ~71% |
| C-PROC-* / C-WF-* | Procedures / Workflows | Mixed | see records |
| C-INT-001..003 | Intelligence | IMPLEMENTED | ~57–71% |
| C-INT-004..005 | Memory / Agent | REJECTED | 0% |
| C-REL-001 | F1 Pipeline | IMPLEMENTED | ~71% |
| C-REL-002..003 | Signing / Signed | BLOCKED | ~14% |

---

`;
  t = t.replace("## 8. Priority Queue", `${matrix}## 8. Priority Queue`);
}

// Update priority queue / highest remaining / stop.
t = t.replace(
  `| **1** | **C-OBS-004** | Window Control Discovery | **PLANNED** | Next executable after C-OBS-003 eng |
| **2** | **C-ACT-004 + C-ACT-005** | Mouse Click + Keyboard Input | PLANNED | Depends on discovery/tree |
| 3 | C-REL-002 | Code Signing A2 | BLOCKED (cert) | Parallel release track |
| 4 | C-ACT-011 | File Provider | BLOCKED (Voice) | Production Before Expansion |
| 5 | C-OBS-007 | Tokenized perception | FUTURE | After interaction |
| 6 | C-REA-004 | In-Conversation Model | FUTURE | Research |
| 7 | C-WF-001 | Daily Coding Session | FUTURE | After procedures |

**Rejected (do not queue):** C-ACT-013; C-INT-004; C-INT-005.

**Just completed eng:** C-OBS-003 Desktop UI Tree → Product Proof (do not start next until Atlas loop selects).`,
  `| **1** | **C-ACT-004 + C-ACT-005** | Mouse Click + Keyboard Input | **PLANNED** | Deps: C-OBS-003/004 eng complete |
| 2 | C-REL-002 | Code Signing A2 | BLOCKED (cert) | Parallel release track |
| 3 | C-ACT-011 | File Provider | BLOCKED (Voice) | Production Before Expansion |
| 4 | C-VER-003 | Wait Conditions | PLANNED | With click/type |
| 5 | C-OBS-007 | Tokenized perception | FUTURE | After interaction |
| 6 | C-REA-004 | In-Conversation Model | FUTURE | Research |
| 7 | C-WF-001 | Daily Coding Session | FUTURE | After procedures |

**Rejected (do not queue):** C-ACT-013; C-INT-004; C-INT-005.

**Just completed eng:** C-OBS-004 Window Control Discovery → Product Proof (do not start next until Atlas loop selects).`,
);

t = t.replace(
  `### **C-OBS-004 Window Control Discovery**

| Factor | Assessment |
| --- | --- |
| Owner Value | Very High |
| Dependencies | C-OBS-003 (eng done; PP parallel) |
| Effort | S–M |
| Risk | Med (UIA naming) |
| Trust Improvement | Very High (enables click/type) |

**Do not begin in the same session that completed C-OBS-003.**

Parallel: C-OBS-003 Owner Product Proof; Authenticode → A2.`,
  `### **C-ACT-004 Mouse Click** (paired with **C-ACT-005 Keyboard Input**)

| Factor | Assessment |
| --- | --- |
| Owner Value | Very High |
| Dependencies | C-OBS-003 + C-OBS-004 eng complete (PP parallel) |
| Effort | M |
| Risk | Med (UIA invoke flakiness → truthful fail) |
| Trust Improvement | Very High |

**Do not begin in the same session that completed C-OBS-004.**

Parallel: C-OBS-003 / C-OBS-004 Owner Product Proof; Authenticode → A2.`,
);

t = t.replace(
  `- [x] Capability row status updated (C-OBS-003)  
- [x] Evidence / Verification paths  
- [x] Blockers register (B-API-001 cleared; B-PP-003 added)  
- [x] Priority queue re-ranked  
- [x] Product Proof matrix updated  
- [x] Engineering Notes  
- [ ] Owner Product Proof for C-OBS-003  `,
  `- [x] Capability row status updated (C-OBS-004)  
- [x] Readiness Scores schema + matrix  
- [x] Evidence / Verification paths  
- [x] Blockers register (B-PP-004 added)  
- [x] Priority queue re-ranked  
- [x] Product Proof matrix updated  
- [x] Engineering Notes  
- [ ] Owner Product Proof for C-OBS-003 / C-OBS-004  `,
);

t = t.replace(
  `| UIA controls | No | N/A | Planned |
| Signed release | No | N/A | Cert blocked |`,
  `| C-OBS-003 Desktop UI Tree | **Yes** | **Required — pending Owner** |
| C-OBS-004 Window Control Discovery | **Yes** | **Required — pending Owner** |
| C-ACT-004/005 Click/Type | No | N/A |
| Signed release | No | N/A | Cert blocked |`,
);

// Fix duplicate matrix rows if Product Proof matrix already had C-OBS-003
t = t.replace(
  `| C-OBS-003 Desktop UI Tree | **Yes** | **Required — pending Owner** |
| Core desktop providers | Yes | Mixed Owner trust |
| Voice | Yes | Pending Accept |
| Intelligence routing | Yes | Owner re-proof |
| Beside / Compound | Yes | Owner confirm |
| C-ACT-004/005 Click/Type | No | N/A |
| File Provider | No | Blocked |
| C-OBS-003 Desktop UI Tree | **Yes** | **Required — pending Owner** |
| C-OBS-004 Window Control Discovery | **Yes** | **Required — pending Owner** |
| C-ACT-004/005 Click/Type | No | N/A |
| Signed release | No | N/A | Cert blocked |`,
  `| C-OBS-003 Desktop UI Tree | **Yes** | **Required — pending Owner** |
| C-OBS-004 Window Control Discovery | **Yes** | **Required — pending Owner** |
| Core desktop providers | Yes | Mixed Owner trust |
| Voice | Yes | Pending Accept |
| Intelligence routing | Yes | Owner re-proof |
| Beside / Compound | Yes | Owner confirm |
| C-ACT-004/005 Click/Type | No | N/A |
| File Provider | No | Blocked |
| Signed release | No | Cert blocked |`,
);

if (!t.includes("**B-PP-004**")) {
  t = t.replace(
    `| **B-PP-003** | C-OBS-003 Desktop UI Tree Product Proof pending | High | C-OBS-003 Trusted | Owner live session per P22.S2 |`,
    `| **B-PP-003** | C-OBS-003 Desktop UI Tree Product Proof pending | High | C-OBS-003 Trusted | Owner live session per P22.S2 |
| **B-PP-004** | C-OBS-004 Window Control Discovery Product Proof pending | High | C-OBS-004 Trusted | Owner live session per P22.S3 |`,
  );
}

t = t.replace(
  `PLANNED spine:
  C-OBS-004 Control Discovery ──► C-ACT-004 Click + C-ACT-005 Keyboard
                                       └─► C-VER-002/003 Retry / Wait`,
  `PLANNED spine:
  C-OBS-003/004 (eng done → PP) ──► C-ACT-004 Click + C-ACT-005 Keyboard
                                       └─► C-VER-002/003 Retry / Wait`,
);

t = t.replace(
  `**Workspace Capability Atlas v2.0** is the authoritative capability roadmap.  
C-OBS-003 engineering slice complete → **Product Proof required**.  
Do not begin C-OBS-004 in this iteration.`,
  `**Workspace Capability Atlas v2.0** is the authoritative capability roadmap.  
C-OBS-004 engineering slice complete → **Product Proof required**.  
Do not begin C-ACT-004 / C-ACT-005 in this iteration.`,
);

t = t.replace(
  `- Starting C-OBS-004 or C-ACT-004 in the same slice as C-OBS-003  `,
  `- Starting C-ACT-004 / C-ACT-005 in the same slice as C-OBS-004  `,
);

fs.writeFileSync(atlasPath, t);
console.log("atlas-readiness-sync: ok");
