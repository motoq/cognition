/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Computes reference frame transformation (basis vector rotation) matrices

use nalgebra as na;

/// Reference frame transformation due to a rotation about the x-axis
///
/// # Argument
///
/// * phi  Rotation of y-z basis vectors about the x-axis
///
/// # Return
///
/// * Direction cosine matrix
///
pub fn rotx(phi: f64) -> na::SMatrix<f64, 3, 3> {
    let cp = phi.cos();
    let sp = phi.sin();
    na::matrix![1.0, 0.0, 0.0 ;
                0.0,  cp,  sp ;
                0.0, -sp,  cp]
}

/// Reference frame transformation due to a rotation about the y-axis
///
/// # Argument
///
/// * theta  Rotation of z-x basis vectors about the y-axis
///
/// # Return
///
/// * Direction cosine matrix
///
pub fn roty(theta: f64) -> na::SMatrix<f64, 3, 3> {
    let ct = theta.cos();
    let st = theta.sin();
    na::matrix![ ct, 0.0, -st ;
                0.0, 1.0, 0.0 ;
                 st, 0.0,  ct]
}

/// Reference frame transformation due to a rotation about the z-axis
///
/// # Argument
///
/// * psi  Rotation of x-y basis vectors about the z-axis
///
/// # Return
///
/// * Direction cosine matrix
///
pub fn rotz(psi: f64) -> na::SMatrix<f64, 3, 3> {
    let cp = psi.cos();
    let sp = psi.sin();
    na::matrix![ cp,  sp, 0.0 ;
                -sp,  cp, 0.0 ;
                0.0, 0.0, 1.0]
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Checks DCM generation rot*() functions against nalgebra
    /// UnitQuaternion while illustrating nalgebra interprets
    /// 3x3 orthonormal matrices as vector rotations instead of
    /// reference frame transformations.  Also confirms the quaternion
    /// '*' operation with a vector acts as a vector rotation.
    #[test]
    fn dcm_check() {
        let pi = std::f64::consts::PI;
        let eps = 1.0e-13;
        //
        let roll = 2.0*pi/3.0;
        let m_roll = rotx(roll);
        let q_roll = na::UnitQuaternion::<f64>::from_axis_angle(
            &na::Vector3::<f64>::x_axis(), roll);
        let qm_roll = na::UnitQuaternion::<f64>::from_matrix(&m_roll);
        //
        let pitch = -pi/3.0;
        let m_pitch = roty(pitch);
        let q_pitch = na::UnitQuaternion::<f64>::from_axis_angle(
            &na::Vector3::<f64>::y_axis(), pitch);
        let qm_pitch = na::UnitQuaternion::<f64>::from_matrix(&m_pitch);
        //
        let yaw = pi/6.0;
        let myaw = rotz(yaw);
        let qyaw = na::UnitQuaternion::<f64>::from_axis_angle(
            &na::Vector3::<f64>::z_axis(), yaw);
        let qmyaw = na::UnitQuaternion::<f64>::from_matrix(&myaw);


        // Original vector - subject to reference frame transformations
        // The matrix version uses traditional left to right order of
        // operation.  The equivalent quaternion implementation is shown
        // multiple ways, illustrating the nalgebra interpretation of the
        // '*' operator and DCM.
        let v0 = na::matrix![1.0 ; 1.0 ; 1.0];
        // Aerospace sequence, yaw, pitch, then roll, via DCMs...
        let v_dcm = m_roll*m_pitch*myaw*v0;
        // ...via quaternions and na '*' op with left to right multiplication
        let v_qcg = (qyaw*q_pitch*q_roll).conjugate()*v0;
        // ...via quaternions and na '*' op with right to left multiplication
        let v_qmt = (q_roll.conjugate()*q_pitch.conjugate()*qyaw.conjugate())*v0;
        // ...via na interpretation of DCM to quaternion and '*' op
        let v_qcgm = (qm_roll*qm_pitch*qmyaw)*v0;

        assert!((v_dcm - v_qcg).norm() < eps);
        assert!((v_dcm - v_qmt).norm() < eps);
        assert!((v_dcm - v_qcgm).norm() < eps);
    }
}
