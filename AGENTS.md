# AGENTS.md

Repository-local instructions for agents working in `atext`.

## Base Guidance

- Follow the shared Keel operating model in [/home/alex/workspace/spoke-sh/keel/AGENTS.md](/home/alex/workspace/spoke-sh/keel/AGENTS.md).
- This file only adds `atext`-specific guidance that should be applied on top of the upstream Keel instructions.

## Keel Upgrade Workflow

When asked to update Keel in this repository, use this sequence:

1. Update the Nix flake so the repo points at the intended Keel version.
   - Adjust [`flake.nix`](/home/alex/workspace/spoke-sh/atext/flake.nix) and refresh [`flake.lock`](/home/alex/workspace/spoke-sh/atext/flake.lock) if the input pin changes.
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
   - Adjust [`Cargo.toml`](/home/alex/workspace/spoke-sh/atext/Cargo.toml) and refresh [`Cargo.lock`](/home/alex/workspace/spoke-sh/atext/Cargo.lock).
   - If the Nix packaging path asks for a new vendoring hash, update the relevant entry in [`flake.nix`](/home/alex/workspace/spoke-sh/atext/flake.nix) before proceeding.
2. Build or otherwise exercise the upgraded renderer dependency from the repo environment.
   - The goal is to prove the new `txtplot` revision builds inside `atext`, not just that the lockfile changed.
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
