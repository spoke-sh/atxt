# Still Image Decode and Txtplot Rendering - SRS

## Summary

Epic: VDfKtP4Yp
Goal: Define the canonical still-image frame model, add a static-image decode path, and deliver a txtplot-backed braille plus ASCII fallback renderer with a thin CLI and deterministic proofs.

## Scope

### In Scope

- [SCOPE-01] Define the shared still-image frame contract and decode-facing model needed by renderers.
- [SCOPE-02] Decode the first supported static raster image families from a filesystem path into the shared frame contract.
- [SCOPE-03] Render still-image frames through a txtplot-backed braille path and an internal ASCII fallback selected by shared planning.
- [SCOPE-04] Expose a thin CLI render command for still images using the shared pipeline and terminal defaults.
- [SCOPE-05] Add deterministic fixtures and reviewable proofs for direct and degraded output paths.

### Out of Scope

- [SCOPE-06] GIF, video, timed rendering, or contact-sheet work.
- [SCOPE-07] Audio, document rendering, or non-image decode work.
- [SCOPE-08] Inline image protocols, non-txtplot visual backends, or advanced color-fidelity/dithering work beyond a truthful first slice.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The crate must define a canonical `VisualFrame`-style still-image surface that captures pixel geometry and renderer-agnostic raster data without leaking txtplot-specific types into the public media model. | SCOPE-01 | FR-01 | cargo test |
| SRS-02 | The crate must decode the first supported static raster image families from a filesystem path into the shared still-image frame contract. | SCOPE-02 | FR-02 | cargo test |
| SRS-03 | The crate must render a shared still-image frame through a txtplot-backed braille path when `RenderPlan` selects a direct still-image render and the terminal profile allows Unicode output. | SCOPE-03 | FR-03 | cargo test |
| SRS-04 | The crate must provide an explicit ASCII fallback renderer for still-image frames when shared planning degrades away from braille output. | SCOPE-03 | FR-04 | cargo test |
| SRS-05 | The crate must expose a thin CLI command that renders a still image from a filesystem path using the shared probe, terminal detection, planning, decode, and render pipeline. | SCOPE-04 | FR-05 | cargo test / vhs |
| SRS-06 | The repo must include representative fixtures and reviewable proofs for both direct braille output and degraded ASCII fallback output. | SCOPE-05 | FR-06 | cargo test / vhs / llm-judge |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Still-image renderer choice must continue to derive from the shared `RenderPlan` and `TerminalProfile` surfaces instead of backend-local terminal heuristics. | SCOPE-03 SCOPE-04 | NFR-01 | cargo test |
| SRS-NFR-02 | The still-image slice must remain deterministic and testable without requiring a live interactive terminal session. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-04 SCOPE-05 | NFR-02 | cargo test / vhs |
| SRS-NFR-03 | txtplot integration must remain behind a narrow adapter boundary so future renderer changes do not redefine the canonical still-image frame contract. | SCOPE-01 SCOPE-03 | NFR-03 | cargo test |
| SRS-NFR-04 | The default CLI and library path must remain auto-configured for ordinary terminals without mandatory renderer-specific flags. | SCOPE-03 SCOPE-04 | NFR-04 | cargo test / vhs |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
