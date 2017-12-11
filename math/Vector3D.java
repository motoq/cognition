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
}
