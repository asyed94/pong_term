# Terminal Pong (Rust) — Stage 1

Terminal-based Pong rendered with ANSI escape sequences. This project is built step-by-step in small, testable stages, with no external dependencies. Stage 1 renders a single static Pong frame to the terminal.

- Simplicity first: favor clear code and small interfaces over complexity.
- Production-oriented: tests, docs, modular structure from the start.

## Roadmap (Incremental Stages)

Each stage is standalone and testable and will be committed separately.

1. Stage 1 (current): Initial setup, static screen renderer, tests, docs.
2. Stage 2: Framebuffer abstraction + basic layout engine tests (still static).
3. Stage 3: Evented terminal input (non-blocking key read), minimal game loop (single-player stub).
4. Stage 4: Local gameplay (ball physics, paddle movement, scoring).
5. Stage 5: Deterministic tick loop, time-step handling, pause/reset.
6. Stage 6: Networking foundations (std::net): server/client handshake, simple state sync.
7. Stage 7: Networked gameplay (authoritative server), message protocol, reconciliation.
8. Stage 8: Polishing: robust error handling, window resize strategy, CI suggestions, docs expansion.

We can adjust based on feedback as we progress.

## How to run

Requirements: Rust toolchain (stable), a terminal capable of ANSI escape sequences.

- Build & run:

  - cargo run
  - The program clears the screen, draws a static frame, then waits for Enter to exit.

- Recommended terminal size: 80x24. If your terminal is larger/smaller, the frame will still render as an 80x24 scene.

## How to test

- Run all tests:
  - cargo test

Stage 1 includes unit tests for border rendering and entity placement.

## Architecture

Modules:

- model: Board/Paddle/Ball and constants (fixed 80x24 layout in this stage).
- render: Converts a Board into a string frame and prints with ANSI clear/home.

High-level flow:
[main] -> [Board::new_static()] -> [render::render_and_print(board)]
-> [render::render_to_string(board)]
-> build_plain_frame (no ANSI)
-> prepend ANSI clear/home

Responsibilities:

- model: pure data; easy to construct and test.
- render: pure transformation (Board -> text); minimal I/O isolation.

## Data Model (Stage 1)

Constants:

- WIDTH: usize = 80
- HEIGHT: usize = 24
- PADDLE_HEIGHT: usize = 5

Structs:

- Paddle { x: usize, y: usize, height: usize }
  - Drawn as vertical '|' glyphs at column x from rows [y, y + height).
- Ball { x: usize, y: usize }
  - Drawn as 'o'.
- Board { width, height, left: Paddle, right: Paddle, ball: Ball }
  - new_static(): creates a centered ball and paddles inset from borders.

## Public API (Stage 1)

Crate exports (re-exported by lib.rs):

- model:
  - WIDTH, HEIGHT, PADDLE_HEIGHT
  - Paddle, Ball, Board (with Board::new_static())
- render:
  - render_to_string(&Board) -> String
  - render_and_print(&Board) -> io::Result<()>

Usage example (src/main.rs):

- let board = Board::new_static();
- render_and_print(&board)?;

## Rendering Details

- ANSI codes:
  - \x1b[2J: clear screen
  - \x1b[H: move cursor to home (1,1)
- Frame content (lines):
  - Top/bottom borders: '+' at corners, '-' across the horizontal.
  - Left/right borders: '|' vertical.
  - Paddles: '|' columns at fixed x for left/right, centered vertically.
  - Ball: 'o' at center.

The renderer builds a 2D char grid then converts to a string with newlines.

## Design Decisions (Stage 1)

- No external libraries: ensures maximal portability and clarity.
- Fixed geometry constants: simplifies initial testing and behavior.
- Separation of model and renderer: model is pure data; renderer is pure transform, easing testability.
- Tests embedded in render module for border/paddle/ball verification.

Alternatives considered:

- Dynamic terminal size via ioctl/termios: deferred to keep Stage 1 minimal.
- Full terminal control (raw mode/cursor hide): postponed until input/game loop stages.

## Development

- Code style: idiomatic Rust, small pure functions, explicit boundaries between I/O and logic.
- Testing philosophy: unit tests on deterministic parts (frame generation).
- Running locally:
  - cargo check, cargo test, cargo run
- No external deps; standard library only.

## Project Structure

- src/
  - main.rs: binary entry point; builds static board and prints frame.
  - lib.rs: module wiring and re-exports.
  - model.rs: data model and Board::new_static.
  - render.rs: ANSI rendering and tests.
- Cargo.toml: crate metadata.
- LICENSE: license file.

## Future: Networking (Outline)

- Use std::net (TCP) for client/server.
- Minimal protocol: input messages from client; periodic authoritative state from server.
- Deterministic server tick; clients render latest known state with simple smoothing (if needed).

## License

See LICENSE for details.
