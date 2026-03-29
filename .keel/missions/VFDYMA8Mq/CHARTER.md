# Inspection-First CLI Surface Alignment - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver an inspection-first CLI slice so operators can see what `atext` knows about an input, why it chose a renderer, and which user-facing commands are actually supported without reading the source. | board: VFDYN9LOV |

## Constraints

- Keep this mission focused on CLI truthfulness and inspection flow, not on adding a brand-new media modality.
- Reuse the existing probe, terminal detection, render planning, and stats surfaces instead of introducing a parallel operator model just for the CLI.
- The operator-facing proof for this mission must be deterministic and reviewable in captured terminal environments, not only in interactive local shells.

## Halting Rules

- DO NOT halt while epic VFDYN9LOV still has unfinished voyage or story work.
- HALT when epic VFDYN9LOV is done and the primary CLI surface truthfully exposes both inspection metadata and the supported command contract.
- YIELD to human after the planning slice is ready for decomposition or sooner if the only remaining question is which inspection format is most useful for operators.
