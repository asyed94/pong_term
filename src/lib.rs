//! Terminal Pong - Stage 3: evented terminal input and game loop.
//!
//! Crate layout:
//! - model: data structures with mutable paddle movement
//! - framebuffer: 2D character buffer for building frames
//! - draw: pure drawing functions (model -> framebuffer)
//! - render: ANSI terminal output with synchronized updates
//! - terminal: terminal utilities and capability detection
//! - input: raw mode terminal input handling
//! - game_loop: main game loop with fixed frame rate
//! - game_session: unified terminal session management
//! - util: utility functions (char/string width calculations)

pub mod draw;
pub mod framebuffer;
pub mod game_loop;
pub mod game_session;
pub mod input;
pub mod model;
pub mod render;
pub mod terminal;
pub mod util;

pub use draw::draw_board_with_message;
pub use framebuffer::FrameBuffer;
pub use game_loop::run_game_loop;
pub use game_session::GameSession;
pub use input::{wait_for_enter_no_echo, InputState};
pub use model::{Ball, BallEvent, Board, Paddle, HEIGHT, PADDLE_HEIGHT, WIDTH};
pub use render::{render_to_string, render_with_message_to_string};
pub use terminal::{print_setup_instructions, RenderStyle};
