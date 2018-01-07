/*
 */
package cognition;

import javafx.application.Application;
import javafx.stage.Stage;
import javafx.scene.Group;
import javafx.scene.Scene;
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

import cognition.math.Matrix3X3;
import cognition.math.Basis3D;


/**
 *
 * @author none
 */
public class Cognition extends Application {
  private final double sceneWidth = 800;
  private final double sceneHeight = 400;
  private PerspectiveCamera camera;
  
  @Override
  public void start(Stage primaryStage) {


    Group sceneRoot = new Group();
    Scene scene = new Scene(sceneRoot, sceneWidth, sceneHeight);
    scene.setFill(Color.BLACK);
    camera = new PerspectiveCamera(true);
    camera.setNearClip(0.1);
    camera.setFarClip(10.0*Math.max(sceneWidth, sceneHeight));
    camera.setTranslateZ(-2.0*Math.max(sceneWidth, sceneHeight));
    scene.setCamera(camera);

    final double axisLength = 0.95*Math.min(sceneWidth, sceneHeight);
    final double axisRadius = axisLength/100.0;
      // Note that rotations vector rotations - so perform the opposite of
      // a reference frame transformation
    Group xAxis = createAxis(axisLength, axisRadius, "X");
    Matrix3X3 rot = new Matrix3X3();
    rot.rotZ(Math.PI/2.0);
    Affine xTrans = new Affine(rot.get(Basis3D.I, Basis3D.I),
                               rot.get(Basis3D.I, Basis3D.J),
                               rot.get(Basis3D.I, Basis3D.K),
                               axisLength/2.0,
                               rot.get(Basis3D.J, Basis3D.I),
                               rot.get(Basis3D.J, Basis3D.J),
                               rot.get(Basis3D.J, Basis3D.K),
                               0.0,
                               rot.get(Basis3D.K, Basis3D.I),
                               rot.get(Basis3D.K, Basis3D.J),
                               rot.get(Basis3D.K, Basis3D.K),
                               0.0
    );
    xAxis.getTransforms().add(xTrans);
      //
    Group yAxis = createAxis(axisLength, axisRadius, "Y");
    rot.rotZ(Math.PI);
    Affine yTrans = new Affine(rot.get(Basis3D.I, Basis3D.I),
                               rot.get(Basis3D.I, Basis3D.J),
                               rot.get(Basis3D.I, Basis3D.K),
                               0.0,
                               rot.get(Basis3D.J, Basis3D.I),
                               rot.get(Basis3D.J, Basis3D.J),
                               rot.get(Basis3D.J, Basis3D.K),
                               -axisLength/2.0,
                               rot.get(Basis3D.K, Basis3D.I),
                               rot.get(Basis3D.K, Basis3D.J),
                               rot.get(Basis3D.K, Basis3D.K),
                               0.0
    );
    yAxis.getTransforms().add(yTrans);
      //
    Group zAxis = createAxis(axisLength, axisRadius, "Z");
    rot.rotX(Math.PI/2.0);
    Affine zTrans = new Affine(rot.get(Basis3D.I, Basis3D.I),
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
                               -axisLength/2.0
    );
    zAxis.getTransforms().add(zTrans);

    Sphere origin = new Sphere(2.0*axisRadius);
    origin.setDrawMode(DrawMode.FILL);


    Group coordGroup = new Group(xAxis, yAxis, zAxis, origin);


    sceneRoot.getChildren().add(coordGroup);

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
    return new Group(axisBar, axisEnd);
  }



  /**
   * @param args the command line arguments
   */
  public static void main(String[] args) {
    launch(args);
  }


  
}
