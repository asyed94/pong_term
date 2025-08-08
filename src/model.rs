//! Data model for Terminal Pong (Stage 1: static screen).
//!
//! Keep-It-Simple design for clarity and testability.

/// Fixed terminal width in characters.
pub const WIDTH: usize = 80;
/// Fixed terminal height in characters.
pub const HEIGHT: usize = 24;
/// Paddle height in characters.
pub const PADDLE_HEIGHT: usize = 5;

/// A paddle positioned by its top-left cell (x, y) and its height.
/// The paddle is drawn vertically using '|' glyphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paddle {
    pub x: usize,
    pub y: usize,
    pub height: usize,
}

/// The pong ball positioned by its (x, y) cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ball {
    pub x: usize,
    pub y: usize,
}

/// Board contains dimensions and entities to be rendered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub left: Paddle,
    pub right: Paddle,
    pub ball: Ball,
}

impl Board {
    /// Construct a simple static board for Stage 1:
    /// - Ball centered
    /// - Paddles centered vertically, inset from borders
    pub fn new_static() -> Self {
        let width = WIDTH;
        let height = HEIGHT;

        // Inset paddles away from border bars at x=0 and x=width-1
        let left_x = 2;
        let right_x = width.saturating_sub(3);

        let paddle_y = (height.saturating_sub(PADDLE_HEIGHT)) / 2;

        let left = Paddle {
            x: left_x,
            y: paddle_y,
            height: PADDLE_HEIGHT,
        };
        let right = Paddle {
            x: right_x,
            y: paddle_y,
            height: PADDLE_HEIGHT,
        };

        let ball = Ball {
            x: width / 2,
            y: height / 2,
        };

        Board {
            width,
            height,
            left,
            right,
            ball,
        }
    }
}
