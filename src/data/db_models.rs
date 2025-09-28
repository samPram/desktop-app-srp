use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::data::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = motorcycles)]
pub struct Motorcycle {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub engine_cc: i32,
    pub vin: Option<String>,
    pub license_plate: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = motorcycles)]
pub struct NewMotorcycle {
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub engine_cc: i32,
    pub vin: Option<String>,
    pub license_plate: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = customers)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = customers)]
pub struct NewCustomer {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = operators)]
pub struct Operator {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub role: String, // "operator", "supervisor", "admin"
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = operators)]
pub struct NewOperator {
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub role: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = test_sessions)]
pub struct TestSession {
    pub id: i32,
    pub motorcycle_id: i32,
    pub customer_id: Option<i32>,
    pub operator_id: i32,
    pub test_type: String, // "power_curve", "max_power", "road_load", "custom"
    pub status: String, // "running", "completed", "cancelled", "failed"
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration_seconds: Option<i32>,
    pub max_rpm: Option<f32>,
    pub max_speed: Option<f32>,
    pub max_hp: Option<f32>,
    pub max_torque: Option<f32>,
    pub max_hp_rpm: Option<f32>,
    pub max_torque_rpm: Option<f32>,
    pub avg_oil_temp: Option<f32>,
    pub avg_afr: Option<f32>,
    pub weather_conditions: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = test_sessions)]
pub struct NewTestSession {
    pub motorcycle_id: i32,
    pub customer_id: Option<i32>,
    pub operator_id: i32,
    pub test_type: String,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub weather_conditions: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = test_data_points)]
pub struct TestDataPoint {
    pub id: i32,
    pub test_session_id: i32,
    pub timestamp: NaiveDateTime,
    pub time_offset_ms: i32, // Milliseconds from test start
    pub rpm: f32,
    pub speed_kmh: f32,
    pub horsepower: f32,
    pub torque_nm: f32,
    pub oil_temp_c: Option<f32>,
    pub air_fuel_ratio: Option<f32>,
    pub throttle_position: Option<f32>,
    pub engine_load: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = test_data_points)]
pub struct NewTestDataPoint {
    pub test_session_id: i32,
    pub timestamp: NaiveDateTime,
    pub time_offset_ms: i32,
    pub rpm: f32,
    pub speed_kmh: f32,
    pub horsepower: f32,
    pub torque_nm: f32,
    pub oil_temp_c: Option<f32>,
    pub air_fuel_ratio: Option<f32>,
    pub throttle_position: Option<f32>,
    pub engine_load: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = calibration_settings)]
pub struct CalibrationSetting {
    pub id: i32,
    pub name: String,
    pub setting_type: String, // "rpm_calibration", "torque_calibration", "temperature_calibration"
    pub value: f32,
    pub unit: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = calibration_settings)]
pub struct NewCalibrationSetting {
    pub name: String,
    pub setting_type: String,
    pub value: f32,
    pub unit: String,
    pub description: Option<String>,
    pub is_active: bool,
}

// Helper struct for aggregated test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    pub test_session: TestSession,
    pub motorcycle: Motorcycle,
    pub customer: Option<Customer>,
    pub operator: Operator,
    pub data_points_count: i64,
}

// Helper struct for power curve analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerCurveData {
    pub rpm: f32,
    pub horsepower: f32,
    pub torque_nm: f32,
    pub timestamp: NaiveDateTime,
}