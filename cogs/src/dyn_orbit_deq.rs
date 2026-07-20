/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Differential equations for orbital motion.  Only central body gravity
//! models are currently incorporated.
//!
//! # Author
//!
//! *  Kurt Motekew  2026/07/20  Initial

use nalgebra as na;

use crate::mth_ode::Ode;
use crate::dyn_gravity::Gravity;

pub struct OrbitDeq {
    grav: Box<dyn Gravity>,
}

impl OrbitDeq {
    pub fn new(grav: Box<dyn Gravity>) -> Self {
        Self {
            grav,
        }
    }
}

impl Ode for OrbitDeq {
    /// Given a state vector, return the time derivative
    ///
    /// # Arguments
    ///
    /// * x  state vector, position and velocity, 6x1, units dependent on
    ///      gravity model used to initialize
    ///
    /// # Return
    ///
    /// * Time derivative of state vector, 6x1
    ///
    fn xdot<const R: usize>(&self,
                            _t: f64,
                            x: &na::SMatrix<f64, R, 1>)
                                                     -> na::SMatrix<f64, R, 1> {
        let a = self.grav.gravt(&x.fixed_view::<3, 1>(0, 0).into());
        let mut xd = x.clone();
        for ii in 0..3 {
            xd[(ii, 0)] = x[(3+ii, 0)];
            xd[(3+ii, 0)] = a[ii];
        }
        xd
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::dyn_two_body_gravity::TwoBodyGravity;

    #[test]
    fn gravity_ode() {
        let twobdy = Box::new(TwoBodyGravity::new(1.0));
        let eom = OrbitDeq::new(twobdy);

        let x = na::matrix![1.0 ; 1.0 ; 1.0 ; 0.5 ; 0.5 ; 0.5];
        let _dx = eom.xdot(0.0, &x);
    }

}

