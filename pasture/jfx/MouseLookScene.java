/*
 c  MouseLookScene.java
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

package cognition. jfx;

import javafx.scene.Group;
import javafx.scene.Scene;
import javafx.scene.input.MouseEvent;
import javafx.scene.PerspectiveCamera;

import cognition.math.Basis3D;
import cognition.math.Vector3D;
import cognition.math.Matrix3X3;

/**
 * Creates a JavaFX Scene with mouse control that works by adjusting the
 * camera location and orientation instead of the underlying objects within
 * the Scene.  Zoom is provided by the right (secondary) mouse button.  Other
 * buttons strafe the camera about the origin of the scene.  Higher angular
 * and zoom rates are available through use of the Shift key.  This class
 * provides some of the basic functionality that Java3D's SimpleUniverse
 * class contained.
 *
 * @author Kurt Motekew
 * @since  20180426
 */
public class MouseLookScene extends Scene {
  private final PerspectiveCamera camera;
  private double sceneX = 0.0;
  private double sceneY = 0.0;

  private final Matrix3X3 cameraAtt = new Matrix3X3();
  private final Vector3D  cameraPos = new Vector3D();
  private final CognAffine cameraTransform = new CognAffine();

  public MouseLookScene(Group root, double sceneWidth, double sceneHeight) {
    super(root, sceneWidth, sceneHeight, true);
    final double maxDim = Math.max(sceneWidth, sceneHeight);
    final double minDim = Math.min(sceneWidth, sceneHeight);
    camera = new PerspectiveCamera(true);
    camera.setNearClip(0.1);
    camera.setFarClip(10.0*maxDim);
    cameraAtt.identity();
    cameraPos.set(Basis3D.K, -2.0*maxDim);
    cameraTransform.set(cameraAtt, cameraPos);
    camera.getTransforms().setAll(cameraTransform);

    setCamera(camera);

      // Reference point based on mouse click
    setOnMousePressed(event -> {
      sceneX = event.getSceneX();
      sceneY = event.getSceneY();
    });
      // Track mouse drag from reference point for viewing adjustment
      // The shift button increases the rate relative to mouse movement
    setOnMouseDragged((MouseEvent event) -> {
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
          // Flip x & y, reverse x-axis angle
        Vector3D dxy = new Vector3D(newY - sceneY, sceneX - newX,0.);
          // Reset reference point for the next click
        sceneY = newY;
        sceneX = newX;
          // Build transformation to convert 2D screen space movement to
          // 3D transformations - project 3D axes into 2D screen
        Matrix3X3 ca = new Matrix3X3(cameraAtt);
        Vector3D dxyz = new Vector3D(cameraAtt, dxy);
        Matrix3X3 rx = new Matrix3X3(Basis3D.I,
                                     Math.toRadians(smod*dxyz.get(Basis3D.I)));
        Matrix3X3 ry = new Matrix3X3(Basis3D.J,
                                     Math.toRadians(smod*dxyz.get(Basis3D.J)));
        Matrix3X3 rz = new Matrix3X3(Basis3D.K,
                                     Math.toRadians(smod*dxyz.get(Basis3D.K)));
        Matrix3X3 dr = new Matrix3X3(rx, ry, rz);
          // Update camera attitude
        cameraAtt.mult(dr, ca);
          // Update camera position based on location magnitude and attitude
          // Location of origin relative to camera position in camera coords
        Vector3D r_o_c_c = new Vector3D(0., 0., cameraPos.norm());
          // Transform to origin coordinates, etc.
        Vector3D r_o_c_o = new Vector3D(cameraAtt, r_o_c_c);
        Vector3D r_c_o_o = new Vector3D(r_o_c_o);
        r_c_o_o.mult(-1.0);
        cameraPos.set(r_c_o_o);
          // Update cameraTransform with new position and attitude
        cameraTransform.set(cameraAtt, cameraPos);
      }
    });
  }
}
