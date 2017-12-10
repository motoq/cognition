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
   * Create a 3X3 matrix with values initizlized to zero.
   */
  public Matrix3X3() {
    super(3, 3);
  }

  /**
   * <code>Basis3D</code> based accessor method.
   *
   * @param  row  Row for the element to be returned.
   * @param  col  Column for the element to be returned.
   *
   * @return  Value stored at requested (row,col)
   */
  public double get(Basis3D row, Basis3D col) {
    return get(row.ordinal(), col.ordinal());
  }

  /**
   * <code>Basis3D</code> based accessor method.
   *
   * @param  row    Row for the element to be set.
   * @param  col    Column for the element to be set.
   * @param  value  Value to store at requested (row,col)
   */
  public void set(Basis3D row, Basis3D col, double value) {
    set(row.ordinal(), col.ordinal(), value);
  }

}
