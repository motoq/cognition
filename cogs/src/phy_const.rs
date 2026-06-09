//! Constants tied to physical properties.  Canonical units are used such
//! that one DU is the gravitational parameter scaling factor (approximately
//! the planetary body radius) and one TU is defined resulting in
//! GM = 1 DU^3/TU^2.  An orbit grazing the surface of the central body
//! (semimajor axis = 1DU) will have an orbital period of TPI*TU.

/// Gravitational parameter = 1 DU^3/TU^2 
pub const GM: f64 = 1.0;
/// Gravitational scale factor = 1 DU
pub const DU: f64 = 1.0;
/// Planetary body semimajor axis set to one DU for this sim
pub const RE: f64 = DU;
