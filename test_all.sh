#!/bin/bash

# Comprehensive test script that runs all tests and checks
# This ensures code quality before pushing to CI

set -e  # Exit on error

echo "================================================"
echo "Running Comprehensive Test Suite"
echo "================================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        exit 1
    fi
}

# 1. Format check
echo -e "${YELLOW}[1/5]${NC} Checking code formatting..."
cargo fmt -- --check
print_status $? "Code formatting check"
echo ""

# 2. Clippy linting with warnings as errors
echo -e "${YELLOW}[2/5]${NC} Running Clippy linter..."
cargo clippy -- -D warnings
print_status $? "Clippy linting"
echo ""

# 3. Build check
echo -e "${YELLOW}[3/5]${NC} Building project..."
cargo build
print_status $? "Build"
echo ""

# 4. Run tests
echo -e "${YELLOW}[4/5]${NC} Running tests..."
cargo test --verbose
print_status $? "Tests"
echo ""

# 5. Documentation check
echo -e "${YELLOW}[5/5]${NC} Checking documentation..."
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
print_status $? "Documentation"
echo ""

echo "================================================"
echo -e "${GREEN}All checks passed successfully!${NC}"
echo "================================================"
