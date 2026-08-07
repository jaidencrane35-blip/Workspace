# F9 — Fast Speech Quality Investigation (P16.30)

Owner finding (repository truth): **Fast speech quality degrades.**

## Scope

Investigate WinRT segmentation, buffering, phrase timing, continuous session behaviour, recognizer configuration, and recognition lifecycle. Determine fixable vs configurable vs inherent. Do **not** migrate WRAP without objective evidence of a better alternative.

## Current WRAP (frozen unless disproven)

- `windows::Media::SpeechRecognition::SpeechRecognizer`
- Scenario: **Dictation** topic constraint
- Session: `ContinuousRecognitionSession` with `SpeechContinuousRecognitionMode::Default`
- Finish: session AutoStop / ResultGenerated stitching (P16.8 Conversation Continuity)
- Timeouts set today (`create_compiled_recognizer`):
  - `InitialSilenceTimeout` = 15s (post-Ready settle aid)
  - `BabbleTimeout` = 20s
  - `EndSilenceTimeout` **intentionally unset** — RecognizeAsync-era residue; ContinuousRecognitionSession owns end-of-utterance

## What Workspace controls

| Lever | Effect on fast speech | Change in P16.30 |
| --- | --- | --- |
| Continuous vs RecognizeAsync | Continuous already stitches mid-phrase (R2) | None — keep |
| InitialSilence / Babble | Affect silence / babble abort, not syllable rate | None — no evidence shortening helps fast speech |
| EndSilence | Not authoritative for ContinuousRecognitionSession | None — do not reintroduce |
| UI Ready/Listening | First-word capture (R1/R11/R35) | Unrelated to fast-speech word error rate |
| Intent Grammar | Downstream of transcript | Cannot fix recognition WER |

## Inherent WinRT behaviour (objective framing)

Microsoft’s dictation recognizer performs **on-device / cloud-assisted** segmentation. Under rapid speech:

1. Hypothesis updates can lag syllable rate.
2. AutoStop / result boundaries may split or drop function words.
3. Workspace receives **final / intermediate ResultGenerated text** — it cannot re-decode the audio buffer inside the WRAP.

There is **no public WinRT API** in this stack to raise “fast speech aggressiveness” independent of EndSilence-style RecognizeAsync knobs that we already rejected for cutting mid-phrase.

## Migration cost (if a future WRAP wins)

| Candidate | Evidence required | Rough cost |
| --- | --- | --- |
| Azure Speech SDK continuous | Side-by-side WER on same Owner fast-speech script | New crate dependency, privacy/network policy, permission UX rewrite |
| Windows.Media.SpeechRecognition alternatives | Same scenario already in use | N/A |
| Web Speech API in WebView | Browser sandbox; not desktop Operator grade | Reject for production Operator |

**Decision:** Do **not** migrate. WRAP remains WinRT ContinuousRecognitionSession until a measured superior WRAP exists on the Owner’s fast-speech script.

## Residual risk (shipping)

Rapid speech may still produce imperfect transcripts. Mitigation is **product**, not WRAP rewrite:

- F10 review → Send (Owner sees text before execution)
- Intent Grammar tolerates ordinary desktop phrasing once text is correct enough
- Truthful Conversation when Intent cannot parse

## Conclusion

F9 is **primarily inherent WinRT dictation behaviour** under rapid speech, not a Workspace Intent routing bug. Configurable timeouts were reviewed; no safe change improves fast speech without reintroducing mid-speech cut-off (R2). Migration not justified without evidence.
