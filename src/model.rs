//! Game model: Board, Paddle, Ball.
//! Stage 3: Added mutable paddle movement with constraints.

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 24;
pub const PADDLE_HEIGHT: usize = 5;
pub const PADDLE_SPEED: usize = 1; // How many cells paddle moves per update

#[derive(Debug, Clone, PartialEq)]
pub struct Paddle {
    pub x: usize,
    pub y: usize,
    pub height: usize,
}

impl Paddle {
    /// Move paddle up by PADDLE_SPEED, respecting board boundaries.
    pub fn move_up(&mut self) {
        // Ensure paddle doesn't go above the top border (y=1)
        if self.y > 1 + PADDLE_SPEED {
            self.y -= PADDLE_SPEED;
        } else if self.y > 1 {
            self.y = 1;
        }
    }

    /// Move paddle down by PADDLE_SPEED, respecting board boundaries.
    pub fn move_down(&mut self, board_height: usize) {
        // Ensure paddle doesn't go below the bottom border
        let max_y = board_height.saturating_sub(self.height + 1);
        if self.y + PADDLE_SPEED < max_y {
            self.y += PADDLE_SPEED;
        } else if self.y < max_y {
            self.y = max_y;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ball {
    pub x: usize,
    pub y: usize,
    pub dx: i8, // velocity x (-1, 0, or 1)
    pub dy: i8, // velocity y (-1, 0, or 1)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub left: Paddle,
    pub right: Paddle,
    pub ball: Ball,
}

impl Board {
    /// Create a static board with paddles and ball at initial positions.
    pub fn new_static() -> Self {
        let paddle_y = (HEIGHT - PADDLE_HEIGHT) / 2;
        Board {
            width: WIDTH,
            height: HEIGHT,
            left: Paddle {
                x: 1,
                y: paddle_y,
                height: PADDLE_HEIGHT,
            },
            right: Paddle {
                x: WIDTH - 2,
                y: paddle_y,
                height: PADDLE_HEIGHT,
            },
            ball: Ball {
                x: WIDTH / 2,
                y: HEIGHT / 2,
                dx: 0,
                dy: 0,
            },
        }
    }

    /// Create a new game board with ball velocity for active gameplay.
    pub fn new_game() -> Self {
        let mut board = Self::new_static();
        // Set initial ball velocity (will be used in Stage 4)
        board.ball.dx = 1;
        board.ball.dy = 1;
        board
    }

    /// Move left paddle up.
    pub fn move_left_paddle_up(&mut self) {
        self.left.move_up();
    }

    /// Move left paddle down.
    pub fn move_left_paddle_down(&mut self) {
        self.left.move_down(self.height);
    }

    /// Move right paddle up.
    pub fn move_right_paddle_up(&mut self) {
        self.right.move_up();
    }

    /// Move right paddle down.
    pub fn move_right_paddle_down(&mut self) {
        self.right.move_down(self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paddle_movement_constraints() {
        let mut board = Board::new_static();

        // Test upward movement constraint
        for _ in 0..20 {
            board.move_left_paddle_up();
        }
        assert_eq!(board.left.y, 1); // Should stop at top border

        // Test downward movement constraint
        for _ in 0..30 {
            board.move_left_paddle_down();
        }
        assert_eq!(board.left.y, HEIGHT - PADDLE_HEIGHT - 1); // Should stop at bottom border

        // Test right paddle similarly
        for _ in 0..20 {
            board.move_right_paddle_up();
        }
        assert_eq!(board.right.y, 1);
    }
}
