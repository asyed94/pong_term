//! Terminal utilities: capability checks and render styles.

use std::io::{self, Write};

/// Check if the terminal supports Unicode (UTF-8).
/// Simple heuristic: check LANG/LC_ALL environment variables.
pub fn supports_unicode() -> bool {
    if let Ok(lang) = std::env::var("LANG") {
        if lang.contains("UTF-8") || lang.contains("utf8") {
            return true;
        }
    }
    if let Ok(lc_all) = std::env::var("LC_ALL") {
        if lc_all.contains("UTF-8") || lc_all.contains("utf8") {
            return true;
        }
    }
    false
}

/// Print terminal setup instructions and wait for user confirmation.
pub fn print_setup_instructions(required_width: usize, required_height: usize) -> io::Result<()> {
    let mut stdout = io::stdout();

    writeln!(
        stdout,
        "╔═══════════════════════════════════════════════════════╗"
    )?;
    writeln!(
        stdout,
        "║            Terminal Pong - Setup Check                ║"
    )?;
    writeln!(
        stdout,
        "╚═══════════════════════════════════════════════════════╝"
    )?;
    writeln!(stdout)?;
    writeln!(
        stdout,
        "Recommended terminal size: {}×{} or larger",
        required_width, required_height
    )?;
    writeln!(stdout)?;
    writeln!(stdout, "The game board will be displayed below.")?;
    writeln!(stdout, "Please adjust your terminal window size until:")?;
    writeln!(stdout, "  • The entire board is visible")?;
    writeln!(stdout, "  • The board appears as a complete rectangle")?;
    writeln!(stdout, "  • No lines are wrapped or cut off")?;
    writeln!(stdout)?;
    writeln!(stdout, "Press Enter when ready to continue...")?;
    stdout.flush()?;

    // Wait for Enter without echoing typed characters
    crate::input::wait_for_enter_no_echo()?;
    Ok(())
}

/// Terminal render style based on capabilities.
#[derive(Debug, Clone, Copy)]
pub struct RenderStyle {
    pub border_horizontal: char,
    pub border_vertical: char,
    pub border_corner_tl: char,
    pub border_corner_tr: char,
    pub border_corner_bl: char,
    pub border_corner_br: char,
    pub paddle: char,
    pub ball: char,
}

impl RenderStyle {
    /// ASCII-only style (fallback).
    pub fn ascii() -> Self {
        RenderStyle {
            border_horizontal: '-',
            border_vertical: '|',
            border_corner_tl: '+',
            border_corner_tr: '+',
            border_corner_bl: '+',
            border_corner_br: '+',
            paddle: '|',
            ball: 'o',
        }
    }

    /// Unicode box-drawing style (enhanced).
    pub fn unicode() -> Self {
        RenderStyle {
            border_horizontal: '─',
            border_vertical: '│',
            border_corner_tl: '┌',
            border_corner_tr: '┐',
            border_corner_bl: '└',
            border_corner_br: '┘',
            paddle: '█',
            ball: '●',
        }
    }

    /// Auto-detect best style based on terminal capabilities.
    pub fn auto() -> Self {
        // Allow forcing ASCII to avoid font/terminal artifacts with block glyphs
        if std::env::var("PONG_FORCE_ASCII").is_ok() {
            return Self::ascii();
        }
        if supports_unicode() {
            Self::unicode()
        } else {
            Self::ascii()
        }
    }
}
