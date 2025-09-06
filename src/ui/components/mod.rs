//! Reusable UI components module
//!
//! This module contains all reusable UI components that can be used
//! across different parts of the application.

pub mod dashboard;
pub mod login_form;
pub mod gauge;
pub mod performance_graph;

// Future components will be added here, for example:
// pub mod test_controls;
// pub mod customer_selector;

// Re-exports for commonly used components
pub use dashboard::Dashboard;
pub use login_form::LoginForm;
pub use gauge::{CircularGauge, ValueDisplay};
pub use performance_graph::PerformanceGraph;
