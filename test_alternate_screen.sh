echo 'Before game - this text should remain after game exits'
echo | timeout 2 cargo run --quiet 2>&1 | head -20
echo 'After game - terminal should be clean (no game frames above)'
