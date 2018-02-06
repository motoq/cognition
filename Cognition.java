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
import javafx.stage.Stage;
import javafx.stage.Screen;
import javafx.scene.Group;
import javafx.scene.Scene;
import javafx.scene.input.MouseEvent;
import javafx.scene.input.KeyCode;
//import javafx.scene.control.Button;
//import javafx.scene.layout.StackPane;
//import javafx.event.ActionEvent;
//import javafx.event.EventHandler;
import javafx.scene.PerspectiveCamera;
import javafx.scene.transform.Affine;
import javafx.scene.paint.Color;
import javafx.scene.paint.PhongMaterial;
import javafx.scene.shape.DrawMode;
import javafx.scene.shape.Sphere;
import javafx.scene.shape.Cylinder;
import javafx.scene.text.Text;
import javafx.geometry.Rectangle2D;
import javafx.util.Duration;
import javafx.animation.Timeline;
import javafx.animation.KeyFrame;

import cognition.math.Basis3D;
import cognition.math.Quaternion;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;
import cognition.math.Angles;
import cognition.jfx.CognAffine;
import cognition.jfx.meshmodels.SparkySpacecraftBuilder;

/**
 * Currently just a test ground for 3D JavaFX
 *
 * @author Kurt Motekew
 */
public class Cognition extends Application {
  private PerspectiveCamera camera;
  private double sceneX = 0.0;
  private double sceneY = 0.0;

  private final Matrix3X3 cameraAtt = new Matrix3X3();
  private final Vector3D  cameraPos = new Vector3D();
  private final CognAffine cameraTransform = new CognAffine();

  private final Matrix3X3 sparkyAtt = new Matrix3X3();
  private final Vector3D  sparkyPos = new Vector3D();
  private final CognAffine sparkyTransform = new CognAffine();

  private Timeline simulationTimeline;
  private final double dtmills = 100.0;
  private long cycles = 0L;
  private boolean working = false;
  
  @Override
  public void start(Stage primaryStage) {
    final double fraction = 0.75;
    Rectangle2D screenBounds = Screen.getPrimary().getVisualBounds();
    final double sceneWidth = fraction*screenBounds.getWidth();
    final double sceneHeight = fraction*screenBounds.getHeight();
    final double maxDim = Math.max(sceneWidth, sceneHeight);
    final double minDim = Math.min(sceneWidth, sceneHeight);
    Group sceneRoot = new Group();
    Scene scene = new Scene(sceneRoot, sceneWidth, sceneHeight, true);
    scene.setFill(Color.BLACK);
    camera = new PerspectiveCamera(true);
    camera.setNearClip(0.1);
    camera.setFarClip(10.0*maxDim);
    cameraAtt.identity();
    cameraPos.set(Basis3D.K, -2.0*maxDim);
    cameraTransform.set(cameraAtt, cameraPos);
    camera.getTransforms().setAll(cameraTransform);

    scene.setCamera(camera);

      // Cartesian Axis
    final double axisLength = fraction*minDim;
    final double axisRadius = axisLength/200.0;
    Group coordGroup = createAxes(axisLength, axisRadius);

    //Group sparky = createStickSparky(axisLength/10);
    SparkySpacecraftBuilder spb = new SparkySpacecraftBuilder();
    Group sparky = spb.instantiate();
    Matrix3X3 yaw = new Matrix3X3();
    Matrix3X3 pitch = new Matrix3X3();
    yaw.rotZ(Math.toRadians(45));
    pitch.rotY(Math.toRadians(-45));
    sparkyAtt.mult(pitch,yaw);
      // Reference frame transformation to rotation
    sparkyAtt.transpose();
    sparkyPos.set(Basis3D.I, 0.1*minDim);
    sparkyPos.set(Basis3D.J, 0.1*minDim);
    sparkyPos.set(Basis3D.K, 0.05*minDim);
    sparkyTransform.set(sparkyAtt, sparkyPos);
    sparky.getTransforms().add(sparkyTransform);
    
    

    Group sceneGroup;
    sceneGroup = new Group(coordGroup, sparky);

      // Master Group orients everything with Z up
    sceneGroup.getTransforms().add(jFX2Comp());
    
    sceneRoot.getChildren().add(sceneGroup);

    scene.setOnKeyPressed(event -> {
      KeyCode key = event.getCode();
      Matrix3X3 drot = null;
      switch(key) {
        case P:
          simulationTimeline.playFromStart();
          break;
        case S:
        case F:
        case E:
        case D:
        case A:
        case G:
          drot = steer(key);
          break;
      }
      if (drot != null) {
        Matrix3X3 rot = new Matrix3X3();
        rot.mult(drot, sparkyAtt);
        sparkyAtt.set(rot);
        sparkyTransform.set(sparkyAtt, sparkyPos);
      }
    });

      // Reference point based on mouse click
    scene.setOnMousePressed(event -> {
      sceneX = event.getSceneX();
      sceneY = event.getSceneY();
    });
      // Track mouse drag from reference point for viewing adjustment
      // The shift button increases the rate relative to mouse movement
    scene.setOnMouseDragged((MouseEvent event) -> {
        // If the right mouse button is pressed, we are zooming
        // Otherwise, we are strafing the camera position
      if (event.isSecondaryButtonDown()) {
        double smod = 1.0;
        if (event.isShiftDown()) {
          smod = 3.0;
        }
          // Compute camera offset from origin adjustment as a
          // scale factor based on the drag distance and the
          // maximum scene dimension
        double newY = event.getSceneY();
        double scale = 1.0 + smod*(sceneY - newY)/maxDim;
        cameraPos.mult(scale);
        sceneY = newY;
        cameraTransform.set(cameraAtt, cameraPos);
      } else {
          // First compute X and Y axes rotations based on mouse
          // movement - Tenth of a degree rotation per pixel
        double smod = 0.1;
        if (event.isShiftDown()) {
          smod = 0.3;
        }
        double newY = event.getSceneY();
        double newX = event.getSceneX();
        double xAng = Math.toRadians(smod*(sceneY - newY));
        double yAng = Math.toRadians(smod*(sceneX - newX));
        sceneY = newY;
        sceneX = newX;
          // Next compute X and Y rotation transformations
        Matrix3X3 rx = new Matrix3X3();
        Matrix3X3 ry = new Matrix3X3();
        Matrix3X3 dr = new Matrix3X3();
        rx.rotX(-xAng);
        ry.rotY(yAng);
        dr.mult(ry, rx);
        Matrix3X3 ca = new Matrix3X3();
        ca.set(cameraAtt);
        cameraAtt.mult(dr, ca);
          // Finally compute location based on camera attitude
        Vector3D r_o_c_c = new Vector3D(0., 0., cameraPos.norm());
        Vector3D r_o_c_o = new Vector3D();
        r_o_c_o.mult(cameraAtt, r_o_c_c);
        Vector3D r_c_o_o = new Vector3D();
        r_c_o_o.set(r_o_c_o);
        r_c_o_o.mult(-1.0);
        cameraPos.set(r_c_o_o);
          // Update cameraTransform with new position and attitude
        cameraTransform.set(cameraAtt, cameraPos);
      }
    });

    simulationTimeline = new Timeline(
      new KeyFrame(new Duration(dtmills), t-> {
        cycles++;
        if (working) {
          System.out.println("Still Working");
        } else {
          working = true;
          sparkyPos.mult(1.01);
          sparkyTransform.set(sparkyAtt, sparkyPos);
          System.out.println("Time:  " + cycles*dtmills/1000.0);
          working = false;
        }
      })
    );
    simulationTimeline.setCycleCount(Timeline.INDEFINITE);
    
    primaryStage.setTitle("Axis");
    primaryStage.setScene(scene);
    primaryStage.show();

    Stage gxStage = new Stage();
    gxStage.setTitle("Blank Stage");
		gxStage.show();	

/*

    Button btn = new Button();
    btn.setText("Launch");
    btn.setOnAction(e -> System.out.println("Launch Application"));
    
    StackPane root = new StackPane();
    root.getChildren().add(btn);

*/
  }

  public Affine jFX2Comp() {
    Quaternion q = new Quaternion();
    Quaternion q1 = new Quaternion();
    Quaternion q2 = new Quaternion();
  
    q1.set(Angles.PIO2, Basis3D.I);
    q2.set(-Angles.PIO2, Basis3D.K);
    q.mult(q1, q2);    
    q.conj();
    Matrix3X3 rot = new Matrix3X3(q);
    
    return new CognAffine(rot);
  }

  public Group createAxes(double length, double radius) {
    double delta = length/2.0;
      // Note that rotations vector rotations - so perform the opposite of
      // a reference frame transformation
    Group xAxis = createAxis(length, radius, "X");
    Matrix3X3 rot = new Matrix3X3();
    rot.rotZ(Angles.PIO2);
    Vector3D trans = new Vector3D();
    trans.set(Basis3D.I, delta);
    Affine xTrans = new CognAffine(rot, trans);
    xAxis.getTransforms().add(xTrans);
      //
    Group yAxis = createAxis(length, radius, "Y");
    rot.identity();
    trans.zero();
    trans.set(Basis3D.J, delta);
    Affine yTrans = new CognAffine(rot, trans);
    yAxis.getTransforms().add(yTrans);
      //
    Group zAxis = createAxis(length, radius, "Z");
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
    Affine tTrans = new CognAffine(rot, trans);
    Text text = new Text(0.0, 0.0, axis + "-Axis");
    text.setFill(Color.WHITE);
    text.getTransforms().add(tTrans);


    return new Group(axisBar, axisEnd, text);
  }

  public Matrix3X3 steer(KeyCode key) {
    Matrix3X3 drot = new Matrix3X3();
    drot.identity();
    switch(key) {
      case S:
        drot.rotX(Math.toRadians(5.0));
        break;
      case F:
        drot.rotX(Math.toRadians(-5.0));
        break;
      case E:
        drot.rotY(Math.toRadians(-5.0));         // Nose down
        break;
      case D:
        drot.rotY(Math.toRadians(5.0));
        break;
      case A:
        drot.rotZ(Math.toRadians(-5.0));
        break;
      case G:
        drot.rotZ(Math.toRadians(5.0));
        break;
    }
    return drot;
  }

  /**
   * @param args the command line arguments
   */
  public static void main(String[] args) {
    launch(args);
  }
  
}
