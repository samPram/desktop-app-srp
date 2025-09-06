# Project Guidelines - Desktop App (Rust + egui)

This project is a desktop application built using Rust and the egui framework. Follow these guidelines when working on this project.

## Project Structure

```
desktop-app/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application logic
│   ├── ui/                  # UI components and layouts
│   │   ├── mod.rs
│   │   ├── main_window.rs
│   │   └── components/      # Reusable UI components
│   ├── core/                # Business logic and data models
│   │   ├── mod.rs
│   │   └── models.rs
│   └── utils/               # Utility functions and helpers
├── assets/                  # Static assets (icons, images, etc.)
├── tests/                   # Integration tests
├── docs/                    # Documentation
├── Cargo.toml              # Project configuration and dependencies
└── README.md               # Project overview
```

## Dependencies Management

### Core Dependencies
- `egui` - Immediate mode GUI framework
- `eframe` - Framework for egui applications
- `serde` - Serialization/deserialization (for settings, data)
- `tokio` - Async runtime (if needed for async operations)

### Development Dependencies
- `cargo-watch` - Auto-rebuild during development
- `cargo-audit` - Security vulnerability scanning

## Development Workflow

### Setup
1. Install Rust toolchain (latest stable)
2. Install development tools: `cargo install cargo-watch cargo-audit`
3. Clone and build: `cargo build`

### Running the Application
- Development: `cargo run`
- With auto-reload: `cargo watch -x run`
- Release build: `cargo run --release`

### Code Style and Formatting
- Use `rustfmt` for code formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Maximum line length: 100 characters
- Use meaningful variable and function names

## Testing Strategy

### Unit Tests
- Place unit tests in the same file as the code being tested using `#[cfg(test)]`
- Test business logic in `src/core/` modules
- Run tests: `cargo test`

### Integration Tests
- Place integration tests in `tests/` directory
- Test complete user workflows and UI interactions
- Use egui's testing utilities for UI testing

### Running Tests
- All tests: `cargo test`
- Specific test: `cargo test test_name`
- With output: `cargo test -- --nocapture`

## Build and Deployment

### Development Build
- `cargo build` - Debug build with symbols
- `cargo run` - Build and run in debug mode

### Release Build
- `cargo build --release` - Optimized release build
- `cargo run --release` - Build and run in release mode

### Platform-Specific Builds
- Windows: Ensure proper icon and manifest files
- macOS: Consider app bundle creation
- Linux: Test on different distributions

## egui-Specific Guidelines

### UI Design Principles
- Keep UI responsive and immediate
- Use egui's built-in widgets when possible
- Implement custom widgets only when necessary
- Follow platform UI conventions

### State Management
- Use `egui::Context` for UI state
- Separate business logic from UI logic
- Use `Arc<Mutex<T>>` for shared state between threads
- Implement `Default` for application state structs

### Performance Considerations
- Minimize allocations in UI code
- Use `egui::util::cache` for expensive computations
- Profile with `cargo flamegraph` if performance issues arise

## Error Handling
- Use `Result<T, E>` for fallible operations
- Create custom error types using `thiserror` crate
- Display user-friendly error messages in UI
- Log errors appropriately for debugging

## Configuration and Settings
- Store user settings in platform-appropriate directories
- Use `serde` for serializing/deserializing settings
- Provide sensible defaults for all settings
- Allow runtime configuration changes through UI

## Security Considerations
- Validate all user inputs
- Use `cargo audit` regularly to check for vulnerabilities
- Be cautious with file system operations
- Sanitize data before display

## Documentation
- Document public APIs with rustdoc comments
- Keep README.md updated with build and usage instructions
- Document complex algorithms and business logic
- Include examples in documentation

## Version Control
- Use meaningful commit messages
- Keep commits atomic and focused
- Use feature branches for new functionality
- Tag releases with semantic versioning

## Before Submitting Changes
1. Run `cargo fmt` to format code
2. Run `cargo clippy` and fix warnings
3. Run `cargo test` to ensure all tests pass
4. Run `cargo build --release` to ensure release build works
5. Test the application manually for basic functionality
