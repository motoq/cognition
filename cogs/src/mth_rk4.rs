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
///         Output: state vector propagated by dt
///
/// # Author
///
/// *  Kurt Motekew  2026/07/26  Initial
pub fn rk4<const R: usize>(
    deq: &impl Ode<R>,
    dt: f64,
    tmt0: &mut f64,
    x: &mut na::SMatrix<f64, R, 1>,
) {
    // No integration to do
    if dt == 0.0 {
        return;
    }

    // first
    let x0 = x.clone();
    let mut time = *tmt0;
    let mut xd = deq.xdot(time, &x0);
    let mut xa = dt*xd;
    let mut xx = 0.5*xa + x0;
    // second
    time += 0.5*dt;
    xd = deq.xdot(time, &xx);
    let mut q = dt*xd;
    xx = x0 + 0.5*q;
    xa += q + q;
    // third
    xd = deq.xdot(time, &xx);
    q = dt*xd;
    xx = x0 + q;
    xa += q + q;
    // forth - update member variables vs. locals
    time += dt;
    xd = deq.xdot(time, &xx);
    *tmt0 = time;
    *x = x0 + (xa + dt*xd)/6.0;
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
        let mut t: f64 = 0.0;
        let mut x = na::matrix![1.0 ; 1.0 ; 1.0 ; 0.5 ; 0.5 ; 0.5];
        rk4(&eom, dt, &mut t, &mut x);
    }

}

