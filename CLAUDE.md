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

## Serial Port Setup

### Environment Requirements
For ESP32/Arduino communication, ensure proper serial port access:

1. **User Permissions**: User must be in `dialout` group
   ```bash
   sudo usermod -a -G dialout $USER
   # Logout and login after adding to group
   ```

2. **Setup Script**: Run the provided setup script
   ```bash
   ./setup_serial.sh
   ```

3. **Supported Devices**:
   - ESP32 with CP210x USB bridge (VID: 10c4, PID: ea60)
   - Arduino Uno/Nano with FTDI or CH340 chips
   - Any device creating `/dev/ttyUSB*` or `/dev/ttyACM*`

### Troubleshooting Serial Issues
- **"Device or resource busy"**: Close Arduino IDE or other serial programs
- **"Permission denied"**: Check user is in dialout group
- **"Port not found"**: Reconnect USB device, check `lsusb` output
- **Driver issues**: Load CP210x driver with `sudo modprobe cp210x`

## Arduino Communication Protocol

### Data Format
The ESP32 sends JSON data every 100ms in this format:
```json
{"rpm":3500.1,"speed":75.50,"oilTemp":85.3,"torque":18.45,"horsepower":12.30,"afr":14.70}
```

### Motor Specifications (110cc-250cc)
- **RPM Range**: 0-12000 RPM (max 15000 RPM rejected)
- **Speed Range**: 0-200 km/h
- **Torque Range**: 0-150 Nm (clamped in Arduino)
- **Power Range**: 0-200 HP (clamped in Arduino)
- **Oil Temperature**: -40°C to 200°C (0.0 when sensor disabled)
- **AFR Range**: 8.0-25.0 (0.0 when sensor disabled)

### Commands
- **START**: Begin dyno testing
- **STOP**: Stop dyno testing
- **Manual control**: Hardware start/stop button on pin 14

### Sensor Configuration
- **Engine RPM**: Pin 27 (pickup coil/hall sensor)
- **Roller Speed**: Pin 25 (hall sensor/optocoupler)
- **Oil Temperature**: Pin 34 (analog, optional)
- **AFR**: Pin 35 (analog, optional)
- **Start/Stop**: Pin 14 (digital button)