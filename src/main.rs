use pong_term::{print_setup_instructions, run_game_loop, Board, GameSession, HEIGHT, WIDTH};

fn main() -> std::io::Result<()> {
    // Print setup instructions (in normal screen)
    print_setup_instructions(WIDTH, HEIGHT)?;

    // Enter game session (alternate screen + raw mode + hide cursor)
    let session = GameSession::enter()?;

    // Display the static board with controls inside
    let board = Board::new_static();

    // Render the board with controls message (shorter to fit within borders)
    session.render_board_with_message(
        &board,
        "W/S: Left | ↑/↓: Right | Space: Pause | Enter: Start",
    )?;

    // Wait for user to press Enter to start the game
    session.wait_for_enter()?;

    // Clear screen before starting game
    session.clear_screen()?;

    // Run the game loop (will handle its own cleanup)
    let game_result = run_game_loop(&session);

    // GameSession Drop will handle cleanup (exit alternate screen, restore terminal)
    drop(session);

    // Show exit message in the normal screen
    println!("Thanks for playing Terminal Pong!");

    // Return the game result
    game_result
}
