/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! # Author
//!
//! * Kurt Motekew 2026/08/12  Initial based on eom::GroundPoint

use nalgebra as na;

use crate::utl_const::DEG_PER_RAD;

/// Longitude, latitude, altitude order is a right handed system
pub enum GeodeticElement {
    /// Longitude, radians
    LON,
    /// Latitude, radians
    LAT,
    /// Altitude above oblate spheroid, DU
    ALT,
}

pub struct GroundPoint {
    /// Holds geodetic coordinates, indexed by GeodeticElement enum
    lla: [f64; 3],
    /// Cartesian position, DU
    cart: na::SMatrix<f64, 3, 1>,
}

//
// Constructors
//

impl Default for GroundPoint {
    /// # Return
    ///
    /// * Ground point at Null Island (0, 0, 0)
    ///
    fn default() -> Self {
        Self {
            lla: [0.0, 0.0, 0.0],
            cart: na::matrix![1.0 ; 0.0 ; 0.0],
        }
    }
}

impl GroundPoint {
    pub fn from_geodetic(
        coords: &[(GeodeticElement, f64); 3], 
        re: f64,
        flat: f64,
    ) -> Self {
        let mut lat: f64 = 0.0;
        let mut lon: f64 = 0.0;
        let mut alt: f64 = 0.0;
        for coord in coords {
            let (ctype, cvalue) = coord;
            match ctype {
                GeodeticElement::LON => lon = *cvalue,
                GeodeticElement::LAT => lat = *cvalue,
                GeodeticElement::ALT => alt = *cvalue,
            }
        }
        let lla = [lon, lat, alt];
        let cart: na::SMatrix<f64, 3, 1> = geodetic_to_cart(&lla, re, flat);
        Self { lla, cart }
    }
}

//
// Public immutable accessor methods
//

impl GroundPoint {
    /// Returns the requested geodetic element value
    ///
    /// # Argument
    ///
    /// * elem  Geodetic element value to return
    ///
    /// # Return
    ///
    /// * Value of the indicated geodetic element, radians or DU
    ///
    pub fn geodetic(&self, elem: GeodeticElement) -> f64 {
        self.lla[elem as usize]
    }

    /// Cartesian position vector of the ground point.
    ///
    /// # Return
    ///
    /// * Position, DU
    ///
    pub fn cartesian(&self) -> na::SMatrix<f64, 3, 1> {
        self.cart
    }
}

//
// IO
//

impl std::fmt::Display for GroundPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(Lat (deg), Lon (deg), alt (DU)): ({} {} {})\nCartesian {} DU",
            self.geodetic(GeodeticElement::LAT)*DEG_PER_RAD,
            self.geodetic(GeodeticElement::LON)*DEG_PER_RAD,
            self.geodetic(GeodeticElement::ALT)*DEG_PER_RAD,
            self.cartesian(),
        )
    }
}


//
// Local Functions
//

fn geodetic_to_cart(
    lla: &[f64; 3],
    re: f64,
    flat: f64
) -> na::SMatrix<f64, 3, 1> {
    let slat = lla[1].sin();
    let clat = lla[1].cos();
    let ecc2 = flat*(2.0 - flat);
    let n = re/(1.0 - ecc2*slat*slat).sqrt();
    let nph = n + lla[2];
    na::matrix![nph*clat*lla[0].cos() ;
                nph*clat*lla[0].sin() ;
                (n*(1.0 - ecc2) + lla[2])*slat]
}

//
// Unit tests
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utl_const::RAD_PER_DEG;
    use crate::phy_const::RE;
    use crate::phy_const::FLAT;
    use crate::phy_const::DU_PER_M;

    #[test]
    fn lla_cart() {
        let gd_coords: [(GeodeticElement, f64); 3] = [
            (GeodeticElement::LON, 0.0),
            (GeodeticElement::LAT, 0.0),
            (GeodeticElement::ALT, 0.0),
        ];
        let gp =  GroundPoint::from_geodetic(&gd_coords, RE, FLAT);
        let xyz = gp.cartesian();
        assert!((xyz[0] - RE).abs() < 10.0*f64::EPSILON);
        assert!((xyz[1] - 0.0).abs() < 10.0*f64::EPSILON);
        assert!((xyz[2] - 0.0).abs() < 10.0*f64::EPSILON);

        let gd_coords: [(GeodeticElement, f64); 3] = [
            (GeodeticElement::LAT,  38.3477*RAD_PER_DEG),
            (GeodeticElement::LON, -75.0774*RAD_PER_DEG),
            (GeodeticElement::ALT,  0.0),
        ];
        let gp =  GroundPoint::from_geodetic(&gd_coords, RE, FLAT);
        let xyz_gp = gp.cartesian();
        let xyz_gt = DU_PER_M*na::matrix![1289778.2 ; -4839659.64 ; 3935784.66];
        println!("Truth {}", xyz_gt);
        println!("Comp {}", xyz_gp);
        // 1 cm tol
        let eps = 0.01*DU_PER_M;
        assert!((xyz_gt - xyz_gp).norm() < eps);
    }
}
