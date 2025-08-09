//! Game loop: input → update → render at fixed frame rate.
//! Stage 4: Added ball physics updates.

use crate::game_session::GameSession;
use crate::input::{poll_input, InputState};
use crate::model::Board;
use std::io;
use std::thread;
use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS as u64);

/// Game state for managing pause functionality.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Running,
    Paused,
    Quit,
}

/// Run the main game loop with 60 FPS and conditional rendering.
/// Returns Ok(()) on clean exit, or an error if something went wrong.
pub fn run_game_loop(session: &GameSession) -> io::Result<()> {
    // Initialize game board with moving ball
    let mut board = Board::new_game();
    let mut state = GameState::Paused;
    let mut last_render_state = GameState::Running;
    let mut last_rendered_board = board.clone(); // Track last rendered board for conditional rendering

    // Initial render
    session.render_board(&board)?;

    // Game loop
    let mut _last_frame = Instant::now();

    while state != GameState::Quit {
        let frame_start = Instant::now();

        // Input phase
        let input = poll_input()?;
        handle_input(input, &mut board, &mut state);

        // Update phase - ball physics when game is running
        if state == GameState::Running {
            let _ball_event = board.update_ball();
            // We can use ball_event later for sounds/effects
        }

        // Render phase - only render when something actually changed
        match state {
            GameState::Running => {
                // Only render if board changed or we're coming from pause
                if board != last_rendered_board || last_render_state != GameState::Running {
                    session.render_board(&board)?;
                    last_rendered_board = board.clone();
                    last_render_state = GameState::Running;
                }
            }
            GameState::Paused => {
                // Only render pause menu when first paused
                if last_render_state != GameState::Paused {
                    session.render_pause_menu(&board)?;
                    last_render_state = GameState::Paused;
                }
            }
            GameState::Quit => {}
        }

        // Frame rate limiting
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }

        _last_frame = frame_start;
    }

    // Cleanup handled by GameSession Drop

    Ok(())
}

/// Process input and update game state.
fn handle_input(input: InputState, board: &mut Board, state: &mut GameState) {
    // Check for quit first (highest priority)
    if input.quit {
        *state = GameState::Quit;
        return;
    }

    // Check for pause toggle
    if input.pause {
        *state = match *state {
            GameState::Running => GameState::Paused,
            GameState::Paused => GameState::Running,
            GameState::Quit => GameState::Quit,
        };
    }

    // Process movement only when running
    if *state == GameState::Running {
        // Process all active inputs in this frame
        // Note: if both up and down are pressed, they cancel out (no movement)
        if input.left_up && !input.left_down {
            board.move_left_paddle_up();
        } else if input.left_down && !input.left_up {
            board.move_left_paddle_down();
        }

        if input.right_up && !input.right_down {
            board.move_right_paddle_up();
        } else if input.right_down && !input.right_up {
            board.move_right_paddle_down();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Board;

    #[test]
    fn test_input_handling() {
        let mut board = Board::new_static();
        let mut state = GameState::Running;

        let initial_left_y = board.left.y;
        let initial_right_y = board.right.y;

        // Test left paddle movement
        let mut input = InputState::new();
        input.left_up = true;
        handle_input(input, &mut board, &mut state);
        assert!(board.left.y < initial_left_y);

        // Test right paddle movement
        let mut input = InputState::new();
        input.right_down = true;
        handle_input(input, &mut board, &mut state);
        assert!(board.right.y > initial_right_y);

        // Test pause
        let mut input = InputState::new();
        input.pause = true;
        handle_input(input, &mut board, &mut state);
        assert_eq!(state, GameState::Paused);

        // Movement should not work when paused
        let paused_left_y = board.left.y;
        let mut input = InputState::new();
        input.left_down = true;
        handle_input(input, &mut board, &mut state);
        assert_eq!(board.left.y, paused_left_y);

        // Unpause
        let mut input = InputState::new();
        input.pause = true;
        handle_input(input, &mut board, &mut state);
        assert_eq!(state, GameState::Running);

        // Test quit
        let mut input = InputState::new();
        input.quit = true;
        handle_input(input, &mut board, &mut state);
        assert_eq!(state, GameState::Quit);
    }
}
