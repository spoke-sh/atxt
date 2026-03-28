# INSTRUCTIONS.md

Procedural instructions and workflow guidance for agents working with `atext` under Keel.

## Path Mapping

- When the user refers to a project using the `~[project]` syntax (e.g., `~keel`), it refers to the sibling directory `../[project]` relative to the current workspace root.

## Human Interaction & Pokes

Keel's autonomous flow is governed by a physical battery metaphor, but the charge is now derived from real repository activity rather than a synthetic heartbeat file.

If a human user pokes you (for example, "I'm poking you" or "Wake up"), you MUST:

1. **Inspect the Charge**: Run `just keel heartbeat` to see whether recent repository activity is still energizing the board and whether the worktree is carrying uncommitted energy.
2. **Autonomous Scan**: Run `just keel mission next --status` and `just keel pulse` to identify any new work that has become ready or materialized.
3. **Visual Confirmation**: Run `just keel flow --scene` to verify whether the light is ON or whether the board is idle waiting for a real move.
4. **Diagnose**: Run `just keel doctor` before attempting any other work.

## Global Hygiene Checklist

Apply these checks to every change before finalizing work:

1. **Doctor First**: `just keel doctor` is the source of truth for board integrity. Run it at the start of every session. If the doctor reports errors or short circuits, fix those diagnostic orders before attempting other work or architectural changes.
2. **The Health Loop**: Use `just keel health --scene` for high-level triage. Subsystems are mapped as follows:
   - **NEURAL**: Stories (ID consistency, AC completion)
   - **MOTOR**: Voyages (structure, SRS/SDD authorship)
   - **STRATEGIC**: Epics (PRD, goal lineage)
   - **SENSORY**: Bearings (research, evidence quality)
   - **SKELETAL**: ADRs (architecture decisions)
   - **VITAL**: Missions (strategic achievement)
   - **AUTONOMIC**: Routines (cadence, materialization)
   - **CIRCULATORY**: Workflow (graph integrity, topology)
   - **PACEMAKER**: Heartbeat (derived repository activity and open-loop warning state)
   - **KINETIC**: Delivery (backlog liquidity, execution capacity)
3. **Pacemaker Protocol**: The system's heartbeat is derived from Git and worktree activity and inspected with `just keel heartbeat`. A clean repo falls back to the latest commit; a dirty repo uses the freshest changed path it can observe. `just keel doctor` warns when the worktree carries uncommitted energy, and the sealing commit is what clears that warning. The installed pre-commit hook keeps quality checks and tests tied to the commit boundary, and the commit-msg hook appends `doctor --status` to the message body.
4. **Gardening First**: Tend to the garden by fixing `just keel doctor` errors, discharging automated backlog, and resolving structural drift BEFORE notifying the human operator or requesting input.
5. **Notification Threshold**: Only request human intervention when you reach a manual lane that requires design direction or a decision on application behavior.
6. **Automated Guardrails**: You no longer need to run `just quality` or `just test` manually before every commit. The git pre-commit hook, installed via `just keel hooks install`, automatically enforces these checks. If a commit fails, resolve the reported lint or test failures and try again.
7. **Lifecycle Before Commit**: Run board-mutating lifecycle commands before the atomic commit when they generate or rewrite `.keel` artifacts. After the transition, inspect `git status` and include the resulting `.keel` churn in the same commit.
8. **Atomic Commits**: Commit once per logical unit of work using Conventional Commits.

## Autonomous Backlog Discharge

As long as the system is autonomous and the circuit is healthy, you are responsible for discharging the delivery backlog.

1. **Identify Ready Work**: Scan the delivery lane for stories in `backlog` that are not blocked by dependencies.
2. **Autonomous Start**: For each ready story, execute `just keel story start <id>`.
3. **Rube Goldberg Loop**: Transitioning a story to `in-progress` mutates the repository, which refreshes the derived heartbeat and keeps the circuit closed while you continue moving work.
4. **Priority**: Discharging the backlog is the primary tactical objective once energized. Continue until the backlog is empty or the circuit trips.
5. **Loop Closure**: After every successful implementation or transition, land a sealing commit that captures the resulting board and code changes. The pacemaker warning is cleared by committing the dirty worktree, not by touching a synthetic heartbeat file.

## Drift Management and Correction

When the 3D Drift Globe indicates significant divergence between project planning (Intent) and implementation (Reality):

1. **Upgrade Keel:** Ensure the repository is tracking the latest stabilization features by following the [Keel Upgrade Workflow](/home/alex/workspace/spoke-sh/atext/AGENTS.md#keel-upgrade-workflow).
2. **Heal the Engine:** Resolve any `keel doctor` findings introduced by the upgrade or existing structural drift.
3. **Course Correction:** Prioritize work that reduces the **Drift Magnitude**. This may include:
   - Synchronizing the Keel board with actual source code state.
   - Recording missing verification evidence.
   - Closing out stalled stories or epics.
4. **Sailing Alignment:** While immediate docking (zero drift) is not always required, agents must ensure the current development vector is pointed toward the same goal as the human intent.
