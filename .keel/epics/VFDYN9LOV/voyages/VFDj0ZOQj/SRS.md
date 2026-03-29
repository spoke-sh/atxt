# CLI Contract Wiring And Inspection Output - SRS

## Summary

Epic: VFDYN9LOV
Goal: Deliver a truthful inspection-first CLI slice by aligning the public command contract, exposing shared probe and render-plan metadata, and publishing deterministic direct/degraded proofs for operator workflows.

## Scope

### In Scope

- [SCOPE-01] Align the top-level CLI usage string and dispatch table with the commands the binary actually supports today.
- [SCOPE-02] Expose the existing stats renderer through the CLI or remove it from the public contract in the same slice.
- [SCOPE-03] Add an `inspect <path>` command that reports media classification, probe completeness, metadata, terminal profile, render mode, output kind, degraded state, and planning reason.
- [SCOPE-04] Keep inspection output tied to the shared `ProbeResult`, `TerminalProfile`, and `RenderPlan` contracts rather than renderer-local reimplementation.
- [SCOPE-05] Add deterministic tests, direct/degraded proof captures, and doc updates for the inspection-oriented CLI surface.

### Out of Scope

- [SCOPE-06] New media decoders, new renderer families, or deeper modality support beyond existing image, timed-visual, audio, video, and document classification paths.
- [SCOPE-07] A new CLI framework, nested subcommand tree, or interactive TUI inspector.
- [SCOPE-08] A long-term structured machine API beyond what is necessary to keep the first inspection slice truthful.
- [SCOPE-09] Release automation, packaging work, or unrelated navigation/globe features.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The CLI must advertise only the commands it actually dispatches, and the top-level usage text must stay aligned with the implemented command matcher. | SCOPE-01 | FR-01 | cargo test |
| SRS-02 | The CLI must either expose the existing stats surface as a working top-level command or remove it from the public contract and docs in the same change slice. | SCOPE-01 SCOPE-02 | FR-04 | cargo test / manual |
| SRS-03 | `atext inspect <path>` must report media kind, probe completeness, and relevant probe metadata for representative supported inputs without requiring source inspection. | SCOPE-03 | FR-02 | cargo test |
| SRS-04 | `atext inspect <path>` must include terminal-profile and render-plan context, including session mode, Unicode reliability, chosen render mode, degraded state, output kind, and planning reason. | SCOPE-03 SCOPE-04 | FR-03 | cargo test |
| SRS-05 | The repository must include reviewable proofs and docs for the inspection-oriented CLI surface, covering direct and degraded terminal paths for the new command contract. | SCOPE-05 | FR-05 | cargo test / manual / llm-judge |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Inspection output must remain deterministic and readable without requiring a live interactive terminal session. | SCOPE-03 SCOPE-05 | NFR-01 | cargo test / manual |
| SRS-NFR-02 | The inspection command must derive its explanation data from the shared `ProbeResult`, `TerminalProfile`, and `RenderPlan` contracts instead of duplicating renderer-local logic in the CLI. | SCOPE-04 | NFR-02 | cargo test |
| SRS-NFR-03 | The first inspection slice must stay narrow enough to land within the current CLI shape, avoiding a broad parser or UX redesign while still resolving the contract drift. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-05 | NFR-03 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
