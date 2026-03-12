use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::{
    StillImageDecodeError, StillImageRenderError, TerminalEnvironment, TimedSequenceDecodeError,
    TimedSequenceSummaryError, decode_still_image, decode_timed_sequence, detect_terminal_profile,
    plan_render, probe_path, render_still_image, summarize_timed_sequence,
};

const USAGE: &str = "usage: atext render <path>";

/// User-facing CLI failures for the current renderable verification slices.
#[derive(Debug)]
pub enum CliError {
    Usage(&'static str),
    StillDecode(StillImageDecodeError),
    TimedDecode(TimedSequenceDecodeError),
    TimedSummary(TimedSequenceSummaryError),
    Render(StillImageRenderError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => f.write_str(message),
            Self::StillDecode(source) => write!(f, "{source}"),
            Self::TimedDecode(source) => write!(f, "{source}"),
            Self::TimedSummary(source) => write!(f, "{source}"),
            Self::Render(source) => write!(f, "{source}"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::StillDecode(source) => Some(source),
            Self::TimedDecode(source) => Some(source),
            Self::TimedSummary(source) => Some(source),
            Self::Render(source) => Some(source),
            Self::Usage(_) => None,
        }
    }
}

/// Run the current thin CLI contract against a captured terminal environment.
pub fn run_cli(args: &[String], env: &TerminalEnvironment) -> Result<String, CliError> {
    match args {
        [command, path] if command == "render" => render_command(Path::new(path), env),
        _ => Err(CliError::Usage(USAGE)),
    }
}

fn render_command(path: &Path, env: &TerminalEnvironment) -> Result<String, CliError> {
    let probe = probe_path(path);
    let terminal = detect_terminal_profile(env);
    let plan = plan_render(&probe, &terminal);
    let frame = if probe.kind.is_timed_visual() {
        let sequence = decode_timed_sequence(path, &probe).map_err(CliError::TimedDecode)?;
        summarize_timed_sequence(&sequence).map_err(CliError::TimedSummary)?
    } else {
        decode_still_image(path).map_err(CliError::StillDecode)?
    };

    render_still_image(&frame, &plan).map_err(CliError::Render)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::terminal::TerminalEnvironment;
    use gif::{Encoder, Frame, Repeat};
    use image::{Rgba, RgbaImage};

    use super::{CliError, run_cli};

    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TempCliFixture {
        path: PathBuf,
    }

    impl TempCliFixture {
        fn new() -> Self {
            let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!("atext-cli-test-{}-{}.png", process::id(), id));

            let mut image = RgbaImage::new(8, 8);
            for y in 0..8 {
                for x in 0..8 {
                    let pixel = if x < 4 {
                        Rgba([0, 0, 0, 255])
                    } else {
                        Rgba([255, 255, 255, 255])
                    };
                    image.put_pixel(x, y, pixel);
                }
            }
            image.save(&path).expect("fixture image should save");

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempCliFixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    struct TempTimedFixture {
        path: PathBuf,
    }

    impl TempTimedFixture {
        fn new() -> Self {
            let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!("atext-cli-test-{}-{}.gif", process::id(), id));

            let file = File::create(&path).expect("fixture gif should be created");
            let mut encoder = Encoder::new(file, 8, 8, &[]).expect("gif encoder should open");
            encoder
                .set_repeat(Repeat::Infinite)
                .expect("gif repeat should be set");

            for left_dark in [true, false] {
                let mut pixels = Vec::with_capacity(8 * 8 * 3);
                for _y in 0..8 {
                    for x in 0..8 {
                        let dark = if left_dark { x < 4 } else { x >= 4 };
                        let value = if dark { 0 } else { 255 };
                        pixels.extend_from_slice(&[value, value, value]);
                    }
                }

                let mut frame = Frame::from_rgb_speed(8, 8, &pixels, 10);
                frame.delay = 2;
                encoder
                    .write_frame(&frame)
                    .expect("gif frame should be written");
            }

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempTimedFixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    #[test]
    fn cli_renders_fixture_via_shared_direct_braille_path() {
        let fixture = TempCliFixture::new();
        let output = run_cli(
            &["render".to_string(), fixture.path().display().to_string()],
            &TerminalEnvironment {
                term: Some("xterm-256color".to_string()),
                colorterm: Some("truecolor".to_string()),
                no_color: false,
                tmux: false,
                ssh_connection: false,
                stdout_is_tty: true,
                columns: Some(4),
                rows: Some(2),
            },
        )
        .expect("direct braille CLI render should succeed");

        assert_eq!(output, "⣿⣿⠀⠀\n⣿⣿⠀⠀\n");
    }

    #[test]
    fn cli_renders_fixture_without_flags_in_captured_session() {
        let fixture = TempCliFixture::new();
        let output = run_cli(
            &["render".to_string(), fixture.path().display().to_string()],
            &TerminalEnvironment {
                term: Some("dumb".to_string()),
                colorterm: None,
                no_color: true,
                tmux: false,
                ssh_connection: true,
                stdout_is_tty: false,
                columns: Some(4),
                rows: Some(4),
            },
        )
        .expect("captured-session CLI render should succeed");

        assert_eq!(output, "@@  \n@@  \n@@  \n@@  \n");
    }

    #[test]
    fn cli_renders_timed_fixture_via_shared_contact_sheet_path() {
        let fixture = TempTimedFixture::new();
        let output = run_cli(
            &["render".to_string(), fixture.path().display().to_string()],
            &TerminalEnvironment {
                term: Some("xterm-256color".to_string()),
                colorterm: Some("truecolor".to_string()),
                no_color: false,
                tmux: false,
                ssh_connection: false,
                stdout_is_tty: true,
                columns: Some(8),
                rows: Some(2),
            },
        )
        .expect("timed direct CLI render should succeed");

        assert_eq!(output, "⣿⣿⠀⠀⠀⠀⣿⣿\n⣿⣿⠀⠀⠀⠀⣿⣿\n");
    }

    #[test]
    fn cli_renders_timed_fixture_without_flags_in_captured_session() {
        let fixture = TempTimedFixture::new();
        let output = run_cli(
            &["render".to_string(), fixture.path().display().to_string()],
            &TerminalEnvironment {
                term: Some("dumb".to_string()),
                colorterm: None,
                no_color: true,
                tmux: false,
                ssh_connection: true,
                stdout_is_tty: false,
                columns: Some(8),
                rows: Some(4),
            },
        )
        .expect("timed captured-session CLI render should succeed");

        assert_eq!(output, "@@    @@\n@@    @@\n@@    @@\n@@    @@\n");
    }

    #[test]
    fn cli_reports_usage_for_invalid_arguments() {
        let error = run_cli(&["render".to_string()], &TerminalEnvironment::default()).unwrap_err();

        match error {
            CliError::Usage(message) => assert_eq!(message, "usage: atext render <path>"),
            other => panic!("expected usage error, got {other:?}"),
        }
    }
}
