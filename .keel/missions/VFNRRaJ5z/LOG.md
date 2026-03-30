# Library-First Surface for Downstream Integration - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-30T11:35:00

Created mission VFNRRaJ5z in response to downstream integration request from keel project. The keel project wants to use atxt as a direct library dependency for its "mission play" playback engine, eliminating the need for an external binary in `$PATH`. Assessed codebase and found the library surface already exists (`[lib]` target, 40+ public exports) but CLI-only modules leak into it and there is no high-level convenience API for library consumers. Scoped mission to feature-gating, a rendering entry point, and an opt-in `ffprobe` dependency — explicitly excluding crate registry publication and crate splitting.

## 2026-03-30T12:35:00

Completed mission definition. Created epic VFNe2qYHS (Library-First Crate Surface) with PRD, attached to mission. Decomposed into two voyages:

- **VFNeIPz9b** (Feature-Gate CLI and External Dependencies): 3 stories — add Cargo feature gates (VFNeyEfLY), gate video probe in media module (VFNeyTFSl), verify no-default-features compilation (VFNeyhrWR). SRS and SDD authored.
- **VFNeJVpOI** (Library Rendering API and Downstream Proof): 2 stories — implement `render_to_text` entry point (VFNfBamXi), add downstream proof example (VFNfBhLbO). SRS and SDD authored.

Updated charter MG-01 verification to reference epic VFNe2qYHS. Mission ready for activation.

## 2026-03-30T14:25:52

Mission achieved by local system user 'alex'
