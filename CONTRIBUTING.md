# Contributing to anyform

Thank you for your interest in contributing to anyform! This document provides guidelines and information for contributors.

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected vs actual behavior
- Your environment (OS, Rust version, Node.js version, etc.)
- Any relevant logs or error messages

### Suggesting Features

Feature requests are welcome! Please provide:

- A clear description of the feature
- The problem it solves or use case it enables
- Any implementation ideas you have

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests and linting
5. Commit with clear messages
6. Push to your fork
7. Open a pull request

## Development Setup

### Prerequisites

- Rust 1.75+ (for backend)
- Node.js 20+ (for frontend packages)
- wasm-pack (for WASM builds)

### Building

```bash
# Rust library
cargo build

# WASM client
cd anyform-client
wasm-pack build --target web

# npm packages
cd anyform-react
npm install
npm run build
```

### Testing

```bash
# Rust tests
cargo test

# npm package tests
cd anyform-react
npm test
```

### Linting

```bash
# Rust
cargo clippy
cargo fmt --check

# TypeScript
cd anyform-react
npm run lint
```

## Project Structure

```
anyform/
├── anyform/           # Main Rust library (Axum handlers, SeaORM models)
├── anyform-cli/       # CLI binary
├── anyform-client/    # WASM client (Rust → WebAssembly)
├── anyform-core/      # TypeScript validation library (@wordpuppi/anyform-core)
├── anyform-wasm-js/   # TypeScript WASM bindings (@wordpuppi/anyform-wasm-js)
├── anyform-react/     # React hooks (@wordpuppi/anyform-react)
├── anyform-next/      # Next.js integration (@wordpuppi/anyform-next)
├── anyform-wordpress/ # WordPress plugin
├── migration/         # SeaORM database migrations
└── examples/          # Example projects
```

## Commit Messages

We follow conventional commits:

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `test:` Test additions or changes
- `chore:` Maintenance tasks

Example: `feat: add multi-select field support`

## License

By contributing, you agree that your contributions will be licensed under the same MIT/Apache-2.0 dual license as the project.

## Questions?

Feel free to open an issue for any questions about contributing.
