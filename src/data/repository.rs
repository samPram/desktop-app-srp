use diesel::prelude::*;
use chrono::{Utc, NaiveDateTime};
use crate::data::{
    database::{DbConnection, DatabaseError},
    db_models::*,
    schema::*,
};

pub struct TestRepository {
    connection: DbConnection,
}

impl TestRepository {
    pub fn new(connection: DbConnection) -> Self {
        Self { connection }
    }

    // Motorcycle operations
    pub fn create_motorcycle(&self, new_motorcycle: NewMotorcycle) -> Result<Motorcycle, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        diesel::insert_into(motorcycles::table)
            .values(&new_motorcycle)
            .execute(&mut *conn)?;
            
        // Get the last inserted record
        let motorcycle = motorcycles::table
            .order(motorcycles::id.desc())
            .first::<Motorcycle>(&mut *conn)?;
            
        Ok(motorcycle)
    }

    pub fn get_motorcycles(&self) -> Result<Vec<Motorcycle>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let results = motorcycles::table
            .order(motorcycles::brand.asc())
            .load::<Motorcycle>(&mut *conn)?;
            
        Ok(results)
    }

    pub fn get_motorcycle_by_id(&self, id: i32) -> Result<Motorcycle, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let motorcycle = motorcycles::table
            .find(id)
            .first::<Motorcycle>(&mut *conn)?;
            
        Ok(motorcycle)
    }

    // Customer operations
    pub fn create_customer(&self, new_customer: NewCustomer) -> Result<Customer, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        diesel::insert_into(customers::table)
            .values(&new_customer)
            .execute(&mut *conn)?;
            
        // Get the last inserted record
        let customer = customers::table
            .order(customers::id.desc())
            .first::<Customer>(&mut *conn)?;
            
        Ok(customer)
    }

    pub fn get_customers(&self) -> Result<Vec<Customer>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let results = customers::table
            .order(customers::name.asc())
            .load::<Customer>(&mut *conn)?;
            
        Ok(results)
    }

    // Operator operations
    pub fn create_operator(&self, new_operator: NewOperator) -> Result<Operator, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        diesel::insert_into(operators::table)
            .values(&new_operator)
            .execute(&mut *conn)?;
            
        // Get the last inserted record
        let operator = operators::table
            .order(operators::id.desc())
            .first::<Operator>(&mut *conn)?;
            
        Ok(operator)
    }

    pub fn get_operators(&self) -> Result<Vec<Operator>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let results = operators::table
            .filter(operators::is_active.eq(true))
            .order(operators::username.asc())
            .load::<Operator>(&mut *conn)?;
            
        Ok(results)
    }

    pub fn get_operator_by_username(&self, username: &str) -> Result<Operator, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let operator = operators::table
            .filter(operators::username.eq(username))
            .filter(operators::is_active.eq(true))
            .first::<Operator>(&mut *conn)?;
            
        Ok(operator)
    }

    // Test session operations
    pub fn create_test_session(&self, new_session: NewTestSession) -> Result<TestSession, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        diesel::insert_into(test_sessions::table)
            .values(&new_session)
            .execute(&mut *conn)?;
            
        // Get the last inserted record
        let session = test_sessions::table
            .order(test_sessions::id.desc())
            .first::<TestSession>(&mut *conn)?;
            
        Ok(session)
    }

    pub fn update_test_session_status(
        &self, 
        session_id: i32, 
        status: &str, 
        end_time: Option<NaiveDateTime>
    ) -> Result<TestSession, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        diesel::update(test_sessions::table.find(session_id))
            .set((
                test_sessions::status.eq(status),
                test_sessions::end_time.eq(end_time),
                test_sessions::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut *conn)?;
            
        // Get the updated record
        let updated_session = test_sessions::table
            .find(session_id)
            .first::<TestSession>(&mut *conn)?;
            
        Ok(updated_session)
    }

    pub fn update_test_session_summary(
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
        let mut conn = self.connection.lock().unwrap();
        
        diesel::update(test_sessions::table.find(session_id))
            .set((
                test_sessions::max_rpm.eq(max_rpm),
                test_sessions::max_speed.eq(max_speed),
                test_sessions::max_hp.eq(max_hp),
                test_sessions::max_torque.eq(max_torque),
                test_sessions::max_hp_rpm.eq(max_hp_rpm),
                test_sessions::max_torque_rpm.eq(max_torque_rpm),
                test_sessions::avg_oil_temp.eq(avg_oil_temp),
                test_sessions::avg_afr.eq(avg_afr),
                test_sessions::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut *conn)?;
            
        // Get the updated record
        let updated_session = test_sessions::table
            .find(session_id)
            .first::<TestSession>(&mut *conn)?;
            
        Ok(updated_session)
    }

    pub fn get_test_sessions(&self, limit: Option<i64>) -> Result<Vec<TestSummary>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let mut query = test_sessions::table
            .inner_join(motorcycles::table)
            .inner_join(operators::table)
            .left_join(customers::table)
            .order(test_sessions::start_time.desc())
            .into_boxed();
            
        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }
        
        let results: Vec<(TestSession, Motorcycle, Operator, Option<Customer>)> = query
            .load(&mut *conn)?;
            
        let summaries = results.into_iter().map(|(session, motorcycle, operator, customer)| {
            // Get data points count for this session
            let data_count = test_data_points::table
                .filter(test_data_points::test_session_id.eq(session.id))
                .count()
                .get_result::<i64>(&mut *conn)
                .unwrap_or(0);
                
            TestSummary {
                test_session: session,
                motorcycle,
                customer,
                operator,
                data_points_count: data_count,
            }
        }).collect();
        
        Ok(summaries)
    }

    pub fn get_test_sessions_by_motorcycle(&self, motorcycle_id: i32) -> Result<Vec<TestSession>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let sessions = test_sessions::table
            .filter(test_sessions::motorcycle_id.eq(motorcycle_id))
            .order(test_sessions::start_time.desc())
            .load::<TestSession>(&mut *conn)?;
            
        Ok(sessions)
    }

    // Test data operations
    pub fn add_test_data_point(&self, data_point: NewTestDataPoint) -> Result<TestDataPoint, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        diesel::insert_into(test_data_points::table)
            .values(&data_point)
            .execute(&mut *conn)?;
            
        // Get the last inserted record
        let point = test_data_points::table
            .order(test_data_points::id.desc())
            .first::<TestDataPoint>(&mut *conn)?;
            
        Ok(point)
    }

    pub fn add_test_data_points(&self, data_points: Vec<NewTestDataPoint>) -> Result<usize, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let inserted_count = diesel::insert_into(test_data_points::table)
            .values(&data_points)
            .execute(&mut *conn)?;
            
        Ok(inserted_count)
    }

    pub fn get_test_data_points(&self, session_id: i32) -> Result<Vec<TestDataPoint>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let points = test_data_points::table
            .filter(test_data_points::test_session_id.eq(session_id))
            .order(test_data_points::time_offset_ms.asc())
            .load::<TestDataPoint>(&mut *conn)?;
            
        Ok(points)
    }

    pub fn get_power_curve_data(&self, session_id: i32) -> Result<Vec<PowerCurveData>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let points = test_data_points::table
            .filter(test_data_points::test_session_id.eq(session_id))
            .order(test_data_points::rpm.asc())
            .load::<TestDataPoint>(&mut *conn)?;
            
        let curve_data = points.into_iter().map(|point| PowerCurveData {
            rpm: point.rpm,
            horsepower: point.horsepower,
            torque_nm: point.torque_nm,
            timestamp: point.timestamp,
        }).collect();
        
        Ok(curve_data)
    }

    // Statistics and analytics
    pub fn get_session_statistics(&self, session_id: i32) -> Result<Option<(f32, f32, f32, f32)>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        use diesel::dsl::*;
        
        let stats: Option<(Option<f32>, Option<f32>, Option<f32>, Option<f32>)> = test_data_points::table
            .filter(test_data_points::test_session_id.eq(session_id))
            .select((
                max(test_data_points::rpm),
                max(test_data_points::horsepower),
                max(test_data_points::torque_nm),
                max(test_data_points::speed_kmh),
            ))
            .first(&mut *conn)
            .optional()?;
            
        match stats {
            Some((Some(max_rpm), Some(max_hp), Some(max_torque), Some(max_speed))) => {
                Ok(Some((max_rpm, max_hp, max_torque, max_speed)))
            }
            _ => Ok(None)
        }
    }

    // Calibration settings
    pub fn get_calibration_settings(&self) -> Result<Vec<CalibrationSetting>, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        let settings = calibration_settings::table
            .filter(calibration_settings::is_active.eq(true))
            .order(calibration_settings::setting_type.asc())
            .load::<CalibrationSetting>(&mut *conn)?;
            
        Ok(settings)
    }

    pub fn update_calibration_setting(&self, setting_id: i32, new_value: f32) -> Result<CalibrationSetting, DatabaseError> {
        let mut conn = self.connection.lock().unwrap();
        
        diesel::update(calibration_settings::table.find(setting_id))
            .set((
                calibration_settings::value.eq(new_value),
                calibration_settings::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut *conn)?;
            
        // Get the updated record
        let updated_setting = calibration_settings::table
            .find(setting_id)
            .first::<CalibrationSetting>(&mut *conn)?;
            
        Ok(updated_setting)
    }
}