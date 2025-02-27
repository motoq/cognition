/*
 * Copyright 2025 Kurt Motekew
 *      
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use nalgebra as na;

/*
 * Computes the tangent point on a unit circle given a location
 * and pointing vector from that location.  The side of the circle
 * most closely aligns with the pointing vector is chosen for the
 * returned tangent point.
 *
 * @param  pos  Position external to circle, origin of pointing vector
 * @param  pnt  Pointing vector for which the tangent line will be
 *              closest to (there are two tangent points for each point
 *              not on the circle).
 *
 * @return   Location of the tangent point.  If the originating position
 *           is within the circle, then the location on the circle
 *           closest to the position is returned (the line from the
 *           origin through pos to the circle).
 *
 * @author  Kurt Motekew  2022/01/29  Initial, C++ version
 * @author  Kurt Motekew  2025/02/24  Rust version
 */
pub fn tangent(pos: &na::SMatrix<f64, 2, 1>,
               pnt: &na::SMatrix<f64, 2, 1>) -> na::SMatrix<f64, 2, 1> {
    let r2 = pos.dot(&pos);
    let rmag = r2.sqrt();
    let rhat =pos/rmag;
    // Inside or on the circle
    let s2 = r2 - 1.0;
    if s2 <= 0.0 {
        return rhat;
    }

    // Sine and Cosine of angle between position vector and tangent
    // pointing vector
    let s = s2.sqrt();
    let sa = 1.0/rmag;
    let ca = s*sa;

    // Orthogonal to rhat - used to form linear combo to tangent point
    let rhat_orth = na::matrix![-rhat[1] ; rhat[0]];
    // Along rhat component and normal components
    let tpa = (rmag - s*ca)*rhat;
    let tpn = s*sa*rhat_orth;
    if pnt.dot(&rhat_orth) > 0.0 {
        return tpa + tpn;
    } else {
        return tpa - tpn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_tangent() {
        let pos = na::matrix![0.354799500241270 ; -1.570944720424161];
        let pnt = na::matrix![-7.327042094401748e-02 ; -1.170312541765953e+00];
        let tp = tangent(&pos, &pnt);
        let tpt = na::matrix![-0.627824961922289 ; -0.778354557504019];
        let err = (tp - tpt).norm();
        assert!(err < 1.0e-10);
    }
}

/*
 * Computes the intersection point on a unit circle given a location
 * and pointing vector from that location.  Make sure to use with the
 * NoSolutionException that will be thrown if the pointing vector misses
 * the circle.
 *
 * @tparam  T  Data type
 *
 * @param  pos  Position external to circle, origin of pointing vector
 * @param  pnt  Pointing vector
 *
 * @return   Location of intersection on the circle
 *
 * @throws  NoSolutionException When the pointing vector does not
 *                              intersect the circle.
 *
 * @author  Kurt Motekew  2022/01/27
 */
/*
template<typename T>
Eigen::Matrix<T, 2, 1> intersect(const Eigen::Matrix<T, 2, 1>& pos,
                                 const Eigen::Matrix<T, 2, 1>& pnt)
{
  Eigen::Matrix<T, 2, 1> pnt_hat {pnt.normalized()};

  T alpha {pnt_hat(0)*pnt_hat(0) + pnt_hat(1)*pnt_hat(1)};
  T beta {pos(0)*pnt_hat(0) + pos(1)*pnt_hat(1)};
  T gamma {pos(0)*pos(0) + pos(1)*pos(1)};

  T zero {static_cast<T>(0)};
  T one {static_cast<T>(1)};
  T d {beta*beta - alpha*(gamma - one)};
  if (d >= zero) {
    T s = -(beta + std::sqrt(d))/alpha;
    return pos + s*pnt_hat;
  } else {
    throw NoSolutionException("unit_circle::intersect");
  }
}
*/
