/*
 c  Cognition.java
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

package cognition;

import javafx.application.Application;
import javafx.beans.property.DoubleProperty;
import javafx.beans.property.SimpleDoubleProperty;
import javafx.stage.Stage;
import javafx.scene.Group;
import javafx.scene.Scene;
//import javafx.scene.control.Button;
//import javafx.scene.layout.StackPane;
//import javafx.event.ActionEvent;
//import javafx.event.EventHandler;
import javafx.scene.PerspectiveCamera;
import javafx.scene.transform.Affine;
import javafx.scene.transform.Rotate;
import javafx.scene.paint.Color;
import javafx.scene.paint.PhongMaterial;
import javafx.scene.shape.DrawMode;
import javafx.scene.shape.Sphere;
import javafx.scene.shape.Cylinder;
import javafx.scene.text.Text;

import cognition.math.Basis3D;
import cognition.math.Quaternion;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;


/**
 * Currently just a test ground for 3D JavaFX
 *
 * @author Kurt Motekew
 */
public class Cognition extends Application {
  private final double sceneWidth = 800;
  private final double sceneHeight = 400;
  private PerspectiveCamera camera;
  private double sceneX = 0.0;
  private double sceneY = 0.0;
  private double fixedXAngle = 0.0;
  private double fixedYAngle = 0.0;
  private final DoubleProperty angleX = new SimpleDoubleProperty(0.0);
  private final DoubleProperty angleY = new SimpleDoubleProperty(0.0);
  
  @Override
  public void start(Stage primaryStage) {
    Group sceneRoot = new Group();
    Scene scene = new Scene(sceneRoot, sceneWidth, sceneHeight, true);
    scene.setFill(Color.BLACK);
    camera = new PerspectiveCamera(true);
    camera.setNearClip(0.1);
    camera.setFarClip(10.0*Math.max(sceneWidth, sceneHeight));
    camera.setTranslateZ(-2.0*Math.max(sceneWidth, sceneHeight));
    scene.setCamera(camera);

      // Cartesian Axis
    final double axisLength = 0.95*Math.min(sceneWidth, sceneHeight);
    final double axisRadius = axisLength/100.0;
    Group coordGroup = createAxes(axisLength, axisRadius);

      // Master Group orients everything with Z up
    Group sceneGroup = new Group(coordGroup);
    sceneGroup.getTransforms().add(jFX2Comp());
    
    
    sceneRoot.getChildren().add(sceneGroup);

      // Initial Mouse control will rotate the group vs. change
      // the camera perspective.  Note the use of bindings
    Rotate xRotate = new Rotate(0, Rotate.X_AXIS);
    Rotate yRotate = new Rotate(0, Rotate.Y_AXIS);
    coordGroup.getTransforms().addAll(xRotate, yRotate);
    xRotate.angleProperty().bind(angleX);
    yRotate.angleProperty().bind(angleY);
      // Reference point based on mouse click
    scene.setOnMousePressed(event -> {
      sceneX = event.getSceneX();
      sceneY = event.getSceneY();
      fixedXAngle = angleX.get();
      fixedYAngle = angleY.get();
    });
      // Track mouse draft from reference point for viewing adjustment
    scene.setOnMouseDragged(event -> {
      angleY.set(fixedXAngle - (sceneX - event.getSceneY()));
      angleX.set(fixedYAngle + sceneY - event.getSceneX());
    });
    
    
    
    
    
    primaryStage.setTitle("Axis");
    primaryStage.setScene(scene);
    primaryStage.show();




/*

    Button btn = new Button();
    btn.setText("Launch");
    btn.setOnAction(e -> System.out.println("Launch Application"));
    
    StackPane root = new StackPane();
    root.getChildren().add(btn);
    
    Scene scene = new Scene(root, 300, 250);
    
    primaryStage.setTitle("Cognition Launch Pad");
    primaryStage.setScene(scene);
    primaryStage.show();
*/
  }

  public Affine jFX2Comp() {
    Quaternion q = new Quaternion();
    Quaternion q1 = new Quaternion();
    Quaternion q2 = new Quaternion();
  
    q1.set(Math.PI/2.0, Basis3D.I);
    q2.set(-Math.PI/2.0, Basis3D.K);
    q.mult(q1, q2);    
    q.conj();
    Matrix3X3 rot = new Matrix3X3(q);
    
    return affineJFX(rot);
  }

  public Group createAxes(double length, double radius) {
    double delta = length/2.0;
      // Note that rotations vector rotations - so perform the opposite of
      // a reference frame transformation
    Group xAxis = createAxis(length, radius, "X");
    Matrix3X3 rot = new Matrix3X3();
    rot.rotZ(Math.PI/2.0);
    Vector3D trans = new Vector3D();
    trans.set(Basis3D.I, delta);
    Affine xTrans = affineJFX(rot, trans);
    xAxis.getTransforms().add(xTrans);
      //
    Group yAxis = createAxis(length, radius, "Y");
    rot.identity();
    trans.zero();
    trans.set(Basis3D.J, delta);
    Affine yTrans = affineJFX(rot, trans);
    yAxis.getTransforms().add(yTrans);
      //
    Group zAxis = createAxis(length, radius, "Z");
    rot.rotX(-Math.PI/2.0);
    trans.zero();
    trans.set(Basis3D.K, delta);
    Affine zTrans = affineJFX(rot, trans);
    zAxis.getTransforms().add(zTrans);
      //
    Sphere origin = new Sphere(1.5*radius);
    origin.setDrawMode(DrawMode.FILL);

    return new Group(xAxis, yAxis, zAxis, origin);
  }

  public Group createAxis(double length, double radius, String axis) {
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

    Matrix3X3 rot = new Matrix3X3();
    Matrix3X3 r1 = new Matrix3X3();
    Matrix3X3 r2 = new Matrix3X3();
    r1.rotY(Math.PI);
    r2.rotX(0);
    rot.mult(r2,r1);
    Vector3D trans = new Vector3D();
    //trans.set(Basis3D.I, radius);
    trans.set(Basis3D.J, length/2.0 + 4.0*radius);
    Affine tTrans = affineJFX(rot, trans);
    Text text = new Text(0.0, 0.0, axis + "-Axis");
    text.setFill(Color.WHITE);
    text.getTransforms().add(tTrans);


    return new Group(axisBar, axisEnd, text);
  }

  public Affine affineJFX(Matrix3X3 rot) {
    Affine at = new Affine(rot.get(Basis3D.I, Basis3D.I),
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
                           0.0
    );
    return at;
  }

  public Affine affineJFX(Matrix3X3 rot, Vector3D trans) {
    Affine at = new Affine(rot.get(Basis3D.I, Basis3D.I),
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
                           trans.get(Basis3D.K)
    );
    return at;
  }



  /**
   * @param args the command line arguments
   */
  public static void main(String[] args) {
    launch(args);
  }


  
}
