/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! OdeSolver managing RK4 based integration of the orbital EOM
//!
//! # Author
//!
//! *  Kurt Motekew  2026/07/22  Initial

use nalgebra as na;

use crate::dyn_orbit_deq::OrbitDeq;
use crate::mth_ode::Ode;
use crate::mth_ode_solver::OdeSolver;
use crate::mth_rk4::rk4;

/// EOM and current state vector that will be propagated through RK4
/// numerical integration by dt
pub struct OrbitRk4 {
    orbit: OrbitDeq,
    dt: f64,
    tmt0: f64,
    pv: na::SMatrix<f64, 6, 1>,
}

impl OrbitRk4 {
    /// # Argument
    ///
    /// * orbit  The differential equations representing an orbit
    /// * dt     Integration step size to be used for this ODE solver
    /// * tmt0   Time associated with state vector
    /// * pv     Position and velocity state vector
    pub fn new(
        orbit: OrbitDeq,
        dt: f64,
        tmt0: f64,
        pv: na::SMatrix<f64, 6, 1>
    ) -> Self {

        Self {
            orbit,
            dt,
            tmt0,
            pv,
        }
    }
}

impl OdeSolver<6> for OrbitRk4 {
    /// # Return
    ///
    /// * Time of system associated with current state vector
    ///
    fn time(&self) -> f64 {
        self.tmt0
    }

    /// # Return
    ///
    /// * Current state vector, position and velocity
    ///
    fn state_vector(&self) -> na::SMatrix<f64, 6, 1> {
        self.pv
    }

    /// # Return
    ///
    /// * First derivative of current state vector, velocity and acceleration
    ///
    fn state_vector_dot(&self) -> na::SMatrix<f64, 6, 1> {
        self.orbit.xdot(self.tmt0, &self.pv)
    }

    /// Integrate EOM by internal step size.  Time and state vector are
    /// updated.
    ///
    /// # Return
    ///
    /// * Time associated with updated state vector.  For the RK4 integrator
    ///   equal to tmt0 + dt
    ///
    fn step(&mut self) -> f64 {
        rk4(&self.orbit, self.dt, &mut self.tmt0, &mut self.pv); 
        self.tmt0
    }

    /// Integrate EOM by supplied step size.  Time and state vector are
    /// updated.
    ///
    /// # Argument
    ///
    /// * dt  Integration step size
    ///
    /// # Return
    ///
    /// * Time associated with updated state vector
    ///
    fn step_dt(&mut self, dt: f64) -> f64 {
        rk4(&self.orbit, dt, &mut self.tmt0, &mut self.pv); 
        self.tmt0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::dyn_two_body_gravity::TwoBodyGravity;
    use crate::dyn_orbit_deq::OrbitDeq;

    #[test]
    fn rk4_grav() {
        let twobdy = Box::new(TwoBodyGravity::new(1.0));
        let eom = OrbitDeq::new(twobdy);
        let dt: f64 = 1.0;
        let t: f64 = 0.0;
        let x = na::matrix![1.0 ; 1.0 ; 1.0 ; 0.5 ; 0.5 ; 0.5];
        let _rk4 = OrbitRk4::new(eom, dt, t, x);
    }

}

