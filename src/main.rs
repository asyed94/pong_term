use pong_term::{print_setup_instructions, render_with_message_and_print, Board, HEIGHT, WIDTH};

fn main() -> std::io::Result<()> {
    // Print setup instructions
    print_setup_instructions(WIDTH, HEIGHT)?;

    // Stage 2: static board, single frame with enhanced visuals
    // Display the board with the exit message inside it
    let board = Board::new_static();
    render_with_message_and_print(&board, "Press Enter to exit...")?;

    // Wait for user to press Enter without printing anything
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    Ok(())
}
