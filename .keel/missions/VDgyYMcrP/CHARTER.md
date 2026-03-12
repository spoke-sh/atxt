# Audio Waveform And Spectrogram Verification Slice - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver the first end-to-end audio verification slice for atxt so a filesystem audio clip can be probed, decoded into a shared audio summary, rendered as a waveform or spectrogram through shared planning, and reviewed from the CLI with deterministic degraded output. | board: VDgyaiPm5 |

## Constraints

- Keep this mission focused on bounded audio summaries for filesystem audio, with WAV support as the required first decode path; playback, streaming, and richer semantic audio analysis remain out of scope.
- Reuse the shared probe, terminal detection, and render-planning surfaces so audio lands on the canonical media-to-text pipeline instead of audio-local terminal heuristics.
- The operator-facing proof for this mission must be a bounded, deterministic `just mission` signal over a canonical audio fixture showing both direct and degraded terminal paths.

## Halting Rules

- DO NOT halt while epic VDgyaiPm5 still has unfinished voyage or story work.
- HALT when epic VDgyaiPm5 is done and the primary `just mission` proof path shows a truthful audio summary through both direct and degraded terminal modes.
- YIELD to human after mission achievement for final `keel mission verify` review, or sooner if the only remaining question is whether the chosen fixture and summary density are useful enough.
