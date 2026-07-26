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
