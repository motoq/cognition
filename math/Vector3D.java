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

}
