/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use nalgebra as na;

pub fn unit_vec_angle(u1: &na::SMatrix<f64, 3, 1>,
                      u2: &na::SMatrix<f64, 3, 1>) -> f64 {
    let eps: f64 = 0.00001;
    let cang = u1.dot(&u2);
    // Unit vector dot product can exceed +/-1.0 due to roundoff
    // Very small angles via atan (also accomodates dot exceeding 1.0)
    if cang <= -1.0 {
        std::f64::consts::PI
    } else if (u1 - u2).norm() < eps {
        u1.cross(&u2).norm().atan()
    } else {
        cang.acos()
    }
}
