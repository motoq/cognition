/*
 c  Vector3D.java
 c
 c  Copyright (C) 2017 Kurt Motekew
 c
 c  This library is free software; you can redistribute it and/or
 c  modify it under the terms of the GNU Lesser General Public
 c  License as published by the Free Software Foundation; either
 c  version 2.1 of the License, or (at your option) any later version.
 c
 c  This library is distributed in the hope that it will be useful,
 c  but WITHOUT ANY WARRANTY; without even the implied warranty of
 c  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 c  Lesser General Public License for more details.
 c
 c  You should have received a copy of the GNU Lesser General Public
 c  License along with this library; if not, write to the Free Software
 c  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 c  02110-1301 USA
 */

package cognition.math;

import cognition.math.tensor.TVector;

/**
 * Specialized three dimensional vector class for common engineering
 * applications.
 *
 * @author Kurt Motekew
 * @since 20171203     
 */
public class Vector3D extends TVector {

  /**
   * Create a 3x1 matrix with values initialized to zero.
   */
  public Vector3D() {
    super(3);
  }

  /**
   * Create a 3x1 matrix and initialize elements with scalar inputs
   *
   * @param  x  First element value
   * @param  y  Second element value
   * @param  z  Third element value
   */
  public Vector3D(double x, double y, double z) {
    super(3);
    set(0, x);
    set(1, y);
    set(2, z);
  }

  /**
   * <code>Basis3D</code> based accessor method.
   *
   * @param  ii  Index for the element to be returned.
   *
   * @return  Value stored at requested index.
   */
  public double get(Basis3D ii) { return get(ii.ordinal()); }

  /**
   * <code>Basis3D</code> based accessor method.
   *
   * @param  ii     Index for the element to be set.
   * @param  value  Value to store at requested index.
   */
  public void set(Basis3D ii, double value) { set(ii.ordinal(), value); }

  /**
   * Set this vector to the product of the input [3x3] matrix and
   * [3x1] vector.  this = mtx*vec
   * 
   * @param  mtx  [3x3] matrix on left side of the operand
   * @param  vec  [3x1] vector on the right side of the operand
   */
  public void mult(Matrix3X3 mtx, Vector3D vec) {
    final double x = vec.get(0);
    final double y = vec.get(1);
    final double z = vec.get(2);
    set(0, x*mtx.get(0,0) + y*mtx.get(0,1) + z*mtx.get(0,2)); 
    set(1, x*mtx.get(1,0) + y*mtx.get(1,1) + z*mtx.get(1,2)); 
    set(2, x*mtx.get(2,0) + y*mtx.get(2,1) + z*mtx.get(2,2)); 
  }

  /**
   * q*vq  (quaternion multiplication performed left to right)
   * <P>
   * Set this vector with the result of applying a reference frame
   * transformation to the input <code>Vector3D</code> with the input
   * <code>Quaternion</code>.
   *
   * @param  q  Unit quaternion
   * @param  v  Vector to be subjected to a reference frame transformation.
   *            The components of the vector change but the vector itself
   *            remains the same.
   */
  public void transform(Quaternion q, Vector3D v) {
    final double q0 = q.get(Q.Q0);
    final double qi = q.get(Q.QI);
    final double qj = q.get(Q.QJ);
    final double qk = q.get(Q.QK);

    final double q0q0 = q0*q0;
    final double q0qi = q0*qi;
    final double q0qj = q0*qj;
    final double q0qk = q0*qk;
    final double qiqj = qi*qj;
    final double qiqk = qi*qk;
    final double qjqk = qj*qk;

    final double q11 = 2.0*(q0q0 + qi*qi) - 1.0;
    final double q21 = 2.0*(qiqj + q0qk);
    final double q31 = 2.0*(qiqk - q0qj);
    final double q12 = 2.0*(qiqj - q0qk);
    final double q22 = 2.0*(q0q0 + qj*qj) - 1.0;
    final double q32 = 2.0*(qjqk + q0qi);
    final double q13 = 2.0*(qiqk + q0qj);
    final double q23 = 2.0*(qjqk - q0qi);
    final double q33 = 2.0*(q0q0 + qk*qk) - 1.0;

    set(0, q11*v.get(0) + q21*v.get(1) + q31*v.get(2));
    set(1, q12*v.get(0) + q22*v.get(1) + q32*v.get(2));
    set(2, q13*v.get(0) + q23*v.get(1) + q33*v.get(2));
  }

  /**
   * qvq*  (quaternion multiplication performed left to right)
   * <P>
   * Set this vector with the result rotating the input <code>Vector3D</code>
   * using the input <code>Quaternion</code>.
   *
   * @param  q  Unit quaternion
   * @param  v  Vector to be rotated (while the reference frame remains
   *            the same).
   */
  public void rotate(Quaternion q, Vector3D v) {
    final double q0 = q.get(Q.Q0);
    final double qi = q.get(Q.QI);
    final double qj = q.get(Q.QJ);
    final double qk = q.get(Q.QK);

    final double q0q0 = q0*q0;
    final double q0qi = q0*qi;
    final double q0qj = q0*qj;
    final double q0qk = q0*qk;
    final double qiqj = qi*qj;
    final double qiqk = qi*qk;
    final double qjqk = qj*qk;

    final double q11 = 2.0*(q0q0 + qi*qi) - 1.0;
    final double q21 = 2.0*(qiqj + q0qk);
    final double q31 = 2.0*(qiqk - q0qj);
    final double q12 = 2.0*(qiqj - q0qk);
    final double q22 = 2.0*(q0q0 + qj*qj) - 1.0;
    final double q32 = 2.0*(qjqk + q0qi);
    final double q13 = 2.0*(qiqk + q0qj);
    final double q23 = 2.0*(qjqk - q0qi);
    final double q33 = 2.0*(q0q0 + qk*qk) - 1.0;

    set(0, q11*v.get(0) + q12*v.get(1) + q13*v.get(2));
    set(1, q21*v.get(0) + q22*v.get(1) + q23*v.get(2));
    set(2, q31*v.get(0) + q32*v.get(1) + q33*v.get(2));
  }
}
