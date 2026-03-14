set positional-arguments

# Show available recipes.
default:
  @just --list

# Show the current project progress and canonical verification proofs.
screen:
  cargo build --bin atext >/dev/null
  just mission-status
  printf '\n'
  cargo run --quiet --bin atext -- screen
  printf '\n'
  just video-signal
  printf '\n'
  printf '\033[2moperator guide:\033[0m README.md (verification modes and current proof path)\n'
  printf '\033[2mrelease gate:\033[0m RELEASE.md (manual release checklist)\n'

# Print the canonical video verification signal.
video-signal:
  #!/usr/bin/env bash
  set -euo pipefail

  repo_root="{{justfile_directory()}}"
  fixture="$repo_root/src/testdata/multimodal_test.mp4"
  target_dir="${CARGO_TARGET_DIR:-$repo_root/target}"
  if [[ "$target_dir" != /* ]]; then
    target_dir="$repo_root/$target_dir"
  fi
  atext_bin="$target_dir/debug/atext"

  if [[ ! -f "$fixture" ]]; then
    printf 'error: video mission fixture not found: %s\n' "$fixture" >&2
    exit 1
  fi

  cargo build --bin atext >/dev/null

  printf 'atext video signal\n'
  printf 'fixture: %s\n' "${fixture#$repo_root/}"
  printf 'rendering multimodal summary...\n'
  TERM=xterm-256color COLORTERM=truecolor COLUMNS=80 LINES=24 "$atext_bin" render "$fixture"

# Build the workspace binary.
build:
  cargo build --bin atext

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all --check

# Run cargo check across the workspace.
cargo-check:
  cargo check --all-targets --all-features

clippy:
  cargo clippy --all-targets --all-features -- -W clippy::all -D warnings

# Run the workspace test suite with cargo-nextest.
test:
  cargo nextest run

# Run workspace documentation tests.
doctest:
  cargo test --doc

# Run the atext CLI with arbitrary arguments.
run *args:
  cargo run --quiet --bin atext -- {{args}}

# Print the canonical timed-sequence verification signal.
signal:
  #!/usr/bin/env bash
  set -euo pipefail

  repo_root="{{justfile_directory()}}"
  fixture="$repo_root/src/testdata/half-swap.gif"
  target_dir="${CARGO_TARGET_DIR:-$repo_root/target}"
  if [[ "$target_dir" != /* ]]; then
    target_dir="$repo_root/$target_dir"
  fi
  atext_bin="$target_dir/debug/atext"

  if [[ ! -f "$fixture" ]]; then
    printf 'error: mission fixture not found: %s\n' "$fixture" >&2
    exit 1
  fi

  cargo build --bin atext >/dev/null

  if command -v ffprobe >/dev/null 2>&1; then
    dimensions="$(ffprobe -v error -select_streams v:0 -show_entries stream=width,height -of csv=p=0:s=x "$fixture" | head -n1)"
    frame_count="$(ffprobe -v error -count_frames -select_streams v:0 -show_entries stream=nb_read_frames -of csv=p=0 "$fixture" | head -n1)"
  else
    dimensions="unknown"
    frame_count="unknown"
  fi

  render_direct() {
    local output_file="$1"
    local command_string
    printf -v command_string 'cd %q && TERM=xterm-256color COLORTERM=truecolor COLUMNS=8 LINES=2 %q render %q' "$repo_root" "$atext_bin" "$fixture"
    script -qec "$command_string" /dev/null | tr -d '\r' >"$output_file"
  }

  render_degraded() {
    local output_file="$1"
    TERM=dumb NO_COLOR=1 SSH_CONNECTION=mission COLUMNS=8 LINES=4 "$atext_bin" render "$fixture" >"$output_file"
  }

  direct_output="$(mktemp)"
  degraded_output="$(mktemp)"
  trap 'rm -f "$direct_output" "$degraded_output"' EXIT

  render_direct "$direct_output"
  render_degraded "$degraded_output"

  direct_expected=$'⣿⣿⠀⠀⠀⠀⣿⣿\n⣿⣿⠀⠀⠀⠀⣿⣿\n'
  degraded_expected=$'@@    @@\n@@    @@\n@@    @@\n@@    @@\n'

  if [[ "$(cat "$direct_output")"$'\n' != "$direct_expected" ]]; then
    printf 'error: direct mission render did not produce braille signal\n' >&2
    cat "$direct_output" >&2
    exit 1
  fi

  if [[ "$(cat "$degraded_output")"$'\n' != "$degraded_expected" ]]; then
    printf 'error: degraded mission render did not produce the expected ascii fallback signal\n' >&2
    cat "$degraded_output" >&2
    exit 1
  fi

  printf 'atext signal\n'
  printf 'fixture: %s\n' "${fixture#$repo_root/}"
  printf 'dimensions: %s\n' "$dimensions"
  printf 'source frames: %s\n' "$frame_count"
  printf 'direct terminal: interactive tty, truecolor, contact-sheet braille path\n'
  printf 'direct render:\n'
  cat "$direct_output"
  printf 'degraded terminal: captured session, dumb/no-color, ascii fallback\n'
  printf 'degraded render:\n'
  cat "$degraded_output"
  printf 'summary: the same timed visual input remains reviewable as one contact-sheet signal across direct and degraded terminal paths\n'

# Print the canonical audio verification signal.
audio-signal:
  #!/usr/bin/env bash
  set -euo pipefail

  repo_root="{{justfile_directory()}}"
  fixture="$repo_root/src/testdata/pulse.wav"
  target_dir="${CARGO_TARGET_DIR:-$repo_root/target}"
  if [[ "$target_dir" != /* ]]; then
    target_dir="$repo_root/$target_dir"
  fi
  atext_bin="$target_dir/debug/atext"

  if [[ ! -f "$fixture" ]]; then
    printf 'error: mission fixture not found: %s\n' "$fixture" >&2
    exit 1
  fi

  cargo build --bin atext >/dev/null

  render_direct() {
    local output_file="$1"
    local command_string
    printf -v command_string 'cd %q && TERM=xterm-256color COLORTERM=truecolor COLUMNS=16 LINES=4 %q render %q' "$repo_root" "$atext_bin" "$fixture"
    script -qec "$command_string" /dev/null | tr -d '\r' >"$output_file"
  }

  render_degraded() {
    local output_file="$1"
    TERM=dumb NO_COLOR=1 SSH_CONNECTION=mission COLUMNS=16 LINES=4 "$atext_bin" render "$fixture" >"$output_file"
  }

  direct_output="$(mktemp)"
  degraded_output="$(mktemp)"
  trap 'rm -f "$direct_output" "$degraded_output"' EXIT

  render_direct "$direct_output"
  render_degraded "$degraded_output"

  direct_expected=$'⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n⠉⠉⠉⠉⠀⠀⠀⠀⠉⠉⠉⠉⠀⠀⠀⠀\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⠉⠉⠉⠉⠀⠀⠀⠀⠉⠉⠉⠉\n'
  degraded_expected=$'                \n########        \n                \n        ########\n'

  if [[ "$(cat "$direct_output")"$'\n' != "$direct_expected" ]]; then
    printf 'error: direct audio mission render did not produce braille signal\n' >&2
    cat "$direct_output" >&2
    exit 1
  fi

  if [[ "$(cat "$degraded_output")"$'\n' != "$degraded_expected" ]]; then
    printf 'error: degraded audio mission render did not produce the expected ascii fallback signal\n' >&2
    cat "$degraded_output" >&2
    exit 1
  fi

  printf 'atext audio signal\n'
  printf 'fixture: %s\n' "${fixture#$repo_root/}"
  printf 'direct terminal: interactive tty, truecolor, waveform braille path\n'
  printf 'direct render:\n'
  cat "$direct_output"
  printf 'degraded terminal: captured session, dumb/no-color, ascii fallback\n'
  printf 'degraded render:\n'
  cat "$degraded_output"
  printf 'summary: the same audio input remains reviewable as one waveform signal across direct and degraded terminal paths\n'

# Print the current or latest mission completion summary.
mission-status:
  #!/usr/bin/env bash
  set -euo pipefail
  shopt -s nullglob

  select_rank() {
    case "$1" in
      active) echo 60 ;;
      achieved) echo 50 ;;
      paused) echo 40 ;;
      defining) echo 30 ;;
      verified) echo 20 ;;
      abandoned) echo 10 ;;
      *) echo 0 ;;
    esac
  }

  frontmatter_value() {
    local file="$1"
    local key="$2"
    sed -n "s/^${key}: //p" "$file" | head -n1
  }

  mission_files=(.keel/missions/*/README.md)
  if ((${#mission_files[@]} == 0)); then
    printf '%s\n' "atext mission" "status: none" "note: no missions found"
    exit 0
  fi

  selected_file=""
  selected_rank=-1
  selected_stamp=""

  for file in "${mission_files[@]}"; do
    status="$(frontmatter_value "$file" status)"
    rank="$(select_rank "$status")"
    stamp="$(frontmatter_value "$file" updated_at)"
    case "$status" in
      active) stamp="$(frontmatter_value "$file" activated_at)" ;;
      achieved) stamp="$(frontmatter_value "$file" achieved_at)" ;;
      verified) stamp="$(frontmatter_value "$file" verified_at)" ;;
    esac
    stamp="${stamp:-0000-00-00T00:00:00}"

    if (( rank > selected_rank )) || { (( rank == selected_rank )) && [[ "$stamp" > "$selected_stamp" ]]; }; then
      selected_file="$file"
      selected_rank="$rank"
      selected_stamp="$stamp"
    fi
  done

  mission_id="$(frontmatter_value "$selected_file" id)"
  mission_title="$(frontmatter_value "$selected_file" title)"
  mission_status="$(frontmatter_value "$selected_file" status)"

  printf 'atext mission\n'
  printf 'id: %s\n' "$mission_id"
  printf 'title: %s\n' "$mission_title"
  printf 'status: %s\n' "$mission_status"

  case "$mission_status" in
    achieved)
      printf 'next: nix develop -c keel mission verify %s\n' "$mission_id"
      ;;
    active)
      printf 'next: nix develop -c keel mission next %s\n' "$mission_id"
      ;;
  esac

# Print project progress visualizations.
# Show the 3D Drift Globe prototype.
globe:
  cargo build --bin atext >/dev/null
  cargo run --quiet --bin atext -- globe

quality: fmt-check cargo-check clippy

keel *args:
  keel {{args}}

coverage args="":
  mkdir -p coverage
  if [[ -n "{{args}}" ]]; then cargo llvm-cov nextest {{args}}; else cargo llvm-cov nextest --lcov --output-path ./coverage/lcov.info; fi

check: quality test doctest

flake-check:
  nix flake check
