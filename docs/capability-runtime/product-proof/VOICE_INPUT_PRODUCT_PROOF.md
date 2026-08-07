# Voice Input — Product Proof
## P16 / P16.11 — Owner checklist

| Field | Value |
| --- | --- |
| **Program** | P16 Voice Input (+ P16.11 technology validation) |
| **Status** | **Pending live Product Owner acceptance** (**not** Product Complete) |
| **Role** | Conversation input device |

---

## Please test (once Workspace is open — leave it running)

### Voice
1. Click mic → wait for **Ready** (green) before speaking  
2. Speak immediately after Ready — first words should be kept  
3. Speak a long natural turn with short pauses; click mic to finish  
4. Confirm Workspace does **not** freeze or crash  
5. Confirm Settings does **not** open unless you click mic after a permission message  

### Permission
1. If mic is denied: Conversation explains why; click mic again → Settings opens once  
2. Allow Workspace in Windows Settings; return without quitting Workspace  
3. Expect automatic re-check and **✓ Voice ready** (no second Settings spam)  
4. Quit and relaunch later: Settings must **not** reopen by itself once granted  

### Natural language
- Open GPT  
- Open YT  
- Open Git  
- Open VSCode  
- Open Cursor  
- Open Edge  
- Open Chrome  
- Open Settings  
- Open ChatGPT beside Cursor  
- Open YouTube beside GPT  
- Open my browser  
- Close Settings  
- Bring Cursor forward  
- Maximize Cursor  
- Take a screenshot  
- Capture this window  
- Can you hear me?  
- What can you do with voice?  

### Browser
- Site abbreviations open the site (not a wrong executable)  
- Chrome / Edge phrasing brings the browser forward or opens as expected  

### Expected
- Ordinary desktop language works (no exact capitalization)  
- Unsupported asks get an honest nearby suggestion  
- Shell glass is readable; mic Ready / Listening / speaking states are obvious  
- No Provider / WinRT / HRESULT language  

When finished, close Workspace normally and tell engineering testing is complete.

---

## Closure stamp (Owner only)

~~P16 Voice Input — PERMANENTLY CLOSED — ACCEPTED — REPOSITORY TRUTH — DO NOT REOPEN~~ (not yet)

Until acceptance: `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING`  
