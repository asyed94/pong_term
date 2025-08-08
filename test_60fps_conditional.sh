#!/bin/bash

echo "=== Testing 60 FPS and Conditional Rendering ==="
echo ""
echo "This test will verify:"
echo "1. The game runs at 60 FPS (16ms frame time)"
echo "2. Conditional rendering only updates when needed"
echo ""
echo "Instructions:"
echo "1. When the game starts, DON'T move any paddles"
echo "2. The screen should remain static (no flicker)"
echo "3. Move a paddle - rendering should update smoothly"
echo "4. Stop moving - rendering should stop updating"
echo "5. Press Space to pause and check FPS shows 60"
echo ""
echo "Starting game in 3 seconds..."
sleep 3

cargo run
