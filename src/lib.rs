//! Terminal Pong - Stage 2: framebuffer abstraction (no external dependencies).
//!
//! Crate layout:
//! - model: data structures and fixed static layout
//! - framebuffer: 2D character buffer for building frames
//! - draw: pure drawing functions (model -> framebuffer)
//! - render: ANSI terminal output
//! - terminal: terminal utilities and capability detection

pub mod draw;
pub mod framebuffer;
pub mod model;
pub mod render;
pub mod terminal;

pub use framebuffer::FrameBuffer;
pub use model::{Ball, Board, Paddle, HEIGHT, PADDLE_HEIGHT, WIDTH};
pub use render::{render_and_print, render_to_string, render_with_message_and_print};
pub use terminal::{print_setup_instructions, RenderStyle};
