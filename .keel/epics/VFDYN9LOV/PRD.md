# CLI Inspection And Surface Alignment - Product Requirements

## Problem Statement

atext's user-facing CLI contract is drifting from reality: the usage string advertises stats without dispatching it, probe and render-plan metadata are invisible to operators, and the documented target UX overstates the current surface. We need an inspection-first slice that makes the shipped CLI truthful, reviewable, and useful for agent workflows before adding the next media modality.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make the CLI contract truthful so operators can trust the command list and usage text. | Every documented top-level CLI command is either implemented and tested or removed from the advertised surface in the same slice. | `atext` usage, README examples, and CLI dispatch stay aligned. |
| GOAL-02 | Expose the probe, terminal, and render-planning decision path through a first-class inspection command. | An operator can inspect a media path and understand media kind, probe completeness, terminal profile, selected render mode, and fallback reason without reading code. | `atext inspect <path>` produces deterministic reviewable output for representative fixtures. |
| GOAL-03 | Keep inspection and stats output stable in remote, captured, and CI-like environments. | Direct and degraded terminal proofs remain readable and deterministic for inspection-oriented commands. | Story proofs cover both local and captured sessions for inspect/stats output. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Agent Operator | An agent or developer debugging media rendering decisions from a terminal-heavy workflow. | A truthful explanation of what `atext` detected and why it rendered or degraded output the way it did. |
| Terminal-Native Developer | A human reviewing media regressions over ssh, tmux, or CI logs. | Stable commands and inspection output that match the real product surface. |
| Project Maintainer | A contributor evolving the CLI and README over time. | One clear contract for what the CLI promises today so docs and behavior do not drift apart. |

## Scope

### In Scope

- [SCOPE-01] Align the shipped CLI usage text and dispatch table with the commands the binary actually supports.
- [SCOPE-02] Expose the existing project-progress stats surface through the CLI or intentionally remove it from the advertised contract.
- [SCOPE-03] Add a first inspection command that reports probe results, terminal profile, render plan, and fallback rationale for a given media path.
- [SCOPE-04] Add deterministic proofs and tests for direct and degraded inspection-oriented CLI output.
- [SCOPE-05] Update README and adjacent operator docs so the documented target UX matches the implemented CLI surface.

### Out of Scope

- [SCOPE-06] New media decoders or renderer families beyond what is required to inspect existing paths truthfully.
- [SCOPE-07] Rich interactive TUI exploration of inspection data.
- [SCOPE-08] A long-term machine API such as a versioned JSON schema unless the first slice requires a minimal structured export to stay truthful.
- [SCOPE-09] Release automation, packaging changes, or unrelated globe/navigation work.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The CLI must advertise only the commands it actually dispatches, and the dispatch table must cover every documented top-level command in the same slice. | GOAL-01 | must | Prevents users from being misled by stale usage text or README examples. |
| FR-02 | The CLI must expose a first-class `inspect <path>` flow that surfaces media classification, probe completeness, relevant metadata, and the selected render plan before any rendering-specific debugging requires source inspection. | GOAL-02 | must | Inspection is the missing operator affordance that explains render behavior. |
| FR-03 | The `inspect` output must include enough terminal context to explain degraded behavior, including session mode, Unicode reliability, and the planning reason when output falls back. | GOAL-02 GOAL-03 | must | Operators need to know whether the environment or the asset caused a fallback. |
| FR-04 | The CLI must either wire the existing `stats` surface into the advertised command contract or remove it from the public surface and docs in the same change slice. | GOAL-01 GOAL-03 | must | A half-advertised operator command is contract drift, not a roadmap. |
| FR-05 | The repository must include representative direct and degraded proofs for the new inspection-oriented command surface. | GOAL-03 | must | Keeps the operator contract reviewable in CI and protects it from silent drift. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Inspection output must remain deterministic and readable without requiring a live interactive terminal session. | GOAL-02 GOAL-03 | must | Agent workflows and CI logs need stable text, not ephemeral local-only views. |
| NFR-02 | The inspection command must reuse the shared `ProbeResult`, `TerminalProfile`, and `RenderPlan` contracts rather than duplicating renderer-local logic in the CLI. | GOAL-02 | must | Keeps the explanation path faithful to the actual media pipeline. |
| NFR-03 | The first inspection slice must stay narrow enough to land without redesigning the entire CLI around flags, sub-subcommands, or a new UI framework. | GOAL-01 GOAL-02 | must | The immediate problem is contract drift and missing introspection, not CLI reinvention. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| CLI contract alignment | Rust CLI tests plus usage-string assertions | Story-level `cargo test` evidence covering dispatch and usage behavior |
| Inspection output | Fixture-backed CLI proofs in direct and degraded environments | Story-level direct/degraded captures for `inspect <path>` |
| Stats surface truthfulness | CLI tests plus manual proof output | Story-level evidence showing whether `stats` is wired or intentionally removed |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A textual inspection summary is sufficient for the first operator-facing slice without introducing a machine-stable JSON format immediately. | The mission may need a structured output path sooner than expected. | Validate the first inspect output against operator and agent needs during execution. |
| The existing stats renderer is close enough to salvage as part of this slice. | The slice may need to drop or redesign stats instead of simply wiring it. | Inspect current stats behavior early in decomposition. |
| The shared probe and planning surfaces already expose enough signal to explain the first render decisions honestly. | The epic may need small additions to metadata or planning reasons before inspection is useful. | Verify inspection output against representative image, GIF/video, audio, and document paths. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the first inspect command default to human-readable text only, or should it also offer a minimal structured mode for agents? | Epic owner | Open |
| Is wiring `stats` worth preserving, or should the command be retired if it does not reflect a stable operator need? | Epic owner | Open |
| How much README target UX should be tightened now versus left as future-facing roadmap language? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `atext`'s documented top-level CLI surface matches the commands the binary actually supports.
- [ ] An operator can run `atext inspect <path>` and understand media classification, key metadata, terminal constraints, and render-planning rationale without reading source code.
- [ ] Reviewable proofs exist for both direct and degraded inspection-oriented CLI output paths.
<!-- END SUCCESS_CRITERIA -->
