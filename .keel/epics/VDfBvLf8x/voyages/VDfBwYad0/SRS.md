# Probe and Capability Foundations - SRS

## Summary

Epic: VDfBvLf8x
Goal: Define the first stable media-probe model and terminal capability profile so render planning can auto-detect inputs, choose safe defaults, and degrade explicitly under tmux, ssh, and low-capability terminals.

## Scope

### In Scope

- [SCOPE-01] Define the canonical `ProbeResult` and `TerminalProfile` data contracts used by renderer planning.
- [SCOPE-03] Add initial probe and capability detection entry points for the first supported media and terminal families.
- [SCOPE-04] Define conservative fallback rules for ambiguous or hostile terminal environments.

### Out of Scope

- [SCOPE-02] Full decoder support for every container or codec family.
- [SCOPE-05] Final production renderer implementations beyond the planning and detection slice.
- [SCOPE-06] Specialized terminal escape protocols or inline image transports.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The crate must define a canonical `ProbeResult` surface that describes media family, geometry, timing, audio metadata, and unknown-partial states without renderer-specific leakage. | SCOPE-01 | FR-01 | cargo test |
| SRS-02 | The crate must define a canonical `TerminalProfile` surface that captures color support, Unicode reliability, animation viability, multiplexers, remoteness, and terminal size for planning decisions. | SCOPE-01 | FR-02 | cargo test |
| SRS-03 | The crate must provide detection and planning entry points that combine probe and capability data into explicit render defaults and conservative fallback decisions. | SCOPE-03 SCOPE-04 | FR-03 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Capability detection and render planning must degrade conservatively when environment signals are missing, contradictory, or incomplete. | SCOPE-04 | NFR-01 | cargo test |
| SRS-NFR-02 | Probe and capability detection behavior must remain deterministic and testable without requiring a live interactive terminal session. | SCOPE-01 SCOPE-03 | NFR-02 | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
