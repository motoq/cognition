package cognition.eg.orbiter;

import javafx.scene.Node;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;

import cognition.ISimModel;

public class Orbiter implements ISimModel {

  @Override
  public void launch() {
    System.out.println("Launching Orbiter!");
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

    return projectIV;
    
  }
}
