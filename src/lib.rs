//! Terminal Pong - Stage 1: static screen (no external dependencies).
//!
//! Crate layout:
//! - model: data structures and fixed static layout
//! - render: ANSI renderer (to string or stdout)

pub mod model;
pub mod render;

pub use model::{Ball, Board, Paddle, HEIGHT, PADDLE_HEIGHT, WIDTH};
pub use render::{render_and_print, render_to_string};
