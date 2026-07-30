# Workspace Desktop Interaction Model

| Field | Value |
|-------|-------|
| **Purpose** | Authoritative product contract for what the user experiences when using Workspace |
| **Owner** | Product |
| **Status** | Binding product definition (2026-07-30) |
| **Audience** | Every engineer, designer, and reviewer working on Workspace — including teams years from now |
| **References** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Related** | [WORKSPACE-REFERENCE-INTERPRETATION.md](WORKSPACE-REFERENCE-INTERPRETATION.md), [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md), [PRODUCT-VISION.md](PRODUCT-VISION.md) |

**This document defines interaction goals and product meaning.**  
**It does not authorise implementation, UI redesign, or architecture change by itself.**  
**Where UI or copy conflicts with this contract, the contract wins.**

---

## 1. Product mission

Workspace is a **desktop environment representation and control layer** for Windows. It shows the user’s existing computing environment — their running applications and working context — and lets them organise and control that environment with clarity. It is not an AI application. It is not a dashboard of settings and panels. It is not a setup wizard that asks people to build a workspace before they can work. The product exists so that opening Workspace feels like looking at **your desktop, made coherent and controllable**, with help available only if you want it.

---

## 2. The user's mental model

When a person opens Workspace, they should believe they are looking at:

- **Their desktop** — the same world of windows they already use on the PC  
- **Their running applications** — real programs in use, not a catalogue they must fill in first  
- **Their working environment** — the shape of how they are working right now  
- **Their current context** — what is open, what is focused, what belongs together  

They should **not** believe they are looking at:

- A project-management tool  
- An application-registration form  
- An AI workflow console  
- An admin console for inventing “workspaces” from empty states  

**Test:** If a user needs a tutorial to understand *what this screen is*, the product has failed the mental model. The screen should read as “my computer’s work surface,” not “an app I have to configure.”

---

## 3. Primary hierarchy

This hierarchy is permanent. Future features may add depth inside a layer; they may not reorder the layers.

```text
Desktop reality
        ↓
Applications
        ↓
Spatial organisation
        ↓
Workspace controls
        ↓
Assistant companion
```

| Layer | What the user experiences | Why it sits here |
|-------|---------------------------|------------------|
| **Desktop reality** | “This is my environment as it is.” Observed presence of the live desktop. | Without reality, Workspace is fiction. Everything else is meaningless. |
| **Applications** | “These are the programs I’m working with.” The subjects of the product. | People work in apps, not in panels. Apps are the content of the desktop. |
| **Spatial organisation** | “I can see how things are arranged and change that arrangement.” Layout in space. | Desktops are spatial. Lists alone do not communicate a working environment. |
| **Workspace controls** | “I can remember, restore, switch density, and steer the environment.” Tools that act *on* the desktop. | Controls serve work; they are not the work. |
| **Assistant companion** | “I can ask for help if I want.” Optional support beside the stage. | Help must never become the reason the product exists. |

**Why this hierarchy exists:**  
People already have a desktop. Workspace’s job is to represent and improve control of that desktop. Applications are what they care about. Space is how desktops communicate. Controls are how Workspace earns trust. The Assistant is a passenger, not the driver.

---

## 4. First five seconds

No implementation detail — only what the interaction must achieve.

1. **Instant recognition** — The person can tell within a glance that they are looking at *their* computing environment (or an honest statement that the live environment cannot be shown yet).  
2. **No setup gate** — They are not asked to create a workspace, register apps, or complete configuration before anything useful appears.  
3. **Applications visible as the subject** — Running or observed applications occupy the centre of attention, not forms or marketing copy.  
4. **Space before paragraphs** — Layout communicates “this is a desktop surface” before they read instructions.  
5. **Help stays optional** — An Assistant may be present at the side; nothing requires talking to it to begin.  
6. **A clear next physical action** — If something is runnable or restorable, it is obvious; if the environment is empty or unavailable, the emptiness is honest and still about the desktop — not about inventing a project.

**Failure of the first five seconds:** landing on create/register/configure, or landing inside chat.

---

## 5. Persistent surfaces

These should feel always available during normal use (exact chrome may vary; the *roles* must not disappear).

| Surface | Always present because… |
|---------|-------------------------|
| **Running / observed applications** | They are the product’s subject. Hiding them turns Workspace into something else. |
| **Workspace surface (the stage)** | There must be one primary place that *is* the environment representation — not a stack of equal admin tabs competing for identity. |
| **Navigation to core areas** | People need a calm way to reach Stage, applications library (if any), saved environments/profiles, and tools — without those areas becoming the home. |
| **Companion rail (Assistant)** | Help should be findable without becoming the home screen; persistence as a **side** surface reinforces “optional.” |
| **Mode control (Flow / Focus)** | Density of the same environment is a core behaviour; the control should remain reachable while working. |

**Why persistence matters:**  
A desktop product fails when every useful thing is buried behind setup. Persistence of the work surface teaches the mental model every time the app opens.

**What must not be “always primary”:**  
Create-workspace wizards, registration forms, diagnostics, developer tools, and long explanatory dashboards.

---

## 6. Workspace modes

From the user’s point of view only.

### Flow

Flow is the **open, productive density** of the same working set.  
Many applications remain visible and easy to reach. The environment feels busy in a useful way — like a desk with several things in view. The user is multitasking without losing track of what belongs to this work moment.

### Focus

Focus is the **quieter, immersive density** of the **same** working set.  
One application (or a small primary set) takes visual priority. Other applications stay available; they are not closed or discarded. The environment feels calmer, not empty of meaning.

### Shared rules for both modes

- The user chooses the mode.  
- Applications stay open across the switch.  
- The Assistant’s place in the product does not change: still companion, still secondary.  
- Modes change **how the environment is presented and organised**, not which product Workspace becomes.

---

## 7. Application philosophy

**Applications are the product.**  
Workspace exists so applications can be seen, organised, remembered, and controlled as a coherent environment.

**Workspace organises them.**  
It represents their presence, supports spatial arrangement, and offers controls to restore and reshape the environment. Organisation is in service of the apps — not in service of filling databases.

**The Assistant supports them.**  
It may help the person understand or navigate the environment. It does not become a substitute for seeing and touching the applications themselves.

**Practical consequence:**  
Any screen that treats application registration, empty project creation, or chat as more important than the living set of applications is violating this philosophy.

---

## 8. Assistant philosophy

### The Assistant may

- **Answer** questions about the workspace and how to use it  
- **Explain** what is on screen, what a control does, or what just happened  
- **Navigate** the person toward the right surface or action (without hiding the stage)  
- **Suggest** optional next steps the person can accept or ignore  

### The Assistant must never

- **Replace the workspace** as the thing the person came to use  
- **Become the primary UI** (full-screen chat as the product identity)  
- **Invent reality** (fake windows, fake apps, or pretend observation)  
- **Require interaction before work begins** (no “talk to me first” onboarding)  
- **Silently take control** of the desktop without clear user intent and permission  

**Placement rule:**  
If removing the Assistant would make the product feel empty, the product is wrong. Removing the Assistant should leave a complete desktop representation and control experience.

---

## 9. Product feel

Workspace should feel:

- **Calm** — not noisy with badges, alerts, and competing panels  
- **Visual** — understood by looking, not by reading a manual  
- **Spatial** — windows and apps occupy space; relationships are visible  
- **Minimal reading** — short labels; layout carries meaning  
- **High information density** — many real apps/windows can be present without turning into a spreadsheet of settings  
- **Direct manipulation** — people act on what they see (select, arrange, restore), rather than filling forms to describe work  
- **Desktop-first** — the PC environment is the centre of gravity  

**Communication rule:**  
The UI should communicate through **layout before text**. If a screen needs a long paragraph to explain what it is, redesign the layout; do not add more copy.

---

## 10. Reference image interpretation

The approved concept images are **authoritative for hierarchy and interaction intent**. They are **not** pixel specifications. Do not copy brands, wallpaper, or exact widgets.

### What to extract from the images

| Quality | What the images show | Why it matters |
|---------|----------------------|----------------|
| **Hierarchy** | Apps fill the stage; Assistant is a right rail; controls surround work | Teaches what is primary forever |
| **Interaction** | User-driven mode change between dense and calm presentations of the same apps | Modes are environmental, not separate products |
| **Priorities** | Real applications dominate; utilities (audio, etc.) support; AI does not own the frame | Prevents AI-first and settings-first drift |
| **Information density** | Flow carries many apps at once; Focus reduces noise without erasing the set | Density is a feature of work, not clutter to eliminate with empty states |
| **Visual balance** | Large stage, thin companion, chrome that frames rather than competes | Balance keeps attention on work |
| **User focus** | Eyes land on applications and space, not on a wizard or chat transcript | Focus defines whether the product feels like a desktop or like software admin |

### What not to extract

- Exact colours, type, scenic backgrounds  
- Specific third-party app brands as requirements  
- Nine layout variants as nine products to ship  
- Decorative complexity that does not serve hierarchy  

---

## 11. Drift examples

These interpretations are incorrect. Reject them in review.

| Drift | What it looks like | Why it violates the product |
|-------|--------------------|-----------------------------|
| **Dashboard** | Equal cards for stats, shortcuts, and “get started” modules | A dashboard is about the tool; Workspace is about the desktop. |
| **Setup wizard** | Create workspace → register apps → then maybe see value | Forces invention of an environment the user already has. |
| **Application manager** | Registry and forms as the hero; observation secondary | Manages catalogue entries instead of representing living work. |
| **AI-first interface** | Chat, briefings, or programmes as the first screen | Makes the Assistant the product; apps become accessories. |
| **Configuration-first workflow** | Settings, profiles, and permissions before any desktop view | Configuration supports work; it must not precede work. |
| **Empty admin stage** | Large void with “create / go to Workspaces” as the only story | Communicates that Workspace is unfinished paperwork, not a desktop layer. |

**Drift test:**  
If a stranger removes the brand name and the Assistant, does the screen still clearly mean “my desktop”? If not, it has drifted.

---

## 12. Future implementation rules

Every future UI milestone must improve **one or more** of:

1. **Desktop representation** — clearer, more honest, more immediate sense of the live environment  
2. **Application visibility** — easier to see what is running and what matters  
3. **Spatial organisation** — stronger layout-of-work, not only lists  
4. **Direct manipulation** — more acting on what is seen  
5. **Workspace clarity** — calmer understanding of modes, restore, and controls *in service of* the desktop  

### Hard rules

- **No milestone may increase the prominence of configuration over work.**  
- **No milestone may make the Assistant more primary than the stage.**  
- **No milestone may invent desktop reality** (fake windows or fake apps as the hero).  
- **No milestone may require setup before first value.**  
- **Reuse** observation, window control, arrangements, and permissions systems; do not invent parallel products.  
- **Concept art** guides hierarchy and feel; it does not demand pixel clones.  

### Review gate for every UI change

Before merge or human approval, answer in writing:

1. Which of the five improvement goals does this change advance?  
2. Did configuration or Assistant prominence increase? If yes, reject or redesign.  
3. Does the first five seconds still match §4?

---

## Contract summary

> Workspace shows and controls the user’s real desktop. Applications are the subject. Space carries meaning. Controls serve work. The Assistant helps from the side. Nothing may turn this product into a dashboard, a setup wizard, or an AI-first application.

---

## Stop

Product definition only.  
Do not treat this file as a coding authorisation.  
Await explicit milestone approval before implementation work.
