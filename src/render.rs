//! ANSI terminal renderer using framebuffer (Stage 2 refactor).
//! This module now focuses on ANSI codes and terminal I/O,
//! delegating drawing logic to the draw module.

use crate::draw::{draw_board, draw_board_with_message};
use crate::framebuffer::FrameBuffer;
use crate::model::Board;
use crate::terminal::RenderStyle;

const ANSI_CLEAR: &str = "\x1b[2J";
const ANSI_HOME: &str = "\x1b[H";

/// Render the board to a String including ANSI clear + home, then the frame.
/// Uses auto-detected render style (Unicode if supported, ASCII fallback).
pub fn render_to_string(board: &Board) -> String {
    let style = RenderStyle::auto();
    let mut fb = FrameBuffer::new(board.width, board.height, ' ');
    draw_board(&mut fb, board, &style);

    let mut s = String::new();
    s.push_str(ANSI_CLEAR);
    s.push_str(ANSI_HOME);
    // Don't add trailing newline to keep within terminal bounds
    let lines = fb.to_string_lines();
    s.push_str(lines.trim_end());
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
    // Don't add trailing newline to keep within terminal bounds
    let lines = fb.to_string_lines();
    s.push_str(lines.trim_end());
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
