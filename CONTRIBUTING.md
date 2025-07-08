# Contributing to Terminator-Dancer

We welcome contributions to the Terminator-Dancer project! This document outlines how to contribute effectively.

## Getting Started

### Prerequisites

- Rust 1.70+ with cargo
- Git for version control
- Basic understanding of Solana transaction processing

### Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/terminator-dancer.git
   cd terminator-dancer
   ```
3. Create a development branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Guidelines

### Code Style

- Follow Rust best practices and idioms
- Use `cargo fmt` to format code
- Use `cargo clippy` to lint code
- Add documentation for public APIs
- Include examples in documentation where helpful

### Testing

- Write comprehensive tests for new functionality
- Ensure all tests pass: `cargo test`
- Add integration tests for complex features
- Update conformance tests when adding new functionality

### Documentation

- Update README.md for significant changes
- Add inline documentation for complex logic
- Include usage examples for new features
- Update architecture documentation as needed

## Types of Contributions

### Bug Reports

When reporting bugs, include:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)

### Feature Requests

For new features, provide:
- Clear use case and motivation
- Detailed description of proposed solution
- Consideration of alternatives
- Impact on existing functionality

### Code Contributions

1. **Small Changes**: Bug fixes, documentation updates, minor improvements
2. **Medium Changes**: New features, significant refactoring
3. **Large Changes**: Major architectural changes, new modules

## Development Process

### Before Starting

- Check existing issues and pull requests
- Discuss large changes in issues first
- Ensure your changes align with project goals

### During Development

- Keep changes focused and atomic
- Write clear commit messages
- Add tests for new functionality
- Update documentation as needed

### Testing Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run demo to verify functionality
cargo run --example demo

# Run the demo script
./demo.sh
```

### Submitting Changes

1. Ensure all tests pass
2. Format and lint your code:
   ```bash
   cargo fmt
   cargo clippy
   ```
3. Create a pull request with:
   - Clear title and description
   - Reference related issues
   - Explanation of changes made
   - Testing performed

## Code Review Process

- All submissions require code review
- Reviews focus on:
  - Code correctness and safety
  - Test coverage
  - Documentation quality
  - Alignment with project goals
- Address feedback constructively
- Maintain a respectful tone

## Project Structure

When adding new functionality, consider:

- **Core Runtime**: Changes to transaction processing
- **Testing Infrastructure**: New test types or frameworks
- **Utilities**: Helper functions and common code
- **Documentation**: User guides and API documentation

## Specific Areas for Contribution

### High Priority

- Full Solana instruction semantics
- Complete signature validation
- Advanced account management
- Cross-program invocation support

### Medium Priority

- Performance optimizations
- Additional conformance tests
- Fuzzing test coverage
- Error handling improvements

### Low Priority

- Documentation improvements
- Code refactoring
- Developer tooling
- Examples and tutorials

## Security Considerations

- Be cautious with cryptographic operations
- Validate all inputs thoroughly
- Consider attack vectors in transaction processing
- Report security issues privately

## Communication

- Use GitHub issues for bug reports and feature requests
- Be respectful and constructive in discussions
- Ask questions if anything is unclear
- Provide helpful feedback to other contributors

## License

By contributing to Terminator-Dancer, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be acknowledged in:
- CONTRIBUTORS.md file
- Release notes for significant contributions
- GitHub contributor graphs

Thank you for contributing to Terminator-Dancer!
