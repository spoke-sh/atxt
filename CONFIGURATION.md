# Configuration Guide

`atext` does not currently use a project-specific runtime config file. Configuration lives in the development environment, the crate manifest, and the eventual CLI and API surfaces.

## Configuration Layers

Use these layers in order:

1. `flake.nix` for the development environment
2. `Cargo.toml` for package metadata and dependency graph
3. `justfile` for standard local workflows
4. CLI flags or Rust API calls for rendering behavior
5. terminal environment signals for capability detection

## Development Environment

If you use Nix:

```bash
nix develop
```

The default shell provides:

- the Rust toolchain with `clippy`, `rustfmt`, `rust-src`, `rust-analyzer`, and `llvm-tools`
- `cargo-nextest` for tests
- `cargo-llvm-cov` for coverage
- `just` for repository workflows
- `ffmpeg` for media probing and fixture generation
- `keel` for structured planning
- `sift` for fast search

## Workflow Commands

The repository-standard command surface is the `justfile`.

| Command | Purpose |
| --- | --- |
| `just fmt` | format the crate |
| `just fmt-check` | check formatting |
| `just cargo-check` | type-check all targets |
| `just clippy` | run lint checks with warnings denied |
| `just test` | run tests via `cargo nextest run` |
| `just doctest` | run documentation tests |
| `just check` | run formatting, type checks, linting, tests, and doctests |
| `just coverage` | generate an LCOV coverage report |
| `just flake-check` | validate the flake |

## Cargo Configuration

`Cargo.toml` is the canonical place for:

- crate name and version
- package metadata
- dependencies and dev-dependencies
- publication posture

If the public capability of the crate changes, update `Cargo.toml` first and then reflect the effect in [README.md](README.md) and [GUIDE.md](GUIDE.md).

## Terminal Environment Inputs

`atext` is expected to auto-detect capabilities from the runtime environment. These are environmental inputs, not project config:

- `TERM`
- `COLORTERM`
- `NO_COLOR`
- `TMUX`
- `SSH_CONNECTION`
- terminal width and height

Future implementation may also inspect whether stdout is a TTY and whether the session can support animation safely.

## Project-specific Environment Variables

No `atext`-specific environment variables are required today.

## Non-goals

These are intentionally outside the current configuration model:

- a mandatory `atext.toml` file for normal operation
- renderer-specific hidden global state
- per-format config systems that bypass shared detection logic

Normal use should stay close to zero-configuration.
