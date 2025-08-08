//! ANSI terminal renderer using framebuffer.
//! Stage 3: Added synchronized output for efficient, flicker-free rendering.

use crate::draw::{draw_board, draw_board_with_message};
use crate::framebuffer::FrameBuffer;
use crate::model::Board;
use crate::terminal::RenderStyle;
use unicode_width::UnicodeWidthStr;

const ANSI_CLEAR: &str = "\x1b[2J";
const ANSI_HOME: &str = "\x1b[H";
const ANSI_HIDE_CURSOR: &str = "\x1b[?25l";
const ANSI_SHOW_CURSOR: &str = "\x1b[?25h";

// Alternate Screen Buffer escape sequences
// These allow us to use a separate screen that won't pollute terminal history
const ANSI_ENTER_ALTERNATE_SCREEN: &str = "\x1b[?1049h";
const ANSI_EXIT_ALTERNATE_SCREEN: &str = "\x1b[?1049l";

// Synchronized Output escape sequences
// These prevent screen tearing during updates
const SYNC_BEGIN: &str = "\x1b[?2026h";
const SYNC_END: &str = "\x1b[?2026l";

/// Render the board to a String including ANSI clear + home, then the frame.
/// Uses auto-detected render style (Unicode if supported, ASCII fallback).
pub fn render_to_string(board: &Board) -> String {
    let style = RenderStyle::auto();
    let mut fb = FrameBuffer::new(board.width, board.height, ' ');
    draw_board(&mut fb, board, &style);

    let mut s = String::new();
    s.push_str(ANSI_CLEAR);
    s.push_str(ANSI_HOME);
    // Keep the full board including trailing spaces
    let lines = fb.to_string_lines();
    // Only remove the final newline, not spaces
    if lines.ends_with('\n') {
        s.push_str(&lines[..lines.len() - 1]);
    } else {
        s.push_str(&lines);
    }
    s
}

/// Render the board with a message inside, to a String including ANSI clear + home.
pub fn render_with_message_to_string(board: &Board, message: &str) -> String {
    let style = RenderStyle::auto();
    let mut fb = FrameBuffer::new(board.width, board.height, ' ');
    draw_board_with_message(&mut fb, board, &style, message);

    let mut s = String::new();
    s.push_str(ANSI_CLEAR);
    s.push_str(ANSI_HOME);
    // Keep the full board including trailing spaces
    let lines = fb.to_string_lines();
    // Only remove the final newline, not spaces
    if lines.ends_with('\n') {
        s.push_str(&lines[..lines.len() - 1]);
    } else {
        s.push_str(&lines);
    }
    s
}

/// Render the board and print to stdout (includes ANSI clear + home).
pub fn render_and_print(board: &Board) -> std::io::Result<()> {
    use std::io::{self, Write};
    let mut out = io::stdout();
    out.write_all(render_to_string(board).as_bytes())?;
    out.flush()
}

/// Render the board with a message inside and print to stdout.
pub fn render_with_message_and_print(board: &Board, message: &str) -> std::io::Result<()> {
    use std::io::{self, Write};
    let mut out = io::stdout();
    out.write_all(render_with_message_to_string(board, message).as_bytes())?;
    out.flush()
}

/// Render with synchronized output for smooth, tear-free updates.
/// This is the main rendering function for the game loop.
pub fn render_synchronized(board: &Board) -> std::io::Result<()> {
    use std::io::{self, Write};

    let style = RenderStyle::auto();
    let mut fb = FrameBuffer::new(board.width, board.height, ' ');
    draw_board(&mut fb, board, &style);

    let mut out = io::stdout();

    // Begin synchronized output
    out.write_all(SYNC_BEGIN.as_bytes())?;

    // Clear and draw
    out.write_all(ANSI_CLEAR.as_bytes())?;
    out.write_all(ANSI_HOME.as_bytes())?;
    out.write_all(ANSI_HIDE_CURSOR.as_bytes())?;

    let lines = fb.to_string_lines();
    // Only remove the final newline, not spaces
    if lines.ends_with('\n') {
        out.write_all(lines[..lines.len() - 1].as_bytes())?;
    } else {
        out.write_all(lines.as_bytes())?;
    }

    // End synchronized output
    out.write_all(SYNC_END.as_bytes())?;

    out.flush()
}

/// Render the pause menu with synchronized output.
pub fn render_pause_menu(board: &Board) -> std::io::Result<()> {
    use std::io::{self, Write};

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

    let style = RenderStyle::auto();
    let mut fb = FrameBuffer::new(board.width, board.height, ' ');
    draw_board(&mut fb, board, &style);

    let mut out = io::stdout();

    // Begin synchronized output
    out.write_all(SYNC_BEGIN.as_bytes())?;

    // Clear and draw board
    out.write_all(ANSI_CLEAR.as_bytes())?;
    out.write_all(ANSI_HOME.as_bytes())?;
    out.write_all(ANSI_HIDE_CURSOR.as_bytes())?;

    let lines = fb.to_string_lines();
    // Only remove the final newline, not spaces
    if lines.ends_with('\n') {
        out.write_all(lines[..lines.len() - 1].as_bytes())?;
    } else {
        out.write_all(lines.as_bytes())?;
    }

    // Draw pause menu overlay (centered), dedented and width-normalized
    let raw_lines: Vec<&str> = pause_message.lines().collect();
    let mut menu_lines: Vec<String> = raw_lines
        .iter()
        .map(|l| l.trim_start().to_string())
        .collect();

    // Compute display width of each line and overall width
    let line_widths: Vec<usize> = menu_lines
        .iter()
        .map(|l| UnicodeWidthStr::width(l.as_str()))
        .collect();
    let menu_height = menu_lines.len();
    let menu_width = line_widths.iter().copied().max().unwrap_or(0);

    // Right-pad each line to menu_width so the right edge is perfectly vertical
    for (s, w) in menu_lines.iter_mut().zip(line_widths.iter()) {
        if *w < menu_width {
            let pad = " ".repeat(menu_width - *w);
            s.push_str(&pad);
        }
    }

    let start_y = (board.height.saturating_sub(menu_height)) / 2;
    let start_x = (board.width.saturating_sub(menu_width)) / 2;

    // Position cursor and draw menu
    for (i, line) in menu_lines.iter().enumerate() {
        let y = start_y + i + 1; // +1 for 1-based terminal coordinates
        let x = start_x + 1;
        write!(out, "\x1b[{};{}H{}", y, x, line)?;
    }

    // End synchronized output
    out.write_all(SYNC_END.as_bytes())?;

    out.flush()
}

/// Show the cursor when exiting the game.
pub fn show_cursor() -> std::io::Result<()> {
    use std::io::{self, Write};
    let mut out = io::stdout();
    out.write_all(ANSI_SHOW_CURSOR.as_bytes())?;
    out.flush()
}

/// Hide the cursor during gameplay.
pub fn hide_cursor() -> std::io::Result<()> {
    use std::io::{self, Write};
    let mut out = io::stdout();
    out.write_all(ANSI_HIDE_CURSOR.as_bytes())?;
    out.flush()
}

/// Enter alternate screen buffer (saves current screen and switches to a new buffer).
pub fn enter_alternate_screen() -> std::io::Result<()> {
    use std::io::{self, Write};
    let mut out = io::stdout();
    out.write_all(ANSI_ENTER_ALTERNATE_SCREEN.as_bytes())?;
    out.flush()
}

/// Exit alternate screen buffer (restores original screen).
pub fn exit_alternate_screen() -> std::io::Result<()> {
    use std::io::{self, Write};
    let mut out = io::stdout();
    out.write_all(ANSI_EXIT_ALTERNATE_SCREEN.as_bytes())?;
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Board, HEIGHT, WIDTH};

    #[test]
    fn top_and_bottom_borders_are_correct_length_and_chars() {
        let b = Board::new_static();
        let s = render_to_string(&b);
        // Skip ANSI codes and get actual frame content
        let frame_start = s
            .find(|c: char| c == '+' || c == '┌')
            .expect("frame should have border");
        let frame = &s[frame_start..];
        let lines: Vec<&str> = frame.lines().collect();
        assert_eq!(lines.len(), HEIGHT);
        assert_eq!(lines[0].chars().count(), WIDTH);
        assert_eq!(lines[HEIGHT - 1].chars().count(), WIDTH);

        // For ASCII style (tests use ASCII for predictability)
        if lines[0].chars().next().unwrap() == '+' {
            // Top border pattern
            let top: Vec<char> = lines[0].chars().collect();
            assert_eq!(top[0], '+');
            assert_eq!(top[WIDTH - 1], '+');
            for i in 1..(WIDTH - 1) {
                assert_eq!(top[i], '-');
            }

            // Bottom border pattern
            let bot: Vec<char> = lines[HEIGHT - 1].chars().collect();
            assert_eq!(bot[0], '+');
            assert_eq!(bot[WIDTH - 1], '+');
            for i in 1..(WIDTH - 1) {
                assert_eq!(bot[i], '-');
            }
        }
    }

    #[test]
    fn paddles_and_ball_positions() {
        let b = Board::new_static();
        let s = render_to_string(&b);
        let frame_start = s
            .find(|c: char| c == '+' || c == '┌')
            .expect("frame should have border");
        let frame = &s[frame_start..];
        let lines: Vec<Vec<char>> = frame.lines().map(|ln| ln.chars().collect()).collect();

        // Check ball (could be 'o' or '●' depending on style)
        let ball_char = lines[b.ball.y][b.ball.x];
        assert!(ball_char == 'o' || ball_char == '●');

        // Check left paddle (could be '|' or '█')
        for y in b.left.y..(b.left.y + b.left.height) {
            if y > 0 && y < b.height - 1 {
                let paddle_char = lines[y][b.left.x];
                assert!(paddle_char == '|' || paddle_char == '█');
            }
        }

        // Check right paddle
        for y in b.right.y..(b.right.y + b.right.height) {
            if y > 0 && y < b.height - 1 {
                let paddle_char = lines[y][b.right.x];
                assert!(paddle_char == '|' || paddle_char == '█');
            }
        }
    }
}
