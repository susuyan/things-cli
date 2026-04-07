#!/bin/bash
# Basic CLI tests - version, help, config

source "$(dirname "$0")/lib.sh"

print_section "Basic Commands"

# Test version
assert_output_contains \
    "$THINGS_BIN --version" \
    "things" \
    "things --version shows version"

# Test help
assert_output_contains \
    "$THINGS_BIN --help" \
    "Usage:" \
    "things --help shows usage"

assert_output_contains \
    "$THINGS_BIN --help" \
    "Commands:" \
    "things --help shows commands list"

# Test config commands
print_section "Config Commands"

assert_success \
    "$THINGS_BIN config show" \
    "config show displays configuration"

print_summary
