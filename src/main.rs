use pong_term::{
    enter_alternate_screen, exit_alternate_screen, print_setup_instructions,
    render_with_message_to_string, run_game_loop, wait_for_enter_no_echo, Board, HEIGHT, WIDTH,
};
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    // Print setup instructions (in normal screen)
    print_setup_instructions(WIDTH, HEIGHT)?;

    // Enter alternate screen for the game
    enter_alternate_screen()?;

    // Display the static board with controls inside
    let board = Board::new_static();

    // Render the board with controls message (shorter to fit within borders)
    let rendered = render_with_message_to_string(
        &board,
        "W/S: Left | ↑/↓: Right | Space: Pause | Enter: Start",
    );

    // Print the rendered board
    print!("{}", rendered);
    io::stdout().flush()?;

    // Wait for user to press Enter to start the game (no echo)
    wait_for_enter_no_echo()?;

    // Clear screen before starting game
    print!("\x1b[2J\x1b[H");
    io::stdout().flush()?;

    // Run the game loop (will handle its own cleanup)
    let game_result = run_game_loop();

    // Exit alternate screen (restores original terminal content)
    exit_alternate_screen()?;

    // Show exit message in the normal screen
    println!("Thanks for playing Terminal Pong!");

    // Return the game result
    game_result
}
