/*
 c  CognAffine.java
 c
 c  Copyright (C) 2018 Kurt Motekew
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

package cognition.jfx;

import javafx.scene.transform.Affine;

import cognition.math.Basis3D;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;

/**
 * Specialization of the JFX Affine class allowing for construction
 * and modification via the cognition.math library.
 *
 * Rotation is applied first, followed by translation
 * 
 * Note that when applied to a JavaFX Node as a Transform, the input direction
 * cosine matrices are rotation (direct, or active) transformations, rotating
 * vector locations instead of transforming vector components to another
 * reference frame.  Therefore, if the attitude of a Node within the display
 * reference frame is computed, take the transpose before passing to this
 * class.
 *
 * @author Kurt Motekew
 * @since 20180117
 */
public class CognAffine extends Affine {

  /**
   * Create the do nothing affine transformation - no translation and
   * zero rotation.
   */
  public CognAffine() {
  }

  /**
   * Instantiate with input transformations
   *
   * @param  rot    Axis rotation (direct transformation)
   * @param  trans  Translation
   */
  public CognAffine(Matrix3X3 rot, Vector3D trans) {
    set(rot, trans);
  }

  /**
   * @param  rot  Instantiate with input axis rotation (direct transformation)
   *              and no translation.
   */
  public CognAffine(Matrix3X3 rot) {
    setAll(rot);
  }

  /**
   * @param  trans  Instantiate with this translation and no rotation.
   */
  public CognAffine(Vector3D trans) {
    setAll(trans);
  }

  /**
   * Set the rotation and translation
   *
   * @param  rot    Axis rotation (direct transformation)
   * @param  trans  Translation
   */
  public final void set(Matrix3X3 rot, Vector3D trans) {
    setToTransform(rot.get(Basis3D.I, Basis3D.I),
                   rot.get(Basis3D.I, Basis3D.J),
                   rot.get(Basis3D.I, Basis3D.K),
                   trans.get(Basis3D.I),
                   rot.get(Basis3D.J, Basis3D.I),
                   rot.get(Basis3D.J, Basis3D.J),
                   rot.get(Basis3D.J, Basis3D.K),
                   trans.get(Basis3D.J),
                   rot.get(Basis3D.K, Basis3D.I),
                   rot.get(Basis3D.K, Basis3D.J),
                   rot.get(Basis3D.K, Basis3D.K),
                   trans.get(Basis3D.K));
  }

  /**
   * Set the rotation and zero translation
   *
   * @param  rot  Axis rotation (direct transformation)
   */
  public final void setAll(Matrix3X3 rot) {
    setToTransform(rot.get(Basis3D.I, Basis3D.I),
                   rot.get(Basis3D.I, Basis3D.J),
                   rot.get(Basis3D.I, Basis3D.K),
                   0.0,
                   rot.get(Basis3D.J, Basis3D.I),
                   rot.get(Basis3D.J, Basis3D.J),
                   rot.get(Basis3D.J, Basis3D.K),
                   0.0,
                   rot.get(Basis3D.K, Basis3D.I),
                   rot.get(Basis3D.K, Basis3D.J),
                   rot.get(Basis3D.K, Basis3D.K),
                   0.0);
  }

  /**
   * Set translation and zero rotation.
   *
   * @param  trans  Translation.
   */
  public final void setAll(Vector3D trans) {
    setToTransform(1.0, 0.0, 0.0, trans.get(Basis3D.I),
                   0.0, 1.0, 0.0, trans.get(Basis3D.J),
                   0.0, 0.0, 1.0, trans.get(Basis3D.K));
  }

  /**
   * Set to the do nothing transformation - identity rotation and
   * zero translation.
   */
  public final void reset() {
    setToTransform(1.0, 0.0, 0.0, 0.0,
                   0.0, 1.0, 0.0, 0.0,
                   0.0, 0.0, 1.0, 0.0);
  }
}
