use chrono::Utc;
use std::sync::Arc;
use crate::data::{
    repository::TestRepository,
    db_models::*,
    database::DatabaseError,
};

pub struct TestHistoryService {
    repository: Arc<TestRepository>,
}

impl TestHistoryService {
    pub fn new(repository: Arc<TestRepository>) -> Self {
        Self { repository }
    }

    /// Start a new test session
    pub fn start_test_session(
        &self,
        motorcycle_id: i32,
        customer_id: Option<i32>,
        operator_id: i32,
        test_type: &str,
        notes: Option<String>,
    ) -> Result<TestSession, DatabaseError> {
        let new_session = NewTestSession {
            motorcycle_id,
            customer_id,
            operator_id,
            test_type: test_type.to_string(),
            status: "running".to_string(),
            start_time: Utc::now().naive_utc(),
            weather_conditions: None,
            notes,
        };

        self.repository.create_test_session(new_session)
    }

    /// Complete a test session
    pub fn complete_test_session(
        &self,
        session_id: i32,
        max_rpm: Option<f32>,
        max_speed: Option<f32>,
        max_hp: Option<f32>,
        max_torque: Option<f32>,
        max_hp_rpm: Option<f32>,
        max_torque_rpm: Option<f32>,
        avg_oil_temp: Option<f32>,
        avg_afr: Option<f32>,
    ) -> Result<TestSession, DatabaseError> {
        // Update session status to completed
        self.repository.update_test_session_status(session_id, "completed", Some(Utc::now().naive_utc()))?;
        
        // Update session summary data
        self.repository.update_test_session_summary(
            session_id,
            max_rpm,
            max_speed,
            max_hp,
            max_torque,
            max_hp_rpm,
            max_torque_rpm,
            avg_oil_temp,
            avg_afr,
        )
    }

    /// Cancel a test session
    pub fn cancel_test_session(&self, session_id: i32) -> Result<TestSession, DatabaseError> {
        self.repository.update_test_session_status(session_id, "cancelled", Some(Utc::now().naive_utc()))
    }

    /// Record a data point during testing
    pub fn record_data_point(
        &self,
        session_id: i32,
        time_offset_ms: i32,
        rpm: f32,
        speed_kmh: f32,
        horsepower: f32,
        torque_nm: f32,
        oil_temp_c: Option<f32>,
        air_fuel_ratio: Option<f32>,
    ) -> Result<TestDataPoint, DatabaseError> {
        let data_point = NewTestDataPoint {
            test_session_id: session_id,
            timestamp: Utc::now().naive_utc(),
            time_offset_ms,
            rpm,
            speed_kmh,
            horsepower,
            torque_nm,
            oil_temp_c,
            air_fuel_ratio,
            throttle_position: None,
            engine_load: None,
        };

        self.repository.add_test_data_point(data_point)
    }

    /// Get recent test sessions with summary info
    pub fn get_recent_tests(&self, limit: Option<i64>) -> Result<Vec<TestSummary>, DatabaseError> {
        self.repository.get_test_sessions(limit)
    }

    /// Get power curve data for a specific test
    pub fn get_power_curve(&self, session_id: i32) -> Result<Vec<PowerCurveData>, DatabaseError> {
        self.repository.get_power_curve_data(session_id)
    }

    /// Get all test sessions for a specific motorcycle
    pub fn get_motorcycle_test_history(&self, motorcycle_id: i32) -> Result<Vec<TestSession>, DatabaseError> {
        self.repository.get_test_sessions_by_motorcycle(motorcycle_id)
    }

    /// Create or get a motorcycle record
    pub fn create_motorcycle(
        &self,
        brand: String,
        model: String,
        year: i32,
        engine_cc: i32,
        vin: Option<String>,
        license_plate: Option<String>,
        notes: Option<String>,
    ) -> Result<Motorcycle, DatabaseError> {
        let new_motorcycle = NewMotorcycle {
            brand,
            model,
            year,
            engine_cc,
            vin,
            license_plate,
            notes,
        };

        self.repository.create_motorcycle(new_motorcycle)
    }

    /// Get all motorcycles
    pub fn get_motorcycles(&self) -> Result<Vec<Motorcycle>, DatabaseError> {
        self.repository.get_motorcycles()
    }

    /// Create or get a customer record
    pub fn create_customer(
        &self,
        name: String,
        email: Option<String>,
        phone: Option<String>,
        address: Option<String>,
        notes: Option<String>,
    ) -> Result<Customer, DatabaseError> {
        let new_customer = NewCustomer {
            name,
            email,
            phone,
            address,
            notes,
        };

        self.repository.create_customer(new_customer)
    }

    /// Get all customers
    pub fn get_customers(&self) -> Result<Vec<Customer>, DatabaseError> {
        self.repository.get_customers()
    }

    /// Get operators
    pub fn get_operators(&self) -> Result<Vec<Operator>, DatabaseError> {
        self.repository.get_operators()
    }

    /// Get operator by username
    pub fn get_operator_by_username(&self, username: &str) -> Result<Operator, DatabaseError> {
        self.repository.get_operator_by_username(username)
    }

    /// Generate test statistics
    pub fn get_test_statistics(&self, session_id: i32) -> Result<Option<TestStatistics>, DatabaseError> {
        if let Some((max_rpm, max_hp, max_torque, max_speed)) = self.repository.get_session_statistics(session_id)? {
            let data_points = self.repository.get_test_data_points(session_id)?;
            
            let avg_rpm = if !data_points.is_empty() {
                data_points.iter().map(|p| p.rpm).sum::<f32>() / data_points.len() as f32
            } else {
                0.0
            };

            let avg_hp = if !data_points.is_empty() {
                data_points.iter().map(|p| p.horsepower).sum::<f32>() / data_points.len() as f32
            } else {
                0.0
            };

            let avg_torque = if !data_points.is_empty() {
                data_points.iter().map(|p| p.torque_nm).sum::<f32>() / data_points.len() as f32
            } else {
                0.0
            };

            Ok(Some(TestStatistics {
                max_rpm,
                max_hp,
                max_torque,
                max_speed,
                avg_rpm,
                avg_hp,
                avg_torque,
                total_data_points: data_points.len() as i32,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestStatistics {
    pub max_rpm: f32,
    pub max_hp: f32,
    pub max_torque: f32,
    pub max_speed: f32,
    pub avg_rpm: f32,
    pub avg_hp: f32,
    pub avg_torque: f32,
    pub total_data_points: i32,
}