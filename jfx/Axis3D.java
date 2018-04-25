/*
 c  Axis3D.java
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

import java.util.Collection;
import java.util.ArrayList;

import javafx.scene.Node;
import javafx.scene.Group;
import javafx.scene.shape.Sphere;  
import javafx.scene.shape.Cylinder;
import javafx.scene.text.Text;
import javafx.scene.paint.Color;
import javafx.scene.paint.PhongMaterial;
import javafx.scene.shape.DrawMode;
import javafx.scene.transform.Affine;

import cognition.math.Basis3D;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;
import cognition.math.Angles;

/**
 * A JavaFX 3D axis that can be used to depict a Cartesian or other
 * reference frame.
 *
 * @author Kurt Motekew
 * @since  20180423
 */
public class Axis3D extends Group {

  /**
   * Create a JavaFX Group representing a 3D axis.
   *
   * @param  length  Length of each axis in pixels
   * @param  radius  Radius of axis in pixels (text and end marker make
   *                 use of more space)
   * @param  axis    Axis label.  'X' will be red, 'Y' green, and 'Z' blue.
   *                 Anything else will be blue.
   * @param  tics    The number of tick marks on each axis.  Currently,
   *                 tick marks are labeled 1 through nTick, so they will
   *                 have to represent some form of normalized values if
   *                 an integer scale isn't convenient.  If zero, no tick
   *                 marks are added.
   */
  public Axis3D(double length, double radius, String axis, int tics) {
    super(createAxis(length, radius, axis, tics));
  }

  /**
   * See constructor interface
   */
  private static Group createAxis(double length, double radius,
                                                 String axis, int tics) {
    Cylinder axisBar = new Cylinder(radius, length);
    Sphere axisEnd = new Sphere(2.0*radius);
    PhongMaterial mat = new PhongMaterial();
    switch (axis.toLowerCase()) {
      case "x":
        mat.setDiffuseColor(Color.RED);
        mat.setSpecularColor(Color.TOMATO);
        break;
      case "y":
        mat.setDiffuseColor(Color.FORESTGREEN);
        mat.setSpecularColor(Color.LIMEGREEN);
        break;
      case "z":
        mat.setDiffuseColor(Color.DEEPSKYBLUE);
        mat.setSpecularColor(Color.BLUE);
        break;
      default:
        mat.setDiffuseColor(Color.DEEPSKYBLUE);
        mat.setSpecularColor(Color.BLUE);
    }
    axisBar.setMaterial(mat);
    axisBar.setDrawMode(DrawMode.FILL);
    axisEnd.setMaterial(mat);
    axisEnd.setDrawMode(DrawMode.FILL);
    axisEnd.setTranslateY(length/2.0);

      // Rotations and translations for axis label.
    Matrix3X3 r1 = new Matrix3X3(Basis3D.J, Angles.PI);
    Matrix3X3 r2 = new Matrix3X3(Basis3D.I, Angles.PIO2);
    Matrix3X3 rot = new Matrix3X3(r2, r1);
    Vector3D trans = new Vector3D();
    trans.set(Basis3D.J, length/2.0 + 4.0*radius);
    trans.set(Basis3D.K, radius);
    Affine tTrans = new CognAffine(rot, trans);
    Text text = new Text(0.0, 0.0, axis + "-Axis");
    text.setFill(Color.WHITE);
    text.getTransforms().add(tTrans);

      // Add axis components and label
    Collection<Node> axisComponents = new ArrayList<>();
    axisComponents.add(axisBar);
    axisComponents.add(axisEnd);
    axisComponents.add(text);

      // Same rotation for tick marks, translate relative to '-' bar end
    double offset = -length/2.0;
    double dl = length/tics;
    for (int ii=1; ii<=tics; ii++) {
      text = new Text(0.0, 0.0, "" + ii);
      text.setFill(Color.WHITE);
      trans.set(Basis3D.J, offset + ii*dl);
      tTrans = new CognAffine(rot, trans);
      text.getTransforms().add(tTrans);
      axisComponents.add(text);
    }

    return new Group(axisComponents);
  }
}
