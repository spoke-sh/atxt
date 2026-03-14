use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::{
    AudioDecodeError, AudioRenderError, StillImageDecodeError, StillImageRenderError,
    TerminalEnvironment, TimedSequenceDecodeError, TimedSequenceSummaryError,
    VideoDecodeError, VideoRenderError,
    decode_audio_summary, decode_still_image, decode_timed_sequence, detect_terminal_profile,
    plan_render, probe_path, render_audio_summary, render_still_image, summarize_timed_sequence,
};

const USAGE: &str = "usage: atext render <path> | atext screen | atext stats | atext globe";

/// User-facing CLI failures for the current renderable verification slices.
#[derive(Debug)]
pub enum CliError {
    Usage(&'static str),
    StillDecode(StillImageDecodeError),
    TimedDecode(TimedSequenceDecodeError),
    TimedSummary(TimedSequenceSummaryError),
    AudioDecode(AudioDecodeError),
    VideoDecode(VideoDecodeError),
    Render(StillImageRenderError),
    AudioRender(AudioRenderError),
    VideoRender(VideoRenderError),
    Stats(Box<dyn Error>),
    Screen(Box<dyn Error>),
    Globe(Box<dyn Error>),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => f.write_str(message),
            Self::StillDecode(source) => write!(f, "{source}"),
            Self::TimedDecode(source) => write!(f, "{source}"),
            Self::TimedSummary(source) => write!(f, "{source}"),
            Self::AudioDecode(source) => write!(f, "{source}"),
            Self::VideoDecode(source) => write!(f, "{source}"),
            Self::Render(source) => write!(f, "{source}"),
            Self::AudioRender(source) => write!(f, "{source}"),
            Self::VideoRender(source) => write!(f, "{source}"),
            Self::Stats(source) => write!(f, "stats failure: {source}"),
            Self::Screen(source) => write!(f, "screen failure: {source}"),
            Self::Globe(source) => write!(f, "globe failure: {source}"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::StillDecode(source) => Some(source),
            Self::TimedDecode(source) => Some(source),
            Self::TimedSummary(source) => Some(source),
            Self::AudioDecode(source) => Some(source),
            Self::VideoDecode(source) => Some(source),
            Self::Render(source) => Some(source),
            Self::AudioRender(source) => Some(source),
            Self::VideoRender(source) => Some(source),
            Self::Stats(source) => Some(source.as_ref()),
            Self::Screen(source) => Some(source.as_ref()),
            Self::Globe(source) => Some(source.as_ref()),
            Self::Usage(_) => None,
        }
    }
}

/// Run the current thin CLI contract against a captured terminal environment.
pub fn run_cli(args: &[String], env: &TerminalEnvironment) -> Result<String, CliError> {
    match args {
        [command, path] if command == "render" => render_command(Path::new(path), env),
        [command] if command == "screen" => screen_command(env),
        [command] if command == "globe" => {
            if env.stdout_is_tty {
                run_interactive_globe(env).map(|_| String::new()).map_err(CliError::Globe)
            } else {
                let drift = crate::globe::probe_project_drift();
                let width = env.columns.unwrap_or(80);
                let height = env.rows.unwrap_or(40);
                crate::render_drift_globe(0.5, 0.5, &drift, width, height).map_err(CliError::Globe)
            }
        }
        _ => Err(CliError::Usage(USAGE)),
    }
}

fn run_interactive_globe(env: &TerminalEnvironment) -> Result<(), Box<dyn Error>> {
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use std::io::{Write, stdout};
    use std::time::Duration;

    enable_raw_mode()?;
    let mut stdout = stdout();
    
    // Clear screen and hide cursor
    write!(stdout, "\x1b[2J\x1b[H\x1b[?25l")?;
    stdout.flush()?;

    let mut angle_x = 0.5;
    let mut angle_y = 0.5;
    let drift = crate::globe::probe_project_drift();

    loop {
        let width = env.columns.unwrap_or(80);
        let height = env.rows.unwrap_or(40);
        let frame = crate::render_drift_globe(angle_x, angle_y, &drift, width, height)?;
        
        // Reset cursor to top-left and print frame
        write!(stdout, "\x1b[H{}", frame)?;
        stdout.flush()?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Up => angle_x -= 0.1,
                    KeyCode::Down => angle_x += 0.1,
                    KeyCode::Left => angle_y -= 0.1,
                    KeyCode::Right => angle_y += 0.1,
                    _ => {}
                }
            }
        }
    }

    // Show cursor and disable raw mode
    write!(stdout, "\x1b[?25h")?;
    stdout.flush()?;
    disable_raw_mode()?;
    println!(); // Move to next line after interactive session

    Ok(())
}

fn screen_command(env: &TerminalEnvironment) -> Result<String, CliError> {
    let mut output = String::new();

    // 1. Navigation Chart (Drift Globe + POIs + Lighthouse)
    let drift = crate::globe::probe_project_drift();
    let width = env.columns.unwrap_or(80);
    let height = env.rows.unwrap_or(40);
    output.push_str(&crate::render_drift_globe(0.5, 0.5, &drift, width, height).map_err(CliError::Screen)?);
    output.push_str("\n---\n\n");

    // 2. Canonical Media Proofs (Truth)
    let fixtures = [
        ("Static Image Proof (half-dark.png)", "src/testdata/half-dark.png"),
        (
            "Timed Sequence Proof (half-swap.gif)",
            "src/testdata/half-swap.gif",
        ),
        ("Audio Waveform Proof (pulse.wav)", "src/testdata/pulse.wav"),
        ("Audio Waveform Proof (pulse.mp3)", "src/testdata/pulse.mp3"),
        ("Audio Waveform Proof (pulse.ogg)", "src/testdata/pulse.ogg"),
        ("Audio Waveform Proof (pulse.flac)", "src/testdata/pulse.flac"),
        ("Multimodal Video Proof (multimodal_test.mp4)", "src/testdata/multimodal_test.mp4"),
    ];

    for (label, path_str) in fixtures {
        output.push_str(&format!("\x1b[1m{}\x1b[0m\n", label));
        let path = Path::new(path_str);
        if path.exists() {
            output.push_str(&render_command(path, env)?);
        } else {
            output.push_str(&format!("error: fixture not found at {}\n", path_str));
        }
        output.push_str("\n");
    }

    Ok(output)
}

fn render_command(path: &Path, env: &TerminalEnvironment) -> Result<String, CliError> {
    let probe = probe_path(path);
    let terminal = detect_terminal_profile(env);
    let plan = plan_render(&probe, &terminal);

    if probe.kind == crate::media::MediaKind::Video {
        let summary = crate::video::decode_video_summary(path, &probe).map_err(CliError::VideoDecode)?;
        crate::video::render_video_summary(&summary, &plan).map_err(CliError::VideoRender)
    } else if probe.kind.is_timed_visual() {
        let sequence = decode_timed_sequence(path, &probe).map_err(CliError::TimedDecode)?;
        let frame = summarize_timed_sequence(&sequence).map_err(CliError::TimedSummary)?;
        render_still_image(&frame, &plan).map_err(CliError::Render)
    } else if probe.kind == crate::media::MediaKind::Audio {
        let summary = decode_audio_summary(path, &probe).map_err(CliError::AudioDecode)?;
        render_audio_summary(&summary, &plan).map_err(CliError::AudioRender)
    } else {
        let frame = decode_still_image(path).map_err(CliError::StillDecode)?;
        render_still_image(&frame, &plan).map_err(CliError::Render)
    }
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
            CliError::Usage(message) => {
                assert_eq!(message, "usage: atext render <path> | atext screen | atext stats | atext globe")
            }
            other => panic!("expected usage error, got {other:?}"),
        }
    }
}
