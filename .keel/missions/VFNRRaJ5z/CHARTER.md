# Library-First Surface for Downstream Integration - Charter

Archetype: Voyage

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver a library-first public surface so a downstream Rust crate can probe, plan, decode, and render media to terminal text by depending on `atext` directly, without routing through the CLI entry point or requiring external binaries in `$PATH`. | board: _TBD_ |

## Strategy

- Gate CLI-only modules (`cli`, `globe`, `stats`) behind a `cli` Cargo feature so the default library surface exposes only the canonical probe-to-render pipeline.
- Introduce a high-level rendering entry point (e.g. `render_to_text`) that chains probe → plan → decode → render for a given path and terminal profile, giving library consumers a single-call path without requiring them to understand pipeline internals.
- Make `ffprobe`-dependent video probing opt-in via a Cargo feature so library consumers that only need image and audio rendering carry no external process dependency.
- Preserve the existing CLI behavior unchanged when the `cli` feature is active; the binary target enables it by default.

## Constraints

- Do not split into a separate `atxt-core` crate; the library already exists and the single-crate structure is sufficient for path and git dependencies.
- Do not pursue crate registry publication in this mission; distribution is out of scope.
- Keep the normalized media model and canonical pipeline untouched; this mission reshapes the crate surface, not the rendering architecture.
- The operator-facing proof must show a minimal Rust program that depends on `atext` as a library, renders a canonical fixture to text, and compiles without the `cli` feature or `ffprobe` on `$PATH`.

## Halting Rules

- DO NOT halt while the linked epic still has unfinished voyage or story work.
- HALT when the library surface compiles cleanly without CLI dependencies, the high-level rendering entry point works for still image, animated image, and audio paths, and the proof program demonstrates downstream usage.
- YIELD to human after achievement for verification review, or sooner if the only remaining question is whether the feature-gate boundaries are drawn at the right modules.
