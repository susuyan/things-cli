#!/bin/bash
# E2E Test Runner - Run all test suites

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

echo "══════════════════════════════════════════════════════════════"
echo "  Things CLI - E2E Test Suite"
echo "══════════════════════════════════════════════════════════════"
echo ""
echo "This will run end-to-end tests against your actual Things 3 database."
echo "Test data will be created and may remain in Things after the test."
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

echo ""
echo "Building project..."
cargo build --quiet

echo ""
echo "Running tests..."
echo ""

# Track overall results
OVERALL_PASSED=0
OVERALL_FAILED=0

# Run each test suite
for test_script in "$SCRIPT_DIR"/test_*.sh; do
    if [ -f "$test_script" ]; then
        echo "--------------------------------------------------------------"
        echo "Running: $(basename "$test_script")"
        echo "--------------------------------------------------------------"
        if bash "$test_script"; then
            ((OVERALL_PASSED++)) || true
        else
            ((OVERALL_FAILED++)) || true
        fi
        echo ""
    fi
done

# Summary
echo "══════════════════════════════════════════════════════════════"
echo "  E2E Test Suite Complete"
echo "══════════════════════════════════════════════════════════════"
echo "  Test files passed: $OVERALL_PASSED"
echo "  Test files failed: $OVERALL_FAILED"
echo ""

if [ $OVERALL_FAILED -eq 0 ]; then
    echo "  ✅ All test suites passed!"
    exit 0
else
    echo "  ❌ Some test suites failed!"
    exit 1
fi
