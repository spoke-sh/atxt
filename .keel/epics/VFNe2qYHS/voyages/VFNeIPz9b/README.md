---
# system-managed
id: VFNeIPz9b
status: done
epic: VFNe2qYHS
created_at: 2026-03-30T12:25:55
# authored
title: Feature-Gate CLI and External Dependencies
index: 1
updated_at: 2026-03-30T12:31:59
started_at: 2026-03-30T14:17:41
completed_at: 2026-03-30T14:21:37
---

# Feature-Gate CLI and External Dependencies

> Separate CLI-only modules and ffprobe dependency behind Cargo feature gates so the default library surface is clean for downstream consumers.

## Documents

<!-- BEGIN DOCUMENTS -->
| Document | Description |
|----------|-------------|
| [SRS.md](SRS.md) | Requirements and verification criteria |
| [SDD.md](SDD.md) | Architecture and implementation details |
| [VOYAGE_REPORT.md](VOYAGE_REPORT.md) | Narrative summary of implementation and evidence |
| [COMPLIANCE_REPORT.md](COMPLIANCE_REPORT.md) | Traceability matrix and verification proof |
<!-- END DOCUMENTS -->

## Stories

<!-- BEGIN GENERATED -->
**Progress:** 3/3 stories complete

| Title | Type | Status |
|-------|------|--------|
| [Add Cargo Feature Gates for CLI and Video Modules](../../../../stories/VFNeyEfLY/README.md) | refactor | done |
| [Gate Video Probe Path in Media Module](../../../../stories/VFNeyTFSl/README.md) | refactor | done |
| [Verify No-Default-Features Library Compilation](../../../../stories/VFNeyhrWR/README.md) | feat | done |
<!-- END GENERATED -->
