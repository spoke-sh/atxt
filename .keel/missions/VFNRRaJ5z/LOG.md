# Library-First Surface for Downstream Integration - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-30T11:35:00

Created mission VFNRRaJ5z in response to downstream integration request from keel project. The keel project wants to use atxt as a direct library dependency for its "mission play" playback engine, eliminating the need for an external binary in `$PATH`. Assessed codebase and found the library surface already exists (`[lib]` target, 40+ public exports) but CLI-only modules leak into it and there is no high-level convenience API for library consumers. Scoped mission to feature-gating, a rendering entry point, and an opt-in `ffprobe` dependency — explicitly excluding crate registry publication and crate splitting.
