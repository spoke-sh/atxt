# Agent Instructions

## Path Mapping

- When the user refers to a project using the `~[project]` syntax (e.g., `~keel`), it refers to the sibling directory `../[project]` relative to the current workspace root.

## Drift Management and Correction

When the 3D Drift Globe indicates significant divergence between project planning (Intent) and implementation (Reality):

1. **Upgrade Keel:** Ensure the repository is tracking the latest stabilization features by following the [Keel Upgrade Workflow](#keel-upgrade-workflow).
2. **Heal the Engine:** Resolve any `keel doctor` findings introduced by the upgrade or existing structural drift.
3. **Course Correction:** Prioritize work that reduces the **Drift Magnitude**. This may include:
   - Synchronizing the Keel board with actual source code state.
   - Recording missing verification evidence.
   - Closing out stalled stories or epics.
4. **Sailing Alignment:** While immediate docking (zero drift) is not always required, agents must ensure the current development vector is pointed toward the same goal as the human intent.
