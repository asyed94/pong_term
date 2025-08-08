//! ANSI terminal renderer for a static Pong frame (Stage 1).
//! No external dependencies.

use crate::model::{Ball, Board, Paddle};

const ANSI_CLEAR: &str = "\x1b[2J";
const ANSI_HOME: &str = "\x1b[H";

/// Render the board to a String including ANSI clear + home, then the frame.
pub fn render_to_string(board: &Board) -> String {
    let mut s = String::new();
    s.push_str(ANSI_CLEAR);
    s.push_str(ANSI_HOME);
    s.push_str(&build_plain_frame(board));
    s
}

/// Render the board and print to stdout (includes ANSI clear + home).
pub fn render_and_print(board: &Board) -> std::io::Result<()> {
    use std::io::{self, Write};
    let mut out = io::stdout();
    out.write_all(render_to_string(board).as_bytes())?;
    out.flush()
}

/// Build the frame without ANSI codes (plain text with newlines).
/// Kept private; unit tests in this module use it directly.
fn build_plain_frame(board: &Board) -> String {
    let w = board.width;
    let h = board.height;

    // Guard against degenerate sizes.
    let w = w.max(2);
    let h = h.max(2);

    let mut grid: Vec<Vec<char>> = vec![vec![' '; w]; h];

    // Top and bottom borders: +----+
    for x in 0..w {
        grid[0][x] = if x == 0 || x == w - 1 { '+' } else { '-' };
        grid[h - 1][x] = if x == 0 || x == w - 1 { '+' } else { '-' };
    }
    // Vertical borders: |    |
    for y in 1..h - 1 {
        grid[y][0] = '|';
        grid[y][w - 1] = '|';
    }

    // Draw paddles and ball inside borders.
    draw_paddle(&mut grid, &board.left, w, h);
    draw_paddle(&mut grid, &board.right, w, h);
    draw_ball(&mut grid, &board.ball, w, h);

    // Convert to String with newlines.
    let mut out = String::with_capacity((w + 1) * h);
    for y in 0..h {
        for x in 0..w {
            out.push(grid[y][x]);
        }
        out.push('\n');
    }
    out
}

fn draw_paddle(grid: &mut [Vec<char>], p: &Paddle, w: usize, h: usize) {
    if w < 3 || h < 3 {
        return; // too small to draw interior
    }
    let x = p.x.min(w.saturating_sub(2)); // keep inside right border
    if x == 0 || x >= w - 1 {
        return; // avoid drawing over borders
    }
    let start_y = p.y.max(1);
    let end_y = (p.y.saturating_add(p.height)).min(h.saturating_sub(1));
    for y in start_y..end_y {
        if y > 0 && y < h - 1 {
            grid[y][x] = '|';
        }
    }
}

fn draw_ball(grid: &mut [Vec<char>], b: &Ball, w: usize, h: usize) {
    if w < 3 || h < 3 {
        return;
    }
    let x = b.x.min(w.saturating_sub(2));
    let y = b.y.min(h.saturating_sub(2));
    if x > 0 && x < w - 1 && y > 0 && y < h - 1 {
        grid[y][x] = 'o';
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Board, HEIGHT, WIDTH};

    #[test]
    fn top_and_bottom_borders_are_correct_length_and_chars() {
        let b = Board::new_static();
        let s = super::build_plain_frame(&b);
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines.len(), HEIGHT);
        assert_eq!(lines[0].len(), WIDTH);
        assert_eq!(lines[HEIGHT - 1].len(), WIDTH);

        // Top border pattern
        let top = lines[0].as_bytes();
        assert_eq!(top[0] as char, '+');
        assert_eq!(top[WIDTH - 1] as char, '+');
        for i in 1..(WIDTH - 1) {
            assert_eq!(top[i] as char, '-');
        }

        // Bottom border pattern
        let bot = lines[HEIGHT - 1].as_bytes();
        assert_eq!(bot[0] as char, '+');
        assert_eq!(bot[WIDTH - 1] as char, '+');
        for i in 1..(WIDTH - 1) {
            assert_eq!(bot[i] as char, '-');
        }
    }

    #[test]
    fn paddles_and_ball_positions() {
        let b = Board::new_static();
        let s = super::build_plain_frame(&b);
        let lines: Vec<Vec<char>> = s.lines().map(|ln| ln.chars().collect()).collect();

        // Check ball
        assert_eq!(lines[b.ball.y][b.ball.x], 'o');

        // Check left paddle
        for y in b.left.y..(b.left.y + b.left.height) {
            if y > 0 && y < b.height - 1 {
                assert_eq!(lines[y][b.left.x], '|');
            }
        }

        // Check right paddle
        for y in b.right.y..(b.right.y + b.right.height) {
            if y > 0 && y < b.height - 1 {
                assert_eq!(lines[y][b.right.x], '|');
            }
        }
    }
}
