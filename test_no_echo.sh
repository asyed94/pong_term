#!/bin/bash

echo "=== Testing No-Echo Functionality ==="
echo ""
echo "This test will send 'test123' + Enter twice to the game."
echo "If no-echo is working, you should NOT see 'test123' on screen."
echo ""
echo "Running game with test input..."
echo ""

# Send "test123" and then Enter twice (once for setup, once for board)
(echo -e "test123\ntest456\n"; sleep 1) | timeout 2 cargo run --quiet 2>&1 | head -30

echo ""
echo "=== Test Complete ==="
echo "If you didn't see 'test123' or 'test456' echoed above, the no-echo is working!"
