#!/bin/bash

# Local security verification script based on GitHub Actions workflows
# This script runs security audits for both Rust and frontend components

set -e

echo "🔒 Running Security verification checks..."

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

print_section "Rust Security Audit"

# Check if cargo-audit is installed
if ! command -v cargo-audit &> /dev/null; then
    print_warning "cargo-audit not found. Installing..."
    if cargo install cargo-audit; then
        print_success "cargo-audit installed successfully"
    else
        print_error "Failed to install cargo-audit"
        exit 1
    fi
fi

# Run Rust security audit
print_section "Running cargo audit"
if cargo audit; then
    print_success "No Rust security vulnerabilities found"
else
    print_warning "Rust security audit found issues (check .cargo/audit.toml for ignored advisories)"
fi

# Check for common security issues in Rust code
print_section "Checking for common Rust security patterns"

# Check for unsafe code
if find src -name "*.rs" -exec grep -l "unsafe" {} \; | grep -q .; then
    print_warning "Found 'unsafe' code blocks. Review for security implications:"
    find src -name "*.rs" -exec grep -l "unsafe" {} \;
else
    print_success "No unsafe code blocks found"
fi

# Check for unwrap() calls that could panic
if find src -name "*.rs" -exec grep -l "\.unwrap()" {} \; | grep -q .; then
    print_warning "Found '.unwrap()' calls that could cause panics:"
    find src -name "*.rs" -exec grep -n "\.unwrap()" {} + | head -10
    echo "... (showing first 10, review all instances)"
else
    print_success "No .unwrap() calls found"
fi

# Check for TODO comments related to security
if find src -name "*.rs" -exec grep -i "TODO.*security\|FIXME.*security" {} \; | grep -q .; then
    print_warning "Found security-related TODO/FIXME comments:"
    find src -name "*.rs" -exec grep -in "TODO.*security\|FIXME.*security" {} +
else
    print_success "No security-related TODO/FIXME comments found"
fi

print_section "Frontend Security Audit"

if [ -d "neo-gui" ]; then
    cd neo-gui
    
    # Check if npm is available
    if ! command -v npm &> /dev/null; then
        print_error "npm not found. Please install Node.js and npm."
        exit 1
    fi
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        print_warning "Installing npm dependencies..."
        npm ci
    fi
    
    # Run npm audit
    print_section "Running npm audit"
    if npm audit --audit-level=high; then
        print_success "No high-severity npm vulnerabilities found"
    else
        print_warning "npm audit found high-severity vulnerabilities"
    fi
    
    # Run audit-ci if configured
    if [ -f ".audit-ci.json" ]; then
        print_section "Running audit-ci security check"
        if npx audit-ci --config .audit-ci.json; then
            print_success "audit-ci security check passed"
        else
            print_warning "audit-ci found security issues"
        fi
    fi
    
    # Check for common frontend security issues
    print_section "Checking for common frontend security patterns"
    
    # Check for dangerouslySetInnerHTML
    if find src -name "*.tsx" -o -name "*.ts" | xargs grep -l "dangerouslySetInnerHTML" 2>/dev/null | grep -q .; then
        print_warning "Found 'dangerouslySetInnerHTML' usage (XSS risk):"
        find src -name "*.tsx" -o -name "*.ts" | xargs grep -n "dangerouslySetInnerHTML" 2>/dev/null
    else
        print_success "No dangerouslySetInnerHTML usage found"
    fi
    
    # Check for eval() usage
    if find src -name "*.tsx" -o -name "*.ts" | xargs grep -l "eval(" 2>/dev/null | grep -q .; then
        print_warning "Found 'eval()' usage (code injection risk):"
        find src -name "*.tsx" -o -name "*.ts" | xargs grep -n "eval(" 2>/dev/null
    else
        print_success "No eval() usage found"
    fi
    
    # Check for hardcoded secrets or keys
    if find src -name "*.tsx" -o -name "*.ts" | xargs grep -i "api[_-]key\|secret\|password\|token" 2>/dev/null | grep -v "// TODO\|// FIXME\|interface\|type\|import" | grep -q .; then
        print_warning "Potential hardcoded secrets found (review manually):"
        find src -name "*.tsx" -o -name "*.ts" | xargs grep -in "api[_-]key\|secret\|password\|token" 2>/dev/null | grep -v "// TODO\|// FIXME\|interface\|type\|import" | head -5
        echo "... (showing first 5, review all instances)"
    else
        print_success "No obvious hardcoded secrets found"
    fi
    
    cd ..
else
    print_warning "neo-gui directory not found, skipping frontend security checks"
fi

print_section "General Security Checks"

# Check for exposed configuration files
if find . -name "*.env" -o -name "*.key" -o -name "*.pem" | grep -v ".gitignore" | grep -q .; then
    print_warning "Found potential sensitive files:"
    find . -name "*.env" -o -name "*.key" -o -name "*.pem" | grep -v ".gitignore"
else
    print_success "No sensitive files found in repository"
fi

# Check .gitignore for common patterns
if [ -f ".gitignore" ]; then
    if grep -q "\.env\|\.key\|\.pem\|secret" .gitignore; then
        print_success ".gitignore contains security-related patterns"
    else
        print_warning ".gitignore might be missing security-related patterns"
    fi
else
    print_warning ".gitignore file not found"
fi

echo -e "\n${GREEN}🔒 Security verification checks completed!${NC}"
echo -e "${BLUE}Review any warnings above and ensure they are addressed before production.${NC}"