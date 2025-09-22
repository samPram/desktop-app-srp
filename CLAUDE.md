# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Development Commands
- Run application: `cargo run`
- Run with auto-reload: `cargo watch -x run` (requires `cargo install cargo-watch`)
- Build project: `cargo build`
- Build optimized release: `cargo run --release`

### Code Quality
- Format code: `cargo fmt`
- Lint code: `cargo clippy`
- Security audit: `cargo audit` (requires `cargo install cargo-audit`)

### Testing
- Run all tests: `cargo test`
- Run integration tests: `cargo test --test integration_tests`
- Run with output: `cargo test -- --nocapture`

## Architecture Overview

This is a Rust-based desktop application for motorcycle dynamometer testing built with the egui immediate-mode GUI framework.

### Application Structure
- **State Management**: Uses enum-based state machine (`AppState`) to switch between Login and Dashboard views
- **UI Framework**: egui with eframe for native window management
- **Component Architecture**: Modular UI components in `src/ui/components/`
- **Data Models**: Centralized data structures in `src/core/models.rs`

### Key Components
- **DesktopApp**: Main application controller with state management and menu system
- **LoginForm**: Handles operator authentication
- **Dashboard**: Main testing interface with real-time data display
- **Gauge Components**: Circular gauges and value displays for sensor data
- **Data Models**: Operator, Customer, Motorcycle, TestSession, and TestParameters

### Data Flow
1. Application starts in Login state
2. Successful login transitions to Dashboard state with authenticated Operator
3. Dashboard manages test sessions and real-time parameter updates
4. State persistence through eframe storage system

### Configuration
- Window size: 1200x800 (min: 800x600)
- Custom styling: Larger fonts (16px body, 24px headings) and spacing for workshop environment
- Persistence enabled for app state restoration

### Testing Strategy
- Unit tests for individual components
- Integration tests for complete workflows
- UI tests planned for future implementation
- Basic data validation tests included