#!/bin/bash
# Batch operations tests

source "$(dirname "$0")/lib.sh"

print_section "Batch: Template"

# Generate template
assert_output_contains \
    "$THINGS_BIN batch template" \
    "project" \
    "batch template outputs project template"

assert_output_contains \
    "$THINGS_BIN batch template" \
    "to-do" \
    "batch template outputs todo template"

print_section "Batch: Import"

# Create test JSON file
cat > /tmp/e2e_batch.json << 'EOF'
[
  {
    "type": "to-do",
    "attributes": {
      "title": "E2E Batch Import 1",
      "when": "today"
    }
  },
  {
    "type": "to-do",
    "attributes": {
      "title": "E2E Batch Import 2",
      "tags": ["e2e", "batch"]
    }
  }
]
EOF

# Import batch
assert_success \
    "$THINGS_BIN batch import /tmp/e2e_batch.json" \
    "Import batch from JSON file"

# Cleanup
rm -f /tmp/e2e_batch.json

print_summary
