#!/bin/bash
# Project management tests

source "$(dirname "$0")/lib.sh"

print_section "Project: Add"

# Simple project
assert_success \
    "project_add 'E2E Test Project - Simple'" \
    "Add simple project"

sleep 1

# Project with todos
assert_success \
    "project_add 'E2E Test Project - With Todos' --todos 'Task 1,Task 2,Task 3'" \
    "Add project with initial todos"

sleep 1

# Project with details
assert_success \
    "project_add 'E2E Test Project - Detailed' --notes 'Project notes' --when today" \
    "Add project with notes and when"

print_section "Project: List"

# List projects
assert_output_contains \
    "$THINGS_BIN list projects" \
    "Projects" \
    "list projects shows projects header"

# Verify our test projects exist
assert_output_contains \
    "$THINGS_BIN list projects" \
    "E2E Test Project" \
    "Test projects appear in list"

print_summary
