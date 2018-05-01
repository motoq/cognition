/*
 c  JFX2ComputationalFrame.java
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

import cognition.math.Basis3D;
import cognition.math.Matrix3X3;
import cognition.math.Quaternion;
import cognition.math.Angles;

/**
 * A JavaFX Affine transformation used to convert a traditional
 * z-axis up computational reference frame into the JavaFX display
 * reference frame.
 *
 * @author Kurt Motekew
 * @since  20180430
 */
public class JFX2ComputationalFrame extends CognAffine {
  /**
   * Create a JavaFX Affine transformation that converts the default
   * graphics y-axis up and z-axis out of the page reference frame into
   * one where the z-axis is up and the x-axis points out of the page.
   */
  public JFX2ComputationalFrame() {
    Quaternion q = new Quaternion();
    Quaternion q1 = new Quaternion();
    Quaternion q2 = new Quaternion();
  
    q1.set(Angles.PIO2, Basis3D.I);
    q2.set(-Angles.PIO2, Basis3D.K);
    q.mult(q1, q2);    
    q.conj();
    Matrix3X3 rot = new Matrix3X3(q);

    setAll(rot);
  }
}
