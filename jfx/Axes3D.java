/*
 c  Axes3D.java
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

import javafx.scene.Group;
import javafx.scene.shape.Sphere;  
import javafx.scene.shape.DrawMode;
import javafx.scene.transform.Affine;

import cognition.math.Basis3D;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;
import cognition.math.Angles;

/**
 * A JavaFX 3D Cartesian reference frame axes.  Each axis is the same length.
 *
 * @author Kurt Motekew
 * @since  20180424
 */
public class Axes3D extends Group {

  /**
   * Create a JavaFX Group representing a 3D Cartesian reference frame.
   *
   * @param  length  Length of each axis in pixels
   * @param  radius  Radius of axis in pixels (text and end marker make
   *                 use of more space)
   * @param  nTicks  The number of tick marks on each axis.  Currently,
   *                 tick marks are labeled 1 through nTick, so they will
   *                 have to represent some form of normalized values if 
   *                 an integer scale isn't convenient.  If zero, no tick
   *                 marks are added.
   */
  public Axes3D(double length, double radius, int nTicks) {
    super(createAxes(length, radius, nTicks));
  }

  /**
   * See constructor interface
   */
  private static Group createAxes(double length, double radius, int nTicks) {
    double delta = length/2.0;
      // Note that these are vector rotations - so perform the
      // opposite of a reference frame transformation           
    Group xAxis = new Axis3D(length, radius, "X", nTicks);
    Matrix3X3 rot = new Matrix3X3(Basis3D.K, Angles.PIO2);
    Vector3D trans = new Vector3D();
    trans.set(Basis3D.I, delta);
    Affine xTrans = new CognAffine(rot, trans);
    xAxis.getTransforms().add(xTrans);
      //
    Group yAxis = new Axis3D(length, radius, "Y", nTicks);
    rot.identity();
    trans.zero();
    trans.set(Basis3D.J, delta);
    Affine yTrans = new CognAffine(rot, trans);
    yAxis.getTransforms().add(yTrans);
      //
    Group zAxis = new Axis3D(length, radius, "Z", nTicks);
    rot.rotX(-Angles.PIO2);
    trans.zero();
    trans.set(Basis3D.K, delta);
    Affine zTrans = new CognAffine(rot, trans);
    zAxis.getTransforms().add(zTrans);      
      //
    Sphere origin = new Sphere(1.5*radius);
    origin.setDrawMode(DrawMode.FILL);

    return new Group(xAxis, yAxis, zAxis, origin);
  }
}
