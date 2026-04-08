#!/bin/bash
# Release script for things-cli

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')

echo -e "${YELLOW}Current version: $CURRENT_VERSION${NC}"
echo ""

# Check if we're in a git repo
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not a git repository${NC}"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: You have uncommitted changes${NC}"
    echo "Please commit or stash them first."
    exit 1
fi

# Parse arguments
if [ -z "$1" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo ""
    echo "Examples:"
    echo "  ./scripts/release.sh 0.2.0    # Release version 0.2.0"
    echo "  ./scripts/release.sh patch    # Bump patch version (0.1.0 -> 0.1.1)"
    echo "  ./scripts/release.sh minor    # Bump minor version (0.1.0 -> 0.2.0)"
    echo "  ./scripts/release.sh major    # Bump major version (0.1.0 -> 1.0.0)"
    exit 1
fi

VERSION_ARG=$1

# Calculate new version
if [ "$VERSION_ARG" = "patch" ]; then
    NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$NF = $NF + 1;} 1' | sed 's/ /./g')
elif [ "$VERSION_ARG" = "minor" ]; then
    NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$(NF-1) = $(NF-1) + 1; $NF = 0;} 1' | sed 's/ /./g')
elif [ "$VERSION_ARG" = "major" ]; then
    NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$1 = $1 + 1; $(NF-1) = 0; $NF = 0;} 1' | sed 's/ /./g')
else
    NEW_VERSION=$VERSION_ARG
fi

# Validate version format
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo -e "${RED}Error: Invalid version format: $NEW_VERSION${NC}"
    echo "Version must be in format: x.y.z (e.g., 0.1.0)"
    exit 1
fi

echo -e "${GREEN}New version: $NEW_VERSION${NC}"
echo ""

# Confirm
read -p "Continue with release? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Update version in Cargo.toml
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Update Cargo.lock
cargo update --workspace

# Commit version bump
git add Cargo.toml Cargo.lock
git commit -m "Bump version to $NEW_VERSION"

# Create tag
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

echo ""
echo -e "${GREEN}✓ Version bumped to $NEW_VERSION${NC}"
echo -e "${GREEN}✓ Created tag v$NEW_VERSION${NC}"
echo ""
echo "To push the release:"
echo -e "  ${YELLOW}git push origin master --tags${NC}"
echo ""
echo "This will trigger the GitHub Actions release workflow."
