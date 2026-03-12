# atext Constitution

> Prefer truthful terminal-native inspection over brittle high-fidelity illusions.

This document defines the project rules for evolving `atext` without losing the core product intent.

## Document Hierarchy

Use repository guidance in this order:

`README.md -> CONSTITUTION.md -> ARCHITECTURE.md -> CONFIGURATION.md / GUIDE.md / RELEASE.md -> code`

When code and docs disagree, update the docs in the same change slice or treat the change as incomplete.

## Core Belief

Humans decide the product direction and compatibility posture. Contributors and agents implement within that contract. The project should remain coherent enough that a new format adapter or renderer strengthens the system instead of creating another one-off path.

## Project Model

`atext` is expected to become both:

- a reusable Rust media-to-text engine
- a CLI for direct terminal use and agentic workflows

The shared rendering core matters more than any single interface.

## Principles

### 1. Terminal-first reliability

Default behavior should work in ordinary terminals, across `tmux`, `ssh`, and constrained environments, before specialized terminal features are considered.

### 2. One canonical intermediate representation

Different media formats may require different decoders, but they should converge into a small set of shared normalized forms before rendering.

### 3. Auto-detect before burdening the user

Input probing and terminal capability detection should happen automatically whenever possible. Flags exist to override, not to make the default path usable.

### 4. Verification beats imitation

The primary question is whether a human or agent can inspect the media truthfully and efficiently. Decorative fidelity is secondary.

### 5. Degradation must be explicit and useful

When the environment cannot support the preferred renderer, `atext` should choose a fallback that remains informative and predictable.

### 6. Adapters are replaceable, render contracts are stable

Codec-specific or backend-specific logic should stay behind narrow boundaries. The normalized render contract is the stable center.

### 7. Documentation moves with structure

When module ownership, workflows, or compatibility assumptions change, update the matching foundational document in the same change.

### 8. Research claims trail working product

Ideas about world models, multimodal training, or compressed perception are interesting, but the repository should not present them as established outcomes ahead of working tooling.

## Human and Agent Responsibilities

| Human responsibilities | Contributor and agent responsibilities |
| --- | --- |
| Decide the product direction | Implement focused, coherent changes |
| Approve compatibility posture | Respect the canonical pipeline |
| Choose when to support new environments | Keep docs and workflows aligned with code |
| Review release and packaging decisions | Add tests and fixtures with new behavior |

## Extension Contract

When adding capability:

1. Probe the input once and normalize the result.
2. Keep decoder concerns separate from renderer concerns.
3. Reuse shared terminal capability logic instead of re-detecting ad hoc.
4. Expose only the public API that downstream users actually need.
5. Leave the repository easier to reason about than before the change.

## Non-goals

The project should resist these failure modes by default:

- a separate rendering pipeline per input format
- mandatory hand-tuned terminal configuration
- hidden fallback behavior that changes output unpredictably
- product messaging that outruns the actual implementation

## Evolution

This constitution should change when the project meaningfully changes. If `atext` grows beyond the current bootstrap model, update this document and [ARCHITECTURE.md](ARCHITECTURE.md) together.
