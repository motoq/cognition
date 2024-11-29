/*
 c  TVector.java
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

package cognition.math.tensor;

/**
 * Vector class.  Note the get() and set() methods are offset based
 * such that the first index is zero (C/C++ and Java convention).
 * The ndx accessor methods are one based (FORTRAN, Matlab).     
 *
 * @author Kurt Motekew
 * @since 20171203     
 */
public class TVector extends TMatrix {
  private final double[] vals;

  /**
   * Initialize with all elements set to zero.
   *
   * @param  rows  Dimension, the number of elements in this vector
   */
  public TVector(int rows) {
    super(rows, 1);
    vals = super.valuesPtr();
  }

  /**
   * Initialize a column vector given an array of values.
   *
   * @param  vec  Array of values to copy into this vector.  The length
   *              of the input array will be the number of rows.
   */
  public TVector(double[] vec) {
    super(vec.length, 1);
    vals = super.valuesPtr();
    System.arraycopy(vec, 0, vals, 0, vec.length);
  }

  /**
   * Initialize given an input TVector
   *
   * @param  vec  Values are copied
   */
  public TVector(TVector vec) {
    this(vec.vals.length);
    System.arraycopy(vec.vals, 0, vals, 0, vec.vals.length);
  }

  /**
   * Copy values from an input array into this vector.
   *
   * @param  vec  Values from vec will be copied to this TVector.  The
   *              number of elements must match.
   *
   * @throws IllegalArgumentException if dimensions do not match
   */
  public final void set(double[] vec) {
    if (vec.length != size()) {
      throw new IllegalArgumentException("TVector.set:  Can't set a " +
            size() + " TVector with a " + "double[" + vec.length + "].");
    }
    System.arraycopy(vec, 0, vals, 0, vec.length);
  }

  /**
   * Offset based accessor method
   *
   * @param  ii  Row, 0 <= ii < numRows
   *
   * @return  Value stored in the iith offset
   */
  public final double get(int ii) {
    return vals[ii];
  }

  /**
   * Offset based accessor method
   *
   * @param  ii     Row, 0 <= ii < numRows
   * @param  value  Value to store in the iith offset
   */
  public final void set(int ii, double value) {
    vals[ii] = value;
  }

  /**
   * Index based accessor method
   *
   * @param  row  Row, 1 <= ii <= numRows
   *
   * @return  Value of ith element
   */
  public final double ndx(int row) {
    return vals[row-1];
  }

  /**
   * Index based accessor method
   *
   * @param  row    Row, 1 <= ii <= numRows
   * @param  value  Value to store as the ith element
   */
  public final void ndx(int row, double value) {
    vals[row-1] = value;
  }
}
