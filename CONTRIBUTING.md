# Contributing to NeoRust

First off, thank you for considering contributing to NeoRust! It's people like you that make NeoRust such a great tool for the Neo N3 ecosystem.

## 🤝 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 🎯 How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** to demonstrate the steps
- **Describe the behavior you observed** and explain why it's a problem
- **Explain which behavior you expected** to see instead
- **Include logs, stack traces, and error messages**
- **Include your environment details** (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- **Use a clear and descriptive title**
- **Provide a detailed description** of the suggested enhancement
- **Provide specific examples** to demonstrate how it would work
- **Describe the current behavior** and explain how your suggestion improves it
- **Explain why this enhancement would be useful** to most NeoRust users

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

## 🛠️ Development Process

### Setting Up Your Development Environment

```bash
# Clone your fork
git clone https://github.com/your-username/NeoRust.git
cd NeoRust

# Add upstream remote
git remote add upstream https://github.com/R3E-Network/NeoRust.git

# Install dependencies
cargo build --workspace

# Run tests
cargo test --workspace

# For GUI development
cd neo-gui
npm install
npm run dev
```

### Branch Naming Convention

- `feature/` - New features (e.g., `feature/neo-x-support`)
- `fix/` - Bug fixes (e.g., `fix/transaction-signing`)
- `docs/` - Documentation improvements (e.g., `docs/wallet-guide`)
- `refactor/` - Code refactoring (e.g., `refactor/rpc-client`)
- `test/` - Test improvements (e.g., `test/integration-tests`)
- `chore/` - Maintenance tasks (e.g., `chore/update-dependencies`)

### Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Types
- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, etc.)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to build process or auxiliary tools

#### Examples
```
feat(rpc): add support for getproof RPC method

Implements the getproof method for the RPC client, allowing
verification of state proofs from Neo N3 nodes.

Closes #123
```

```
fix(wallet): correct address validation for legacy addresses

The address validation was incorrectly rejecting valid legacy
addresses. This fix updates the regex pattern to handle both
new and legacy address formats.

Fixes #456
```

### Code Style

We use `rustfmt` and `clippy` to maintain consistent code style:

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --workspace --all-features -- -D warnings

# For TypeScript/React (GUI)
cd neo-gui
npm run lint
npm run format
```

### Testing

- **Unit Tests**: Place in the same file as the code using `#[cfg(test)]`
- **Integration Tests**: Place in `tests/` directory
- **Documentation Tests**: Include examples in doc comments
- **GUI Tests**: Use Jest for unit tests and Playwright for E2E tests

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run GUI tests
cd neo-gui
npm test
npm run test:e2e
```

### Documentation

- Document all public APIs
- Include examples in documentation
- Update README.md for user-facing changes
- Update CHANGELOG.md for all changes

```rust
/// Sends a transaction to the Neo N3 network.
/// 
/// # Arguments
/// 
/// * `transaction` - The signed transaction to send
/// 
/// # Returns
/// 
/// Returns the transaction hash if successful.
/// 
/// # Errors
/// 
/// Returns an error if the transaction is invalid or the network is unreachable.
/// 
/// # Examples
/// 
/// ```rust
/// # use neo3_rpc::RpcClient;
/// # async fn example(client: &RpcClient) -> Result<(), Box<dyn std::error::Error>> {
/// let tx = create_transaction()?;
/// let tx_hash = client.send_transaction(&tx).await?;
/// println!("Transaction sent: {}", tx_hash);
/// # Ok(())
/// # }
/// ```
pub async fn send_transaction(&self, transaction: &Transaction) -> Result<TxId, Neo3Error> {
    // Implementation
}
```

## 📋 Pull Request Process

1. **Update Documentation** - Update the README.md with details of changes to the interface
2. **Add Tests** - Ensure new functionality is tested
3. **Update CHANGELOG** - Add your changes to the Unreleased section
4. **Pass CI** - Ensure all CI checks pass
5. **Request Review** - Request review from maintainers
6. **Address Feedback** - Make requested changes promptly
7. **Merge** - Once approved, we'll merge your PR!

### PR Checklist

- [ ] My code follows the code style of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published

## 🏗️ Project Structure

```
NeoRust/
├── src/                    # Main library source
├── crates/                 # Individual crates (future)
├── examples/               # Example applications
├── neo-cli/               # CLI application
├── neo-gui/               # GUI application
├── tests/                 # Integration tests
├── benches/               # Benchmarks
└── docs/                  # Documentation
```

## 🔍 Where to Start?

Looking for a good first issue? Check out:
- Issues labeled [`good first issue`](https://github.com/R3E-Network/NeoRust/labels/good%20first%20issue)
- Issues labeled [`help wanted`](https://github.com/R3E-Network/NeoRust/labels/help%20wanted)
- Documentation improvements
- Test coverage improvements
- Example applications

## 📞 Getting Help

If you need help, you can:
- Open a [Discussion](https://github.com/R3E-Network/NeoRust/discussions)
- Join our community chat (if available)
- Ask questions in issues (label with `question`)

## 🎉 Recognition

Contributors will be:
- Listed in our CONTRIBUTORS.md file
- Mentioned in release notes
- Given credit in documentation

## 📜 License

By contributing to NeoRust, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

Thank you for contributing to NeoRust! 🚀