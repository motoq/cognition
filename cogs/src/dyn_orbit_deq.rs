/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Differential equations for orbital motion.  Only central body gravity
//! models are currently incorporated (no 3rd body, SRP, drag, etc).
//!
//! # Author
//!
//! *  Kurt Motekew  2026/07/20  Initial

use nalgebra as na;

use crate::mth_ode::Ode;
use crate::dyn_gravity::Gravity;

/// Contains the model of the equations of motion (EOM)
pub struct OrbitDeq {
    /// Central gravity model
    grav: Box<dyn Gravity>,
}

impl OrbitDeq {
    /// Initialize EOM with a central body gravity model
    ///
    /// # Argument
    ///
    /// * grav  Gravity model
    ///
    /// # Return
    ///
    /// * Struct capable of satisfying Ode trait
    ///
    pub fn new(grav: Box<dyn Gravity>) -> Self {
        Self {
            grav,
        }
    }
}

impl Ode<6> for OrbitDeq {
    /// Given a state vector, return the time derivative
    ///
    /// # Arguments
    ///
    /// * tmt0  Time since simulation epoch, unused (not needed yet)
    /// * pv    state vector at time tmt0, position and velocity, units
    ///         dependent on gravity model used to initialize
    ///
    /// # Return
    ///
    /// * Time derivative of state vector (velocity and acceleration)
    ///   at tmt0, units consistent with pv
    ///
    fn xdot(
        &self,
        _tmt0: f64,
        pv: &na::SMatrix<f64, 6, 1>
    ) -> na::SMatrix<f64, 6, 1> {
        let acc = self.grav.gravt(&pv.fixed_view::<3, 1>(0, 0).into());
        let va = na::matrix![pv[3] ; pv[4] ; pv[5] ; acc[0] ; acc[1] ; acc[2]];
        va
    }
}
