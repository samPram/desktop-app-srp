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
dekstop-app/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
├── build.rs                    # Build script for resources
├── icon.ico                    # Application icon
│
├── src/
│   ├── main.rs                 # Entry point
│   ├── lib.rs                  # Library root
│   ├── app.rs                  # Main application struct
│   │
│   ├── ui/                     # UI components
│   │   ├── mod.rs
│   │   ├── dashboard.rs        # Main dashboard view
│   │   ├── sidebar.rs          # Navigation sidebar
│   │   ├── gauges.rs           # RPM and speed gauges
│   │   ├── charts.rs           # Power/torque charts
│   │   ├── data_panel.rs       # Live data & sensor readings
│   │   ├── run_controls.rs     # Test run controls
│   │   └── styles.rs           # UI styling and themes
│   │
│   ├── dyno/                   # Dyno hardware interface
│   │   ├── mod.rs
│   │   ├── hardware.rs         # Hardware abstraction
│   │   ├── sensors.rs          # Sensor data handling
│   │   ├── calibration.rs      # Calibration routines
│   │   └── serial_comm.rs      # Serial communication
│   │
│   ├── data/                   # Data management
│   │   ├── mod.rs
│   │   ├── models.rs           # Data structures
│   │   ├── storage.rs          # Data persistence
│   │   ├── calculations.rs     # Power/torque calculations
│   │   ├── run_manager.rs      # Test run management
│   │   └── export.rs           # Data export functionality
│   │
│   ├── config/                 # Configuration
│   │   ├── mod.rs
│   │   ├── settings.rs         # Application settings
│   │   ├── dyno_config.rs      # Dyno-specific configuration
│   │   └── defaults.rs         # Default values
│   │
│   └── utils/                  # Utilities
│       ├── mod.rs
│       ├── math.rs             # Mathematical utilities
│       ├── units.rs            # Unit conversions
│       └── time.rs             # Time utilities
│
├── assets/                     # Static assets
│   ├── fonts/                  # Custom fonts
│   ├── icons/                  # UI icons
│   └── themes/                 # Color themes
│
├── data/                       # Application data
│   ├── runs/                   # Saved test runs
│   ├── configs/                # Configuration files
│   └── exports/                # Exported data
│
├── docs/                       # Documentation
│   ├── API.md
│   ├── HARDWARE.md
│   └── USER_GUIDE.md
│
└── tests/                      # Tests
    ├── integration/
    ├── unit/
    └── fixtures/
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