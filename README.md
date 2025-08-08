# 🎮 Terminal Pong

A production-ready, terminal-based Pong game built in Rust with **zero external dependencies**. Features network multiplayer, automatic terminal capability detection, and clean architecture built incrementally through testable stages.

![Status](https://img.shields.io/badge/Stage-2%20of%208-blue)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?logo=rust)
![Tests](https://img.shields.io/badge/tests-passing-green)

## ✨ Features

- **No External Dependencies** - Pure Rust using only the standard library
- **Terminal Setup Guide** - Interactive prompts to ensure proper display
- **Enhanced Visuals** - Uses Unicode box-drawing characters when available
- **Production-Ready** - Comprehensive tests, modular architecture, continuous documentation
- **Network Multiplayer** (upcoming) - TCP-based authoritative server architecture

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/asyed94/battle-of-the-rustaceans.git
cd pong_term

# Build and run
cargo run

# Run tests
cargo test

# Force Unicode rendering (if not auto-detected)
LANG=en_US.UTF-8 cargo run
```

**Requirements:**

- Rust toolchain (stable)
- Terminal with ANSI escape sequence support
- Terminal size: 80×24 or larger

## 📋 Development Roadmap

| Stage | Status      | Description                                                    |
| ----- | ----------- | -------------------------------------------------------------- |
| 1     | ✅ Complete | Initial setup, static screen renderer, tests, docs             |
| 2     | ✅ Complete | Framebuffer abstraction, Unicode detection, enhanced visuals   |
| 3     | 🔲 TODO     | Evented terminal input (non-blocking), minimal game loop       |
| 4     | 🔲 TODO     | Local gameplay (ball physics, paddle movement, scoring)        |
| 5     | 🔲 TODO     | Deterministic tick loop, time-step handling, pause/reset       |
| 6     | 🔲 TODO     | Networking foundations (std::net TCP): server/client handshake |
| 7     | 🔲 TODO     | Networked gameplay (authoritative server), reconciliation      |
| 8     | 🔲 TODO     | Polish: error handling, resize support, CI, expanded docs      |

## 🏗️ Architecture

### System Overview

```
┌──────────┐     ┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌────────┐
│   main   │────>│  model   │────>│     draw     │────>│ framebuffer  │────>│ render │
└──────────┘     └──────────┘     └──────────────┘     └──────────────┘     └────────┘
     │                │                   │                    │                  │
     v                v                   v                    v                  v
 Terminal      Board::new_static()  draw_board()        FrameBuffer        ANSI+stdout
 Setup Check   Game State           draw_border()       .set()/.get()      Clear/Home
               Paddle/Ball          draw_paddle()     .to_string_lines()   Unicode/ASCII
                                   draw_ball()
```

### Module Structure

| Module        | Purpose                  | Key Components                        |
| ------------- | ------------------------ | ------------------------------------- |
| `model`       | Game state and entities  | `Board`, `Paddle`, `Ball`, constants  |
| `framebuffer` | 2D character buffer      | Safe cell access with bounds checking |
| `draw`        | Pure rendering functions | Entity → framebuffer transformations  |
| `render`      | Terminal I/O             | ANSI codes, stdout management         |
| `terminal`    | Terminal utilities       | Setup instructions, Unicode detection |

## 📊 Data Model

### Core Types

```rust
// Game Constants
const WIDTH: usize = 80;
const HEIGHT: usize = 24;
const PADDLE_HEIGHT: usize = 5;

// Entity Structures
struct Paddle { x: usize, y: usize, height: usize }
struct Ball { x: usize, y: usize }
struct Board {
    width: usize,
    height: usize,
    left: Paddle,
    right: Paddle,
    ball: Ball
}

// Rendering
struct FrameBuffer {
    width: usize,
    height: usize,
    cells: Vec<char>
}

// Style
struct RenderStyle {
    border_horizontal: char,
    border_vertical: char,
    border_corner_tl: char,
    border_corner_tr: char,
    border_corner_bl: char,
    border_corner_br: char,
    paddle: char,
    ball: char
}
```

## 🎨 Visual Styles

The game automatically detects terminal capabilities and chooses the best visual style:

### Unicode Style (UTF-8 terminals)

```
┌──────────────────────┐
│                      │
│ █         ●         █│
│ █                   █│
│ █                   █│
└──────────────────────┘
```

### ASCII Style (fallback)

```
+----------------------+
|                      |
| |         o         ||
| |                   ||
| |                   ||
+----------------------+
```

## 🔧 API Reference

### Public Functions

```rust
// Rendering
render_to_string(&Board) -> String       // Generate frame with ANSI codes
render_and_print(&Board) -> io::Result   // Render directly to stdout

// Terminal Utilities
print_setup_instructions(width, height) -> io::Result<()>
RenderStyle::auto() -> RenderStyle       // Auto-detect best style

// Game State
Board::new_static() -> Board             // Create static game board
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test border_has_expected
```

### Test Coverage

- **FrameBuffer**: Bounds checking, set/get operations, string conversion
- **Drawing**: Border rendering, entity placement, style application
- **Rendering**: ANSI code generation, frame structure validation

## 📁 Project Structure

```
pong_term/
├── src/
│   ├── main.rs         # Entry point with setup instructions
│   ├── lib.rs          # Module exports and documentation
│   ├── model.rs        # Game state and entities
│   ├── framebuffer.rs  # 2D character buffer
│   ├── draw.rs         # Pure drawing functions
│   ├── render.rs       # ANSI terminal output
│   └── terminal.rs     # Terminal utilities
├── Cargo.toml          # Project metadata (no dependencies!)
├── README.md           # This file
└── LICENSE             # License information
```

## 🌐 Future: Network Architecture

### Planned Design

```
     Client A                  Server                  Client B
        │                        │                        │
        ├──Input──────────>      │      <──────Input──────┤
        │                        │                        │
        │                   Authoritative                 │
        │                    Game State                   │
        │                        │                        │
        │<──State Broadcast──────┼──────State Broadcast──>│
        │                        │                        │
     Render                   Tick Loop                Render
```

**Protocol**: TCP with custom binary messages  
**Architecture**: Authoritative server with client prediction  
**Tick Rate**: Fixed 60Hz server tick with interpolation

## 💡 Design Philosophy

### Core Principles

1. **Simplicity First** - Clear code over clever optimizations
2. **No External Dependencies** - Maximum portability and learning value
3. **Incremental Development** - Each stage is complete and testable
4. **Production Quality** - Tests, docs, and error handling from the start

### Key Decisions (Stage 2)

| Decision                   | Rationale                         |
| -------------------------- | --------------------------------- |
| Vec<char> for framebuffer  | Clarity over micro-optimization   |
| Bounds clipping (no panic) | Robustness in edge cases          |
| Free functions over traits | Minimal API surface               |
| Manual terminal adjustment | Simplicity over complex detection |
| Unicode auto-detection     | Better UX when supported          |

## 🤝 Contributing

This project is built incrementally with careful review at each stage. When contributing:

1. Maintain the no-external-dependency constraint
2. Include tests for new functionality
3. Update documentation as needed
4. Follow the existing code style

## 📄 License

See [LICENSE](LICENSE) file for details.

---

_Built with ❤️ in Rust - Learning by doing, one stage at a time_
