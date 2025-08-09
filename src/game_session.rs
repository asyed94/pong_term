//! GameSession: Unified management of alternate screen, raw mode, and terminal output.
//! This ensures consistent terminal state throughout the game lifecycle.

use crate::framebuffer::FrameBuffer;
use crate::model::Board;
use crate::render::render_with_message_to_string;
use crate::terminal::RenderStyle;
use crate::util::str_width;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute, queue,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

// Synchronized Output escape sequences
const SYNC_BEGIN: &str = "\x1b[?2026h";
const SYNC_END: &str = "\x1b[?2026l";

/// Manages the terminal session for the game, including alternate screen,
/// raw mode, cursor visibility, and proper line ending conversion.
pub struct GameSession {
    // No fields needed - just lifecycle management
}

impl GameSession {
    /// Enter game session: alternate screen, raw mode, hide cursor.
    pub fn enter() -> io::Result<Self> {
        // Enter alternate screen first
        let mut out = io::stdout();
        execute!(out, EnterAlternateScreen)?;

        // Enable raw mode for input handling
        terminal::enable_raw_mode()?;

        // Hide cursor for clean display
        execute!(out, Hide)?;

        // Initialize momentum tracker for smooth input
        crate::input::init_momentum();

        Ok(Self {})
    }

    // ============================================================================
    // CORE RENDERING HELPERS (Private)
    // ============================================================================

    /// Convert content to raw mode line endings
    fn to_raw_mode(&self, content: &str) -> String {
        content.replace('\n', "\r\n")
    }

    /// Strip trailing newline if present
    fn strip_trailing_newline<'a>(&self, content: &'a str) -> &'a str {
        content.strip_suffix("\r\n").unwrap_or(content)
    }

    /// Write content with optional synchronized output
    fn write_output(&self, content: &[u8], synchronized: bool) -> io::Result<()> {
        let mut out = io::stdout();

        if synchronized {
            out.write_all(SYNC_BEGIN.as_bytes())?;
        }

        out.write_all(content)?;

        if synchronized {
            out.write_all(SYNC_END.as_bytes())?;
        }

        out.flush()
    }

    /// Render board to framebuffer
    fn render_board_to_buffer(&self, board: &Board) -> FrameBuffer {
        let style = RenderStyle::auto();
        let mut fb = FrameBuffer::new(board.width, board.height, ' ');
        crate::draw::draw_board(&mut fb, board, &style);
        fb
    }

    /// Apply centered overlay to base content
    fn apply_centered_overlay(
        &self,
        base_lines: Vec<&str>,
        overlay_lines: Vec<String>,
        board_width: usize,
        board_height: usize,
    ) -> Vec<String> {
        let mut result: Vec<String> = base_lines.iter().map(|s| s.to_string()).collect();

        // Calculate overlay dimensions and position
        let line_widths: Vec<usize> = overlay_lines
            .iter()
            .map(|l| str_width(l.as_str()))
            .collect();
        let menu_height = overlay_lines.len();
        let menu_width = line_widths.iter().copied().max().unwrap_or(0);

        // Normalize overlay lines (right-pad to uniform width)
        let mut normalized_lines = overlay_lines;
        for (line, width) in normalized_lines.iter_mut().zip(line_widths.iter()) {
            if *width < menu_width {
                let pad = " ".repeat(menu_width - *width);
                line.push_str(&pad);
            }
        }

        // Calculate centered position
        let start_y = board_height.saturating_sub(menu_height) / 2;
        let start_x = board_width.saturating_sub(menu_width) / 2;

        // Apply overlay to base content
        for (i, overlay_line) in normalized_lines.iter().enumerate() {
            let y = start_y + i;
            if y < result.len() {
                let line = &mut result[y];
                // Replace the portion of the base line with the overlay
                if start_x < line.len() {
                    let line_chars: Vec<char> = line.chars().collect();
                    let overlay_chars: Vec<char> = overlay_line.chars().collect();
                    let mut new_line = String::new();

                    // Before overlay
                    new_line.extend(line_chars.iter().take(start_x));
                    // Overlay content
                    new_line.push_str(overlay_line);
                    // After overlay (if any)
                    let end_x = start_x + overlay_chars.len();
                    if end_x < line_chars.len() {
                        new_line.extend(line_chars.iter().skip(end_x));
                    }

                    *line = new_line;
                }
            }
        }

        result
    }

    // ============================================================================
    // UNIFIED RENDERING PIPELINE (Private)
    // ============================================================================

    /// Core rendering pipeline - handles all rendering logic
    fn render_internal(
        &self,
        board: Option<&Board>,
        raw_content: Option<&str>,
        overlay_lines: Option<Vec<String>>,
        clear_screen: bool,
        synchronized: bool,
    ) -> io::Result<()> {
        let mut out = io::stdout();

        // Step 1: Clear screen if requested
        if clear_screen {
            queue!(out, Clear(ClearType::All), MoveTo(0, 0))?;
        }

        // Step 2: Determine base content
        let content = if let Some(raw) = raw_content {
            // Use provided raw content
            raw.to_string()
        } else if let Some(board) = board {
            // Render board to string
            let fb = self.render_board_to_buffer(board);
            fb.to_string_lines()
        } else {
            // No content to render
            return Ok(());
        };

        // Track if we have an overlay and apply it if needed
        let (final_content, has_overlay) = if let Some(overlay) = overlay_lines {
            let result = if let Some(board) = board {
                // Apply overlay to board content
                let base_lines: Vec<&str> = content.lines().collect();
                let overlaid =
                    self.apply_centered_overlay(base_lines, overlay, board.width, board.height);
                overlaid.join("\n")
            } else {
                content
            };
            (result, true)
        } else {
            (content, false)
        };

        // Step 4: Convert to raw mode line endings
        let raw_content = self.to_raw_mode(&final_content);

        // Step 5: Strip trailing newline for board renders
        let final_output = if board.is_some() || has_overlay {
            self.strip_trailing_newline(&raw_content).as_bytes()
        } else {
            raw_content.as_bytes()
        };

        // Step 6: Write output with optional synchronization
        self.write_output(final_output, synchronized)
    }

    // ============================================================================
    // PUBLIC API (Simple Wrappers)
    // ============================================================================

    /// Render arbitrary content, converting line endings for raw mode.
    pub fn render(&self, content: &str) -> io::Result<()> {
        // Simple content render: no board, no overlay, no clear, no sync
        self.render_internal(None, Some(content), None, false, false)
    }

    /// Render the game board with synchronized output.
    pub fn render_board(&self, board: &Board) -> io::Result<()> {
        // Board render: board, no overlay, clear screen, synchronized
        self.render_internal(Some(board), None, None, true, true)
    }

    /// Render the board with a message overlay.
    pub fn render_board_with_message(&self, board: &Board, message: &str) -> io::Result<()> {
        // Use existing helper to render board with message
        let rendered = render_with_message_to_string(board, message);
        // Render the pre-composed content: no board (already rendered), clear screen, no sync
        self.render_internal(None, Some(&rendered), None, true, false)
    }

    /// Render the pause menu with the game board in background.
    pub fn render_pause_menu(&self, board: &Board) -> io::Result<()> {
        let pause_message = "╔════════════════════════════════════════╗\n\
                             ║              GAME PAUSED               ║\n\
                             ╠════════════════════════════════════════╣\n\
                             ║                                        ║\n\
                             ║  Controls:                             ║\n\
                             ║    W/S      - Move left paddle         ║\n\
                             ║    ↑/↓      - Move right paddle        ║\n\
                             ║    Space    - Pause/Resume game        ║\n\
                             ║    Q        - Quit to main menu        ║\n\
                             ║                                        ║\n\
                             ║  Game Info:                            ║\n\
                             ║    FPS: 60                             ║\n\
                             ║    Board: 80×24                        ║\n\
                             ║                                        ║\n\
                             ║        Press SPACE to resume           ║\n\
                             ║                                        ║\n\
                             ╚════════════════════════════════════════╝";

        // Convert pause message to lines, trimming leading whitespace
        let overlay_lines: Vec<String> = pause_message
            .lines()
            .map(|l| l.trim_start().to_string())
            .collect();

        // Render board with pause menu overlay: board, overlay, clear screen, synchronized
        self.render_internal(Some(board), None, Some(overlay_lines), true, true)
    }

    /// Clear the screen (accounting for raw mode).
    pub fn clear_screen(&self) -> io::Result<()> {
        let mut out = io::stdout();
        execute!(out, Clear(ClearType::All), MoveTo(0, 0))?;
        out.flush()
    }

    /// Wait for Enter key press (raw mode is already active).
    pub fn wait_for_enter(&self) -> io::Result<()> {
        // Raw mode is already active, just read input
        loop {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Enter {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl Drop for GameSession {
    fn drop(&mut self) {
        // Cleanup in reverse order, suppressing all errors
        let mut out = io::stdout();

        // Show cursor
        let _ = execute!(out, Show);

        // Disable raw mode
        let _ = terminal::disable_raw_mode();

        // Exit alternate screen
        let _ = execute!(out, LeaveAlternateScreen);

        // Final flush
        let _ = out.flush();
    }
}
