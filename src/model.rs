//! Game model: Board, Paddle, Ball.
//! Stage 4: Added ball physics and collision detection.

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 24;
pub const PADDLE_HEIGHT: usize = 5;
pub const PADDLE_SPEED: usize = 1; // How many cells paddle moves per update
pub const BALL_SPEED_DIVISOR: usize = 2; // Ball moves every N frames (higher = slower)

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

impl Ball {
    /// Update ball position based on velocity.
    pub fn update_position(&mut self) {
        // Safe conversion with bounds checking
        let new_x = self.x as i32 + self.dx as i32;
        let new_y = self.y as i32 + self.dy as i32;

        // Ensure positions stay within reasonable bounds
        if new_x >= 0 && new_x < WIDTH as i32 {
            self.x = new_x as usize;
        }
        if new_y >= 0 && new_y < HEIGHT as i32 {
            self.y = new_y as usize;
        }
    }

    /// Reverse horizontal direction (paddle hit).
    pub fn bounce_horizontal(&mut self) {
        self.dx = -self.dx;
    }

    /// Reverse vertical direction (wall hit).
    pub fn bounce_vertical(&mut self) {
        self.dy = -self.dy;
    }

    /// Reset ball to center with specified direction.
    /// towards_left: true means ball goes left (after right player scores)
    pub fn reset(&mut self, board_width: usize, board_height: usize, towards_left: bool) {
        self.x = board_width / 2;
        self.y = board_height / 2;

        // Direction away from scorer
        self.dx = if towards_left { -1 } else { 1 };

        // Start with straight horizontal movement, can randomize later
        self.dy = 0;
    }
}

/// Events that can occur during ball physics updates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BallEvent {
    None,
    WallBounce,
    PaddleBounce,
    LeftGoal,  // Right player scores
    RightGoal, // Left player scores
}

/// Where on the paddle the ball hit.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PaddleHitLocation {
    TopEdge,    // Very top - strong upward angle
    TopMid,     // Upper area - moderate upward angle
    Center,     // Center area - straight
    BottomMid,  // Lower area - moderate downward angle
    BottomEdge, // Very bottom - strong downward angle
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub left: Paddle,
    pub right: Paddle,
    pub ball: Ball,
    pub frame_counter: usize, // Track frames for ball speed control
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
            frame_counter: 0,
        }
    }

    /// Create a new game board with ball velocity for active gameplay.
    pub fn new_game() -> Self {
        let mut board = Self::new_static();
        // Set initial ball velocity - start towards right with slight upward angle
        board.ball.dx = 1;
        board.ball.dy = -1;
        board
    }

    /// Update ball physics - returns event for feedback.
    pub fn update_ball(&mut self) -> BallEvent {
        // Adjust speed based on angle - angled balls move slightly faster
        let speed_divisor = if self.ball.dy != 0 {
            // Ball is angled - move every 3 frames out of 4 (faster)
            if self.frame_counter % 4 == 3 {
                // Skip this frame
                self.frame_counter += 1;
                return BallEvent::None;
            }
            BALL_SPEED_DIVISOR
        } else {
            // Ball is straight - normal speed
            BALL_SPEED_DIVISOR
        };

        // Only move ball based on speed divisor
        self.frame_counter += 1;
        if self.frame_counter % speed_divisor != 0 {
            return BallEvent::None;
        }

        // Move ball
        self.ball.update_position();

        // 1. Check paddle collisions FIRST (priority)
        if let Some(paddle_hit) = self.check_paddle_collision_with_angle() {
            self.ball.bounce_horizontal();

            // Apply angle based on where ball hit the paddle
            match paddle_hit {
                PaddleHitLocation::TopEdge => {
                    self.ball.dy = -1; // Strong upward angle
                }
                PaddleHitLocation::TopMid => {
                    self.ball.dy = -1; // Moderate upward angle
                }
                PaddleHitLocation::Center => {
                    self.ball.dy = 0; // Ball goes straight
                }
                PaddleHitLocation::BottomMid => {
                    self.ball.dy = 1; // Moderate downward angle
                }
                PaddleHitLocation::BottomEdge => {
                    self.ball.dy = 1; // Strong downward angle
                }
            }

            return BallEvent::PaddleBounce;
        }

        // 2. Check wall collisions
        if self.check_wall_collision() {
            self.ball.bounce_vertical();
            return BallEvent::WallBounce;
        }

        // 3. Check for goals
        if self.ball.x == 0 {
            // Left goal - right player scores
            self.ball.reset(self.width, self.height, true); // Ball goes left
            self.frame_counter = 0; // Reset frame counter
            return BallEvent::LeftGoal;
        }
        if self.ball.x >= self.width - 1 {
            // Right goal - left player scores
            self.ball.reset(self.width, self.height, false); // Ball goes right
            self.frame_counter = 0; // Reset frame counter
            return BallEvent::RightGoal;
        }

        BallEvent::None
    }

    /// Check if ball collides with either paddle and return hit location.
    fn check_paddle_collision_with_angle(&self) -> Option<PaddleHitLocation> {
        // Left paddle collision
        if self.ball.x == self.left.x
            && self.ball.y >= self.left.y
            && self.ball.y < self.left.y + self.left.height
        {
            return Some(self.get_paddle_hit_location(&self.left));
        }

        // Right paddle collision
        if self.ball.x == self.right.x
            && self.ball.y >= self.right.y
            && self.ball.y < self.right.y + self.right.height
        {
            return Some(self.get_paddle_hit_location(&self.right));
        }

        None
    }

    /// Determine where on the paddle the ball hit.
    fn get_paddle_hit_location(&self, paddle: &Paddle) -> PaddleHitLocation {
        let relative_y = self.ball.y.saturating_sub(paddle.y);

        // Paddle is 5 units tall (0-4 relative positions)
        // 0 = top edge, 4 = bottom edge
        match relative_y {
            0 => PaddleHitLocation::TopEdge,    // Very top
            1 => PaddleHitLocation::TopMid,     // Upper area
            2 => PaddleHitLocation::Center,     // Center
            3 => PaddleHitLocation::BottomMid,  // Lower area
            _ => PaddleHitLocation::BottomEdge, // Very bottom (4+)
        }
    }

    /// Check if ball collides with either paddle.
    #[cfg(test)]
    fn check_paddle_collision(&self) -> bool {
        // Left paddle collision
        if self.ball.x == self.left.x {
            if self.ball.y >= self.left.y && self.ball.y < self.left.y + self.left.height {
                return true;
            }
        }

        // Right paddle collision
        if self.ball.x == self.right.x {
            if self.ball.y >= self.right.y && self.ball.y < self.right.y + self.right.height {
                return true;
            }
        }

        false
    }

    /// Check if ball hits top or bottom wall.
    fn check_wall_collision(&self) -> bool {
        // Top wall (accounting for border at y=0)
        if self.ball.y <= 1 && self.ball.dy < 0 {
            return true;
        }

        // Bottom wall (accounting for border at y=HEIGHT-1)
        if self.ball.y >= self.height - 2 && self.ball.dy > 0 {
            return true;
        }

        false
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

    #[test]
    fn test_ball_movement() {
        let mut ball = Ball {
            x: 10,
            y: 10,
            dx: 1,
            dy: -1,
        };
        ball.update_position();
        assert_eq!(ball.x, 11);
        assert_eq!(ball.y, 9);

        ball.dx = -1;
        ball.dy = 1;
        ball.update_position();
        assert_eq!(ball.x, 10);
        assert_eq!(ball.y, 10);
    }

    #[test]
    fn test_ball_bounce() {
        let mut ball = Ball {
            x: 10,
            y: 10,
            dx: 1,
            dy: 1,
        };

        ball.bounce_horizontal();
        assert_eq!(ball.dx, -1);

        ball.bounce_vertical();
        assert_eq!(ball.dy, -1);
    }

    #[test]
    fn test_wall_collision() {
        let mut board = Board::new_static();

        // Test top wall collision
        board.ball.y = 1;
        board.ball.dy = -1;
        assert!(board.check_wall_collision());

        // Test bottom wall collision
        board.ball.y = HEIGHT - 2;
        board.ball.dy = 1;
        assert!(board.check_wall_collision());

        // No collision in middle
        board.ball.y = HEIGHT / 2;
        assert!(!board.check_wall_collision());
    }

    #[test]
    fn test_paddle_collision() {
        let mut board = Board::new_static();

        // Position ball at left paddle
        board.ball.x = board.left.x;
        board.ball.y = board.left.y + 2; // Middle of paddle
        assert!(board.check_paddle_collision());

        // Position ball at right paddle
        board.ball.x = board.right.x;
        board.ball.y = board.right.y + 2;
        assert!(board.check_paddle_collision());

        // Ball misses paddle
        board.ball.x = board.left.x;
        board.ball.y = 0; // Above paddle
        assert!(!board.check_paddle_collision());
    }

    #[test]
    fn test_goal_detection() {
        let mut board = Board::new_static();

        // Test left goal - position ball just before the goal with velocity towards it
        // Paddles are at y=9-13, so position ball above at y=5
        board.ball.x = 1;
        board.ball.dx = -1;
        board.ball.dy = 0; // No vertical movement
        board.ball.y = 5; // Above the paddle range
        board.frame_counter = 1; // Ensure ball moves on next update
        let event = board.update_ball();
        assert_eq!(event, BallEvent::LeftGoal);
        assert_eq!(board.ball.x, WIDTH / 2); // Ball reset to center

        // Test right goal - position ball just before the goal
        board.ball.x = WIDTH - 2;
        board.ball.dx = 1;
        board.ball.dy = 0; // No vertical movement
        board.ball.y = 5; // Above the paddle range
        board.frame_counter = 1; // Ensure ball moves on next update
        let event = board.update_ball();
        assert_eq!(event, BallEvent::RightGoal);
        assert_eq!(board.ball.x, WIDTH / 2);
    }

    #[test]
    fn test_paddle_angle_variation() {
        let _board = Board::new_static();

        // Test each position on a paddle at y=10 (positions 10-14)
        let mut test_board = Board::new_static();
        let paddle = Paddle {
            x: 1,
            y: 10,
            height: 5,
        };

        // Position 0: Top edge (y=10)
        test_board.ball.y = 10;
        let location = test_board.get_paddle_hit_location(&paddle);
        assert_eq!(location, PaddleHitLocation::TopEdge);

        // Position 1: Top mid (y=11)
        test_board.ball.y = 11;
        let location = test_board.get_paddle_hit_location(&paddle);
        assert_eq!(location, PaddleHitLocation::TopMid);

        // Position 2: Center (y=12)
        test_board.ball.y = 12;
        let location = test_board.get_paddle_hit_location(&paddle);
        assert_eq!(location, PaddleHitLocation::Center);

        // Position 3: Bottom mid (y=13)
        test_board.ball.y = 13;
        let location = test_board.get_paddle_hit_location(&paddle);
        assert_eq!(location, PaddleHitLocation::BottomMid);

        // Position 4: Bottom edge (y=14)
        test_board.ball.y = 14;
        let location = test_board.get_paddle_hit_location(&paddle);
        assert_eq!(location, PaddleHitLocation::BottomEdge);
    }

    #[test]
    fn test_ball_speed_control() {
        let mut board = Board::new_static();
        board.ball.x = 10;
        board.ball.dx = 1;
        board.frame_counter = 0;

        // First frame - ball shouldn't move
        let event = board.update_ball();
        assert_eq!(event, BallEvent::None);
        assert_eq!(board.ball.x, 10); // Ball didn't move
        assert_eq!(board.frame_counter, 1);

        // Second frame - ball should move
        let _event = board.update_ball();
        assert_eq!(board.ball.x, 11); // Ball moved
        assert_eq!(board.frame_counter, 2);
    }
}
