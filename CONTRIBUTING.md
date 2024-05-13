# Contributing Guidelines

Welcome to the project! We're thrilled that you're interested in contributing. Before you get started, please take a moment to review the following guidelines.

## Code of Conduct

Please note that this project is governed by our [Code of Conduct](./CODE_OF_CONDUCT.md). By participating in this project, you agree to abide by its terms.

## How to Contribute

### Reporting Bugs

If you encounter a bug while using the project, please [open an issue](../../issues) on GitHub. Be sure to include detailed information about the bug, including steps to reproduce it.

### Suggesting Enhancements

If you have an idea for an enhancement or a new feature, we'd love to hear about it! [Open an issue](../../issues) and describe your suggestion in detail. 

### Contributing Code

1. Fork the repository and create your branch from `main`.
2. Make your changes and ensure they follow the project's coding style and conventions.
3. Write tests to cover your changes if applicable.
4. Ensure all tests pass by running `cargo test`.
5. Commit your changes with a clear and descriptive commit message.
6. Push your changes to your fork and [submit a pull request](../../pulls) to the `main` branch of the main repository.

## Development Setup

To set up your development environment:

1. Install Rust nightly toolchain using [rustup](https://rustup.rs/).
2. Install the `x86_64-unknown-none` target using `rustup target add x86_64-unknown-none`.
3. Install bootimage tool by running `cargo install bootimage`.

## Code Style

We follow the Rust community's [official style guidelines](https://github.com/rust-lang/rfcs/blob/master/text/1688-style-guide.md). Please ensure your code adheres to these guidelines.

## License

By contributing to this project, you agree that your contributions will be licensed under the project's [MIT License](./LICENSE).

If you have any questions or need further assistance, feel free to reach out to the maintainers. Happy coding!
