/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! 6DOF system supporting integration of EOM to determine translational
//! and rotational dynamics.

use nalgebra as na;

use cogs::mth_ode_solver::OdeSolver;

/// Translational motion currently implemented with generic model
/// and solver
///
/// #Author
///
/// Kurt Motekew  2026/10/14
///
pub struct Orbiter6Dof {
    orbit: Box<dyn OdeSolver<6>>,
}

/// Constructor
impl Orbiter6Dof {
    /// # Argument
    ///
    /// * orbit  Equations of motion within a solver used to propagate
    ///          translational motion
    ///
    pub fn new(orbit: Box<dyn OdeSolver<6>>) -> Self {
        Self { orbit }
    }
}

/// Orbit propagation - mutates structure
impl Orbiter6Dof {
    /// Propagate by time increment
    ///
    /// # Arguments
    ///
    /// * dt  Time increment to propagate
    /// * pv  Propagated position and velocity vector, units consistend
    ///       with initializing EOM.
    ///
    /// # Return
    ///
    /// * Updated time
    ///
    pub fn propagate(
        &mut self,
        dt: f64,
        pv: &mut na::SMatrix<f64, 6, 1>,
    ) -> f64 {
        let tnow = self.orbit.step_dt(dt);
        *pv = self.orbit.state_vector();
        tnow
    }
}

