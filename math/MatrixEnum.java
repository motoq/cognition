/*
 c  MatrixEnum.java
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
 * Matrix class where the two enum types define the number of rows
 * and columns along with allowing enum based accessor methods.
 * Different enum types can be used for the row and column definitions.
 * <P>
 * Instantiation via the constructor may appear a bit awkward given
 * an element of the enum is required:
 * <pre>
 * {@code
 * MatrixEnum<Basis3D, Q> m3x4 = new MatrixEnum<>(Basis3D.I, Q.Q0);
 * }
 * </pre>
 * The factory method may be more aesthetically pleasing
 * <pre>
 * {@code
 * MatrixEnum<Basis3D, Q> m3x4 = MatrixEnum.factory(Basis3D.class, Q.class);
 * }
 * </pre>
 *
 * @param  <I>  An enum type that sets the number of rows and is used for
 *              row indexing
 * @param  <J>  An enum type that sets the number of columns and is used
 *              for column indexing
 *
 * @author Kurt Motekew
 * @since 20171203
 */
public class MatrixEnum<I extends Enum<I>, J extends Enum<J>> extends TMatrix {

  /**
   * Instantiate via the constructor and "sample" enum elements.
   * Different enum types can be used for the row and column
   * definitions.  See the class description for an example.  All
   * elements initialized to zero.
   *
   * @param  ii  Any element of the enum I
   * @param  jj  Any element of the enum J
   */
  public MatrixEnum(I ii, J jj) {
    super(ii.getDeclaringClass().getEnumConstants().length,
          jj.getDeclaringClass().getEnumConstants().length);
  }

  /**
   * Accessor method using I and J Enum values
   *
   * @param  eRow  Row index
   * @param  eCol  Column index
   *
   * @return  Value stored at requested row/column
   */
  public double get(I eRow, J eCol) {
    return super.get(eRow.ordinal(), eCol.ordinal());
  }

  /**
   * Accessor method using I and J Enum values
   *
   * @param  eRow   Row index
   * @param  eCol   Column index
   * @param  value  Value to store at requested row/column
   */
  public void set(I eRow, J eCol, double value) {
    super.set(eRow.ordinal(), eCol.ordinal(), value);
  }

  /**
   * Creates an enum based Matrix given the enum Classes to use.  See
   * class description for an example.  All elements initizlized to zero.
   *
   * @param  <I>  An enum type that sets the number of rows and is used for
   *              row indexing
   * @param  <J>  An enum type that sets the number of columns and is used
   *              for column indexing                                                             
   * @param  enI  enum.class for row definition
   * @param  enJ  enum.class for column definition
   *                        
   * @return  Matrix based on requested enum
   */
  public static <I extends Enum<I>, J extends Enum<J>> MatrixEnum<I, J>
                                                  factory(final Class<I> enI,
                                                          final Class<J> enJ) {

    MatrixEnum<I, J> mtx = new MatrixEnum<>(enI.getEnumConstants()[0],
                                                  enJ.getEnumConstants()[0]);
    return mtx;
  }
}
