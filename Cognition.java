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
import javafx.application.Platform;
import javafx.stage.Stage;
import javafx.stage.Screen;
import javafx.scene.Group;
import javafx.scene.Scene;
import javafx.scene.control.Label;
import javafx.scene.control.TextField;
import javafx.scene.control.Button;
import javafx.scene.layout.HBox;
import javafx.scene.layout.VBox;
import javafx.scene.layout.BorderPane;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.geometry.Insets;
import javafx.scene.input.KeyCode;
//import javafx.scene.layout.StackPane;
//import javafx.event.ActionEvent;
//import javafx.event.EventHandler;
import javafx.scene.paint.Color;
import javafx.scene.layout.Priority;
import javafx.scene.layout.Region;
import javafx.geometry.Rectangle2D;
import javafx.geometry.Pos;

import cognition.math.Basis3D;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;
import cognition.jfx.SimulationTimeline;
import cognition.jfx.MouseLookScene;
import cognition.jfx.CognAffine;
import cognition.jfx.JFX2ComputationalFrame;
import cognition.jfx.Axes3D;
import cognition.jfx.meshmodels.SparkySpacecraftBuilder;

/**
 * Currently just a test ground for 3D JavaFX
 *
 * @author Kurt Motekew
 */
public class Cognition extends Application {
  private final Matrix3X3 sparkyAtt = new Matrix3X3();
  private final Vector3D  sparkyPos = new Vector3D();
  private final CognAffine sparkyTransform = new CognAffine();
  private boolean packageLoaded = false;

  @Override
  public void start(Stage primaryStage) {
    final double fraction = 0.75;
    Rectangle2D screenBounds = Screen.getPrimary().getVisualBounds();
    final double sceneWidth = fraction*screenBounds.getWidth();
    final double sceneHeight = fraction*screenBounds.getHeight();
    final double maxDim = Math.max(sceneWidth, sceneHeight);
    final double minDim = Math.min(sceneWidth, sceneHeight);
    Group sceneRoot = new Group();
    Scene scene = new MouseLookScene(sceneRoot, sceneWidth, sceneHeight);
    
    scene.setFill(Color.BLACK);

      // Cartesian Axis
    final double axisLength = fraction*minDim;
    final double axisRadius = axisLength/200.0;
    Group coordGroup = new Axes3D(axisLength, axisRadius, 10);

    //Group sparky = createStickSparky(axisLength/10);
    SparkySpacecraftBuilder spb = new SparkySpacecraftBuilder();
    Group sparky = spb.instantiate();
    Matrix3X3 yaw = new Matrix3X3(Basis3D.K, Math.toRadians(45.0));
    Matrix3X3 pitch = new Matrix3X3(Basis3D.J, Math.toRadians(-45));
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
    sceneGroup.getTransforms().add(new JFX2ComputationalFrame());

    sceneRoot.getChildren().add(sceneGroup);

    SimulationTimeline stl = new SimulationTimeline();
    
    scene.setOnKeyPressed(event -> {
      KeyCode key = event.getCode();
      Matrix3X3 drot = null;
      switch(key) {
        case P:
          stl.pause();
          break;
        case R:
          stl.play();
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

    Label simLabel = new Label("Simulation Package:");
    final TextField simField = new TextField();
    simField.setPrefColumnCount(32);
    final Button loadBtn = new Button("Load");
    HBox simEntry = new HBox(10., simLabel, simField, loadBtn);
    simEntry.setAlignment(Pos.CENTER);
    
    Label timeLabel = new Label("Time (TU):");
    TextField timeField = new TextField();
    timeField.setDisable(true);
    HBox timeArea = new HBox(10., timeLabel, timeField);
    timeArea.setAlignment(Pos.CENTER);
    HBox.setMargin(timeLabel, new Insets(0., 0., 0., 5.));
    final Button playBtn = new Button(">");
    playBtn.setDisable(true);
    final Button pauseBtn = new Button("||");
    pauseBtn.setDisable(true);
    Button exitBtn = new Button("Exit");
    Region regionRM = new Region();
    HBox.setHgrow(regionRM, Priority.ALWAYS);
    HBox startStopArea = new HBox(10., timeArea, playBtn, pauseBtn,
                                       regionRM, exitBtn);
    
    VBox controlArea = new VBox(10., simEntry, startStopArea);
    controlArea.setStyle("-fx-padding: 10;" + 
                      "-fx-border-style: solid inside;" + 
                      "-fx-border-width: 2;" +
                      "-fx-border-insets: 5;" + 
                      "-fx-border-radius: 5;" + 
                      "-fx-border-color: blue;");
    
    Label newDataLabel = new Label("Data Window:");
    Label tmpDataLabel = new Label("Combobox");
    final Button newDataBtn = new Button("New");
    newDataBtn.setDisable(true);
    Label newTableLabel = new Label("Table Window:");
    Label tmpTableLabel = new Label("Combobox");
    final Button newTableBtn = new Button("New");
    newTableBtn.setDisable(true);
    Region regionM = new Region();
    HBox.setHgrow(regionM, Priority.ALWAYS);
    HBox dataArea = new HBox(10., newDataLabel, tmpDataLabel, newDataBtn,
                            regionM, newTableLabel, tmpTableLabel, newTableBtn);
    dataArea.setAlignment(Pos.CENTER);
    //HBox.setMargin(newDataLabel, new Insets(0., 0., 0., 5.));
    //HBox.setMargin(tmpDataLabel, new Insets(0., 0., 0., 5.));
    //HBox.setMargin(newDataButton, new Insets(0., 0., 0., 5.));
    dataArea.setStyle("-fx-padding: 10;" + 
                      "-fx-border-style: solid inside;" + 
                      "-fx-border-width: 2;" +
                      "-fx-border-insets: 5;" + 
                      "-fx-border-radius: 5;" + 
                      "-fx-border-color: blue;");
    //controlArea.setCenter(startStopArea);
    //controlArea.setLeft(timeArea);
    //controlArea.setRight(exitBtn);
    //BorderPane.setMargin(exitBtn, new Insets(0., 5., 0., 0.));
    //BorderPane.setAlignment(startStopArea, Pos.CENTER);
    
    Image splashImage = new Image("cognition/cognition.png");
    ImageView splashIV = new ImageView();
    splashIV.setImage(splashImage);
    splashIV.setFitWidth(640);
    splashIV.setPreserveRatio(true);
    splashIV.setSmooth(true);
    splashIV.setCache(true);

    
    
    final BorderPane simMain = new BorderPane();
    simMain.setTop(controlArea);
    simMain.setBottom(dataArea);
    simMain.setCenter(splashIV);
    
    
    
    exitBtn.setOnAction(e -> Platform.exit());
    //VBox simMain = new VBox(5., simEntry, controlArea);
    final Scene simScene = new Scene(simMain);
    
    final Stage gxStage = new Stage();
    
    final Stage ps = primaryStage;
    loadBtn.setOnAction(e -> {
      if (!packageLoaded) {
        System.out.println(simField.getText());
        gxStage.show();
        packageLoaded = true;
        loadBtn.setDisable(true);
        simField.setDisable(true);
        playBtn.setDisable(false);
        pauseBtn.setDisable(false);
        newDataBtn.setDisable(false);
        newTableBtn.setDisable(false);
        ISimModel sim = new TestProject(); 
        simMain.setCenter(sim.getRoot());
        sim.launch();
        ps.sizeToScene();
      }
    });
    
    primaryStage.setScene(simScene);
    primaryStage.setTitle("Cognition");
    primaryStage.show();

    gxStage.setScene(scene);
    gxStage.setTitle("Axis");
			

/*
    Button btn = new Button();
    btn.setText("Launch");
    btn.setOnAction(e -> System.out.println("Launch Application"));
    
    StackPane root = new StackPane();
    root.getChildren().add(btn);
*/
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
