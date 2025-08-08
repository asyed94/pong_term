#!/bin/bash

echo "=== Testing Ball Physics ==="
echo ""
echo "This test demonstrates the ball physics implementation:"
echo "1. Ball moves at 1 cell per frame (60 FPS)"
echo "2. Ball bounces off paddles (horizontal bounce)"
echo "3. Ball bounces off walls (vertical bounce)"
echo "4. Ball resets to center after goals"
echo ""
echo "Instructions:"
echo "1. Press Enter to start the game setup"
echo "2. Press Space to unpause and watch the ball"
echo "3. Move paddles with W/S and ↑/↓ to hit the ball"
echo "4. Watch the ball bounce off walls and paddles"
echo "5. Let the ball pass a paddle to see goal reset"
echo ""
echo "Starting game in 3 seconds..."
sleep 3

cargo run
