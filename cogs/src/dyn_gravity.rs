/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use nalgebra as na;


pub trait GravityPotential {
    fn gravt(&self, pos: &na::SMatrix<f64, 3, 1>) -> na::SMatrix<f64, 3, 1>;
}


pub struct GravityPointModel {
    gm: f64,
}

impl GravityPointModel {
    pub fn new(gm: f64) -> Self {
        Self {
            gm,
        }
    }
}


impl GravityPotential for GravityPointModel {
    fn gravt(&self, pos: &na::SMatrix<f64, 3, 1>) -> na::SMatrix<f64, 3, 1> {
        let rmag = pos.norm();
        let invr = 1.0/rmag;
        -self.gm*(invr*invr*invr)*pos
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_acc(pos: &na::SMatrix<f64, 3, 1>,
               eom: &impl GravityPotential) -> na::SMatrix<f64, 3, 1> {
        eom.gravt(&pos)
    }

    #[test]
    fn gravity_2bdy() {
        let eps = 1.0e-13;
        let twobdy = GravityPointModel::new(1.0);
        let pos = na::matrix![1.0 ; 1.0 ; 1.0];

        let acc = na::matrix![ -0.192450089729875 ;
                               -0.192450089729875 ;
                               -0.192450089729875 ];
        assert!((twobdy.gravt(&pos) - acc).norm() < eps);
        assert!((twobdy.gravt(&pos) - get_acc(&pos, &twobdy)).norm() < eps);
    }

}
