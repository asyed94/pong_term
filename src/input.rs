//! Cross-platform terminal input handling using crossterm with momentum-based movement.
//! This avoids key repeat delay issues by implementing movement momentum.

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::cell::RefCell;
use std::io;
use std::time::Duration;

/// Maximum momentum frames per key press.
/// This determines how long paddles continue moving after a key press.
/// Higher values = longer movement continuation.
const MAX_MOMENTUM: u8 = 5;

/// Input state containing all active inputs for this frame.
/// Uses momentum to handle smooth movement without key repeat delays.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct InputState {
    pub left_up: bool,
    pub left_down: bool,
    pub right_up: bool,
    pub right_down: bool,
    pub quit: bool,
    pub pause: bool,
}

impl InputState {
    /// Create a new empty input state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any input is active.
    pub fn has_input(&self) -> bool {
        self.left_up
            || self.left_down
            || self.right_up
            || self.right_down
            || self.quit
            || self.pause
    }
}

/// Momentum tracker for smooth paddle movement.
/// When a key is pressed, momentum is set and gradually decreases.
struct MomentumTracker {
    left_up_momentum: u8,
    left_down_momentum: u8,
    right_up_momentum: u8,
    right_down_momentum: u8,
}

impl MomentumTracker {
    fn new() -> Self {
        Self {
            left_up_momentum: 0,
            left_down_momentum: 0,
            right_up_momentum: 0,
            right_down_momentum: 0,
        }
    }

    /// Add momentum when a key is pressed.
    /// This resets the momentum to max value (smooth continuous movement).
    fn add_momentum(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.left_up_momentum = MAX_MOMENTUM;
                self.left_down_momentum = 0; // Cancel opposite direction
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.left_down_momentum = MAX_MOMENTUM;
                self.left_up_momentum = 0; // Cancel opposite direction
            }
            KeyCode::Up => {
                self.right_up_momentum = MAX_MOMENTUM;
                self.right_down_momentum = 0; // Cancel opposite direction
            }
            KeyCode::Down => {
                self.right_down_momentum = MAX_MOMENTUM;
                self.right_up_momentum = 0; // Cancel opposite direction
            }
            _ => {}
        }
    }

    /// Decay momentum over time and return current input state.
    fn get_state(&mut self) -> (bool, bool, bool, bool) {
        // Decay momentum by 1 each frame
        if self.left_up_momentum > 0 {
            self.left_up_momentum -= 1;
        }
        if self.left_down_momentum > 0 {
            self.left_down_momentum -= 1;
        }
        if self.right_up_momentum > 0 {
            self.right_up_momentum -= 1;
        }
        if self.right_down_momentum > 0 {
            self.right_down_momentum -= 1;
        }

        (
            self.left_up_momentum > 0,
            self.left_down_momentum > 0,
            self.right_up_momentum > 0,
            self.right_down_momentum > 0,
        )
    }
}

// Thread-local momentum tracker for safe access
thread_local! {
    static MOMENTUM: RefCell<MomentumTracker> = RefCell::new(MomentumTracker::new());
    static LAST_PAUSE_STATE: RefCell<bool> = RefCell::new(false);
}

/// Initialize the momentum tracker for smooth input handling.
/// This resets the momentum tracker to initial state.
pub fn init_momentum() {
    MOMENTUM.with(|m| {
        *m.borrow_mut() = MomentumTracker::new();
    });
    LAST_PAUSE_STATE.with(|p| {
        *p.borrow_mut() = false;
    });
}

/// Wait for the user to press Enter without echoing any typed characters.
/// This should only be called from within a GameSession where raw mode is already active.
pub fn wait_for_enter_no_echo() -> io::Result<()> {
    // Wait for Enter key (raw mode is already active in GameSession)
    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.code == KeyCode::Enter {
                break;
            }
        }
    }
    Ok(())
}

/// Poll for keyboard input using crossterm with momentum tracking.
/// This provides smooth movement without key repeat delays.
pub fn poll_input() -> io::Result<InputState> {
    // Process all pending key events
    let mut pause_pressed = false;
    let mut quit_pressed = false;
    let mut space_key_seen = false;

    // Poll for events with zero timeout (non-blocking)
    while event::poll(Duration::ZERO)? {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char(' ') => {
                    space_key_seen = true;
                    // Only trigger pause on key press, not hold
                    LAST_PAUSE_STATE.with(|state| {
                        let mut last_state = state.borrow_mut();
                        if !*last_state {
                            pause_pressed = true;
                            *last_state = true;
                        }
                    });
                }
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    quit_pressed = true;
                }
                // Movement keys add momentum
                KeyCode::Char('w')
                | KeyCode::Char('W')
                | KeyCode::Char('s')
                | KeyCode::Char('S')
                | KeyCode::Up
                | KeyCode::Down => {
                    MOMENTUM.with(|m| {
                        m.borrow_mut().add_momentum(code);
                    });
                }
                _ => {}
            }
        }
    }

    // Reset pause state if no space key events received
    if !space_key_seen {
        LAST_PAUSE_STATE.with(|state| {
            *state.borrow_mut() = false;
        });
    }

    // Get current movement state from momentum
    let (left_up, left_down, right_up, right_down) = MOMENTUM.with(|m| m.borrow_mut().get_state());

    Ok(InputState {
        left_up,
        left_down,
        right_up,
        right_down,
        quit: quit_pressed,
        pause: pause_pressed,
    })
}
