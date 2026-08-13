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

use crate::phy_const::RE;

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
            cart: na::matrix![RE ; 0.0 ; 0.0],
        }
    }
}

/// Public immutable accessor methods
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


/*
//
// Local Functions
//

fn geodetic_to_cart(lla: &[f64; 3]) -> na::SMatrix<f64, 3, 1> {
    let lat = lla[0];
    let lon = lla[1];
    let alt = lla[2];





    self.lla = lla;


  m_lat = lat;
  m_lon = lon;
  m_alt = alt;

    let slat = lla[0].sin();
    let clat = lla[0].cos();
  double clat {std::cos(lat)};
  double n {phy_const::earth_smaj/
            std::sqrt(1.0 - phy_const::ecc2*slat*slat)};
  double nph {n + alt};
  m_xyz(0) = nph*clat*std::cos(lon);
  m_xyz(1) = nph*clat*std::sin(lon);
  m_xyz(2) = (n*(1.0 - phy_const::ecc2) + alt)*slat;

*/
