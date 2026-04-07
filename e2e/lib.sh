#!/bin/bash
# E2E Test Library - Shared functions for Things CLI testing

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# CLI binary path
THINGS_BIN="${THINGS_BIN:-cargo run --quiet --}"

#######################################
# Print functions
#######################################

print_header() {
    echo ""
    echo -e "${BLUE}══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}══════════════════════════════════════════════════════════════${NC}"
}

print_section() {
    echo ""
    echo -e "${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}  ✓ $1${NC}"
    ((TESTS_PASSED++)) || true
}

print_error() {
    echo -e "${RED}  ✗ $1${NC}"
    ((TESTS_FAILED++)) || true
}

print_warning() {
    echo -e "${YELLOW}  ⚠ $1${NC}"
}

print_info() {
    echo -e "  ℹ $1"
}

#######################################
# Test assertion functions
#######################################

assert_success() {
    local cmd="$1"
    local description="${2:-$cmd}"

    if eval "$cmd" > /dev/null 2>&1; then
        print_success "$description"
        return 0
    else
        print_error "$description"
        return 1
    fi
}

assert_output_contains() {
    local cmd="$1"
    local expected="$2"
    local description="${3:-$cmd}"

    local output
    output=$(eval "$cmd" 2>&1) || true

    if echo "$output" | grep -q "$expected"; then
        print_success "$description"
        return 0
    else
        print_error "$description (expected '$expected' in output)"
        return 1
    fi
}

assert_failure() {
    local cmd="$1"
    local description="${2:-$cmd}"

    if ! eval "$cmd" > /dev/null 2>&1; then
        print_success "$description"
        return 0
    else
        print_error "$description (expected failure)"
        return 1
    fi
}

#######################################
# CLI helper functions
#######################################

things() {
    $THINGS_BIN "$@"
}

todo_add() {
    things todo add "$@"
}

project_add() {
    things project add "$@"
}

area_add() {
    things area add "$@"
}

#######################################
# Test summary
#######################################

print_summary() {
    echo ""
    echo -e "${BLUE}══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  Test Summary${NC}"
    echo -e "${BLUE}══════════════════════════════════════════════════════════════${NC}"
    echo -e "  Passed: ${GREEN}$TESTS_PASSED${NC}"
    echo -e "  Failed: ${RED}$TESTS_FAILED${NC}"
    echo ""

    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "${GREEN}  All tests passed!${NC}"
        return 0
    else
        echo -e "${RED}  Some tests failed!${NC}"
        return 1
    fi
}

#######################################
# Setup and teardown
#######################################

setup_test_env() {
    print_header "Things CLI E2E Tests"
    print_info "CLI Binary: $THINGS_BIN"
    print_info "Date: $(date)"

    # Check Things 3 is installed
    if [ ! -d "/Applications/Things3.app" ]; then
        print_error "Things 3 is not installed at /Applications/Things3.app"
        exit 1
    fi
    print_success "Things 3 is installed"

    # Check if Things is running
    if pgrep -x "Things3" > /dev/null; then
        print_success "Things 3 is running"
    else
        print_warning "Things 3 is not running (some tests may fail)"
    fi
}

cleanup_test_data() {
    print_section "Cleanup"
    print_info "Test data created during this run may remain in Things"
    print_info "Manual cleanup may be required"
}
