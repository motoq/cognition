/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! J2 gravitational acceleration model
//!
//! # Author
//!
//! *  Kurt Motekew  2026/08/6  Initial

use nalgebra as na;

use crate::dyn_gravity::Gravity;

/// Contains the modeling parameter required to compute J2 gravitational
/// acceleration
pub struct J2Gravity {
    /// Gravitational parameter, DU^2/TU^3
    gm: f64,
    /// Oblateness
    j2: f64,
}

impl J2Gravity {
    /// # Argument
    ///
    /// * gm  Gravitational parameter, DU^2/TU^3
    /// * j2  Oblateness
    pub fn new(gm: f64, j2: f64) -> Self {
        Self {
            gm,
            j2,
        }
    }
}

impl Gravity for J2Gravity {
    /// Given a position vector, return the J2 based gravitational
    /// acceleration.  Input distance units must be compatible with
    /// the value of GM used to initialize the structure.  Output
    /// units will match those of GM.  For this zonal model, the
    /// z-axis is assumed to be the line of symmetry.
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
        let ri = pos[0];
        let rj = pos[1];
        let rk = pos[2];
        let rk2 = rk*rk;

        let rmag2 = ri*ri + rj*rj + rk*rk;
        let invr2 = 1.0/rmag2;
        let invr = invr2.sqrt();
        let invr3 = invr*invr2;

        // 2-body
        let mut acc = -self.gm*invr3*pos;

        let c1 = 1.5*self.j2*invr2*invr3;
        let c2 = 5.0*rk2*invr2;

        // Update with J2
        acc[0] -= c1*ri*(1.0 - c2);
        acc[1] -= c1*rj*(1.0 - c2);
        acc[2] -= c1*rk*(3.0 - c2);

        acc
    }
}
