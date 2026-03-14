# Interactive Strategic Navigation - Product Requirements

## Problem Statement

The `atext Navigation Chart` is a powerful 3D visualization, but it is currently static. Operators cannot explore the project state space, and the "game" of reducing drift lacks real-time feedback. We need an interactive terminal loop that brings the globe to life.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Implement a live interactive terminal loop in the `atext globe` command. | Command enters interactive mode and captures input. | Live TTY session support. |
| GOAL-02 | Enable 3D rotation via arrow keys or automatic spinning. | Smooth rotation response to keyboard events. | Dynamic 3D perspective control. |
| GOAL-03 | Provide real-time metadata display for destinations (Epics) as they are navigated. | Epic details appear based on camera orientation. | Strategic context during exploration. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Navigator | A project operator exploring the project state space. | Fluid 3D rotation and POI discovery. |

## Scope

### In Scope

- [SCOPE-01] Non-blocking TTY input capture for arrow keys and 'q'.
- [SCOPE-02] Cursor-controlled frame refreshing using ANSI escape codes.
- [SCOPE-03] Interactive globe rotation around X and Y axes.
- [SCOPE-04] Real-time POI metadata surfacing during navigation.

### Out of Scope

- [SCOPE-05] Mouse interaction or terminal resize handling (first slice).
- [SCOPE-06] Multi-player or networked state synchronization.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The system must put the terminal into raw mode to capture non-buffered key events. | GOAL-01 | must | Enables real-time control. |
| FR-02 | Arrow keys must adjust the rotation angles of the 3D globe in real-time. | GOAL-02 | must | Core navigation mechanic. |
| FR-03 | The system must display the name of the nearest POI when the camera aligns with it. | GOAL-03 | must | Strategic context. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The interactive loop must not consume more than 5% CPU idle. | GOAL-01 | must | Efficiency. |
| NFR-02 | Terminal state (raw mode) must be correctly restored on exit. | GOAL-01 | must | TTY hygiene. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Interaction | Manual | TTY feels responsive and arrow keys rotate the globe. |
| Resource Usage | Manual | `top` indicates low CPU usage during idle. |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Standard ANSI escape codes are supported by the target terminal. | Rendering failure. | Use established Ghostty/Tmux profiles. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should we use a crate like `crossterm` for cross-platform TTY? | Epic Owner | Decided: Use minimal dependency first. |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `atext globe` enters an interactive session rather than printing a static frame.
- [ ] Users can rotate the 3D globe using arrow keys.
- [ ] The terminal area is updated smoothly without scrolling (using escape codes).
- [ ] Pressing `q` or `Ctrl-C` exits the session cleanly.
- [ ] Current mission metadata is highlighted when the lighthouse is in view.
<!-- END SUCCESS_CRITERIA -->
