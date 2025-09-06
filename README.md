# Desktop Dyno Testing Application

A desktop application built with Rust and egui for motorcycle dynamometer testing and data management.

## Features

- Real-time dyno testing with live parameter monitoring
- Customer and motorcycle management
- Test session recording and data analysis
- Operator management system
- Data export and reporting capabilities

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

## Prerequisites

- Rust toolchain (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd desktop-app
   ```

2. Install development tools (optional but recommended):
   ```bash
   cargo install cargo-watch cargo-audit
   ```

3. Build the project:
   ```bash
   cargo build
   ```

## Usage

### Development

Run the application in development mode:
```bash
cargo run
```

Run with auto-reload during development:
```bash
cargo watch -x run
```

### Release

Build and run optimized release version:
```bash
cargo run --release
```

## Development

### Code Style

- Run formatter: `cargo fmt`
- Run linter: `cargo clippy`
- Run tests: `cargo test`

### Testing

- Unit tests: `cargo test`
- Integration tests: `cargo test --test integration_tests`
- Run with output: `cargo test -- --nocapture`

### Security

Run security audit:
```bash
cargo audit
```

## Dependencies

### Core Dependencies
- `egui` - Immediate mode GUI framework
- `eframe` - Framework for egui applications
- `serde` - Serialization/deserialization
- `thiserror` - Error handling
- `env_logger` - Logging
- `tokio` - Async runtime (optional)

## Contributing

1. Follow Rust naming conventions
2. Run `cargo fmt` before committing
3. Ensure `cargo clippy` passes without warnings
4. Add tests for new functionality
5. Update documentation as needed

## License

[Add your license information here]