#!/bin/bash
# Todo management tests

source "$(dirname "$0")/lib.sh"

print_section "Todo: Add"

# Simple todo
assert_success \
    "todo_add 'E2E Test - Simple Todo'" \
    "Add simple todo"

sleep 1

# Todo with details
assert_success \
    "todo_add 'E2E Test - Detailed Todo' --notes 'Test notes' --when today --tags 'e2e,test'" \
    "Add todo with notes, when, and tags"

sleep 1

# Todo with checklist
assert_success \
    "todo_add 'E2E Test - Checklist Todo' --checklist 'Item 1,Item 2,Item 3'" \
    "Add todo with checklist"

sleep 1

# Todo with deadline
assert_success \
    "todo_add 'E2E Test - Deadline Todo' --deadline '2026-12-31'" \
    "Add todo with deadline"

sleep 1

# Repeating todo
assert_success \
    "todo_add 'E2E Test - Repeating Todo' --repeat week --repeat-until 2026-12-31" \
    "Add repeating todo"

sleep 1

# Multiple todos
assert_success \
    "todo_add 'E2E Batch 1' 'E2E Batch 2' 'E2E Batch 3'" \
    "Add multiple todos at once"

print_section "Todo: List"

# List inbox
assert_output_contains \
    "$THINGS_BIN list inbox" \
    "Inbox" \
    "list inbox shows inbox header"

# List today
assert_output_contains \
    "$THINGS_BIN list today" \
    "Today" \
    "list today shows today header"

# Search for test todos
assert_output_contains \
    "$THINGS_BIN search 'E2E Test'" \
    "E2E" \
    "search finds test todos"

print_summary
