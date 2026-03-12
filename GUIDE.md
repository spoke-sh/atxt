# Contributor Guide

This guide explains how to extend `atext` without fighting the intended product shape.

## Start Here

1. Read [README.md](README.md) for the product intent.
2. Read [CONSTITUTION.md](CONSTITUTION.md) for the governing rules.
3. Read [ARCHITECTURE.md](ARCHITECTURE.md) for the pipeline boundary.
4. Use this guide for concrete workflows.

## Local Setup

### With Nix

```bash
nix develop
```

### Common commands

```bash
just
just mission
just signal
just mission-status
just fmt
just cargo-check
just clippy
just test
just check
```

## Common Extension Workflows

### Add support for a new media format

Use this path when the input format is new but the output categories are not.

1. Extend probing so the asset is identified early and consistently.
2. Add decoder logic behind a narrow boundary.
3. Normalize the decoded data into an existing shared media category.
4. Reuse existing renderers where possible.
5. Add fixture coverage that proves the output is still useful in a terminal context.

### Add a new renderer family

Use this path when the media model is already sufficient but the textual presentation is not.

1. Consume normalized media data rather than raw codec output.
2. Route renderer selection through shared terminal capability logic.
3. Make degradation rules explicit.
4. Add regression fixtures for the new output mode.

### Improve terminal capability detection

1. Keep detection logic centralized.
2. Prefer explicit capability fields over one-off environment checks in renderers.
3. Document any new assumptions in [CONFIGURATION.md](CONFIGURATION.md).

### Add a new public API type

1. Add the type in the owning module.
2. Re-export it from `src/lib.rs` only if it is part of the intended crate contract.
3. Update [README.md](README.md) and [ARCHITECTURE.md](ARCHITECTURE.md) if the type changes how users should understand the system.

## Fixtures and Regression Strategy

This project should grow with terminal-reviewable fixtures, not only opaque tests.

Prefer adding:

- small input fixtures that exercise the decoding path
- deterministic textual snapshots for renderer output
- metadata-level assertions for probe behavior
- fallback-path tests for low-capability terminals

When possible, test the degraded output too. That is part of the product, not an edge case.

## Searching and Planning

The Nix shell includes two optional tools for larger changes:

- `sift` for fast local search
- `keel` for structured planning and coordination

Example:

```bash
sift search . "terminal capability"
```

## Change Hygiene

Before considering a change complete:

1. Make sure the code lives on the correct side of the pipeline boundary.
2. Make sure terminal capability logic is still centralized.
3. Make sure docs changed with structural changes.
4. Make sure new behavior has at least one regression-friendly test or fixture.

## What To Avoid

- direct codec-specific logic inside renderers
- terminal probing scattered across multiple modules
- renderer-only flags becoming mandatory for normal use
- claiming format support before the degraded path is usable
