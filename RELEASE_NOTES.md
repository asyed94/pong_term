# Terminal Pong v0.4.1 - Release Notes

**Stage 4: Enhanced Input & Physics** ✅

## What's New

### 🎯 5-Zone Paddle Physics

Paddles now feature 5 distinct collision zones that affect ball trajectory:

- **Zone 1 (Top Edge)**: Strong upward deflection
- **Zone 2 (Top Mid)**: Moderate upward angle
- **Zone 3 (Center)**: Straight horizontal shot
- **Zone 4 (Bottom Mid)**: Moderate downward angle
- **Zone 5 (Bottom Edge)**: Strong downward deflection

### ⚡ Enhanced Input System

- **Momentum-based controls**: Frame-independent paddle movement ensures smooth, consistent controls at any frame rate
- **Input accumulation**: Captures input between frames for maximum responsiveness
- **Unified GameSession**: Consolidated terminal state management with zero code duplication

### 🎮 Improved Ball Physics

- Ball moves every 2 frames (30 effective FPS) for optimal gameplay feel
- Angled shots move slightly faster for added realism
- Maintains 60 FPS display rate for smooth visuals

## Technical Details

### Performance

- **CPU Usage**: <1% idle, <2% active
- **Memory**: ~1MB footprint
- **Frame Rate**: 60 FPS with conditional rendering

### Architecture

- New `GameSession` module for terminal state management
- Unified rendering pipeline with no duplication
- Proper alternate screen and raw mode handling
- Clean separation between game logic and terminal I/O

## Installation

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/asyed94/pong_term/releases):

- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

### Build from Source

```bash
git clone https://github.com/asyed94/pong_term.git
cd pong_term
cargo build --release
```

#

- **W/S**: Left paddle up/down
- **↑/↓**: Right paddle up/down
- **Space**: Pause/Resume
- **Q**: Quit to menu

## Requirements

- Terminal with ANSI escape code support
- Minimum 80×24 character display
- Rust 1.70+ (for building from source)

## Next: Stage 5

Coming in v0.5.0:

- Score tracking and display
- Win conditions
- Game state management

---

**Pure Rust • Terminal Graphics**
