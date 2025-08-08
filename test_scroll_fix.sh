#!/bin/bash

echo "=== Testing Scroll Wheel Input Fix ==="
echo ""
echo "This test will simulate mouse scroll events to verify the fix."
echo "The game should remain responsive and not show delayed input processing."
echo ""
echo "Instructions:"
echo "1. The game will start"
echo "2. Try scrolling with your mouse wheel rapidly"
echo "3. Then immediately press W/S or arrow keys"
echo "4. The paddles should respond immediately without delay"
echo "5. Press Q to quit when done testing"
echo ""
echo "Starting game in 3 seconds..."
sleep 3

cargo run
