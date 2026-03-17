#!/bin/bash
# Build script for Notepad++ Rust Edition (Linux/Unix)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_TYPE="${1:-release}"
VERBOSE="${VERBOSE:-0}"

# Print colored message
print_msg() {
    local color=$1
    shift
    echo -e "${color}$@${NC}"
}

# Print header
print_header() {
    echo ""
    print_msg "$BLUE" "═══════════════════════════════════════════════════════════"
    print_msg "$BLUE" "  Notepad++ Rust Edition - Build Script"
    print_msg "$BLUE" "═══════════════════════════════════════════════════════════"
    echo ""
}

# Check prerequisites
check_prerequisites() {
    print_msg "$BLUE" "→ Checking prerequisites..."

    # Check Rust
    if ! command -v cargo &> /dev/null; then
        print_msg "$RED" "✗ Error: Rust/Cargo not found"
        print_msg "$YELLOW" "  Install from: https://rustup.rs/"
        exit 1
    fi

    local rust_version=$(rustc --version | awk '{print $2}')
    print_msg "$GREEN" "  ✓ Rust: $rust_version"

    # Check for platform-specific dependencies
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Check for GTK4 (future requirement)
        if pkg-config --exists gtk4 2>/dev/null; then
            local gtk_version=$(pkg-config --modversion gtk4)
            print_msg "$GREEN" "  ✓ GTK4: $gtk_version"
        else
            print_msg "$YELLOW" "  ⚠ GTK4 not found (will be needed for Linux GUI)"
            print_msg "$YELLOW" "    Install: sudo apt install libgtk-4-dev"
        fi
    fi
}

# Clean build artifacts
clean_build() {
    print_msg "$BLUE" "→ Cleaning build artifacts..."
    cargo clean
    print_msg "$GREEN" "  ✓ Clean complete"
}

# Run tests
run_tests() {
    print_msg "$BLUE" "→ Running tests..."

    if [ "$VERBOSE" = "1" ]; then
        cargo test --workspace -- --nocapture
    else
        cargo test --workspace
    fi

    local test_exit=$?
    if [ $test_exit -eq 0 ]; then
        print_msg "$GREEN" "  ✓ All tests passed"
    else
        print_msg "$RED" "  ✗ Tests failed"
        exit $test_exit
    fi
}

# Run clippy
run_clippy() {
    print_msg "$BLUE" "→ Running clippy..."

    cargo clippy --all-targets --all-features -- -D warnings

    local clippy_exit=$?
    if [ $clippy_exit -eq 0 ]; then
        print_msg "$GREEN" "  ✓ No clippy warnings"
    else
        print_msg "$RED" "  ✗ Clippy found issues"
        exit $clippy_exit
    fi
}

# Check formatting
check_format() {
    print_msg "$BLUE" "→ Checking code formatting..."

    cargo fmt -- --check

    local fmt_exit=$?
    if [ $fmt_exit -eq 0 ]; then
        print_msg "$GREEN" "  ✓ Code is formatted correctly"
    else
        print_msg "$YELLOW" "  ⚠ Code needs formatting"
        print_msg "$YELLOW" "    Run: cargo fmt"
        return 1
    fi
}

# Build the project
build_project() {
    print_msg "$BLUE" "→ Building ($BUILD_TYPE)..."

    local cargo_args=""
    if [ "$BUILD_TYPE" = "release" ]; then
        cargo_args="--release"
    fi

    if [ "$VERBOSE" = "1" ]; then
        cargo_args="$cargo_args --verbose"
    fi

    cargo build $cargo_args

    local build_exit=$?
    if [ $build_exit -eq 0 ]; then
        print_msg "$GREEN" "  ✓ Build successful"
    else
        print_msg "$RED" "  ✗ Build failed"
        exit $build_exit
    fi
}

# Create distribution
create_dist() {
    print_msg "$BLUE" "→ Creating distribution..."

    local dist_dir="$SCRIPT_DIR/dist"
    mkdir -p "$dist_dir"

    if [ "$BUILD_TYPE" = "release" ]; then
        local binary="$SCRIPT_DIR/target/release/notepad-plus"
    else
        local binary="$SCRIPT_DIR/target/debug/notepad-plus"
    fi

    if [ -f "$binary" ]; then
        cp "$binary" "$dist_dir/"
        print_msg "$GREEN" "  ✓ Binary copied to: $dist_dir/"

        # Get binary size
        local size=$(du -h "$dist_dir/notepad-plus" | cut -f1)
        print_msg "$GREEN" "    Size: $size"
    else
        print_msg "$RED" "  ✗ Binary not found: $binary"
        exit 1
    fi
}

# Print summary
print_summary() {
    echo ""
    print_msg "$BLUE" "═══════════════════════════════════════════════════════════"
    print_msg "$GREEN" "  Build Complete!"
    print_msg "$BLUE" "═══════════════════════════════════════════════════════════"
    echo ""

    if [ "$BUILD_TYPE" = "release" ]; then
        print_msg "$GREEN" "Binary: $SCRIPT_DIR/dist/notepad-plus"
        print_msg "$YELLOW" "Run:    $SCRIPT_DIR/dist/notepad-plus"
    else
        print_msg "$GREEN" "Binary: $SCRIPT_DIR/target/debug/notepad-plus"
        print_msg "$YELLOW" "Run:    cargo run"
    fi

    echo ""
}

# Main execution
main() {
    print_header

    cd "$SCRIPT_DIR"

    # Parse options
    case "$1" in
        clean)
            clean_build
            exit 0
            ;;
        test)
            check_prerequisites
            run_tests
            exit 0
            ;;
        check)
            check_prerequisites
            check_format
            run_clippy
            exit 0
            ;;
        release|debug)
            BUILD_TYPE="$1"
            ;;
        *)
            if [ -n "$1" ]; then
                print_msg "$RED" "Unknown build type: $1"
                echo "Usage: $0 [release|debug|clean|test|check]"
                exit 1
            fi
            ;;
    esac

    check_prerequisites
    check_format || true  # Don't fail on format check
    run_clippy
    run_tests
    build_project
    create_dist
    print_summary
}

# Run main
main "$@"
