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

#[cfg(test)]
mod tests {
    use super::{ColorSupport, Multiplexer, SessionMode, TerminalProfile, TerminalSize};

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
}
