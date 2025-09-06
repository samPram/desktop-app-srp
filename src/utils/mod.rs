//! Utility functions and helper modules
//!
//! This module contains utility functions, helpers, and common functionality
//! that can be used across the application.

/// Formatting utilities for displaying values in the UI
pub mod formatting {
    /// Format RPM value for display
    pub fn format_rpm(rpm: f32) -> String {
        format!("{:.0} RPM", rpm)
    }

    /// Format power value for display
    pub fn format_power(power_hp: f32) -> String {
        format!("{:.1} HP", power_hp)
    }

    /// Format torque value for display
    pub fn format_torque(torque_nm: f32) -> String {
        format!("{:.1} Nm", torque_nm)
    }

    /// Format temperature for display
    pub fn format_temperature(temp_c: f32) -> String {
        format!("{:.1}°C", temp_c)
    }

    /// Format air-fuel ratio for display
    pub fn format_afr(afr: f32) -> String {
        format!("{:.2}:1", afr)
    }
}

/// Validation utilities
pub mod validation {
    /// Validate RPM range (typical motorcycle range)
    pub fn is_valid_rpm(rpm: f32) -> bool {
        rpm >= 0.0 && rpm <= 20000.0
    }

    /// Validate power range (reasonable motorcycle power range)
    pub fn is_valid_power(power_hp: f32) -> bool {
        power_hp >= 0.0 && power_hp <= 500.0
    }

    /// Validate torque range
    pub fn is_valid_torque(torque_nm: f32) -> bool {
        torque_nm >= 0.0 && torque_nm <= 300.0
    }

    /// Validate temperature range
    pub fn is_valid_temperature(temp_c: f32) -> bool {
        temp_c >= -50.0 && temp_c <= 200.0
    }
}

/// Time and date utilities
pub mod time {
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Get current timestamp in milliseconds
    pub fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Format timestamp for display
    pub fn format_timestamp(timestamp_ms: u64) -> String {
        // This is a simplified implementation
        // In a real application, you might want to use chrono crate
        format!("{}ms", timestamp_ms)
    }
}
