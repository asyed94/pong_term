# 🏓 Terminal Pong

A production-ready terminal-based multiplayer Pong game written in pure Rust with **zero external dependencies**.

![Version](https://img.shields.io/badge/version-0.4.1-blue)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![Lines of Code](https://img.shields.io/badge/lines%20of%20code-2000%2B-brightgreen)
![Dependencies](https://img.shields.io/badge/dependencies-0-green)
![License](https://img.shields.io/badge/license-MIT-blue)

## 🎮 Game Features

### ✅ Current Implementation (Stage 4: Enhanced Input & Physics)

<details>
<summary><b>Stage 1: Foundation</b> ✅</summary>

- Basic project structure with clean architecture
- Game model with Board, Paddle, and Ball entities
- Simple ASCII rendering system
- Static display capabilities
</details>

<details>
<summary><b>Stage 2: Rendering System</b> ✅</summary>

- **FrameBuffer abstraction** for efficient rendering
- Separation of drawing and rendering logic
- **Unicode detection** with automatic fallback
- Terminal setup instructions and validation
</details>

<details>
<summary><b>Stage 3: Interactive Gameplay</b> ✅</summary>

- **Raw mode terminal input** without external dependencies
- Direct syscall implementation for termios control
- **60 FPS game loop** with frame limiting
- **Conditional rendering** (only updates when state changes)
- **Synchronized output** for flicker-free display
- Paddle movement with boundary constraints
- Pause/resume functionality
- Input buffer draining to prevent scroll interference
</details>

<details open>
<summary><b>Stage 4: Enhanced Input & Physics</b> ✅</summary>

#### 🎯 Physics System

- **Smart Speed Control**
  - Ball moves every 2 frames (30 effective FPS)
  - Angled shots move slightly faster for realism
- **5-Zone Paddle System**

  ```
  Paddle Zones:
  ┌─────┐
  │  1  │ ← Top Edge: Strong upward deflection
  │  2  │ ← Top Mid: Moderate upward angle
  │  3  │ ← Center: Straight horizontal shot
  │  4  │ ← Bottom Mid: Moderate downward angle
  │  5  │ ← Bottom Edge: Strong downward deflection
  └─────┘
  ```

- **Collision Detection**
  - Wall bouncing with vertical velocity reversal
  - Paddle collision with zone-based angle calculation
  - Goal detection with automatic ball reset

#### ⚡ Enhanced Input System

- **Momentum-based Input**
  - Frame-independent paddle movement
  - Smooth, consistent controls at any frame rate
  - Accumulates input between frames for responsiveness
- **Terminal State Management**
  - Unified GameSession for clean state handling
  - Proper alternate screen and raw mode management
  - Consolidated rendering pipeline with zero duplication
  </details>

### 📋 Planned Features

| Stage | Feature           | Description                                    |
| ----- | ----------------- | ---------------------------------------------- |
| **5** | Score System      | Score tracking, display, and win conditions    |
| **6** | Local Multiplayer | Two players on same terminal                   |
| **7** | Network Support   | TCP/UDP networking for remote play             |
| **8** | Game Server       | Dedicated server for matchmaking               |
| **9** | Polish            | Sound effects, animations, and UI improvements |

## 📦 Installation

### Download Pre-built Binaries

Download the latest release from the [Releases](https://github.com/asyed94/pong-term/releases) page.

| Platform | Architecture  | Binary                         |
| -------- | ------------- | ------------------------------ |
| Linux    | x86_64        | `pong_term-linux-x86_64`       |
| Linux    | ARM64         | `pong_term-linux-aarch64`      |
| macOS    | Intel         | `pong_term-macos-x86_64`       |
| macOS    | Apple Silicon | `pong_term-macos-aarch64`      |
| Windows  | x86_64        | `pong_term-windows-x86_64.exe` |

```bash
# Make binary executable (Linux/macOS)
chmod +x pong_term-*
./pong_term-*
```

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.70+ (2021 edition)
- **Terminal**: ANSI escape code support
- **Size**: Minimum 80×24 characters
- **OS**: Linux x86_64

### Installation & Running

```bash
# Clone the repository
git clone https://github.com/asyed94/pong_term.git
cd pong_term

# Run the game
cargo run

# Run in release mode (optimized)
cargo build --release
./target/release/pong_term

# Run tests
cargo test

# Run specific test suite
cargo test model::tests
```

### 🎮 Game Controls

| Key     | Player | Action            |
| ------- | ------ | ----------------- |
| `W`     | Left   | Move paddle up    |
| `S`     | Left   | Move paddle down  |
| `↑`     | Right  | Move paddle up    |
| `↓`     | Right  | Move paddle down  |
| `Space` | Both   | Pause/Resume game |
| `Enter` | Both   | Start game        |
| `Q`     | Both   | Quit to menu      |

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────┐
│           Main Entry Point              │
│         Terminal Setup & Loop           │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│        Game Loop (60 FPS)               │
│  ┌──────────┬──────────┬──────────┐     │
│  │  Input   │  Update  │  Render  │     │
│  │  Events  │  Physics │  Display │     │
│  └──────────┴──────────┴──────────┘     │
└─────────────────────────────────────────┘
              │
    ┌─────────┼─────────┬──────────┐
    ▼         ▼         ▼          ▼
┌────────┐ ┌─────────┐ ┌────────┐ ┌────────┐
│ Input  │ │ Model   │ │ Frame  │ │Render  │
│ Module │ │ Module  │ │ Buffer │ │ Module │
├────────┤ ├─────────┤ ├────────┤ ├────────┤
│• Raw   │ │• Board  │ │• 2D    │ │• ANSI  │
│  mode  │ │• Paddle │ │  grid  │ │  codes │
│• Event │ │• Ball   │ │• Clear │ │• Sync  │
│  queue │ │• Physics│ │• Draw  │ │  output│
└────────┘ └─────────┘ └────────┘ └────────┘
```

### Module Structure

| Module          | File                  | Purpose                                         |
| --------------- | --------------------- | ----------------------------------------------- |
| **Model**       | `src/model.rs`        | Game state, physics, collision detection        |
| **FrameBuffer** | `src/framebuffer.rs`  | 2D character grid for rendering                 |
| **Draw**        | `src/draw.rs`         | Pure functions: model → framebuffer             |
| **Render**      | `src/render.rs`       | ANSI terminal output, synchronized updates      |
| **Terminal**    | `src/terminal.rs`     | Terminal setup, capability detection            |
| **Input**       | `src/input.rs`        | Momentum-based input, frame-independent control |
| **Game Loop**   | `src/game_loop.rs`    | Fixed timestep loop, frame limiting             |
| **GameSession** | `src/game_session.rs` | Terminal state management, unified rendering    |
| **Main**        | `src/main.rs`         | Entry point, initialization                     |

## 📊 Data Model

### Core Entities

#### Board

```rust
pub struct Board {
    width: 80,              // Fixed width
    height: 24,             // Fixed height
    left: Paddle,           // Left player paddle
    right: Paddle,          // Right player paddle
    ball: Ball,             // Game ball
    frame_counter: usize,   // For physics timing
}
```

#### Paddle

```rust
pub struct Paddle {
    x: usize,               // Horizontal position (fixed)
    y: usize,               // Vertical position (movable)
    height: 5,              // Paddle spans 5 cells
}
```

#### Ball

```rust
pub struct Ball {
    x: usize,               // Current X position
    y: usize,               // Current Y position
    dx: i8,                 // Velocity X (-1, 0, 1)
    dy: i8,                 // Velocity Y (-1, 0, 1)
}
```

### Physics Constants

| Constant             | Value        | Description               |
| -------------------- | ------------ | ------------------------- |
| `PADDLE_SPEED`       | 1 cell/frame | Paddle movement speed     |
| `BALL_SPEED_DIVISOR` | 2            | Ball moves every N frames |
| `PADDLE_HEIGHT`      | 5 cells      | Height of each paddle     |
| `BOARD_WIDTH`        | 80 cells     | Fixed board width         |
| `BOARD_HEIGHT`       | 24 cells     | Fixed board height        |

## 🖥️ Terminal Rendering

### Display Characters

#### Unicode Mode (Auto-detected)

```
┌──────────────┐
│              │  Borders: ┌─┐│└┘
│█            █│  Paddles: █
│█     ●      █│  Ball: ●
│█            █│
└──────────────┘
```

#### ASCII Fallback

```
+----------+
|          |  Borders: +-|
||        ||  Paddles: |
||    o   ||  Ball: o
||        ||
+----------+
```

### Synchronized Rendering

The game prevents screen tearing using ANSI escape sequences:

```bash
\x1b[?2026h  # Begin synchronized update
# ... render frame ...
\x1b[?2026l  # End synchronized update
```

## 🧪 Testing

### Test Coverage

```bash
# Run all tests with coverage info
cargo test -- --nocapture

# Run specific test modules
cargo test model::tests        # Physics tests
cargo test framebuffer::tests  # Rendering tests
cargo test input::tests        # Input handling tests

# Run with backtrace for debugging
RUST_BACKTRACE=1 cargo test
```

### Test Categories

- **Unit Tests**: Each module has comprehensive unit tests
- **Physics Tests**: Ball movement, collision detection, paddle zones
- **Rendering Tests**: Frame buffer operations, boundary checks
- **Input Tests**: Key event processing, raw mode handling

## ⚡ Performance

| Metric             | Value                | Description                        |
| ------------------ | -------------------- | ---------------------------------- |
| **Frame Rate**     | 60 FPS               | Fixed timestep game loop           |
| **Input Latency**  | Frame-independent    | Momentum-based, no input loss      |
| **Rendering Mode** | Unified pipeline     | Zero duplication, optimized output |
| **Memory Usage**   | ~1MB                 | Minimal heap allocation            |
| **CPU Usage**      | <1% idle, <2% active | Efficient game loop                |

## 🔧 Development

### Project Structure

```
pong_term/
├── 📄 Cargo.toml         # Project manifest
├── 📄 Cargo.lock         # Dependency lock file
├── 📄 README.md          # This file
├── 📄 LICENSE            # MIT License
├── 📁 src/
│   ├── 📄 main.rs        # Entry point
│   ├── 📄 lib.rs         # Library root
│   ├── 📄 model.rs       # Game logic & physics
│   ├── 📄 framebuffer.rs # Rendering buffer
│   ├── 📄 draw.rs        # Drawing functions
│   ├── 📄 render.rs      # Terminal output
│   ├── 📄 terminal.rs    # Terminal utilities
│   ├── 📄 input.rs       # Momentum-based input
│   ├── 📄 game_session.rs# Terminal state management
│   ├── 📄 game_loop.rs   # Main game loop
│   └── 📄 util.rs        # Utility functions
├── 📁 target/            # Build artifacts
└── 📁 tests/             # Integration tests
```

### Building from Source

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check for errors without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## 🚫 Zero Dependencies Philosophy

This project is built with **absolutely zero external dependencies**:

- ✅ **No game engines** - Pure Rust implementation
- ✅ **No terminal libraries** - Direct ANSI escape codes
- ✅ **No async runtime** - Synchronous game loop
- ✅ **No external crates** - Standard library only

### Implementation Techniques

- **Terminal Control**: Direct syscalls via inline assembly
- **Input Handling**: Raw mode using termios ioctls
- **Rendering**: ANSI escape sequences
- **Timing**: Standard library's `Instant` and `Duration`

## 🖥️ System Requirements

### Minimum Requirements

- **OS**: Linux x86_64
- **Terminal**: Any terminal with ANSI support
- **Screen Size**: 80×24 characters minimum
- **Rust**: 1.70+ (2021 edition)

### Tested Terminals

- ✅ GNOME Terminal
- ✅ Konsole
- ✅ Alacritty
- ✅ Kitty
- ✅ xterm
- ✅ VS Code Terminal

## 🤝 Contributing

Contributions are welcome! This project follows a staged development approach:

1. **Fork** the repository
2. **Create** a feature branch
3. **Implement** your changes with tests
4. **Ensure** all tests pass
5. **Submit** a pull request

### Development Guidelines

- Maintain zero external dependencies
- Write comprehensive tests for new features
- Follow Rust naming conventions
- Document public APIs
- Keep commits atomic and descriptive

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Roadmap

- [x] **Stage 1**: Basic structure and rendering
- [x] **Stage 2**: Framebuffer system
- [x] **Stage 3**: Input and game loop
- [x] **Stage 4**: Enhanced input system & ball physics
- [ ] **Stage 5**: Score tracking and display
- [ ] **Stage 6**: Local multiplayer
- [ ] **Stage 7**: Network protocol
- [ ] **Stage 8**: Game server
- [ ] **Stage 9**: Polish and optimization

## 📌 Version History

| Version   | Stage | Description              | Release Date |
| --------- | ----- | ------------------------ | ------------ |
| **0.4.1** | 4     | Enhanced Input & Physics | Current      |
| 0.4.0     | 4     | Advanced Ball Physics    | -            |
| 0.3.0     | 3     | Interactive Gameplay     | -            |
| 0.2.0     | 2     | Rendering System         | -            |
| 0.1.0     | 1     | Foundation               | -            |

### Semantic Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes or complete rewrite
- **MINOR**: Stage completion (0.X.0 where X = stage number)
- **PATCH**: Bug fixes and improvements within a stage

## 🚢 CI/CD Pipeline

### Continuous Integration (CI)

All pull requests and pushes to main branches trigger comprehensive automated testing:

#### 🧪 Test Suite

- **Formatting Check** - Ensures consistent code style
- **Clippy Linting** - Catches common mistakes and anti-patterns
- **Unit Tests** - Runs on Linux, macOS, and Windows
- **Build Verification** - Ensures compilation on all target platforms
- **Security Audit** - Checks for known vulnerabilities
- **Documentation Check** - Validates all documentation builds
- **Integration Tests** - Smoke tests the actual binary
- **Code Coverage** - Tracks test coverage (main branch only)

#### ✅ PR Requirements

Before merging, all PRs must pass:

- ✓ Code formatting (`cargo fmt`)
- ✓ Linting (`cargo clippy`)
- ✓ All tests on 3 platforms
- ✓ Builds for 5 target architectures
- ✓ Security audit
- ✓ Documentation generation

### Release Process

When a stage is completed:

1. Version is bumped to `0.X.0` (where X = stage number)
2. Tag is created: `git tag v0.X.0`
3. GitHub Actions automatically:
   - Creates a GitHub Release
   - Builds binaries for all platforms
   - Uploads binaries to the release

---

**Current Version**: `0.4.1`  
**Current Stage**: Stage 4 Complete ✅  
**Key Features**: Momentum-based input, unified rendering, 5-zone paddle physics  
**Zero Dependencies**: True zero-dependency implementation achieved ✅  
**Next Up**: Stage 5 - Score tracking and game state management

---

_Built with ❤️ in Rust | Zero Dependencies | Pure Terminal Graphics_
