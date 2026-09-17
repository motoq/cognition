//! Constants not tied to physical properties

/// Convert radians to degrees
pub const DEG_PER_RAD: f64 = 180.0/std::f64::consts::PI;
/// Convert degrees to radians
pub const RAD_PER_DEG: f64 = std::f64::consts::PI/180.0;

/// Feet to meters
pub const M_PER_FT: f64 = 0.3048;

/// Slug to kg
pub const KG_PER_SLUG: f64 = 14.593902937;
pub const KG_PER_LB: f64 = 0.45359237;
