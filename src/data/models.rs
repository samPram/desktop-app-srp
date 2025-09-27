use std::time::{Duration, Instant};
use crate::dyno::serial_comm::{ArduinoData, ConnectionStatus};
use log::debug;

#[derive(Debug, Clone)]
pub struct DynoData {
    pub rpm: f32,
    pub speed_kmh: f32,
    pub horsepower: f32,
    pub torque: f32,
    pub air_fuel_ratio: f32,
    pub lambda: f32,
    pub intake_temp: f32,
    pub coolant_temp: f32,
    pub oil_pressure: f32,
    pub peak_hp: f32,
    pub peak_torque: f32,
    pub run_id: String,
    pub test_date: String,
    pub test_time: String,
    pub motorcycle: String,
    pub power_curve: Vec<(f32, f32)>,  // (RPM, HP)
    pub torque_curve: Vec<(f32, f32)>, // (RPM, Torque)
    pub oil_pressure_temp: f32,
    last_update: Instant,
    start_time: Instant,
    pub is_simulation_running: bool,
    pub is_test_running: bool,
    pub connection_status: String,
    pub is_hardware_connected: bool,
    // Speed smoothing to prevent flickering to 0
    prev_speed: f32,
    speed_smooth_factor: f32,
}

impl DynoData {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            rpm: 1000.0, // Start at idle
            speed_kmh: 15.0,
            horsepower: 10.0,
            torque: 25.0,
            air_fuel_ratio: 13.2,
            lambda: 0.90,
            intake_temp: 25.3,
            coolant_temp: 98.5,
            oil_pressure: 6590.0,
            peak_hp: 10.0,
            peak_torque: 25.0,
            run_id: "#008".to_string(),
            test_date: "Date: 2024.07.22".to_string(),
            test_time: "Time: Yamaha R1".to_string(),
            motorcycle: "Motor: YZF-R1 00:32:10".to_string(),
            power_curve: Vec::new(), // Start empty for simulation
            torque_curve: Vec::new(), // Start empty for simulation
            last_update: now,
            start_time: now,
            oil_pressure_temp: 65.0,
            is_simulation_running: false,
            is_test_running: false,
            connection_status: "Disconnected".to_string(),
            is_hardware_connected: false,
            prev_speed: 0.0,
            speed_smooth_factor: 0.8, // Higher value = more smoothing
        }
    }

    pub fn update(&mut self) {
        // Only update if simulation is running and test is active
        if !self.is_simulation_running || !self.is_test_running {
            return;
        }
        
        let now = Instant::now();
        if now.duration_since(self.last_update) > Duration::from_millis(100) {
            let elapsed_total = now.duration_since(self.start_time).as_secs_f32();
            
            // Simulate a realistic dyno test progression
            let test_duration = 30.0; // 30 second test
            let test_progress = (elapsed_total % test_duration) / test_duration;
            
            // Progressive RPM increase during test
            let base_rpm = if test_progress < 0.8 {
                // Accelerating phase: gradually increase RPM
                1000.0 + (test_progress / 0.8) * 10000.0 // 1000 to 11000 RPM
            } else {
                // Deceleration phase: gradual decrease
                11000.0 - ((test_progress - 0.8) / 0.2) * 2000.0 // 11000 to 9000 RPM
            };
            
            // Add realistic engine variations
            let rpm_noise = (elapsed_total * 3.0).sin() * 30.0 + (elapsed_total * 7.0).cos() * 15.0;
            self.rpm = (base_rpm + rpm_noise).max(1000.0).min(12000.0);
            
            // Calculate realistic HP and torque based on RPM
            let rpm_factor = (self.rpm - 1000.0) / 11000.0; // 0 to 1
            
            // Realistic motorcycle power curve
            self.horsepower = if rpm_factor < 0.3 {
                10.0 + rpm_factor * 150.0 // Low end: 10-55 HP
            } else if rpm_factor < 0.7 {
                55.0 + (rpm_factor - 0.3) * 250.0 // Mid range: 55-155 HP
            } else {
                155.0 - (rpm_factor - 0.7) * 50.0 // High end: 155-140 HP
            };
            
            // Add power variations
            self.horsepower += (elapsed_total * 2.0).sin() * 3.0;
            self.horsepower = self.horsepower.max(8.0);
            
            // Realistic motorcycle torque curve
            self.torque = if rpm_factor < 0.4 {
                25.0 + rpm_factor * 125.0 // Low end: 25-75 Nm
            } else if rpm_factor < 0.6 {
                75.0 + (rpm_factor - 0.4) * 75.0 // Peak: 75-90 Nm
            } else {
                90.0 - (rpm_factor - 0.6) * 50.0 // High end: 90-70 Nm
            };
            
            // Add torque variations
            self.torque += (elapsed_total * 1.8).cos() * 2.0;
            self.torque = self.torque.max(20.0);
            
            // Speed calculation based on RPM
            self.speed_kmh = (self.rpm / 80.0) + (elapsed_total * 0.5).sin() * 1.5;

            // Update peak values
            if self.horsepower > self.peak_hp {
                self.peak_hp = self.horsepower;
            }
            if self.torque > self.peak_torque {
                self.peak_torque = self.torque;
            }

            // Add data points to curves using RPM (for proper dyno chart)
            self.power_curve.push((self.rpm, self.horsepower));
            self.torque_curve.push((self.rpm, self.torque));

            // Keep curves manageable - remove old points and sort by RPM
            if self.power_curve.len() > 200 {
                self.power_curve.remove(0);
            }
            if self.torque_curve.len() > 200 {
                self.torque_curve.remove(0);
            }
            
            // Sort by RPM to ensure proper line drawing
            self.power_curve.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            self.torque_curve.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            self.last_update = now;
        }
    }

    pub fn update_from_arduino(&mut self, arduino_data: &ArduinoData) {
        self.rpm = arduino_data.rpm;
        
        // Smart speed smoothing to prevent flickering to 0
        let raw_speed = arduino_data.speed;
        if raw_speed > 0.0 {
            // Normal speed update with light smoothing
            self.speed_kmh = if self.prev_speed > 0.0 {
                self.prev_speed * self.speed_smooth_factor + raw_speed * (1.0 - self.speed_smooth_factor)
            } else {
                raw_speed
            };
        } else if self.prev_speed > 5.0 {
            // If speed suddenly drops to 0 but was previously > 5 km/h, apply gradual decay
            self.speed_kmh = self.prev_speed * 0.9;
            if self.speed_kmh < 0.5 {
                self.speed_kmh = 0.0;
            }
        } else {
            // Speed was already low, accept the 0
            self.speed_kmh = 0.0;
        }
        
        // Debug logging for speed changes
        if (self.speed_kmh - raw_speed).abs() > 1.0 {
            debug!("Speed smoothing: raw={:.1} -> smooth={:.1} (prev={:.1})", 
                   raw_speed, self.speed_kmh, self.prev_speed);
        }
        
        self.prev_speed = self.speed_kmh;
        
        self.horsepower = arduino_data.horsepower;
        self.torque = arduino_data.torque;
        self.air_fuel_ratio = arduino_data.afr;
        self.coolant_temp = arduino_data.oil_temp; // Using oil temp as coolant temp
        self.oil_pressure_temp = arduino_data.oil_temp;

        // Update peak values if current values are higher
        if self.horsepower > self.peak_hp {
            self.peak_hp = self.horsepower;
        }
        if self.torque > self.peak_torque {
            self.peak_torque = self.torque;
        }

        // Add data points to curves using RPM (for proper dyno chart)
        if self.rpm > 0.0 {
            self.power_curve.push((self.rpm, self.horsepower));
            self.torque_curve.push((self.rpm, self.torque));

            // Keep curves manageable - remove old points and sort by RPM
            if self.power_curve.len() > 200 {
                self.power_curve.remove(0);
            }
            if self.torque_curve.len() > 200 {
                self.torque_curve.remove(0);
            }
            
            // Sort by RPM to ensure proper line drawing
            self.power_curve.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            self.torque_curve.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        }

        self.last_update = Instant::now();
    }

    pub fn start_simulation(&mut self) {
        self.is_simulation_running = true;
        self.start_time = Instant::now(); // Reset start time when simulation starts
    }

    pub fn stop_simulation(&mut self) {
        self.is_simulation_running = false;
        self.is_test_running = false;
    }

    pub fn start_test(&mut self) {
        if self.is_simulation_running {
            self.is_test_running = true;
            self.start_time = Instant::now(); // Reset start time when test starts
            // Reset peak values for new test
            self.peak_hp = self.horsepower;
            self.peak_torque = self.torque;
            // Clear previous test data
            self.power_curve.clear();
            self.torque_curve.clear();
        }
    }

    pub fn stop_test(&mut self) {
        self.is_test_running = false;
    }

    pub fn reset_data(&mut self) {
        self.is_simulation_running = false;
        self.is_test_running = false;
        self.power_curve.clear();
        self.torque_curve.clear();
        self.peak_hp = 10.0;
        self.peak_torque = 25.0;
        self.rpm = 1000.0;
        self.speed_kmh = 15.0;
        self.horsepower = 10.0;
        self.torque = 25.0;
        self.start_time = Instant::now();
        // Reset speed smoothing
        self.prev_speed = 0.0;
    }

    pub fn update_connection_status(&mut self, status: &ConnectionStatus, is_simulation: bool) {
        match status {
            ConnectionStatus::Disconnected => {
                self.connection_status = "Disconnected".to_string();
                self.is_hardware_connected = false;
            }
            ConnectionStatus::Connecting => {
                self.connection_status = "Connecting...".to_string();
                self.is_hardware_connected = false;
            }
            ConnectionStatus::Connected => {
                if is_simulation {
                    self.connection_status = "Simulation Mode".to_string();
                    self.is_hardware_connected = false;
                } else {
                    self.connection_status = "Connected to Hardware".to_string();
                    self.is_hardware_connected = true;
                }
            }
            ConnectionStatus::Error(msg) => {
                self.connection_status = format!("Error: {}", msg);
                self.is_hardware_connected = false;
            }
        }
    }

    fn generate_power_curve() -> Vec<(f32, f32)> {
        let mut curve = Vec::new();
        for rpm in (1000..=12000).step_by(500) {
            let rpm_f = rpm as f32;
            // Simulate realistic power curve for a motorcycle
            let normalized_rpm = (rpm_f - 1000.0) / 11000.0;
            let power = if normalized_rpm < 0.3 {
                20.0 + normalized_rpm * 150.0
            } else if normalized_rpm < 0.7 {
                65.0 + (normalized_rpm - 0.3) * 150.0
            } else {
                125.0 - (normalized_rpm - 0.7) * 80.0
            };
            curve.push((rpm_f, power));
        }
        curve
    }

    fn generate_torque_curve() -> Vec<(f32, f32)> {
        let mut curve = Vec::new();
        for rpm in (1000..=12000).step_by(500) {
            let rpm_f = rpm as f32;
            let normalized_rpm = (rpm_f - 1000.0) / 11000.0;
            let torque = if normalized_rpm < 0.5 {
                30.0 + normalized_rpm * 120.0
            } else {
                90.0 - (normalized_rpm - 0.5) * 60.0
            };
            curve.push((rpm_f, torque));
        }
        curve
    }
}
