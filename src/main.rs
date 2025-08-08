use pong_term::{render_and_print, Board};

fn main() -> std::io::Result<()> {
    // Stage 1: static board, single frame
    let board = Board::new_static();
    render_and_print(&board)?;

    // Simple pause so the frame can be viewed.
    println!("\nPress Enter to exit...");
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    Ok(())
}
