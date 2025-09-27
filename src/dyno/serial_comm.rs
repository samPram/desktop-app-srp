use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use serialport::{SerialPort, SerialPortType};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use rand::Rng;

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
            rpm: 0.0,
            speed: 0.0,
            oil_temp: 20.0,
            torque: 0.0,
            horsepower: 0.0,
            afr: 14.7,
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
    port: Option<Box<dyn SerialPort>>,
    data: Arc<Mutex<ArduinoData>>,
    status: Arc<Mutex<ConnectionStatus>>,
    running: Arc<Mutex<bool>>,
    simulation_mode: Arc<Mutex<bool>>,
    last_update: Arc<Mutex<Instant>>,
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
        }
    }

    pub fn list_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|port| {
                match port.port_type {
                    SerialPortType::UsbPort(_) => Some(port.port_name),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn connect(&mut self, port_name: &str) -> Result<(), SerialError> {
        *self.status.lock().unwrap() = ConnectionStatus::Connecting;

        let port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(1000))
            .open()
            .map_err(|e| SerialError::OpenFailed(e.to_string()))?;

        self.port = Some(port);
        *self.status.lock().unwrap() = ConnectionStatus::Connected;
        *self.running.lock().unwrap() = true;

        self.start_reading_thread();
        Ok(())
    }

    pub fn disconnect(&mut self) {
        *self.running.lock().unwrap() = false;
        self.port = None;
        *self.status.lock().unwrap() = ConnectionStatus::Disconnected;
    }

    pub fn start_simulation(&mut self) {
        *self.simulation_mode.lock().unwrap() = true;
        *self.running.lock().unwrap() = true;
        *self.status.lock().unwrap() = ConnectionStatus::Connected;
        self.start_simulation_thread();
    }

    pub fn stop_simulation(&mut self) {
        *self.simulation_mode.lock().unwrap() = false;
        *self.running.lock().unwrap() = false;
        *self.status.lock().unwrap() = ConnectionStatus::Disconnected;
    }

    pub fn send_command(&mut self, command: &str) -> Result<(), SerialError> {
        if let Some(ref mut port) = self.port {
            let cmd_bytes = format!("{}\n", command).into_bytes();
            port.write_all(&cmd_bytes)
                .map_err(|e| SerialError::CommunicationError(e.to_string()))?;
            Ok(())
        } else {
            Err(SerialError::CommunicationError("No port connected".to_string()))
        }
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

        thread::spawn(move || {
            let mut buffer = String::new();
            
            while *running.lock().unwrap() {
                // Simulate reading from serial port (since we can't move the port)
                // In real implementation, this would read from the actual port
                thread::sleep(Duration::from_millis(100));
                
                // Check for timeout
                let now = Instant::now();
                if now.duration_since(*last_update.lock().unwrap()) > Duration::from_secs(2) {
                    *status.lock().unwrap() = ConnectionStatus::Error("No data received".to_string());
                }
            }
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

                // Generate realistic dyno test data with some variation
                let base_rpm = 6000.0 + (time * 0.5).sin() * 1000.0;
                let base_speed = 80.0 + (time * 0.3).sin() * 15.0;
                let base_torque = 90.0 + (time * 0.4).cos() * 10.0;
                let base_hp = base_torque * base_rpm / 5252.0; // HP = Torque * RPM / 5252

                let sim_data = ArduinoData {
                    rpm: (base_rpm + rng.gen_range(-50.0..50.0)).max(0.0),
                    speed: (base_speed + rng.gen_range(-5.0..5.0)).max(0.0),
                    oil_temp: 85.0 + rng.gen_range(-2.0..2.0),
                    torque: (base_torque + rng.gen_range(-5.0..5.0)).max(0.0),
                    horsepower: (base_hp + rng.gen_range(-5.0..5.0)).max(0.0),
                    afr: 14.7 + rng.gen_range(-0.5..0.5),
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
                            oil_temp: json["oilTemp"].as_f64().unwrap_or(20.0) as f32,
                            torque: json["torque"].as_f64().unwrap_or(0.0) as f32,
                            horsepower: json["horsepower"].as_f64().unwrap_or(0.0) as f32,
                            afr: json["afr"].as_f64().unwrap_or(14.7) as f32,
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