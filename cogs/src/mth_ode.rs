/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use nalgebra as na;

/// Trait defining a system of 1st order ordinary differential
/// equations
///
/// # Author
///
/// *  Kurt Motekew  2026/07/16  Initial
pub trait Ode<const R: usize> {
    /// Compute the derivative of the state vector
    ///
    /// # Arguments
    ///
    /// * t  Independent variable (e.g., time)
    /// * x  State vector at (time) t
    ///
    /// # Return
    ///
    /// * Derivative of state vector w.r.t. t at t
    ///
    fn xdot(&self,
        t: f64, x: &na::SMatrix<f64, R, 1>) -> na::SMatrix<f64, R, 1>;
}

