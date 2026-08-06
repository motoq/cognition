/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use nalgebra as na;

use cogs::dyn_orbit_deq::OrbitDeq;
use cogs::dyn_orbit_rk4::OrbitRk4;
use cogs::mth_ode_solver::OdeSolver;

pub struct Orbiter6Dof {
    orbit: OrbitRk4,
}

impl Orbiter6Dof {
    pub fn new(
        eom: OrbitDeq,
        dt: f64,
        tmt0: f64,
        pv: na::SMatrix<f64, 6, 1>
    ) -> Self {
        let orbit = OrbitRk4::new(eom, dt, tmt0, pv);
        Self {
            orbit
        }
    }
}

impl Orbiter6Dof {
    pub fn propagate(
        &mut self,
        dt: f64,
        pv: &mut na::SMatrix<f64, 6, 1>
    ) -> f64 {
        let tnow = self.orbit.step_dt(dt);
        *pv = self.orbit.state_vector();
        tnow
    }
}


//(r_s_o_i, q_i2b) = propagate_orbiter(sim_time, &r_s_o_i, &q_i2b);
