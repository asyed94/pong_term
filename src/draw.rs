//! Drawing helpers that render model entities into a FrameBuffer.
//! Pure functions; no ANSI or I/O concerns here.

use crate::framebuffer::FrameBuffer;
use crate::model::{Ball, Board, Paddle};
use crate::terminal::RenderStyle;

/// Draw the outer border using style-specific characters.
pub fn draw_border(fb: &mut FrameBuffer, style: &RenderStyle) {
    let w = fb.width();
    let h = fb.height();

    if w < 2 || h < 2 {
        return;
    }

    // Top row
    fb.set(0, 0, style.border_corner_tl);
    for x in 1..w - 1 {
        fb.set(x, 0, style.border_horizontal);
    }
    fb.set(w - 1, 0, style.border_corner_tr);

    // Bottom row
    fb.set(0, h - 1, style.border_corner_bl);
    for x in 1..w - 1 {
        fb.set(x, h - 1, style.border_horizontal);
    }
    fb.set(w - 1, h - 1, style.border_corner_br);

    // Left and right columns
    for y in 1..h - 1 {
        fb.set(0, y, style.border_vertical);
        fb.set(w - 1, y, style.border_vertical);
    }
}

/// Draw a vertical paddle using style-specific glyphs. Safely clips to borders.
pub fn draw_paddle(fb: &mut FrameBuffer, p: &Paddle, style: &RenderStyle) {
    let w = fb.width();
    let h = fb.height();

    if w < 3 || h < 3 {
        return; // too small for interior content
    }

    let x = p.x.min(w.saturating_sub(2)); // stay inside right border
    if x == 0 || x >= w - 1 {
        return; // avoid drawing over vertical borders
    }

    let start_y = p.y.max(1);
    let end_y = p.y.saturating_add(p.height).min(h.saturating_sub(1));
    for y in start_y..end_y {
        if y > 0 && y < h - 1 {
            fb.set(x, y, style.paddle);
        }
    }
}

/// Draw the ball using style-specific character, clipped away from borders.
pub fn draw_ball(fb: &mut FrameBuffer, b: &Ball, style: &RenderStyle) {
    let w = fb.width();
    let h = fb.height();

    if w < 3 || h < 3 {
        return;
    }
    let x = b.x.min(w.saturating_sub(2));
    let y = b.y.min(h.saturating_sub(2));
    if x > 0 && x < w - 1 && y > 0 && y < h - 1 {
        fb.set(x, y, style.ball);
    }
}

/// Draw text centered at a specific row
pub fn draw_centered_text(fb: &mut FrameBuffer, text: &str, row: usize) {
    let w = fb.width();
    let text_len = text.len();

    if text_len >= w - 2 {
        return; // text too long
    }

    let start_x = (w - text_len) / 2;

    for (i, ch) in text.chars().enumerate() {
        fb.set(start_x + i, row, ch);
    }
}

/// Draw a complete static board into the framebuffer with given style.
/// Provided for convenience where a fully-rendered frame is desired.
pub fn draw_board(fb: &mut FrameBuffer, board: &Board, style: &RenderStyle) {
    draw_border(fb, style);
    draw_paddle(fb, &board.left, style);
    draw_paddle(fb, &board.right, style);
    draw_ball(fb, &board.ball, style);
}

/// Draw board with a message inside
pub fn draw_board_with_message(
    fb: &mut FrameBuffer,
    board: &Board,
    style: &RenderStyle,
    message: &str,
) {
    draw_board(fb, board, style);
    // Draw message in the bottom area, inside the border
    let message_row = board.height - 2; // One row above the bottom border
    draw_centered_text(fb, message, message_row);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Board;
    use crate::terminal::RenderStyle;

    #[test]
    fn border_has_expected_corners_and_edges() {
        let b = Board::new_static();
        let mut fb = FrameBuffer::new(b.width, b.height, ' ');
        let style = RenderStyle::ascii();
        draw_border(&mut fb, &style);
        let s = fb.to_string_lines();
        let lines: Vec<&str> = s.lines().collect();

        // corners
        assert_eq!(lines[0].chars().next().unwrap(), '+');
        assert_eq!(lines[0].chars().last().unwrap(), '+');
        assert_eq!(lines[b.height - 1].chars().next().unwrap(), '+');
        assert_eq!(lines[b.height - 1].chars().last().unwrap(), '+');

        // top/bottom runs
        for (i, ch) in lines[0].chars().enumerate() {
            if i != 0 && i != b.width - 1 {
                assert_eq!(ch, '-');
            }
        }
        for (i, ch) in lines[b.height - 1].chars().enumerate() {
            if i != 0 && i != b.width - 1 {
                assert_eq!(ch, '-');
            }
        }

        // vertical edges
        for y in 1..(b.height - 1) {
            assert_eq!(lines[y].chars().next().unwrap(), '|');
            assert_eq!(lines[y].chars().last().unwrap(), '|');
        }
    }

    #[test]
    fn paddles_and_ball_match_board() {
        let b = Board::new_static();
        let mut fb = FrameBuffer::new(b.width, b.height, ' ');
        let style = RenderStyle::ascii();
        draw_board(&mut fb, &b, &style);
        let s = fb.to_string_lines();
        let grid: Vec<Vec<char>> = s.lines().map(|ln| ln.chars().collect()).collect();

        // ball
        assert_eq!(grid[b.ball.y][b.ball.x], 'o');

        // left paddle
        for y in b.left.y..(b.left.y + b.left.height) {
            if y > 0 && y < b.height - 1 {
                assert_eq!(grid[y][b.left.x], '|');
            }
        }

        // right paddle
        for y in b.right.y..(b.right.y + b.right.height) {
            if y > 0 && y < b.height - 1 {
                assert_eq!(grid[y][b.right.x], '|');
            }
        }
    }
}
