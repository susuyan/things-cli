#!/bin/bash
# Area management tests (uses AppleScript)

source "$(dirname "$0")/lib.sh"

print_section "Area: Add"

# Create area
assert_success \
    "area_add 'E2E Test Area'" \
    "Add area via AppleScript"

print_section "Area: List"

# List areas
assert_output_contains \
    "$THINGS_BIN list areas" \
    "Areas" \
    "list areas shows areas header"

print_summary
