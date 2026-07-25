/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! OdeSolver managing RK4 based integration of the EOM
//!
//! # Author
//!
//! *  Kurt Motekew  2026/07/22  Initial

use nalgebra as na;

use crate::dyn_orbit_deq::OrbitDeq;
use crate::mth_ode_solver::OdeSolver;

pub struct OrbitRk4 {
    orbit: OrbitDeq,
    dt: f64,
    t0: f64,
    x0: na::SMatrix<f64, 6, 1>,
    dx0: na::SMatrix<f64, 6, 1>,
}

impl OrbitRk4 {
    /// # Argument
    ///
    /// * x  stuff
    pub fn new(orbit: OrbitDeq,
               dt: f64,
               t0: f64,
               x0: na::SMatrix<f64, 6, 1>,
               dx0: na::SMatrix<f64, 6, 1>) -> Self {
        Self {
            orbit,
            dt,
            t0,
            x0,
            dx0,
        }
    }
}

impl OdeSolver<6> for OrbitRk4 {
    fn time(&self) -> f64 {
        self.t0
    }
    fn state_vector(&self) -> na::SMatrix<f64, 6, 1> {
        self.x0
    }
    fn state_vector_dot(&self) -> na::SMatrix<f64, 6, 1> {
        self.dx0
    }
    fn step(&self) -> f64 {
        self.t0 + self.dt
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::mth_ode::Ode;                               // for xdot
    use crate::dyn_two_body_gravity::TwoBodyGravity;
    use crate::dyn_orbit_deq::OrbitDeq;

    #[test]
    fn rk4_grav() {
        let twobdy = Box::new(TwoBodyGravity::new(1.0));
        let eom = OrbitDeq::new(twobdy);
        let dt: f64 = 1.0;
        let t: f64 = 0.0;
        let x = na::matrix![1.0 ; 1.0 ; 1.0 ; 0.5 ; 0.5 ; 0.5];
        let dx: na::SMatrix<f64, 6, 1> = eom.xdot(0.0, &x);
        let rk4 = OrbitRk4::new(eom, dt, t, x, dx);
    }

}

