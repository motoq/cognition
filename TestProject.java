package cognition;

import javafx.scene.Node;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;

public class TestProject implements ISimModel {

  @Override
  public void launch() {
    System.out.println("Launching TestProject!");
  }

  @Override
  public Node getRoot() {

    Image projectImage = new Image("cognition/spheres2.png");
    ImageView projectIV = new ImageView();
    projectIV.setImage(projectImage);
    projectIV.setFitWidth(480);
    projectIV.setPreserveRatio(true);
    projectIV.setSmooth(true);
    projectIV.setCache(true);

    return projectIV;
    
  }
}
