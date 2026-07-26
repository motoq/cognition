/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use nalgebra as na;

/// Define functionality that solves ordinary differential equations
/// through numerical integration.
///
/// # Author
///
/// *  Kurt Motekew  2026/07/20  Initial
pub trait OdeSolver<const R: usize> {
    /// # Return
    ///
    /// * Time of system in current state
    ///
    fn time(&self) -> f64;

    /// # Return
    ///
    /// * Current state vector of the system
    ///
    fn state_vector(&self) -> na::SMatrix<f64, R, 1>;

    /// # Return
    ///
    /// * First derivative of current state vector of the system
    ///
    fn state_vector_dot(&self) -> na::SMatrix<f64, R, 1>;

    /// # Return
    ///
    /// * Time associated with updated state vector
    ///
    fn step(&mut self) -> f64;
}

