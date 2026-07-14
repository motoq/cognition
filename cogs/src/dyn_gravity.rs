/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Define a trait where position generates gravitational acceleration
//!
//! # Author
//!
//! *  Kurt Motekew  2026/07/14  Initial

use nalgebra as na;

pub trait Gravity {
    /// Given a position vector, return the gravitational acceleration
    ///
    /// # Arguments
    ///
    /// * pos  Position vector, DU.  This would typically be earth fixed.
    ///
    /// # Return
    ///
    /// * Acceleration Vector, DU/TU^2.  This would typically be acceleration
    ///   w.r.t. inertial space with vector components in earth fixed.  The
    ///   implementation should clarify reference frames and units.
    ///
    fn gravt(&self, pos: &na::SMatrix<f64, 3, 1>) -> na::SMatrix<f64, 3, 1>;
}



// Temporary, just for initial testing
pub struct GravityPointModel {
    gm: f64,
}
// Temporary, just for initial testing
impl GravityPointModel {
    pub fn new(gm: f64) -> Self {
        Self {
            gm,
        }
    }
}
// Temporary, just for initial testing
impl Gravity for GravityPointModel {
    fn gravt(&self, pos: &na::SMatrix<f64, 3, 1>) -> na::SMatrix<f64, 3, 1> {
        let rmag = pos.norm();
        let invr = 1.0/rmag;
        -self.gm*(invr*invr*invr)*pos
    }
}
// Temporary, just for initial testing
#[cfg(test)]
mod tests {
    use super::*;

    fn get_acc(pos: &na::SMatrix<f64, 3, 1>,
               eom: &impl Gravity) -> na::SMatrix<f64, 3, 1> {
        eom.gravt(&pos)
    }

    #[test]
    fn gravity_2bdy() {
        let eps = 1.0e-13;
        let twobdy = GravityPointModel::new(1.0);
        let pos = na::matrix![1.0 ; 1.0 ; 1.0];

        let acc = na::matrix![ -0.192450089729875 ;
                               -0.192450089729875 ;
                               -0.192450089729875 ];
        assert!((twobdy.gravt(&pos) - acc).norm() < eps);
        assert!((twobdy.gravt(&pos) - get_acc(&pos, &twobdy)).norm() < eps);
    }

}
