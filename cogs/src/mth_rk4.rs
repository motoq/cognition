/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use nalgebra as na;

use crate::mth_ode::Ode;

/// Basic RK4 integrator.
///
/// # Arguments
///
/// * deq   The EOM to be integrated by dt
/// * dt    Integration step size.  + for forwards propagation, - for
///         backwards propagation.
/// * tmt0  Input: state vector time tmt0
///         Output: time of output state vector (tmt0 + d0 for RK4)
/// * x     Input: state vector initial conditions
///         Output: state vector propagated by dt
///
/// # Author
///
/// *  Kurt Motekew  2026/07/26  Initial
///
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
    *tmt0 += dt;
    xd = deq.xdot(*tmt0, &xx);
    *x = x0 + (xa + dt*xd)/6.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mth_ode::Ode;
    use crate::mth_rk4::rk4;

    #[test]
    fn rk4_exp() {
        // Create struct to implement the ODE dx/dt = x where the solution
        // is x(t) = x0*exp(t) with I.C. x0
        pub struct Exp;
        impl Ode<1> for Exp {
            fn xdot(
                &self,
                _t: f64,
                x: &na::SMatrix<f64, 1, 1>
            ) -> na::SMatrix<f64, 1, 1> {
                let dx = na::matrix![x[0]];
                dx
            }
        }
        // Create ODE with x0 = 1 such that x(t) = exp(t).
        // Compare to f32 eps just to keep the integration step
        // size from being too small.
        let eom = Exp;
        let dt: f64 = 0.01;
        let mut t: f64 = 0.0;
        let mut x = na::matrix![1.0];
        while t < 1.0 - f64::EPSILON {
          rk4(&eom, dt, &mut t, &mut x);
        }
        // For truth set t to 1.0
        t = 1.0;
        assert!((x[0] - t.exp()).abs() < f32::EPSILON as f64);
    }
}

