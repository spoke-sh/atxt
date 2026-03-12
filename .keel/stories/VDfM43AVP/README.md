---
id: VDfM43AVP
title: Define Still Image Frame Model and Decode Path
type: feat
status: done
created_at: 2026-03-12T08:08:38
updated_at: 2026-03-12T08:22:46
operator-signal: 
scope: VDfKtP4Yp/VDfLYKNEX
index: 3
started_at: 2026-03-12T08:17:17
completed_at: 2026-03-12T08:22:46
---

# Define Still Image Frame Model and Decode Path

## Summary

Define the canonical still-image frame surface and implement the first path-based static-image decoder so later renderers consume one reusable normalized contract instead of opening files or holding backend-specific raster state themselves.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A shared still-image frame model exists for normalized raster data without exposing txtplot-specific types in the public media contract <!-- verify: cargo test, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] The crate can decode the first supported static image families from a filesystem path into the shared frame model <!-- verify: cargo test, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-03] The frame and decode path are deterministic and testable without requiring a live interactive terminal session <!-- verify: cargo test, SRS-NFR-02:start:end, proof: ac-3.log-->
