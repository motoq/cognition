/*
 c  SparkySpacecraftBuilder.java
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

package cognition.jfx.meshmodels;

import javafx.scene.Group;
import javafx.fxml.FXMLLoader;
import javafx.geometry.Bounds;
import javafx.scene.transform.Affine;
import javafx.scene.transform.Scale;
import javafx.scene.shape.Cylinder;

import cognition.math.Basis3D;
import cognition.math.Matrix3X3;
import cognition.math.Vector3D;
import cognition.math.Angles;   
import cognition.jfx.CognAffine;

import java.io.IOException;

public class SparkySpacecraftBuilder {
  private final Matrix3X3 attDCM = new Matrix3X3();
  private final String sparkyFilename = "sparky.fxml";
  private final Scale modelScale = new Scale(2.0, 2.0, 2.0);

  public SparkySpacecraftBuilder() {
    Matrix3X3 roll = new Matrix3X3();
    Matrix3X3 yaw = new Matrix3X3();
    roll.rotX(Angles.PIO2);
    yaw.rotZ(Angles.PIO2);
    attDCM.mult(roll, yaw);
    attDCM.transpose();
  }

  public Group instantiate() {
    FXMLLoader sparkyLoader = new FXMLLoader(
      getClass().getResource(sparkyFilename)
    );
    Group sparky;
    try {
      sparky = sparkyLoader.load();
      Affine att = new CognAffine(attDCM);
      sparky.getTransforms().addAll(att, modelScale);
    } catch (IOException | IllegalStateException ex) {
      System.err.println("Can't find " + sparkyFilename + ":  " + ex);
      sparky = createStickSparky(100.);
    }
    Bounds bnds = sparky.getLayoutBounds();
    double depth = bnds.getDepth();
    double width = bnds.getWidth();
    double height = bnds.getHeight();
    double maxDim = Math.max(Math.max(depth, width), height);
    System.out.println("MaxDim: " + maxDim);
    System.out.println("Depth: " + depth);
    System.out.println("Width: " + width);
    System.out.println("Height: " + height);
    return new Group(sparky);
  }

  private Group createStickSparky(double length) {
    Matrix3X3 rot = new Matrix3X3();
    Vector3D trans = new Vector3D();

    double radius = 0.1*length;
    Cylinder fuselage = new Cylinder(radius, length);
    rot.rotZ(-Angles.PIO2);
    trans.set(Basis3D.I, 0.25*length);
    Affine state = new CognAffine(rot, trans);
    fuselage.getTransforms().add(state);

    Cylinder horizStab = new Cylinder(radius, 0.5*length);

    Cylinder vertStab = new Cylinder(radius, 0.25*length);
    rot.rotX(Angles.PIO2);
    trans.zero();
    trans.set(Basis3D.K, 0.5*0.25*length);
    state = new CognAffine(rot, trans);   
    vertStab.getTransforms().add(state);

    return new Group(fuselage, horizStab, vertStab);
  }

}
