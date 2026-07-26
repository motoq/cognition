/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Two body gravitational acceleration model
//!
//! # Author
//!
//! *  Kurt Motekew  2026/07/14  Initial

use nalgebra as na;

use crate::dyn_gravity::Gravity;

/// Contains the modeling parameter required to compute two-body gravitational
/// acceleration
pub struct TwoBodyGravity {
    /// Gravitational parameter, DU^2/TU^3
    gm: f64,
}

impl TwoBodyGravity {
    /// # Argument
    ///
    /// * gm  Gravitational parameter, DU^2/TU^3
    pub fn new(gm: f64) -> Self {
        Self {
            gm,
        }
    }
}

impl Gravity for TwoBodyGravity {
    /// Given a position vector, return the two-body gravitational
    /// acceleration.  Input distance units must be compatible with
    /// the value of GM used to initialize the structure.  Output
    /// units will match those of GM.  Since this is a point model,
    /// the reference frame only needs to be earth centered.
    ///
    /// # Arguments
    ///
    /// * pos  Position vector, DU
    ///
    /// # Return
    ///
    /// * Acceleration Vector, DU/TU^2
    ///
    fn gravt(&self, pos: &na::SMatrix<f64, 3, 1>) -> na::SMatrix<f64, 3, 1> {
        let rmag = pos.norm();
        let invr = 1.0/rmag;
        -self.gm*(invr*invr*invr)*pos
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_acc(pos: &na::SMatrix<f64, 3, 1>,
               eom: &impl Gravity) -> na::SMatrix<f64, 3, 1> {
        eom.gravt(&pos)
    }


    #[test]
    fn gravity_twobody() {
        let eps = 1.0e-13;
        let twobdy = TwoBodyGravity::new(1.0);
        let pos = na::matrix![1.0 ; 1.0 ; 1.0];

        let acc = na::matrix![ -0.192450089729875 ;
                               -0.192450089729875 ;
                               -0.192450089729875 ];
        assert!((twobdy.gravt(&pos) - acc).norm() < eps);
        assert!((twobdy.gravt(&pos) - get_acc(&pos, &twobdy)).norm() < eps);
    }

}

