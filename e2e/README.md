# E2E Tests for Things CLI

End-to-end tests that run against the actual Things 3 application.

## ⚠️ Warning

These tests create real data in your Things 3 database! Test data will remain after tests complete.

## Structure

```
e2e/
├── README.md           # This file
├── lib.sh              # Shared test library (assertions, helpers)
├── run_all.sh          # Run all test suites
├── test_all.sh         # Legacy comprehensive test
├── test_basic.sh       # Basic CLI tests
├── test_todo.sh        # Todo management tests
├── test_project.sh     # Project management tests
├── test_area.sh        # Area management tests
├── test_batch.sh       # Batch operations tests
└── test_show.sh        # Show and search tests
```

## Running Tests

### Run all tests
```bash
cd e2e
./run_all.sh
```

### Run individual test suites
```bash
cd e2e
./test_basic.sh     # Version, help, config
./test_todo.sh      # Todo CRUD
./test_project.sh   # Project CRUD
./test_area.sh      # Area CRUD
./test_batch.sh     # Batch import
./test_show.sh      # Show and search
```

### Run legacy comprehensive test
```bash
cd e2e
./test_all.sh
```

## Test Library Functions

### Print Functions
- `print_header "Title"` - Print section header
- `print_section "Title"` - Print subsection
- `print_success "Message"` - Success message
- `print_error "Message"` - Error message
- `print_warning "Message"` - Warning message

### Assertion Functions
- `assert_success "command" "description"` - Assert command succeeds
- `assert_failure "command" "description"` - Assert command fails
- `assert_output_contains "command" "expected" "description"` - Assert output contains string

### Helper Functions
- `things <args>` - Run things CLI
- `todo_add <title> [options]` - Add todo
- `project_add <title> [options]` - Add project
- `area_add <title>` - Add area

## Writing New Tests

1. Create a new file: `test_feature.sh`
2. Source the library: `source "$(dirname "$0")/lib.sh"`
3. Write tests using assertion functions
4. End with: `print_summary`

Example:
```bash
#!/bin/bash
source "$(dirname "$0")/lib.sh"

print_section "My Feature"

assert_success \
    "things mycommand" \
    "My command works"

assert_output_contains \
    "things mycommand --help" \
    "Usage:" \
    "Help shows usage"

print_summary
```

## CI/CD Notes

These tests require:
- macOS with Things 3 installed
- Things 3 running
- Auth token configured (for update tests)

Not suitable for headless CI without GUI automation.
