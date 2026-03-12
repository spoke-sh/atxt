set shell := ["bash", "-euo", "pipefail", "-c"]

default:
  @just --list

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all --check

cargo-check:
  cargo check --all-targets --all-features

clippy:
  cargo clippy --all-targets --all-features -- -W clippy::all -D warnings

test:
  cargo nextest run

doctest:
  cargo test --doc

quality: fmt-check cargo-check clippy

keel *args:
  keel {{args}}

coverage args="":
  mkdir -p coverage
  if [[ -n "{{args}}" ]]; then cargo llvm-cov nextest {{args}}; else cargo llvm-cov nextest --lcov --output-path ./coverage/lcov.info; fi

check: quality test doctest

flake-check:
  nix flake check
