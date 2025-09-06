//! Data models and structures for the dyno testing application

use serde::{Deserialize, Serialize};

/// Represents an operator who can perform dyno tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub id: u32,
    pub username: String,
    pub name: String,
    pub is_active: bool,
}

/// Represents a customer who owns motorcycles for testing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub motorcycles: Vec<Motorcycle>,
}

/// Represents a motorcycle to be tested
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motorcycle {
    pub id: u32,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub engine_cc: u16,
    pub vin: Option<String>,
}

/// Real-time testing parameters
#[derive(Debug, Clone, Default)]
pub struct TestParameters {
    pub rpm: f32,
    pub speed_kmh: f32,
    pub power_hp: f32,
    pub torque_nm: f32,
    pub air_fuel_ratio: f32,
    pub engine_temp_c: f32,
    pub ambient_temp_c: f32,
    pub humidity_percent: f32,
    pub barometric_pressure_kpa: f32,
}

/// Represents a complete dyno test session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSession {
    pub id: u32,
    pub customer_id: u32,
    pub motorcycle_id: u32,
    pub operator_id: u32,
    pub start_time: String, // ISO 8601 format
    pub end_time: Option<String>,
    pub max_power_hp: Option<f32>,
    pub max_torque_nm: Option<f32>,
    pub max_rpm: Option<f32>,
    pub test_data: Vec<TestDataPoint>,
}

/// Individual data point during testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDataPoint {
    pub timestamp_ms: u64,
    pub rpm: f32,
    pub power_hp: f32,
    pub torque_nm: f32,
    pub air_fuel_ratio: f32,
    pub engine_temp_c: f32,
}

/// Application state for session management
#[derive(Debug, Default)]
pub struct AppState {
    pub current_operator: Option<Operator>,
    pub selected_customer: Option<Customer>,
    pub selected_motorcycle: Option<Motorcycle>,
    pub current_test_session: Option<TestSession>,
    pub current_parameters: TestParameters,
    pub is_testing: bool,
}
