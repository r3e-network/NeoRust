#!/bin/bash

# Local documentation verification script
# This script checks documentation builds and quality

set -e

echo "📚 Running Documentation verification checks..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print section headers
print_section() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Function to print error
print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in project root directory. Please run from the root of the NeoRust project."
    exit 1
fi

print_section "Rust Documentation"

# Generate Rust documentation
print_section "Building Rust docs"
if cargo doc --workspace --exclude neo-gui --no-deps --document-private-items; then
    print_success "Rust documentation built successfully"
else
    print_error "Rust documentation build failed"
    exit 1
fi

# Check for missing documentation
print_section "Checking for missing documentation"
if cargo doc --workspace --exclude neo-gui --no-deps --document-private-items 2>&1 | grep -q "warning.*missing documentation"; then
    print_warning "Some items are missing documentation:"
    cargo doc --workspace --exclude neo-gui --no-deps --document-private-items 2>&1 | grep "warning.*missing documentation" | head -5
    echo "... (run 'cargo doc' to see all warnings)"
else
    print_success "No missing documentation warnings found"
fi

print_section "mdBook Documentation"

# Check if mdbook is installed
if command -v mdbook &> /dev/null; then
    if [ -f "docs/book.toml" ]; then
        cd docs
        print_section "Building mdBook documentation"
        if mdbook build; then
            print_success "mdBook documentation built successfully"
        else
            print_error "mdBook documentation build failed"
            exit 1
        fi
        
        # Check for broken links if mdbook-linkcheck is available
        if mdbook-linkcheck --version &> /dev/null; then
            print_section "Checking for broken links"
            if mdbook-linkcheck; then
                print_success "No broken links found"
            else
                print_warning "Broken links found in documentation"
            fi
        else
            print_warning "mdbook-linkcheck not found. Install with: cargo install mdbook-linkcheck"
        fi
        
        cd ..
    else
        print_warning "No docs/book.toml found, skipping mdBook checks"
    fi
else
    print_warning "mdbook not found. Install with: cargo install mdbook"
fi

print_section "README and Documentation Files"

# Check README files
required_files=("README.md" "CONTRIBUTING.md" "CHANGELOG.md")
for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        print_success "$file exists"
        
        # Check if README is not empty
        if [ "$file" = "README.md" ]; then
            if [ -s "$file" ]; then
                print_success "README.md is not empty"
            else
                print_warning "README.md is empty"
            fi
        fi
    else
        print_warning "$file is missing"
    fi
done

# Check for example documentation
print_section "Example Documentation"
if [ -d "examples" ]; then
    # Count examples with README files
    readme_count=$(find examples -name "README.md" | wc -l)
    total_examples=$(find examples -name "Cargo.toml" | wc -l)
    
    print_success "Found $readme_count README files in $total_examples example projects"
    
    if [ $readme_count -lt $total_examples ]; then
        print_warning "Some examples are missing README files:"
        for example_dir in $(find examples -name "Cargo.toml" -exec dirname {} \;); do
            if [ ! -f "$example_dir/README.md" ]; then
                echo "  - $example_dir"
            fi
        done
    fi
else
    print_warning "No examples directory found"
fi

# Check API documentation coverage
print_section "API Documentation Coverage"
if cargo doc --workspace --exclude neo-gui --no-deps --document-private-items --quiet 2>&1; then
    # Count public items vs documented items (approximation)
    if [ -d "target/doc" ]; then
        print_success "API documentation generated in target/doc/"
    fi
else
    print_warning "Could not generate API documentation for coverage check"
fi

print_section "Documentation Quality Checks"

# Check for common documentation issues
if find . -name "*.md" -exec grep -l "TODO\|FIXME\|XXX" {} \; | grep -q .; then
    print_warning "Found TODO/FIXME in documentation files:"
    find . -name "*.md" -exec grep -l "TODO\|FIXME\|XXX" {} \; | head -5
else
    print_success "No TODO/FIXME found in documentation"
fi

# Check for broken markdown links (basic check)
if find . -name "*.md" -exec grep -l "](.*\.md)" {} \; | xargs -I {} sh -c 'grep -n "](.*\.md)" "$1" | while read line; do link=$(echo "$line" | sed "s/.*](\([^)]*\)).*/\1/"); if [ ! -f "$(dirname "$1")/$link" ] && [ ! -f "$link" ]; then echo "Broken link in $1: $link"; fi; done' -- {} \; | grep -q .; then
    print_warning "Potential broken markdown links found (manual review needed)"
else
    print_success "No obvious broken markdown links found"
fi

# Check documentation structure
print_section "Documentation Structure"
doc_dirs=("docs" "website" "examples")
for dir in "${doc_dirs[@]}"; do
    if [ -d "$dir" ]; then
        print_success "$dir directory exists"
        file_count=$(find "$dir" -name "*.md" | wc -l)
        print_success "Found $file_count markdown files in $dir"
    else
        print_warning "$dir directory not found"
    fi
done

echo -e "\n${GREEN}📚 Documentation verification checks completed!${NC}"
echo -e "${BLUE}Review any warnings above to improve documentation quality.${NC}"