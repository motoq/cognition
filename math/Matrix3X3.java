/*
 c  Matrix3x3.java
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

import cognition.math.tensor.TMatrix;

/**
 * Specialized three dimensional matrix class for common engineering
 * applications.
 *
 * @author Kurt Motekew
 * @since 20171203     
 */
public class Matrix3X3 extends TMatrix {

  /**
   * Create a 3X3 matrix with values initialized to zero.
   */
  public Matrix3X3() {
    super(3, 3);
  }

  /**
   * Instantiate given an input <code>Matrix3X3</code>
   * 
   * @param  mtx  Copy values into this
   */
  public Matrix3X3(Matrix3X3 mtx) {
    super(3,3);
    this.set(mtx);
  }

  /**
   * Create a 3X3 matrix that is the equivalent quaternion reference
   * frame rotation transformation (not a vector rotation). 
   *                                                       
   * @param  q  Quaternion to convert to a Matrix reference
   *            frame transformation.
   */
  public Matrix3X3(Quaternion q) {
    this();
    set(q);
  }

  /**
   * Create a reference frame transformation given a Cartesian
   * coordinate system axis and angle.
   *
   * @param  axis   Basis vector about which the rotation is made
   * @param  angle  Angle of rotation, radians.
   */
  public Matrix3X3(Basis3D axis, double angle) {
    super(3, 3);
    switch(axis) {
      case I:
        rotX(angle);
        break;
      case J:
        rotY(angle);
        break;
      case K:
        rotZ(angle);
        break;
    }
  }

  /**
   * Create a <code>Matrix3X3</code> by multiplying the two input
   * matrices together.  This = r2*r1
   *
   * @param  r2  Second transformation
   * @param  r1  First transformation
   */
  public Matrix3X3(Matrix3X3 r2, Matrix3X3 r1) {
    super(3,3);
    this.mult(r2, r1);
  }

  /**
   * Create a <code>Matrix3X3</code> by multiplying the three input
   * matrices together.  This = r3*r2*r1
   *
   * @param  r3  Third transformation
   * @param  r2  Second transformation
   * @param  r1  First transformation
   */
  public Matrix3X3(Matrix3X3 r3, Matrix3X3 r2, Matrix3X3 r1) {
    super(3,3);
    Matrix3X3 r2r1 = new Matrix3X3(r2, r1);
    this.mult(r3, r2r1);
  }

  /**
   * <code>Basis3D</code> based accessor method.
   *
   * @param  row  Row for the element to be returned.
   * @param  col  Column for the element to be returned.
   *
   * @return  Value stored at requested (row,col)
   */
  public final double get(Basis3D row, Basis3D col) {
    return get(row.ordinal(), col.ordinal());
  }

  /**
   * <code>Basis3D</code> based accessor method.
   *
   * @param  row    Row for the element to be set.
   * @param  col    Column for the element to be set.
   * @param  value  Value to store at requested (row,col)
   */
  public final void set(Basis3D row, Basis3D col, double value) {
    set(row.ordinal(), col.ordinal(), value);
  }

  /**
   * Sets this matrix to the equivalent quaternion reference
   * frame rotation transformation (not a vector rotation). 
   *                                                       
   * @param  q  Quaternion to convert to a Matrix reference
   *            frame transformation.
   */
  public final void set(Quaternion q) {
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

    set(0,0, 2.0*(q0q0 + qi*qi) - 1.0);
    set(0,1, 2.0*(qiqj + q0qk));       
    set(0,2, 2.0*(qiqk - q0qj));

    set(1,0, 2.0*(qiqj - q0qk));
    set(1,1, 2.0*(q0q0 + qj*qj) - 1.0);
    set(1,2, 2.0*(qjqk + q0qi));

    set(2,0, 2.0*(qiqk + q0qj));
    set(2,1, 2.0*(qjqk - q0qi));
    set(2,2, 2.0*(q0q0 + qk*qk) - 1.0);
  }

  /**
   * Sets this Matrix to be a reference frame transformation representing
   * a rotation about the X-axis by the input angle.
   *
   * @param  alpha  Rotation angle about X-axis, radians
   */
  public final void rotX(double alpha) {
    final double calpha = Math.cos(alpha);
    final double salpha = Math.sin(alpha);

    set(0,0, 1.0);  set(0,1,     0.0);  set(0,2,    0.0);
    set(1,0, 0.0);  set(1,1,  calpha);  set(1,2, salpha);
    set(2,0, 0.0);  set(2,1, -salpha);  set(2,2, calpha);
  }

  /**
   * Sets this Matrix to be a reference frame transformation representing
   * a rotation about the Y-axis by the input angle.
   *
   * @param  alpha  Rotation angle about Y-axis, radians
   */
  public final void rotY(double alpha) {
    final double calpha = Math.cos(alpha);
    final double salpha = Math.sin(alpha);

    set(0,0, calpha);  set(0,1, 0.0);  set(0,2, -salpha);
    set(1,0,    0.0);  set(1,1, 1.0);  set(1,2,     0.0);
    set(2,0, salpha);  set(2,1, 0.0);  set(2,2,  calpha);
  }

  /**
   * Sets this Matrix to be a reference frame transformation representing
   * a rotation about the Z-axis by the input angle.
   *
   * @param  alpha  Rotation angle about Z-axis, radians
   */
  public final void rotZ(double alpha) {
    final double calpha = Math.cos(alpha);
    final double salpha = Math.sin(alpha);

    set(0,0,  calpha);  set(0,1, salpha);  set(0,2, 0.0);
    set(1,0, -salpha);  set(1,1, calpha);  set(1,2, 0.0);
    set(2,0,     0.0);  set(2,1,    0.0);  set(2,2, 1.0);
  }
}
