//! Utility functions for the game.

/// Calculate the display width of a character.
/// For our game, all characters we use are single-width:
/// - ASCII characters
/// - Box-drawing characters (│, ─, ┌, ┐, └, ┘, etc.)
/// - Game symbols (●, █, ↑, ↓)
pub fn char_width(_ch: char) -> usize {
    // All characters we use in the game have display width of 1
    // This includes ASCII, box-drawing characters, and our game symbols
    1
}

/// Calculate the display width of a string.
/// Simply the sum of all character widths.
pub fn str_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_widths() {
        // ASCII
        assert_eq!(char_width('a'), 1);
        assert_eq!(char_width('Z'), 1);
        assert_eq!(char_width(' '), 1);
        assert_eq!(char_width('!'), 1);

        // Box drawing
        assert_eq!(char_width('│'), 1);
        assert_eq!(char_width('─'), 1);
        assert_eq!(char_width('┌'), 1);
        assert_eq!(char_width('╔'), 1);

        // Game symbols
        assert_eq!(char_width('●'), 1);
        assert_eq!(char_width('█'), 1);
        assert_eq!(char_width('↑'), 1);
        assert_eq!(char_width('↓'), 1);
    }

    #[test]
    fn test_str_widths() {
        assert_eq!(str_width("Hello"), 5);
        assert_eq!(str_width("GAME PAUSED"), 11);
        assert_eq!(str_width("↑/↓"), 3);
        assert_eq!(str_width("W/S"), 3);
        assert_eq!(str_width("╔═══╗"), 5);
        assert_eq!(str_width("│●█│"), 4);
    }
}
