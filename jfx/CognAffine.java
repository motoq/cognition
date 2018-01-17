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
 * @author Kurt Motekew
 * @since 20180117
 */
public class CognAffine extends Affine {

  /**
   * Create the do nothing affine transformation - translation is
   * zero and rotation is the identity.
   */
  public CognAffine() {
  }

  /**
   * Instantiate with input transformations
   *
   * @param  rot    Reference frame transformation
   * @param  trans  Translation
   */
  public CognAffine(Matrix3X3 rot, Vector3D trans) {
    set(rot, trans);
  }

  /**
   * @param  rot  Instantiate with this reference frame transformation
   *              and no translation.
   */
  public CognAffine(Matrix3X3 rot) {
    set(rot);
  }

  /**
   * @param  trans  Instantiate with this translation and the identity
   *                reference frame transformation.
   */
  public CognAffine(Vector3D trans) {
    set(trans);
  }

  /**
   * @param  rot    Set the reference frame transformation
   * @param  trans  Set the translation component
   */
  public final void set(Matrix3X3 rot, Vector3D trans) {
    setMxx(rot.get(Basis3D.I, Basis3D.I));                      
    setMxy(rot.get(Basis3D.I, Basis3D.J));
    setMxz(rot.get(Basis3D.I, Basis3D.K));
    setTx(trans.get(Basis3D.I));          
    setMyx(rot.get(Basis3D.J, Basis3D.I));
    setMyy(rot.get(Basis3D.J, Basis3D.J));
    setMyz(rot.get(Basis3D.J, Basis3D.K));
    setTy(trans.get(Basis3D.J));
    setMzx(rot.get(Basis3D.K, Basis3D.I));
    setMzy(rot.get(Basis3D.K, Basis3D.J));
    setMzz(rot.get(Basis3D.K, Basis3D.K));
    setTz(trans.get(Basis3D.K));
  }

  /**
   * @param  rot  Set the reference frame transformation and reset
   *              the translation to zero.
   */
  public final void set(Matrix3X3 rot) {
    setMxx(rot.get(Basis3D.I, Basis3D.I));                      
    setMxy(rot.get(Basis3D.I, Basis3D.J));
    setMxz(rot.get(Basis3D.I, Basis3D.K));
    setTx(0.0);          
    setMyx(rot.get(Basis3D.J, Basis3D.I));
    setMyy(rot.get(Basis3D.J, Basis3D.J));
    setMyz(rot.get(Basis3D.J, Basis3D.K));
    setTy(0.0);
    setMzx(rot.get(Basis3D.K, Basis3D.I));
    setMzy(rot.get(Basis3D.K, Basis3D.J));
    setMzz(rot.get(Basis3D.K, Basis3D.K));
    setTz(0.0);
  }

  /**
   * @param  trans  Set translation and reset the reference frame 
   *                transformation to the identity matrix.
   */
  public final void set(Vector3D trans) {
    setMxx(1.0);                      
    setMxy(0.0);
    setMxz(0.0);
    setTx(trans.get(Basis3D.I));          
    setMyx(0.0);
    setMyy(1.0);
    setMyz(0.0);
    setTy(trans.get(Basis3D.J));
    setMzx(0.0);
    setMzy(0.0);
    setMzz(1.0);
    setTz(trans.get(Basis3D.K));
  }

  /**
   * Set to the do nothing transformation - identity regerence frame
   * transformation and zero translation.
   */
  public void reset() {
    setMxx(1.0);                      
    setMxy(0.0);
    setMxz(0.0);
    setTx(0.0);          
    setMyx(0.0);
    setMyy(1.0);
    setMyz(0.0);
    setTy(0.0);
    setMzx(0.0);
    setMzy(0.0);
    setMzz(1.0);
    setTz(0.0);
  }

}
