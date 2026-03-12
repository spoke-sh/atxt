# Automatic Probing and Terminal Capability Detection - Product Requirements

## Problem Statement

atext needs a single reliable way to identify media inputs and terminal capabilities so it can choose truthful text renderers and fallbacks without forcing users to hand-configure every environment.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Automatically classify the input asset into a canonical probe model that renderer planning can trust. | Probe coverage for the initial media families is defined and testable. | Static image, timed visual sequence, audio, document, and unknown inputs are represented by one shared probe surface. |
| GOAL-02 | Automatically classify terminal constraints and choose safe render defaults without manual tuning. | Capability detection and fallback selection are deterministic under representative terminal environments. | Local, tmux, ssh-like, and low-capability sessions produce explicit `TerminalProfile` and fallback outcomes. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Agent Operator | An agent or developer verifying media-heavy workflows from a shell session. | A truthful default rendering path without hand-tuning formats or terminal flags. |
| Terminal Developer | A human building or debugging terminal-first tooling around atext. | Stable data models and explicit fallbacks that can be tested locally and in CI. |

## Scope

### In Scope

- [SCOPE-01] Define and implement the first shared `ProbeResult` and `TerminalProfile` surfaces used by render planning.
- [SCOPE-03] Add detection entry points that classify initial media families and terminal environments into those shared surfaces.
- [SCOPE-04] Define conservative fallback rules that keep renderer selection explicit when capability or probe data is incomplete.

### Out of Scope

- [SCOPE-02] Full decoder coverage for every image, video, audio, and document container.
- [SCOPE-05] Final renderer implementation for every output mode beyond the planning and detection slice.
- [SCOPE-06] Inline image transports or terminal-specific escape protocol support.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | atext must expose a canonical probe model that captures the media family, dimensions, timing, and audio metadata needed for renderer planning. | GOAL-01 | must | Renderer selection should depend on one shared probe surface rather than per-format shortcuts. |
| FR-02 | atext must expose a canonical terminal capability profile that captures color, Unicode reliability, animation viability, multiplexers, remoteness, and terminal size. | GOAL-02 | must | Terminal-specific heuristics need one shared source of truth. |
| FR-03 | atext must provide detection and planning entry points that combine probe and capability data to choose defaults and explicit fallbacks. | GOAL-01, GOAL-02 | must | Auto-configuration only works when probe and capability detection feed one planning path. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Detection must prefer safe, explicit degradation when capability signals are missing, ambiguous, or hostile. | GOAL-02 | must | Over-optimistic terminal assumptions will produce misleading output in exactly the environments atext is meant to survive. |
| NFR-02 | Probe and capability detection must be deterministic and testable without requiring a live interactive terminal. | GOAL-01, GOAL-02 | must | The behavior needs regression coverage in CI and agent workflows, not only local shells. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Probe model | Rust unit tests against canonical `ProbeResult` shapes | Story-level `cargo test` evidence linked to `SRS-01` |
| Capability profile | Rust unit tests against terminal environment scenarios | Story-level `cargo test` evidence linked to `SRS-02` |
| Fallback planning | Rust unit tests proving conservative renderer defaults | Story-level `cargo test` evidence linked to `SRS-03` and `SRS-NFR-*` |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The first shipping slice only needs coarse media family detection, not full codec detail. | The voyage may under-specify fields required by later renderers. | Revisit after the first renderer implementation slice. |
| Environment variables and terminal size provide enough initial signal for conservative capability detection. | The fallback policy may need richer runtime probing or explicit overrides sooner than expected. | Validate against local, tmux, and ssh-like development sessions. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which future decoder backend should own deeper media sniffing once coarse probing is in place? | Epic owner | Open |
| How much of renderer planning should land in this slice versus a follow-on rendering epic? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A mission-linked epic and planned voyage define the first probe and terminal capability slice with traceable requirements.
- [ ] The implementation stories for the slice are ready to prove canonical probe modeling, terminal capability detection, and conservative fallback planning.
<!-- END SUCCESS_CRITERIA -->
