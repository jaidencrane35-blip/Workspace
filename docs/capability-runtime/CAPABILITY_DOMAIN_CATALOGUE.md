# Capability Domain Catalogue

Permanent Track B domains. Each maps to Layer 3 (Capability Runtime). Intelligence (Layer 4) may *route* to these domains but does not own their effects.

| Domain ID | Name | Primary effect | Trust class |
| --- | --- | --- | --- |
| `app_control` | Application Control | Launch / focus / quit processes | High |
| `window_mgmt` | Window Management | Move / resize / z-order / virtual desktop | High |
| `clipboard` | Clipboard | Read / write clipboard | Medium–High |
| `file_ops` | File Operations | Read / write / move / trash (scoped) | High |
| `desktop_obs` | Desktop Observation | Window/process snapshots (consented) | High (Law XII) |
| `screenshots` | Screenshots | Capture display / window bitmap | High |
| `ocr` | OCR | Text from image / region | Medium |
| `voice_input` | Voice Input | Speech → utterance for Conversation | Medium |
| `automation` | Automation | Synthetic input / sequenced actions | Critical |
| `memory` | Memory | Remember / recall / forget (explicit) | Medium |
| `search` | Search | Local Moments / files / history | Medium |
| `notifications` | Notifications | OS toasts / attention | Low–Medium |
| `terminal` | Terminal | ConPTY / command execution (governed) | Critical |
| `browser` | Browser | Open URL / optional CDP assist | Medium–High |
| `workflow` | Workflow | Multi-step plans across domains | High |

## Non-domains (explicit)

| Tempting surface | Why not a domain |
| --- | --- |
| Chat / agent IDE | Product identity — Layer 2 only |
| Capability launcher menu | Forbidden by Product Constitution |
| Ambient always-on watcher | Law XII — observation is consented, not default |

## Extension rule

New domains require: catalogue entry · contract · adoption matrix row · Owner approval before V1 code.
