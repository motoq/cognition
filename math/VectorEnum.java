/*
 c  VectorEnum.java
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
 * Vector class where an enum defines the number of elements
 * an allows enum based accessor methods.
 * <P>
 * Instantiation via the constructor may appear a bit awkward given
 * an element of the enum is required:
 * <pre>
 * {@code
 * VectorEnum<Basis3D> v3d = new VectorEnum<>(Basis3D.I);
 * }
 * </pre>
 * The factory method may be more aesthetically pleasing
 * <pre>
 * {@code
 * VectorEnum<Basis3D> v3d = VectorEnum.factory(Basis3D.class);
 * }
 * </pre>
 *
 * @param  <I>  An enum type that sets the vector size and is used for indexing
 *
 * @author Kurt Motekew
 * @since 20171203
 */
public class VectorEnum<I extends Enum<I>> extends TVector {

  /**
   * Instantiate via the constructor and a "sample" enum element.
   * See the class description for an example.  All elements
   * initialized to zero.
   *
   * @param  ii  Any element of the enum I
   */
  public VectorEnum(I ii) {
    super(ii.getDeclaringClass().getEnumConstants().length);
  }

  /**
   * Accessor method using Enum I
   *
   * @param  ii  Index for the element to be returned.
   *
   * @return  Value stored at requested index.
   */
  public double get(I ii) {
    return get(ii.ordinal());
  }               

  /**
   * Accessor method using Enum I
   *
   * @param  ii     Index for the element to be set.
   * @param  value  Value to store at requested index.
   */
  public void set(I ii, double value) {
    set(ii.ordinal(), value);
  }

  /**
   * Creates an enum based Vector given the enum Class.  See class
   * description for an example.  All elements initizlized to zero.
   *
   * @param  <I>  An enum type that sets the vector size and is used for indexing
   * @param  enI  enum.class for index definition
   *
   * @return  Vector based on requested enum
   */
  public static <I extends Enum<I>> VectorEnum<I> factory(final Class<I> enI) {
    //I[] evI = enI.getEnumConstants();
    //VectorEnum<I> vec = new VectorEnum<I>(evI[0]);
    VectorEnum<I> vec = new VectorEnum<>(enI.getEnumConstants()[0]);
    return vec;                           
  }

}
