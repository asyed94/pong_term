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
