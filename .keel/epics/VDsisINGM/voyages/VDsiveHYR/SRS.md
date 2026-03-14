# Interactive Terminal Loop and Globe Control - SRS

## Summary

Epic: VDsisINGM
Goal: Implement foundational TTY interaction for the 3D globe.

## Scope

### In Scope

- [SCOPE-01] Non-blocking TTY input capture for arrow keys and 'q'.
- [SCOPE-02] Cursor-controlled frame refreshing using ANSI escape codes.
- [SCOPE-03] Interactive globe rotation around X and Y axes.
- [SCOPE-04] Real-time POI metadata surfacing during navigation.

### Out of Scope

- [SCOPE-05] Mouse interaction or terminal resize handling (first slice).
- [SCOPE-06] Multi-player or networked state synchronization.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The system must put the terminal into raw mode to capture non-buffered key events. | SCOPE-01 | FR-01 | manual |
| SRS-02 | The system must use ANSI CSI codes to refresh the globe area without flicker. | SCOPE-02 | FR-01 | manual |
| SRS-03 | Arrow keys must adjust the rotation angles of the 3D globe in real-time. | SCOPE-03 | FR-02 | manual |
| SRS-04 | The system must display the name of the nearest POI when the camera aligns with it. | SCOPE-04 | FR-03 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The interactive loop must not consume more than 5% CPU idle. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | Terminal state (raw mode) must be correctly restored on exit. | SCOPE-01 | NFR-02 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
