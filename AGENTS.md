# AGENTS.md

Repository-local instructions for agents working in `atxt`.

## Path Mapping

- When the user refers to a project using the `~[project]` syntax (e.g., `~keel`), it refers to the sibling directory `../[project]` relative to the current workspace root.

## Base Guidance

- Follow the shared Keel operating model in [/home/alex/workspace/spoke-sh/keel/AGENTS.md](/home/alex/workspace/spoke-sh/keel/AGENTS.md).
- This file only adds `atxt`-specific guidance that should be applied on top of the upstream Keel instructions.

## Operational Guidance

Keel is an engine with strict constraints. Your primary responsibility is to execute tactical moves that advance the board state while maintaining board integrity.

### Core Principles

1. **Gardening First**: You MUST tend to the garden by fixing `just keel doctor` errors, discharging automated backlog, and resolving structural drift BEFORE notifying the human operator or requesting input.
2. **Heartbeat Hygiene**: Monitor the system's pulse with `just keel heartbeat` and `just keel health --scene`. The pacemaker is derived from repository activity; uncommitted energy in the worktree is tactical debt that should be closed autonomously by landing the sealing commit.
3. **Notification Discipline**: Ping the human operator ONLY when you need input on design direction or application behavior. Resolve technical drift and tactical moves autonomously.

### Canonical Turn Loop

Keel's operator rhythm is the `Orient -> Inspect -> Pull -> Ship -> Close` loop surfaced by `just keel turn`.

- **Orient**: Inspect charge and board stability with `just keel heartbeat`, `just keel health --scene`, `just keel flow --scene`, and `just keel doctor`.
- **Inspect**: Read current demand with `just keel mission next --status`, `just keel pulse`, `just keel roles`, and `just keel next --role <role> --explain` when routing is unclear.
- **Pull**: Select one role-scoped slice with `just keel next --role <role>`.
- **Ship**: Execute the slice, record proof, and advance lifecycle state.
- **Close**: Land the relevant transition and the sealing commit that clears open-loop energy.

### Session Start & Human Interaction

When a human user opens the chat or "pokes" you (for example, "Wake up" or "I'm poking you"), you MUST immediately perform the `Orient` and `Inspect` halves of the turn loop by following the **Human Interaction & Pokes** workflow in [INSTRUCTIONS.md](/home/alex/workspace/spoke-sh/atxt/INSTRUCTIONS.md):

1. **Heartbeat**: Run `just keel heartbeat` to inspect current charge and whether the worktree is carrying uncommitted energy.
2. **Pulse**: Run `just keel health --scene` to check subsystem stability.
3. **Scan**: Run `just keel mission next --status` and `just keel pulse`.
4. **Confirm**: Run `just keel flow --scene` to verify whether the LIGHT IS ON or the board is idle waiting for fresh repository activity.
5. **Diagnose**: Run `just keel doctor` to ensure board integrity before proceeding.

### Procedural Instructions

Follow the formal procedural loops and checklists defined in [INSTRUCTIONS.md](/home/alex/workspace/spoke-sh/atxt/INSTRUCTIONS.md).

## Keel Upgrade Workflow

When asked to update Keel in this repository, use this sequence:

1. Update the Nix flake so the repo points at the intended Keel version.
   - Adjust [`flake.nix`](/home/alex/workspace/spoke-sh/atxt/flake.nix) and refresh [`flake.lock`](/home/alex/workspace/spoke-sh/atxt/flake.lock) if the input pin changes.
2. Build or otherwise exercise the upgraded Keel version from the repo environment.
   - The goal is to prove the new Keel version resolves and runs from this repo, not just that the files changed.
3. Run `just keel doctor` and fix every failing check before proceeding.
   - Do not leave the board in a degraded or drifted state after the upgrade.
4. Run `just keel mission next` after the board is clean and review the recommendation.
   - Ask the user whether they want to execute the recommended next step before taking it.
5. Make a git commit only after the upgraded Keel flow is verified and the board is clean.
   - Include the flake changes, any required board repairs, and any generated `.keel` artifacts that were needed to restore a healthy state.

## Txtplot Upgrade Workflow

When asked to update the `txtplot` crate in this repository, use this sequence:

1. Update the `txtplot` dependency to the intended version or git revision.
   - Adjust [`Cargo.toml`](/home/alex/workspace/spoke-sh/atxt/Cargo.toml) and refresh [`Cargo.lock`](/home/alex/workspace/spoke-sh/atxt/Cargo.lock).
   - If the Nix packaging path asks for a new vendoring hash, update the relevant entry in [`flake.nix`](/home/alex/workspace/spoke-sh/atxt/flake.nix) before proceeding.
2. Build or otherwise exercise the upgraded renderer dependency from the repo environment.
   - The goal is to prove the new `txtplot` revision builds inside `atxt`, not just that the lockfile changed.
3. Run the human-visible render verification path and inspect the result for regressions.
   - Run `just mission` so the canonical verification signal is reviewed in both direct and degraded terminal paths.
   - If the signal changes, treat that as a deliberate rendering change and update expectations or fixtures intentionally.
4. Run the normal quality and packaging checks.
   - Run `just check` and `nix flake check`.
5. Run `just keel doctor` and fix every failing check before proceeding.
   - Do not leave the board in a degraded or drifted state after a renderer dependency change.
6. Run `just keel mission next` after the board is clean and review the recommendation.
   - Ask the user whether they want to execute the recommended next step before taking it.
7. Make a git commit only after the upgraded `txtplot` flow is verified, the human-level render signals have been reviewed, and the board is clean.
   - Include dependency updates, any fixture or expectation changes, and any generated `.keel` artifacts that were needed to restore a healthy state.

## Drift Management and Correction

When the 3D Drift Globe indicates significant divergence between project planning (Intent) and implementation (Reality):

1. **Upgrade Keel:** Ensure the repository is tracking the latest stabilization features by following the [Keel Upgrade Workflow](#keel-upgrade-workflow).
2. **Heal the Engine:** Resolve any `keel doctor` findings introduced by the upgrade or existing structural drift.
3. **Course Correction:** Prioritize work that reduces the **Drift Magnitude**. This may include:
   - Synchronizing the Keel board with actual source code state.
   - Recording missing verification evidence.
   - Closing out stalled stories or epics.
4. **Sailing Alignment:** While immediate docking (zero drift) is not always required, agents must ensure the current development vector is pointed toward the same goal as the human intent.
