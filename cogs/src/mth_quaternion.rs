/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


//! Quaternion type not locked into a convention, suitable for aerospace
//! attitude representation or computer graphics.  Quaternion multiplication
//! is defined as the sum of elements resulting from the outer product of
//! two quaternion 4D arrays.
//!
//! Either active or passive quaternions can be defined.  Operator
//! overloading such that `q*v = q*p*q'` and `v*q = q'*p*q` (where `p`
//! is a pure quaternion with imaginary part equal to `v`, and `q'` is
//! the quaternion complex conjugate) allows unambiguous vector rotation
//! vs. reference frame transformations.
//!
//! Given the (unit length) Euler axis `ehat` and rotation angle `theta`
//! formulation of a unit quaternion `q = cos(theta/2) + ehat*sin(theta/2)`,
//! `theta` is *active* if it rotates a vector within a reference frame.
//! In contranst, `theta` is passive if instead the reference frame basis
//! vectors are rotated the vector remains fixed in in space.
//!
//! # Author
//!
//! * Kurt Motekew 2026/09/02  Initial

use std::ops::{Div, Mul};

use nalgebra as na;

/// Square of quaternion norm magnitude to trigger normalization
const TOL: f64 = 100.0*f64::EPSILON;

/// DCM to quaternion alg selection factor.  0.25 requires
/// the first solved for quaternion element to be at least
/// 1/4 in magnitude or greater.  1.0 would result in the
/// first needing to be 1/2 or larger (see notes in code).
const KAPPA: f64 = 0.25;

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
    /// * Identity (zero rotation) unit quaternion
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
        let mag = (qr*qr + qi.norm_squared()).sqrt();
        Self { qr: qr/mag, qi: qi/mag }
    }

    /// Initialize a unit quaternion given an angle and unit length rotation
    /// axis.  Use case determines if angle is passive or active.  User must
    /// ensure the Euler axis is unit length.
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

    /// Sets this quaternion based on the input direction cosine matrix.
    /// The angle is passive and `q*vq` acts as a reference frame
    /// transformation.
    ///
    /// Method based on discussion in "Quaternion Computation from a Geometric
    /// Point of View" by Malcolm Shuster and Gregory Natanson.
    ///
    /// The first element computed that is greater than KAPPA in magnitude
    /// is used as the starting point to determine the other elements.  See
    /// "DCM to Quaternion and Back Again" by Kurt Motekew for details
    ///
    /// # Argument
    ///
    /// * dmc  Direction cosine matrix, reference frame transformation
    ///        (passive).  No check is performed to ensure the input
    ///        is orthonormal.
    ///
    /// # Return
    ///
    /// * Result  Ok if a unit quaternion can be extracted from this DCM.
    ///
    pub fn try_from_dcm(dcm: &na::Matrix3::<f64>) -> Result<Self, String> {
        let mut tmp = 1.0 + dcm[(0,0)] + dcm[(1,1)] + dcm[(2,2)];
        if tmp > KAPPA {
            tmp = tmp.sqrt();
            let d4 = 0.5/tmp;
            return Ok(Self::new_normalized(
                0.5*tmp,
                &na::matrix![(dcm[(1,2)] - dcm[(2,1)])*d4 ;
                             (dcm[(2,0)] - dcm[(0,2)])*d4 ;
                             (dcm[(0,1)] - dcm[(1,0)])*d4],
            ));
        }

        tmp = 1.0 + dcm[(0,0)] - dcm[(1,1)] - dcm[(2,2)];
        if tmp > KAPPA {
            tmp = tmp.sqrt();
            let d4 = 0.5/tmp;
            return Ok(Self::new_normalized(
                (dcm[(1,2)] - dcm[(2,1)])*d4,
                &na::matrix![0.5*tmp                      ;
                             (dcm[(0,1)] + dcm[(1,0)])*d4 ;
                             (dcm[(0,2)] + dcm[(2,0)])*d4],

            ));
        }

        tmp = 1.0 - dcm[(0,0)] + dcm[(1,1)] - dcm[(2,2)];
        if tmp > KAPPA {
            tmp = tmp.sqrt();
            let d4 = 0.5/tmp;
            return Ok(Self::new_normalized(
                (dcm[(2,0)] - dcm[(0,2)])*d4,
                &na::matrix![(dcm[(0,1)] + dcm[(1,0)])*d4 ;
                             0.5*tmp                      ;
                            (dcm[(1,2)] + dcm[(2,1)])*d4],
            ));
        }

        tmp = 1.0 - dcm[(0,0)] - dcm[(1,1)] + dcm[(2,2)];
        if tmp > KAPPA {
            tmp = tmp.sqrt();
            let d4 = 0.5/tmp;
            return Ok(Self::new_normalized(
                (dcm[(0,1)] - dcm[(1,0)])*d4,
                &na::matrix![(dcm[(0,2)] + dcm[(2,0)])*d4 ;
                             (dcm[(1,2)] + dcm[(2,1)])*d4 ;
                             0.5*tmp],
            ));
        }

        return Err("Can't extract a Quaternion from input DCM".to_string()
            + &dcm.to_string());
    }
}

/// Public immutable (accessor and similar) methods
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

    /// # Return
    ///
    /// * Equivalent DCM (reference frame transformation).  This quaternion
    ///   must be a unit quaternion.
    pub fn dcm(&self) -> na::Matrix3::<f64> {
        let q0q0 = self.qr*self.qr;
        let q0qi = self.qr*self.qi[0];
        let q0qj = self.qr*self.qi[1];
        let q0qk = self.qr*self.qi[2];
        let qiqj = self.qi[0]*self.qi[1];
        let qiqk = self.qi[0]*self.qi[2];
        let qjqk = self.qi[1]*self.qi[2];
        na::matrix![2.0*(q0q0 + self.qi[0]*self.qi[0]) - 1.0,
                    2.0*(qiqj + q0qk),
                    2.0*(qiqk - q0qj) ;
                    2.0*(qiqj - q0qk),
                    2.0*(q0q0 + self.qi[1]*self.qi[1]) - 1.0,
                    2.0*(qjqk + q0qi) ;
                    2.0*(qiqk + q0qj),
                    2.0*(qjqk - q0qi),
                    2.0*(q0q0 + self.qi[2]*self.qi[2]) - 1.0]
    }
}

/// Mutable manipulation
impl Quaternion {
    /// Normalize this quaternion (convert to unit length) if magnitude
    /// differs from 1 by more than 100*f64::EPSILON.
    ///
    /// using criteria and method (Pade approximant) as described
    /// by David Hammen on stackoverflow, Oct 17 '12
    pub fn normalize(&mut self) {
        let n2 = self.qr*self.qr + self.qi.norm_squared();
        let delta = (1.0 - n2).abs();

        if delta < TOL {
        } else if delta < 2.107342e-08 {
            let norm_inv = 2.0/(1.0 + n2);
            self.qr *= norm_inv;
            self.qi *= norm_inv;
        } else {
            let norm_inv = 1.0/n2.sqrt();
            self.qr *= norm_inv;
            self.qi *= norm_inv;
        }
    }
}

/// Immutable functions
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

/// Immutable operator overload
impl Mul for Quaternion {
    type Output = Self;

    /// Quaternion multiplication via the outer product followed by
    /// summation of all elements, grouped by real and complex types.
    /// For passive quaternions, successive rotations are left to right
    /// when combined with the v*q (q.conjugate()*v*q) operator for
    /// reference frame transformations.  For active quaternions, sucessive
    /// rotations are right to left (matrix algebra)  for reference frame
    /// transformations.
    ///
    /// # Argument
    ///
    /// * rhs  The quaternion to multiply this one by.
    ///
    /// # Return
    ///
    /// * The product of this Quaternion and rhs, { self*rhs }
    ///
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

/// fgiesen link and other useful references for the vector rotation operator:
/// `https://fgiesen.wordpress.com/
///     2019/02/09/rotating-a-single-vector-using-a-quaternion/`
/// `https://www.johndcook.com/blog/2021/06/16/faster-quaternion-rotations/`
/// `https://blog.molecular-matters.com/
///     2013/05/24/a-faster-quaternion-vector-multiplication/`
///
impl Mul<na::Vector3<f64>> for Quaternion {
    type Output = na::Vector3<f64>;

    /// Overload Quaternion*Vector3 for the q*v*q.conjugate() operation where
    /// v is treated as the imaginary part of a pure quaternion.
    ///
    /// # Argument
    ///
    /// * rhs  Vector to rotate or transform depending on if the angle
    ///        associated with this quaternion is passive or active.
    ///
    /// # Return
    ///
    /// * q*v = q*v*q.conjugate(), a vector rotation if the angle defining the
    ///   quaternion is passive.  If the angle is active, then this operation
    ///   is a reference frame transformation.
    ///
    fn mul(self, rhs: na::Vector3<f64>) -> Self::Output {
        let tt = 2.0*self.qi.cross(&rhs);
        rhs + self.qr*tt + self.qi.cross(&tt)
    }
}

    /// Overload Quaternion/Vector3 for the q.conjugate()*v*q operation where
    /// v is treated as the imaginary part of a pure quaternion.
    ///
    /// # Argument
    ///
    /// * rhs  Vector to transform or rotate depending on if the angle
    ///        associated with this quaternion is passive or active.
    ///
    /// # Return
    ///
    /// * q/v =q.conjugate()*v*q, a vector reference frame transformation
    ///   (basis vector rotation) if the angle defining the quaternion is
    ///   passive.  If the angle is active, then this is a vector rotation.
    //
    // Previously tried v*q notation, but seemed easier to confuse.  Also,
    // this approach add trait to local struct vs. external struct:
    //
    //     impl Mul<Quaternion> for na::Vector3<f64> {
    //     fn mul(self, rhs: Quaternion) -> Self::Output {
    //
impl Div<na::Vector3<f64>> for Quaternion {
    type Output = na::Vector3<f64>;

    // Derived from above q*v operation
    fn div(self, rhs: na::Vector3<f64>) -> Self::Output {
        let tt = -2.0*self.qi.cross(&rhs);
        rhs + self.qr*tt - self.qi.cross(&tt)
    }
}


/// IO
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
    fn passive_active_unit() {
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

        let r1 = rotz(yaw);
        let r2 = roty(pitch);
        let r3 = rotx(roll);
        let ratt = r3*r2*r1;

        // DCM xform vs explicit q xform
        let dv = ratt*pos - (qatt.conjugate()*qpos*qatt).imaginary();
        assert!(dv.norm() < 10.0*f64::EPSILON);

        // DCM xform vs overload q xform
        let dv = ratt*pos - qatt/pos;
        assert!(dv.norm() < 10.0*f64::EPSILON);

        // DCM xform vs. DCM derived q with overload
        let qatt_dcm = Quaternion::try_from_dcm(&ratt).expect("Bad DCM");
        let dv = ratt*pos - qatt_dcm/pos;
        assert!(dv.norm() < 10.0*f64::EPSILON);

        // explicit xform vs. overload
        let dv = (qatt.conjugate()*qpos*qatt).imaginary() - qatt/pos;
        assert!(dv.norm() < 10.0*f64::EPSILON);

        // explicit q rotation vs. overload
        let dv = (qatt*qpos*qatt.conjugate()).imaginary() - qatt*pos;
        assert!(dv.norm() < 10.0*f64::EPSILON);
    }

    #[test]
    fn passive_itrative() {
        let pos = na::Vector3::new(1.0, 1.0, 1.0);

        let ihat = na::Vector3::new(1.0, 0.0, 0.0);
        let jhat = na::Vector3::new(0.0, 1.0, 0.0);
        let khat = na::Vector3::new(0.0, 0.0, 1.0);

        for yaw in (0..360).step_by(30) {
            let yaw = RAD_PER_DEG * yaw as f64;
            for pitch in (-90..90).step_by(30) {
                let pitch = RAD_PER_DEG * pitch as f64;
                for roll in (-90..90).step_by(30) {
                    let roll = RAD_PER_DEG * roll as f64;

                    let q1 = Quaternion::from_angle_axis(yaw, &khat);
                    let q2 = Quaternion::from_angle_axis(pitch, &jhat);
                    let q3 = Quaternion::from_angle_axis(roll, &ihat);
                    let qatt = q1*q2*q3;

                    let r1 = rotz(yaw);
                    let r2 = roty(pitch);
                    let r3 = rotx(roll);
                    let ratt = r3*r2*r1;

                    let qatt_dcm = Quaternion::try_from_dcm(&ratt)
                        .expect("Bad DCM");

                    let dv = ratt*pos - qatt/pos;
                    assert!(dv.norm() < 10.0*f64::EPSILON);
                    let dv = ratt*pos - qatt_dcm/pos;
                    assert!(dv.norm() < 10.0*f64::EPSILON);

                    let dcm = qatt.dcm();
                    assert!(((ratt*dcm.transpose()).norm()
                            - 3.0_f64.sqrt()).abs() < 10.0*f64::EPSILON);

                    /*
                    println!(
                        "Y: {}  P {}  R {}",
                        yaw/RAD_PER_DEG,
                        pitch/RAD_PER_DEG,
                        roll/RAD_PER_DEG
                    );
                    */
                }
            }
        }
    }
}
