use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use serialport::{SerialPort, SerialPortType};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use rand::Rng;
use log::{info, warn, error, debug};

#[derive(Debug, Error)]
pub enum SerialError {
    #[error("Port not found: {0}")]
    PortNotFound(String),
    #[error("Failed to open port: {0}")]
    OpenFailed(String),
    #[error("Communication error: {0}")]
    CommunicationError(String),
    #[error("Data parsing error: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArduinoData {
    pub rpm: f32,
    pub speed: f32,
    pub oil_temp: f32,
    pub torque: f32,
    pub horsepower: f32,
    pub afr: f32,
}

impl Default for ArduinoData {
    fn default() -> Self {
        Self {
            rpm: 0.0,          // 0-12000 RPM range for 110cc-250cc motors
            speed: 0.0,        // 0-200 km/h max speed
            oil_temp: 0.0,     // 0.0 when sensor disabled, actual range -40 to 200°C
            torque: 0.0,       // 0-150 Nm max torque (clamped in Arduino)
            horsepower: 0.0,   // 0-200 HP max power (clamped in Arduino)
            afr: 0.0,          // 0.0 when sensor disabled, actual range 8.0-25.0
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

pub struct SerialConnection {
    port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
    data: Arc<Mutex<ArduinoData>>,
    status: Arc<Mutex<ConnectionStatus>>,
    running: Arc<Mutex<bool>>,
    simulation_mode: Arc<Mutex<bool>>,
    last_update: Arc<Mutex<Instant>>,
    port_name: Arc<Mutex<String>>,
}

impl SerialConnection {
    pub fn new() -> Self {
        Self {
            port: None,
            data: Arc::new(Mutex::new(ArduinoData::default())),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            running: Arc::new(Mutex::new(false)),
            simulation_mode: Arc::new(Mutex::new(false)),
            last_update: Arc::new(Mutex::new(Instant::now())),
            port_name: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn list_ports() -> Vec<String> {
        info!("Scanning for available serial ports...");
        
        let ports = match serialport::available_ports() {
            Ok(ports) => ports,
            Err(e) => {
                error!("Failed to list serial ports: {}", e);
                return Vec::new();
            }
        };
        
        // Log all ports first for debugging
        info!("All available ports:");
        for port in &ports {
            info!("  Port: {} (type: {:?})", port.port_name, port.port_type);
        }
        
        let usb_ports: Vec<String> = ports
            .into_iter()
            .filter_map(|port| {
                debug!("Processing port: {} (type: {:?})", port.port_name, port.port_type);
                match port.port_type {
                    SerialPortType::UsbPort(info) => {
                        info!("USB serial port found: {} (VID: {:04x}, PID: {:04x})", 
                             port.port_name, info.vid, info.pid);
                        Some(port.port_name)
                    },
                    _ => {
                        debug!("Skipping non-USB port: {}", port.port_name);
                        None
                    }
                }
            })
            .collect();
            
        info!("Found {} USB serial ports: {:?}", usb_ports.len(), usb_ports);
        
        // If no USB ports found, also check if there are any other serial ports
        if usb_ports.is_empty() {
            warn!("No USB serial ports found! You may need to:");
            warn!("1. Connect your Arduino");
            warn!("2. Check if device appears as /dev/ttyACM* or /dev/ttyUSB*");
            warn!("3. Verify permissions: ls -la /dev/ttyUSB* /dev/ttyACM*");
            warn!("4. Add user to dialout group: sudo usermod -a -G dialout $USER");
        }
        
        usb_ports
    }

    pub fn connect(&mut self, port_name: &str) -> Result<(), SerialError> {
        info!("Attempting to connect to serial port: {}", port_name);
        
        // Ensure any existing connection is properly closed first
        self.disconnect();
        
        *self.status.lock().unwrap() = ConnectionStatus::Connecting;
        *self.port_name.lock().unwrap() = port_name.to_string();

        // First check if the port exists and get detailed error info
        let available_ports = Self::list_ports();
        if !available_ports.contains(&port_name.to_string()) {
            let error_msg = format!("Port {} not found in available ports: {:?}", port_name, available_ports);
            error!("{}", error_msg);
            *self.status.lock().unwrap() = ConnectionStatus::Error(error_msg.clone());
            return Err(SerialError::OpenFailed(error_msg));
        }

        info!("Opening serial port {} with 115200 baud, 8N1 configuration", port_name);
        
        // Test if port can be opened with proper timeout
        let port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(2000))
            .data_bits(serialport::DataBits::Eight)
            .flow_control(serialport::FlowControl::None)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .open()
            .map_err(|e| {
                let detailed_error = format!("Failed to open port {}: {} (Error kind: {:?})", port_name, e, e.kind());
                error!("{}", detailed_error);
                
                // Check common error causes
                let user_friendly_error = match e.kind() {
                    serialport::ErrorKind::NoDevice => {
                        format!("Device not found. Make sure Arduino is connected to {}", port_name)
                    },
                    serialport::ErrorKind::InvalidInput => {
                        format!("Invalid port configuration for {}", port_name)
                    },
                    _ => {
                        // Check if error message contains specific error types
                        let error_str = e.to_string().to_lowercase();
                        if error_str.contains("permission denied") {
                            format!("Permission denied. Run: sudo usermod -a -G dialout $USER, then logout/login")
                        } else if error_str.contains("device or resource busy") {
                            format!("Port {} is busy. Close Arduino IDE or other serial programs, then try again", port_name)
                        } else if error_str.contains("no such file or directory") {
                            format!("Port {} not found. Check if device is connected and try refreshing port list", port_name)
                        } else {
                            format!("Port error: {} ({:?})", e, e.kind())
                        }
                    }
                };
                
                error!("User-friendly error: {}", user_friendly_error);
                *self.status.lock().unwrap() = ConnectionStatus::Error(user_friendly_error.clone());
                SerialError::OpenFailed(user_friendly_error)
            })?;

        info!("Successfully opened port {}", port_name);
        self.port = Some(Arc::new(Mutex::new(port)));
        *self.running.lock().unwrap() = true;
        
        info!("Waiting 2 seconds for Arduino to reset...");
        // Give Arduino time to reset after connection
        thread::sleep(Duration::from_millis(2000));

        info!("Starting reading thread for port {}", port_name);
        self.start_reading_thread();
        
        // Set status to connected only after successful thread start
        *self.status.lock().unwrap() = ConnectionStatus::Connected;
        info!("Successfully connected to {}", port_name);
        Ok(())
    }

    pub fn disconnect(&mut self) {
        let port_name = self.port_name.lock().unwrap().clone();
        info!("Disconnecting from port: {}", port_name);
        
        // Stop the reading thread first
        *self.running.lock().unwrap() = false;
        
        // Give reading thread time to exit gracefully
        if self.port.is_some() {
            thread::sleep(Duration::from_millis(100));
        }
        
        // Close the port
        if let Some(port) = self.port.take() {
            // Try to flush and close cleanly
            if let Ok(mut port_lock) = port.lock() {
                let _ = port_lock.flush();
                info!("Port {} flushed and closed", port_name);
            }
        }
        
        *self.status.lock().unwrap() = ConnectionStatus::Disconnected;
        
        info!("Successfully disconnected from {}", port_name);
    }

    pub fn start_simulation(&mut self) {
        info!("Starting simulation mode");
        *self.simulation_mode.lock().unwrap() = true;
        *self.running.lock().unwrap() = true;
        *self.status.lock().unwrap() = ConnectionStatus::Connected;
        self.start_simulation_thread();
        info!("Simulation mode started successfully");
    }

    pub fn stop_simulation(&mut self) {
        *self.simulation_mode.lock().unwrap() = false;
        *self.running.lock().unwrap() = false;
        *self.status.lock().unwrap() = ConnectionStatus::Disconnected;
    }

    pub fn send_command(&mut self, command: &str) -> Result<(), SerialError> {
        if let Some(ref port) = self.port {
            let cmd_bytes = format!("{}\n", command).into_bytes();
            let mut port_lock = port.lock().unwrap();
            port_lock.write_all(&cmd_bytes)
                .map_err(|e| SerialError::CommunicationError(e.to_string()))?;
            info!("Sent command to Arduino: {}", command);
            Ok(())
        } else {
            Err(SerialError::CommunicationError("No port connected".to_string()))
        }
    }

    /// Send STOP command to Arduino to stop dyno testing
    pub fn stop_dyno_test(&mut self) -> Result<(), SerialError> {
        self.send_command("STOP")
    }

    pub fn get_data(&self) -> ArduinoData {
        self.data.lock().unwrap().clone()
    }

    pub fn get_status(&self) -> ConnectionStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn is_simulation_mode(&self) -> bool {
        *self.simulation_mode.lock().unwrap()
    }

    fn start_reading_thread(&self) {
        let data = Arc::clone(&self.data);
        let status = Arc::clone(&self.status);
        let running = Arc::clone(&self.running);
        let last_update = Arc::clone(&self.last_update);
        let port_name = Arc::clone(&self.port_name);
        let port = match &self.port {
            Some(p) => Arc::clone(p),
            None => {
                error!("No port available for reading thread");
                return;
            }
        };

        thread::spawn(move || {
            let port_name_str = port_name.lock().unwrap().clone();
            info!("Reading thread starting for port: {}", port_name_str);

            let mut line = String::new();
            let mut line_count = 0;
            
            info!("Reading thread entering main loop for {}", port_name_str);
            
            while *running.lock().unwrap() {
                line.clear();
                
                let read_result = {
                    let mut port_lock = port.lock().unwrap();
                    let mut reader = BufReader::new(&mut **port_lock);
                    reader.read_line(&mut line)
                };
                
                match read_result {
                    Ok(0) => {
                        // EOF - connection lost
                        warn!("EOF detected on port {} - connection lost", port_name_str);
                        *status.lock().unwrap() = ConnectionStatus::Error("Connection lost".to_string());
                        break;
                    }
                    Ok(bytes_read) => {
                        // Successfully read a line
                        line_count += 1;
                        let trimmed_line = line.trim();
                        
                        if line_count % 50 == 1 {  // Log every 50th line to avoid spam
                            debug!("Read line {} ({} bytes): {}", line_count, bytes_read, trimmed_line);
                        }
                        
                        // Try to parse Arduino data
                        match Self::parse_arduino_json(trimmed_line) {
                            Ok(arduino_data) => {
                                debug!("Successfully parsed Arduino data: RPM={}, HP={}, Torque={}", 
                                       arduino_data.rpm, arduino_data.horsepower, arduino_data.torque);
                                *data.lock().unwrap() = arduino_data;
                                *last_update.lock().unwrap() = Instant::now();
                            }
                            Err(e) => {
                                if line_count % 100 == 1 {  // Log parse errors occasionally
                                    debug!("Failed to parse line as Arduino JSON: {} (Error: {})", trimmed_line, e);
                                }
                            }
                        }
                        
                        // Check for timeout (no valid data received)
                        let now = Instant::now();
                        if now.duration_since(*last_update.lock().unwrap()) > Duration::from_secs(5) {
                            warn!("No valid data received for 5 seconds on port {}", port_name_str);
                            *status.lock().unwrap() = ConnectionStatus::Error("No valid data received".to_string());
                        }
                    }
                    Err(e) => {
                        // Read error
                        if e.kind() == std::io::ErrorKind::TimedOut {
                            // Timeout is normal, continue reading
                            if line_count % 1000 == 0 {  // Log timeout occasionally
                                debug!("Read timeout on port {} (normal, continuing...)", port_name_str);
                            }
                            continue;
                        } else {
                            error!("Read error on port {}: {}", port_name_str, e);
                            *status.lock().unwrap() = ConnectionStatus::Error(format!("Read error: {}", e));
                            break;
                        }
                    }
                }
            }
            
            info!("Reading thread exiting for port: {}", port_name_str);
        });
    }

    fn start_simulation_thread(&self) {
        let data = Arc::clone(&self.data);
        let running = Arc::clone(&self.running);
        let simulation_mode = Arc::clone(&self.simulation_mode);
        let last_update = Arc::clone(&self.last_update);

        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut time = 0.0f32;

            while *running.lock().unwrap() && *simulation_mode.lock().unwrap() {
                time += 0.1;

                // Generate realistic dyno test data for 110cc-250cc motor with some variation
                let base_rpm = 3000.0 + (time * 0.5).sin() * 2000.0;  // 1000-5000 RPM typical range
                let base_speed = 60.0 + (time * 0.3).sin() * 25.0;    // 35-85 km/h typical range
                let base_torque = 15.0 + (time * 0.4).cos() * 8.0;    // 7-23 Nm for small motors
                let base_hp = base_torque * base_rpm / 5252.0; // HP = Torque * RPM / 5252

                let sim_data = ArduinoData {
                    rpm: (base_rpm + rng.gen_range(-100.0..100.0)).max(0.0).min(12000.0),
                    speed: (base_speed + rng.gen_range(-8.0..8.0)).max(0.0).min(200.0),
                    oil_temp: 85.0 + rng.gen_range(-5.0..5.0),  // Simulate oil temp when enabled
                    torque: (base_torque + rng.gen_range(-2.0..2.0)).max(0.0).min(150.0),
                    horsepower: (base_hp + rng.gen_range(-1.0..1.0)).max(0.0).min(200.0),
                    afr: 14.7 + rng.gen_range(-1.0..1.0),  // Simulate AFR when enabled
                };

                *data.lock().unwrap() = sim_data;
                *last_update.lock().unwrap() = Instant::now();

                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn parse_arduino_json(line: &str) -> Result<ArduinoData, SerialError> {
        // Try to find JSON in the line (Arduino sends debug info + JSON)
        if let Some(start) = line.find('{') {
            let json_part = &line[start..];
            if let Some(end) = json_part.find('}') {
                let json_str = &json_part[..=end];
                
                // Parse the JSON
                let parsed: Result<serde_json::Value, _> = serde_json::from_str(json_str);
                match parsed {
                    Ok(json) => {
                        let data = ArduinoData {
                            rpm: json["rpm"].as_f64().unwrap_or(0.0) as f32,
                            speed: json["speed"].as_f64().unwrap_or(0.0) as f32,
                            oil_temp: json["oilTemp"].as_f64().unwrap_or(0.0) as f32,  // Changed default to 0.0 to match Arduino
                            torque: json["torque"].as_f64().unwrap_or(0.0) as f32,
                            horsepower: json["horsepower"].as_f64().unwrap_or(0.0) as f32,
                            afr: json["afr"].as_f64().unwrap_or(0.0) as f32,  // Changed default to 0.0 to match Arduino when disabled
                        };
                        Ok(data)
                    }
                    Err(e) => Err(SerialError::ParseError(format!("JSON parse error: {}", e))),
                }
            } else {
                Err(SerialError::ParseError("Incomplete JSON data".to_string()))
            }
        } else {
            Err(SerialError::ParseError("No JSON found in line".to_string()))
        }
    }
}