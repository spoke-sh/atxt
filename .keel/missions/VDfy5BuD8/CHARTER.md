# Timed Visual Sequence Verification Slice - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver the first end-to-end timed visual sequence slice for atxt so a GIF or short video can be probed, decoded into a normalized sequence representation, transformed into a verification-friendly poster frame or contact sheet, rendered through the existing text renderer path, and inspected from the CLI with deterministic fallback behavior. | board: VDfyDRgBo |

## Constraints

- Keep this mission focused on timed visual summaries for GIF and short video inputs; audio, inline image protocols, and terminal animation remain out of scope.
- Reuse the shared probe, terminal detection, render-planning, and still-image render surfaces so timed media lands on the existing canonical pipeline instead of a parallel terminal backend.
- The operator-facing proof for this mission must be a bounded, deterministic `just mission` signal over a canonical timed-sequence fixture showing both direct and degraded terminal paths.

## Halting Rules

- DO NOT halt while epic VDfyDRgBo still has unfinished voyage or story work.
- HALT when epic VDfyDRgBo is done and the primary `just mission` proof path shows a truthful timed-sequence summary through both direct and degraded terminal modes.
- YIELD to human after mission achievement for final `keel mission verify` review, or sooner if the only remaining question is whether the chosen fixture and summary output are useful enough.
