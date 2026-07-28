/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use nalgebra as na;

use crate::mth_ode::Ode;

/// # Arguments
///
/// * deq   The EOM to be integrated by dt
/// * dt    Integration step size
/// * tmt0  Input: state vector time
///         Output: time of output state vector (tmt0 + t0 for RK4)
/// * x     Input: state vector initial conditions
///         Output: propagated state vector
///
/// # Author
///
/// *  Kurt Motekew  2026/07/26  Initial
pub fn rk4<const R: usize>(
    deq: &impl Ode<R>,
    dt: f64,
    tmt0: f64,
    x: &mut na::SMatrix<f64, R, 1>,
) {

    if dt == 0.0 {
        return;
    }

    let dx = deq.xdot(tmt0, &x);
    *x = dt * *x;

}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::mth_rk4::rk4;
    use crate::dyn_two_body_gravity::TwoBodyGravity;
    use crate::dyn_orbit_deq::OrbitDeq;

    #[test]
    fn rk4_grav() {
        let twobdy = Box::new(TwoBodyGravity::new(1.0));
        let eom = OrbitDeq::new(twobdy);
        let dt: f64 = 1.0;
        let t: f64 = 0.0;
        let mut x = na::matrix![1.0 ; 1.0 ; 1.0 ; 0.5 ; 0.5 ; 0.5];
        rk4(&eom, dt, t, &mut x);

    }

}

