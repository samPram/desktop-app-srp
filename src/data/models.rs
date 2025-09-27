use std::time::{Duration, Instant};
use crate::dyno::serial_comm::ArduinoData;

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
}

impl DynoData {
    pub fn new() -> Self {
        Self {
            rpm: 6850.0,
            speed_kmh: 95.2,
            horsepower: 125.7,
            torque: 98.3,
            air_fuel_ratio: 13.2,
            lambda: 0.90,
            intake_temp: 25.3,
            coolant_temp: 98.5,
            oil_pressure: 6590.0,
            peak_hp: 125.7,
            peak_torque: 98.3,
            run_id: "#008".to_string(),
            test_date: "Date: 2024.07.22".to_string(),
            test_time: "Time: Yamaha R1".to_string(),
            motorcycle: "Motor: YZF-R1 00:32:10".to_string(),
            power_curve: Self::generate_power_curve(),
            torque_curve: Self::generate_torque_curve(),
            last_update: Instant::now(),
            oil_pressure_temp: 65.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) > Duration::from_millis(100) {
            // Simulate slight variations in readings
            let time_factor = (now.elapsed().as_secs_f32() * 0.5).sin();
            self.rpm = 6850.0 + time_factor * 50.0;
            self.speed_kmh = 95.2 + time_factor * 2.0;
            self.horsepower = 125.7 + time_factor * 3.0;
            self.torque = 98.3 + time_factor * 2.0;

            self.last_update = now;
        }
    }

    pub fn update_from_arduino(&mut self, arduino_data: &ArduinoData) {
        self.rpm = arduino_data.rpm;
        self.speed_kmh = arduino_data.speed;
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

        // Update power and torque curves with new data points
        if self.rpm > 0.0 {
            self.power_curve.push((self.rpm, self.horsepower));
            self.torque_curve.push((self.rpm, self.torque));

            // Keep only last 100 points to prevent excessive memory usage
            if self.power_curve.len() > 100 {
                self.power_curve.remove(0);
            }
            if self.torque_curve.len() > 100 {
                self.torque_curve.remove(0);
            }
        }

        self.last_update = Instant::now();
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
