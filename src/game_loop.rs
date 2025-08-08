//! Game loop: input → update → render at fixed frame rate.
//! Stage 4: Added ball physics updates.

use crate::input::{poll_input, InputEvent, Terminal};
use crate::model::Board;
use crate::render::{render_pause_menu, render_synchronized, show_cursor};
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
pub fn run_game_loop() -> io::Result<()> {
    // Initialize terminal in raw mode
    let _terminal = Terminal::enter_raw_mode()?;

    // Initialize game board with moving ball
    let mut board = Board::new_game();
    let mut state = GameState::Paused;
    let mut last_render_state = GameState::Running;
    let mut last_rendered_board = board.clone(); // Track last rendered board for conditional rendering

    // Initial render
    render_synchronized(&board)?;

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
                    render_synchronized(&board)?;
                    last_rendered_board = board.clone();
                    last_render_state = GameState::Running;
                }
            }
            GameState::Paused => {
                // Only render pause menu when first paused
                if last_render_state != GameState::Paused {
                    render_pause_menu(&board)?;
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

    // Cleanup: show cursor before exiting
    show_cursor()?;

    Ok(())
}

/// Process input and update game state.
fn handle_input(input: InputEvent, board: &mut Board, state: &mut GameState) {
    match input {
        InputEvent::Quit => {
            *state = GameState::Quit;
        }
        InputEvent::Pause => {
            *state = match *state {
                GameState::Running => GameState::Paused,
                GameState::Paused => GameState::Running,
                GameState::Quit => GameState::Quit,
            };
        }
        _ if *state == GameState::Running => {
            // Only process movement when not paused
            match input {
                InputEvent::LeftPaddleUp => board.move_left_paddle_up(),
                InputEvent::LeftPaddleDown => board.move_left_paddle_down(),
                InputEvent::RightPaddleUp => board.move_right_paddle_up(),
                InputEvent::RightPaddleDown => board.move_right_paddle_down(),
                _ => {}
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InputEvent;
    use crate::model::Board;

    #[test]
    fn test_input_handling() {
        let mut board = Board::new_static();
        let mut state = GameState::Running;

        let initial_left_y = board.left.y;
        let initial_right_y = board.right.y;

        // Test left paddle movement
        handle_input(InputEvent::LeftPaddleUp, &mut board, &mut state);
        assert!(board.left.y < initial_left_y);

        // Test right paddle movement
        handle_input(InputEvent::RightPaddleDown, &mut board, &mut state);
        assert!(board.right.y > initial_right_y);

        // Test pause
        handle_input(InputEvent::Pause, &mut board, &mut state);
        assert_eq!(state, GameState::Paused);

        // Movement should not work when paused
        let paused_left_y = board.left.y;
        handle_input(InputEvent::LeftPaddleDown, &mut board, &mut state);
        assert_eq!(board.left.y, paused_left_y);

        // Unpause
        handle_input(InputEvent::Pause, &mut board, &mut state);
        assert_eq!(state, GameState::Running);

        // Test quit
        handle_input(InputEvent::Quit, &mut board, &mut state);
        assert_eq!(state, GameState::Quit);
    }
}
