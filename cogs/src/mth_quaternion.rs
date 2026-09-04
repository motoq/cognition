/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


//! Quaternion object meant to demonstrate conventions vs. be used
//! as a quaternion library for heavy lifting.  See the unit test
//! examples.  Emphasis is on conveying the need to indicate if
//! the eigenaxis angle is a basis rotation (passive) or vector
//! rotation (active) through the use of fundamental quaternion
//! definitions and operations.
//!
//! # Author
//!
//! * Kurt Motekew 2026/09/02  Initial

use std::ops::Mul;

use nalgebra as na;

//use crate::utl_const::DEG_PER_RAD;
//use crate::phy_const::DU_PER_ER;

/// The quaternion is represented as real and imaginary components.
/// There is no ambiguity w.r.t. which component is the scalar element
/// when a {scalar, vector} approach is used.
#[derive(Copy, Clone)]
pub struct Quaternion {
    /// Real (scalar) portion of quaternion
    qr: f64,
    /// Imaginery (vector) portion of quaternion
    qi: na::Vector3::<f64>,
}

impl Default for Quaternion {
    /// # Return
    ///
    /// * Identity quaternion
    ///
    fn default() -> Self {
        Self {
            qr: 1.0,
            qi: na::Vector3::<f64>::zeros()
        }
    }
}

impl Quaternion {
    /// Initialize with quaternion components
    ///
    /// # Arguments
    ///
    /// * qr  Real component of the quaternion (the scalar)
    /// * qi  Imaginary components of the quaternion (the vector)
    ///
    /// # Return
    ///
    /// * Initialized quaternion, not necesarrily of unit magnitude
    ///
    pub fn new(qr: f64, qi: &na::Vector3::<f64>) -> Self {
        Self { qr, qi: *qi }
    }

    /// Initialize with quaternion components and normalize to unit magnitude
    ///
    /// # Arguments
    ///
    /// * qr  Real component of the quaternion (the scalar)
    /// * qi  Imaginary components of the quaternion (the vector)
    ///
    /// # Return
    ///
    /// * Initialized quaternion of unit magnitude
    ///
    pub fn new_normalized(qr: f64, qi: &na::Vector3::<f64>) -> Self {
        let mag = qr.abs() + qi.norm();
        Self { qr: qr/mag, qi: qi/mag }
    }

    /// Initialize a unit quaternion given an angle and unit length rotation
    /// axis.
    ///
    /// # Arguments
    ///
    /// * angle  Rotation angle, radians
    /// * axis   Rotation axis, unit length (no check is performed)
    ///
    /// # Return
    ///
    /// * Initialized quaternion of unit magnitude
    ///
    pub fn from_angle_axis(angle: f64, axis:  &na::Vector3::<f64>) -> Self {
        Self { qr: (0.5*angle).cos(), qi: (0.5*angle).sin()*axis }
    }
}

///! Public immutable accessor methods
impl Quaternion {
    /// # Return
    ///
    /// * Real component of the quaternion (the scalar)
    ///
    pub fn real(&self) -> f64 {
        self.qr
    }

    /// # Return
    ///
    /// * Imaginary components of the quaternion (the vector)
    ///
    pub fn imaginary(&self) -> na::Vector3::<f64> {
        self.qi
    }
}

///! Mutable manipulation
impl Quaternion {
    /// Convert this quaternion to a unit quaternion
    pub fn normalize(&mut self) {
        let mag = self.qr.abs() + self.qi.norm();

        self.qr /= mag;
        self.qi /= mag;
    }
}

///! Immutable functions
impl Quaternion {
    /// # Return
    ///
    /// * Complex conjugate of this quaternion
    ///
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            qr: self.qr,
            qi: na::Vector3::new(-self.qi[0], -self.qi[1],-self.qi[2]),
        }
    }
}

///! Immutable operator overload
impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion {
            qr: self.qr*rhs.qr - self.qi[0]*rhs.qi[0]
                               - self.qi[1]*rhs.qi[1]
                               - self.qi[2]*rhs.qi[2],
            qi: na::Vector3::new(self.qr*rhs.qi[0] + self.qi[0]*rhs.qr
                                                   + self.qi[1]*rhs.qi[2]
                                                   - self.qi[2]*rhs.qi[1],
                                 self.qr*rhs.qi[1] - self.qi[0]*rhs.qi[2]
                                                   + self.qi[1]*rhs.qr
                                                   + self.qi[2]*rhs.qi[0],
                                 self.qr*rhs.qi[2] + self.qi[0]*rhs.qi[1]
                                                   - self.qi[1]*rhs.qi[0]
                                                   + self.qi[2]*rhs.qr,
            ),
        }
    }
}

impl Mul<na::Vector3<f64>> for Quaternion {
    type Output = na::Vector3<f64>;

    // https://www.johndcook.com/blog/2021/06/16/faster-quaternion-rotations/
    // https://blog.molecular-matters.com/
    //     2013/05/24/a-faster-quaternion-vector-multiplication/
    fn mul(self, rhs: na::Vector3<f64>) -> Self::Output {
        let tt = 2.0*self.qi.cross(&rhs);
        rhs + self.qr*tt + self.qi.cross(&tt)
    }
}

impl Mul<Quaternion> for na::Vector3<f64> {
    type Output = na::Vector3<f64>;

    // Derived from above q*v operation
    fn mul(self, rhs: Quaternion) -> Self::Output {
        let tt = -2.0*rhs.qi.cross(&self);
        self + rhs.qr*tt - rhs.qi.cross(&tt)
    }
}


///! IO
impl std::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{{} [{} {} {}]}}",
            self.qr,
            self.qi[0],
            self.qi[1],
            self.qi[2],
        )
    }
}



//
// Unit tests
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utl_const::RAD_PER_DEG;
    use crate::mth_dcm::{rotx, roty, rotz};

    #[test]
    fn passive_angle_passive_xform() {
        let ihat = na::Vector3::new(1.0, 0.0, 0.0);
        let jhat = na::Vector3::new(0.0, 1.0, 0.0);
        let khat = na::Vector3::new(0.0, 0.0, 1.0);

        let yaw = RAD_PER_DEG*45.0;
        let pitch = RAD_PER_DEG*30.0;
        let roll = -1.0*RAD_PER_DEG*20.0;

        let q1 = Quaternion::from_angle_axis(yaw, &khat);
        let q2 = Quaternion::from_angle_axis(pitch, &jhat);
        let q3 = Quaternion::from_angle_axis(roll, &ihat);

        let qatt = q1*q2*q3;

        let pos = na::Vector3::new(1.0, 1.0, 1.0);
        let qpos = Quaternion::new(0.0, &pos);

        let pos_qx = qatt.conjugate()*qpos*qatt;

        let r1 = rotz(yaw);
        let r2 = roty(pitch);
        let r3 = rotx(roll);

        let ratt = r3*r2*r1;
        let pos_rx = ratt*pos;

        println!("qpos: {}\nand rpos: {}", &pos_qx, &pos_rx);

        println!(
            "q*vq: {}\nand vq: {}",
            &(qatt.conjugate()*qpos*qatt), &(pos*qatt)
        );
        println!(
            "qvq*: {}\nand qv: {}",
            &(qatt*qpos*qatt.conjugate()), &(qatt*pos)
        );


        assert!((pos_rx - pos_qx.imaginary()).norm() < 10.0*f64::EPSILON);

        let dv = (qatt*qpos*qatt.conjugate()).imaginary() - qatt*pos;
        assert!(dv.norm() < 10.0*f64::EPSILON);

        let dv = (qatt.conjugate()*qpos*qatt).imaginary() - pos*qatt;
        assert!(dv.norm() < 10.0*f64::EPSILON);
    }
}
