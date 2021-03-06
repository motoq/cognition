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
import javafx.scene.Scene;
import javafx.scene.control.Label;
import javafx.scene.control.TextField;
import javafx.scene.control.Button;
import javafx.scene.control.ComboBox;
import javafx.scene.layout.HBox;
import javafx.scene.layout.VBox;
import javafx.scene.layout.BorderPane;
import javafx.scene.control.Tooltip;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.geometry.Insets;
import javafx.scene.layout.Priority;
import javafx.scene.layout.Region;
import javafx.geometry.Pos;

/**
 * Initial infrastructure to replace the Java3D based Vehicle Simulation
 * Environment with a JavaFX one.  The new package name will simply be
 * Cognition since the old VSE was used quite a bit to study and understand
 * various M&S related topics.
 *
 * @author Kurt Motekew
 */
public class Cognition extends Application {
  @Override
  public void start(Stage primaryStage) {
    //
    // Create BorderPane Main Entry Window.  The top portion contains
    // simulation entry and time controls.  The bottom portion allows
    // the user to select output windows as provided by the loaded
    // models.  The center will be replaced with a custom control area
    // specific to the loaded simulation.
    //

      // Simulation package name entry
    Label simLabel = new Label("Simulation Package:");
    final TextField simField = new TextField("cognition.eg.orbiter.Orbiter");
    simField.setPrefColumnCount(32);
    Tooltip tt = new Tooltip("Example Simulation - Enter class in CLASSPATH");
    Tooltip.install(simField, tt);
    final Button loadBtn = new Button("Load");
    HBox simEntry = new HBox(10., simLabel, simField, loadBtn);
    simEntry.setAlignment(Pos.CENTER);
      // Time control area for the simulation - initially disabled
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
    exitBtn.setOnAction(e -> Platform.exit());
    Region regionRM = new Region();
    HBox.setHgrow(regionRM, Priority.ALWAYS);
    HBox startStopArea = new HBox(10., timeArea, playBtn, pauseBtn,
                                       regionRM, exitBtn);
      // Container for package and time control
    VBox controlArea = new VBox(10., simEntry, startStopArea);
    controlArea.setStyle("-fx-padding: 10;" + 
                      "-fx-border-style: solid inside;" + 
                      "-fx-border-width: 2;" +
                      "-fx-border-insets: 5;" + 
                      "-fx-border-radius: 5;" + 
                      "-fx-border-color: blue;");
      // Output options
    Label newDataLabel = new Label("Data Window:");
    final ComboBox<String> rtDispCB = new ComboBox<>();
    final Button newDataBtn = new Button("New");
    newDataBtn.setDisable(true);
    rtDispCB.setOnAction(e -> { if (newDataBtn.isDisabled()) { 
                                  newDataBtn.setDisable(false);
    }});
    newDataBtn.setOnAction(e -> {
      String sval = rtDispCB.getValue();
      System.out.println(sval);
    });
    Label newTableLabel = new Label("Table Window:");
    final ComboBox<String> tableCB = new ComboBox<>();
    final Button newTableBtn = new Button("New");
    newTableBtn.setDisable(true);
    newTableBtn.setOnAction(e -> {
      String sval = tableCB.getValue();
      System.out.println(sval);
    });
    tableCB.setOnAction(e -> { if (newTableBtn.isDisabled()) {
                                 newTableBtn.setDisable(false);
    }});
    Region regionM = new Region();
    HBox.setHgrow(regionM, Priority.ALWAYS);
    HBox dataArea = new HBox(10., newDataLabel, rtDispCB, newDataBtn,
                            regionM, newTableLabel, tableCB, newTableBtn);
    dataArea.setAlignment(Pos.CENTER);
    dataArea.setStyle("-fx-padding: 10;" + 
                      "-fx-border-style: solid inside;" + 
                      "-fx-border-width: 2;" +
                      "-fx-border-insets: 5;" + 
                      "-fx-border-radius: 5;" + 
                      "-fx-border-color: blue;");
      // Initial splash image - to be replaced with loaded simulation
    Image splashImage = new Image("cognition/cognition.png");
    ImageView splashIV = new ImageView();
    splashIV.setImage(splashImage);
    splashIV.setFitWidth(640);
    splashIV.setPreserveRatio(true);
    splashIV.setSmooth(true);
    splashIV.setCache(true);
      // Main window
    final BorderPane simMain = new BorderPane();
    
    simMain.setTop(controlArea);
    simMain.setBottom(dataArea);
    simMain.setCenter(splashIV);

    final Scene simScene = new Scene(simMain);
    final Stage ps = primaryStage;
    primaryStage.setScene(simScene);
    primaryStage.setTitle("Cognition");
    primaryStage.show();

    //
    // Non-trivial callbacks
    //

      // Load Button attempts to create a class from the input package
      // name and if sucessfull, launches the new simulation
    loadBtn.setOnAction(e -> {
      try {
          // Try to load class - disable class inputs if sucessful
        Class<? extends ISimModel> cModel = 
               Class.forName(simField.getText()).asSubclass(ISimModel.class);
        ISimModel sModel = cModel.newInstance();
        loadBtn.setDisable(true);
        simField.setDisable(true);
          // Grab model inputs and launch
        simMain.setCenter(sModel.getRoot());
        rtDispCB.setItems(sModel.getRealtimeDataList());
        tableCB.setItems(sModel.getTableDataList());
        sModel.launch();
          // Activate main window controls and resize model area
        playBtn.setDisable(false);
        ps.sizeToScene();
        ps.setAlwaysOnTop(true);
        ps.setAlwaysOnTop(false);
      } catch(ClassNotFoundException cnfe) {
        System.out.println("Class not found: " + cnfe);
      } catch(InstantiationException ie) {
        System.out.println("Can't instantiate class: " + ie);
      } catch(IllegalAccessException iae) {
        System.out.println("Not allowed access to class: " + iae);
      }
    });
  }

  /**
   * @param args the command line arguments
   */
  public static void main(String[] args) {
    launch(args);
  }

}
