/*
 c  Tensor.java
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

import java.util.Arrays;

/**
 * Tensor class, currently only as the superclass to matrix and vector
 * classes.
 *
 * @author Kurt Motekew
 * @since 20171203
 */
public class Tensor {
  private final int SIZE;
  private final int RANK;
  private final double[] vals;

  /**
   * Initialize as a rank 1 tensor
   *
   * @param  dimension  Number of elements in the vector.
   */
  public Tensor(int dimension) {
    RANK = 1;
    SIZE = dimension;
    vals = new double[SIZE];
  }

  /**
   * Initialize as a rank 2 tensor
   *
   * @param  rows  Number of elements the row index
   * @param  cols  Number of elements in column index
   */
  public Tensor(int rows, int cols) {
    RANK = 2;
    SIZE = rows*cols;
    vals = new double[SIZE];
  }

  /**
   * @return   The rank of the tensor
   */
  public int rank() { return RANK; }
  
  /**
   *  @return   The total number of elements composing the tensor
   */
  public int size() { return SIZE; }
  
  /**
   * Zero every element of this tensor
   */
  public void zero() {
    Arrays.fill(vals, 0.0);
  }

  //public void set(Tensor t) {
  //  System.arraycopy(t.vals, 0, vals, 0, SIZE);
  //}

  /**
   * Multiply this tensor by a scalar
   *
   * @param  sfactor
   */
  public void mult(double sfactor) {
    for (int ii=0; ii<SIZE; ii++) {
      vals[ii] *= sfactor;
    }
  }

  /**
   * Computes the Frobenius norm of the tensor.  For a rank 1 tensor,
   * this method does what you expect it to do - computes the magnitude.
   * For any other tensor, it is just the RSS of each element, useful
   * for testing & validating tensor operations.
   *
   * @return  The square root of the sum of the squares of every
   *          element in this tensor.
   */
  public double norm() {
    double ss = 0.0;
    for (int ii=0; ii<SIZE; ii++) {
      ss += vals[ii]*vals[ii];
    }
    return Math.sqrt(ss);
  }

  /*
   * @return pointer to the array storing all values in this tensor
   */
  double [] valuesPtr() {
    return vals;
  }
}
