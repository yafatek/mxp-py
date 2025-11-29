# Contributing to MXP Python Bindings

Thank you for your interest in contributing to the MXP Python bindings! This document provides guidelines for contributing.

## Types of Contributions

### Code Contributions
- Bug fixes
- Performance optimizations
- New features
- Test coverage improvements
- Documentation improvements

### Documentation
- Code examples
- Tutorials
- API documentation
- Blog posts

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a branch** for your changes
4. **Make your changes** following our guidelines
5. **Test thoroughly**
6. **Submit a pull request**

## Development Setup

### Prerequisites
- Python 3.8 or later
- Rust 1.85 or later
- maturin (`pip install maturin`)

### Build and Test

```bash
# Clone the repository
git clone https://github.com/yafatek/mxp-py
cd mxp-py

# Build in development mode
maturin develop

# Run tests
pytest tests/

# Format Python code
black python/ tests/

# Lint Python code
ruff check python/ tests/

# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy -- -D warnings
```

## Code Standards

### Python Code
- Follow PEP 8 style guide
- Use `black` for formatting
- Use type hints where appropriate
- Write docstrings for all public functions

### Rust Code
- Follow idiomatic Rust conventions
- Use `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Write doc comments for all public APIs

### Testing
- Write tests for all new functionality
- Ensure all tests pass before submitting PR
- Aim for high test coverage

## Pull Request Process

1. **Create an Issue**: Discuss significant changes before coding
2. **Branch Naming**: Use descriptive names (e.g., `feature/async-support`, `fix/memory-leak`)
3. **Commit Messages**: 
   - Use imperative mood (\"Add feature\" not \"Added feature\")
   - First line: Brief summary (50 chars or less)
   - Body: Detailed explanation if needed
4. **Documentation**: Update relevant documentation
5. **Tests**: Include tests for new functionality

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass (`pytest tests/`)
- [ ] Rust code is linted (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`, `black`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (for significant changes)

## Code of Conduct

This project follows the [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.

## Questions?

- Open a [GitHub Issue](https://github.com/yafatek/mxp-py/issues)
- Email: support@yafa.dev

Thank you for contributing to MXP Python bindings!
