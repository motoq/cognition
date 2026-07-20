/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Trait defining a system of 1st order ordinary differential
//! equations
//!
//! # Author
//!
//! *  Kurt Motekew  2026/07/16  Initial

use nalgebra as na;

pub trait Ode {
    /// This method computes the derivative values based on the model
    /// of the system of equations.
    ///
    /// # Arguments
    ///
    /// * t  Time
    /// * x  State vector at time t
    ///
    /// # Return
    ///
    /// * Time derivative of state vector at time t
    ///
    fn xdot<const R: usize>(&self,
        t: f64, x: &na::SMatrix<f64, R, 1>) -> na::SMatrix<f64, R, 1>;
}

