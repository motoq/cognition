/*
 c  Orbiter.java
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

package cognition.eg.orbiter;

import javafx.stage.Stage;
import javafx.scene.Group;
import javafx.scene.Scene;
import javafx.stage.Screen;
import javafx.scene.Node;
import javafx.scene.control.Label;
import javafx.scene.layout.Priority;
import javafx.scene.layout.Region;
import javafx.scene.layout.VBox;
import javafx.scene.layout.HBox;
import javafx.scene.control.Tab;
import javafx.scene.control.TabPane;
import javafx.scene.input.KeyCode;
import javafx.scene.paint.Color;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.geometry.Rectangle2D;
import javafx.geometry.Pos;
import javafx.collections.ObservableList;
import javafx.collections.FXCollections;

import cognition.math.Basis3D;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;
import cognition.jfx.CognAffine;
import cognition.ISimModel;
import cognition.jfx.MouseLookScene;
import cognition.jfx.Axes3D;
import cognition.jfx.JFX2ComputationalFrame;
import cognition.jfx.meshmodels.SparkySpacecraftBuilder;
import cognition.jfx.SimulationTimeline;
import cognition.jfx.gui.DoubleTextField;

public class Orbiter implements ISimModel {
  private final Matrix3X3 sparkyAtt = new Matrix3X3();
  private final Vector3D  sparkyPos = new Vector3D();
  private final CognAffine sparkyTransform = new CognAffine();


  @Override
  public void launch() {
    System.out.println("Launching Orbiter!");
      // Main scene based on fraction of available screen area
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
      // Spacecraft solid model and initial state
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
    
    Stage gxStage = new Stage();
    gxStage.setScene(scene);
    gxStage.setTitle("Axis");
    gxStage.show();

    //
    // Non-trivial callbacks
    //

      // Keyboard controls for orbiter
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
  }

  @Override
  public Node getRoot() {

    Image projectImage = new Image("cognition/eg/orbiter/spheres2.png");
    ImageView projectIV = new ImageView();
    projectIV.setImage(projectImage);
    projectIV.setFitWidth(480);
    projectIV.setPreserveRatio(true);
    projectIV.setSmooth(true);
    projectIV.setCache(true);
    
    // Primary modeling parameters setup area
    
      // Gravitational Parameter
    Label gmLabel = new Label("Gravitational Parameter (DU\u00B3/TU\u00B2):");
    DoubleTextField gmField = new DoubleTextField();
    Region leftRegion = new Region();
    HBox.setHgrow(leftRegion, Priority.ALWAYS);
    HBox gmEntry = new HBox(10., leftRegion, gmLabel, gmField);
    gmEntry.setAlignment(Pos.CENTER);
      // Gravitational Scaling Radius
    Label radiusLabel = new Label("Gravitational Scaling Radius (DU):");
    DoubleTextField radiusField = new DoubleTextField();
    leftRegion = new Region();
    HBox.setHgrow(leftRegion, Priority.ALWAYS);
    HBox radiusEntry = new HBox(10., leftRegion, radiusLabel, radiusField);
    radiusEntry.setAlignment(Pos.CENTER);
      // First column
    VBox grArea = new VBox(10., gmEntry, radiusEntry);
    
      // Central body rotation rate
    Label wLabel = new Label("Central Body Rotation Rate (rad/TU):");
    DoubleTextField wField = new DoubleTextField();
    leftRegion = new Region();
    HBox.setHgrow(leftRegion, Priority.ALWAYS);
    HBox wEntry = new HBox(10., leftRegion, wLabel, wField);
    wEntry.setAlignment(Pos.CENTER);
      // Solid model scale factor (so you can see it depending on units selected
    Label smsfLabel = new Label("Solid Model Scale Factor:");
    DoubleTextField smsfField = new DoubleTextField(1.0);
    leftRegion = new Region();
    HBox.setHgrow(leftRegion, Priority.ALWAYS);
    HBox smsfEntry = new HBox(10., leftRegion, smsfLabel, smsfField);
    smsfEntry.setAlignment(Pos.CENTER);
      // Second column
    VBox etcArea = new VBox(10., wEntry, smsfEntry);
    
    HBox primaryParamsArea = new HBox(10., grArea, etcArea);
    primaryParamsArea.setStyle("-fx-padding: 10;" + 
                      "-fx-border-style: solid inside;" + 
                      "-fx-border-width: 2;" +
                      "-fx-border-insets: 5;" + 
                      "-fx-border-radius: 5;" + 
                      "-fx-border-color: blue;");
    Tab setupTab = new Tab("Setup");
    setupTab.setClosable(false);
    setupTab.setContent(primaryParamsArea);
    
    
    Tab orbitTab = new Tab("Orbit");
    orbitTab.setContent(projectIV);
    orbitTab.setClosable(false);
    orbitTab.setDisable(true);
    
    TabPane tabPane = new TabPane();
		tabPane.getTabs().addAll(setupTab, orbitTab);

    return tabPane;
    
  }
  
  @Override
  public ObservableList<String> getRealtimeDataList() {
    ObservableList<String> rtds = FXCollections.observableArrayList("RTD 1",
                                                                    "RTD 2",
                                                                    "RTD 3"
    );

    return rtds;
  }
  
  @Override
  public ObservableList<String> getTableDataList() {
    ObservableList<String> rtds = FXCollections.observableArrayList("Table 1",
                                                                    "Table 2",
                                                                    "Table 3"
    );

    return rtds;
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
  
  
}
