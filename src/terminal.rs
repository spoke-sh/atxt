use std::env;
use std::io::{self, IsTerminal};

/// Terminal color capability used during renderer planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSupport {
    None,
    Ansi16,
    Ansi256,
    Truecolor,
}

/// Known multiplexers that can affect rendering behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Multiplexer {
    None,
    Tmux,
    Screen,
    Unknown,
}

/// Whether output is targeting an interactive terminal or a captured stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionMode {
    Interactive,
    Captured,
}

/// Terminal cell dimensions used for render planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalSize {
    pub columns: u16,
    pub rows: u16,
}

impl TerminalSize {
    pub const fn new(columns: u16, rows: u16) -> Self {
        Self { columns, rows }
    }
}

/// Captured environment signals used to derive a terminal profile deterministically.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TerminalEnvironment {
    pub term: Option<String>,
    pub colorterm: Option<String>,
    pub no_color: bool,
    pub tmux: bool,
    pub ssh_connection: bool,
    pub stdout_is_tty: bool,
    pub columns: Option<u16>,
    pub rows: Option<u16>,
}

impl TerminalEnvironment {
    /// Capture the current process environment into a testable snapshot.
    pub fn capture() -> Self {
        let (columns, rows) = if io::stdout().is_terminal() {
            crossterm::terminal::size().ok().map(|(c, r)| (Some(c), Some(r))).unwrap_or((None, None))
        } else {
            (
                env::var("COLUMNS").ok().and_then(|v| v.parse().ok()),
                env::var("LINES").ok().and_then(|v| v.parse().ok()),
            )
        };

        Self {
            term: env::var("TERM").ok(),
            colorterm: env::var("COLORTERM").ok(),
            no_color: env::var_os("NO_COLOR").is_some(),
            tmux: env::var_os("TMUX").is_some(),
            ssh_connection: env::var_os("SSH_CONNECTION").is_some(),
            stdout_is_tty: io::stdout().is_terminal(),
            columns,
            rows,
        }
    }
}

/// Shared terminal capability profile used to select renderers and fallbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalProfile {
    pub color_support: ColorSupport,
    pub unicode_reliable: bool,
    pub animation_allowed: bool,
    pub inline_images_supported: bool,
    pub multiplexer: Multiplexer,
    pub is_remote: bool,
    pub session_mode: SessionMode,
    pub size: Option<TerminalSize>,
}

impl Default for TerminalProfile {
    fn default() -> Self {
        Self {
            color_support: ColorSupport::Ansi256,
            unicode_reliable: true,
            animation_allowed: true,
            inline_images_supported: false,
            multiplexer: Multiplexer::None,
            is_remote: false,
            session_mode: SessionMode::Interactive,
            size: None,
        }
    }
}

impl TerminalProfile {
    /// Detect the current terminal profile from captured environment signals.
    pub fn detect() -> Self {
        detect_terminal_profile(&TerminalEnvironment::capture())
    }
}

/// Derive a shared terminal capability profile from a captured environment snapshot.
pub fn detect_terminal_profile(env: &TerminalEnvironment) -> TerminalProfile {
    let term = env.term.as_deref().unwrap_or_default().to_ascii_lowercase();
    let colorterm = env
        .colorterm
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();

    let color_support = if env.no_color || term == "dumb" {
        ColorSupport::None
    } else if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        ColorSupport::Truecolor
    } else if term.contains("256color") {
        ColorSupport::Ansi256
    } else if env.term.is_some() {
        ColorSupport::Ansi16
    } else {
        ColorSupport::None
    };

    let multiplexer = if env.tmux {
        Multiplexer::Tmux
    } else if term.contains("screen") {
        Multiplexer::Screen
    } else {
        Multiplexer::None
    };

    let session_mode = if env.stdout_is_tty {
        SessionMode::Interactive
    } else {
        SessionMode::Captured
    };
    let unicode_reliable = term != "dumb" && !term.is_empty();
    let animation_allowed = matches!(session_mode, SessionMode::Interactive) && env.stdout_is_tty && unicode_reliable;
    let size = match (env.columns, env.rows) {
        (Some(columns), Some(rows)) => Some(TerminalSize::new(columns, rows)),
        _ => None,
    };

    TerminalProfile {
        color_support,
        unicode_reliable,
        animation_allowed,
        inline_images_supported: false,
        multiplexer,
        is_remote: env.ssh_connection,
        session_mode,
        size,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ColorSupport, Multiplexer, SessionMode, TerminalEnvironment, TerminalProfile, TerminalSize,
        detect_terminal_profile,
    };

    #[test]
    fn terminal_profile_defaults_to_interactive_session() {
        let profile = TerminalProfile::default();
        assert_eq!(profile.color_support, ColorSupport::Ansi256);
        assert_eq!(profile.session_mode, SessionMode::Interactive);
        assert_eq!(profile.size, None);
    }

    #[test]
    fn terminal_profile_can_be_constructed_without_live_terminal_access() {
        let profile = TerminalProfile {
            color_support: ColorSupport::Truecolor,
            unicode_reliable: true,
            animation_allowed: false,
            inline_images_supported: false,
            multiplexer: Multiplexer::Tmux,
            is_remote: true,
            session_mode: SessionMode::Captured,
            size: Some(TerminalSize::new(120, 40)),
        };

        assert_eq!(profile.multiplexer, Multiplexer::Tmux);
        assert!(profile.is_remote);
        assert_eq!(profile.session_mode, SessionMode::Captured);
        assert_eq!(profile.size, Some(TerminalSize::new(120, 40)));
    }

    #[test]
    fn detect_terminal_profile_handles_tmux_ssh_and_low_capability_sessions() {
        let profile = detect_terminal_profile(&TerminalEnvironment {
            term: Some("dumb".to_string()),
            colorterm: None,
            no_color: true,
            tmux: true,
            ssh_connection: true,
            stdout_is_tty: false,
            columns: Some(80),
            rows: Some(24),
        });

        assert_eq!(profile.color_support, ColorSupport::None);
        assert_eq!(profile.multiplexer, Multiplexer::Tmux);
        assert!(profile.is_remote);
        assert_eq!(profile.session_mode, SessionMode::Captured);
        assert!(!profile.unicode_reliable);
        assert!(!profile.animation_allowed);
        assert_eq!(profile.size, Some(TerminalSize::new(80, 24)));
    }

    #[test]
    fn detect_terminal_profile_handles_local_truecolor_interactive_sessions() {
        let profile = detect_terminal_profile(&TerminalEnvironment {
            term: Some("xterm-256color".to_string()),
            colorterm: Some("truecolor".to_string()),
            no_color: false,
            tmux: false,
            ssh_connection: false,
            stdout_is_tty: true,
            columns: Some(132),
            rows: Some(40),
        });

        assert_eq!(profile.color_support, ColorSupport::Truecolor);
        assert_eq!(profile.multiplexer, Multiplexer::None);
        assert!(!profile.is_remote);
        assert_eq!(profile.session_mode, SessionMode::Interactive);
        assert!(profile.unicode_reliable);
        assert!(profile.animation_allowed);
        assert_eq!(profile.size, Some(TerminalSize::new(132, 40)));
    }
}
