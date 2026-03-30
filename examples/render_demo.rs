//! Proof program demonstrating `atext` as a library dependency.
//!
//! This example renders canonical fixtures using the high-level
//! `render_to_text` entry point with a synthetic terminal profile,
//! proving the library surface works without the `cli` feature or
//! `ffprobe` on `$PATH`.
//!
//! Run with: cargo run --example render_demo --no-default-features

use std::path::Path;

use atext::{TerminalEnvironment, detect_terminal_profile, render_to_text};

fn main() {
    let profile = detect_terminal_profile(&TerminalEnvironment {
        term: Some("xterm-256color".into()),
        colorterm: None,
        no_color: false,
        tmux: false,
        ssh_connection: false,
        columns: Some(60),
        rows: Some(20),
        stdout_is_tty: false,
    });

    let fixtures: &[(&str, &str)] = &[
        ("Still Image", "src/testdata/half-dark.png"),
        ("Animated GIF", "src/testdata/half-swap.gif"),
        ("Audio (WAV)", "src/testdata/pulse.wav"),
    ];

    for (label, path_str) in fixtures {
        let path = Path::new(path_str);
        println!("=== {} ===", label);
        if !path.exists() {
            println!("  (fixture not found, skipping)\n");
            continue;
        }
        match render_to_text(path, &profile) {
            Ok(text) => println!("{}\n", text),
            Err(e) => println!("  error: {}\n", e),
        }
    }
}
