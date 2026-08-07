//! Constants tied to physical properties.  Canonical units are used such
//! that one DU is the gravitational parameter scaling factor (approximately
//! the planetary body radius) and one TU is defined resulting in
//! GM = 1 DU^3/TU^2.  An orbit grazing the surface of the central body
//! (semimajor axis = 1DU) will have an orbital period of TPI*TU.

/// Gravitational parameter = 1 DU^3/TU^2 
pub const GM: f64 = 1.0;
/// Gravitational scale factor = 1 DU
pub const DU: f64 = 1.0;

/// Ellipsoid radius, GRS80/WGS 84, km
pub const KM_PER_ER: f64 = 6378.1370;
/// Gravitational scaling radius, EGM96/EGM2008, TN 36 TT compatible
pub const KM_PER_DU: f64 = 6378.1363;
/// Earth semimajor axis in DU
pub const ER_PER_DU: f64 = KM_PER_DU/KM_PER_ER;
/// Gravitational parameter, EGM96/EGM2008, TN 36 TT compatible, km^3/s^2
pub const GM_KM3_SEC2: f64 = 398600.4415;

/// EGM96 Zonal
pub const J2: f64 = 1.082626173852223e-03;

/// Earth radius
pub const RE: f64 = ER_PER_DU;

/// Nominal mean angular velocity of earth w.r.t ECI, GRS80/WGS 84, rad/sec
pub const WE_RAD_SEC: f64 = 7292115.0e-11;

/// Definition of a Time Unit (TU)
pub fn sec_per_tu() -> f64 {
    (KM_PER_DU*((KM_PER_DU*KM_PER_DU)/GM_KM3_SEC2)).sqrt()
}

/// Nominal mean angular velocity of earth w.r.t ECI, GRS80/WGS 84, rad/TU
pub fn we_rad_tu() -> f64 {
    WE_RAD_SEC*sec_per_tu()
}

