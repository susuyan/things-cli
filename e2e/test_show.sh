#!/bin/bash
# Show and search tests

source "$(dirname "$0")/lib.sh"

print_section "Show Commands"

# Show inbox
assert_success \
    "$THINGS_BIN show inbox" \
    "show inbox"

# Show today
assert_success \
    "$THINGS_BIN show today" \
    "show today"

# Show with query
assert_success \
    "$THINGS_BIN show 'E2E Test'" \
    "show with search query"

print_section "Search Commands"

# Search
assert_output_contains \
    "$THINGS_BIN search 'E2E'" \
    "E2E" \
    "search finds E2E test items"

print_summary
