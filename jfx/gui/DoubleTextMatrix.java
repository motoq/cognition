/*
 c  IErrorReportable.java
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

package cognition.jfx.gui;

import javafx.scene.layout.GridPane;
import javafx.scene.layout.Pane;

import cognition.util.IErrorReportable;

public class DoubleTextMatrix extends Pane implements IErrorReportable {

  public DoubleTextMatrix(int rows, int columns, DoubleTextField dtf) {
    GridPane gp = new GridPane();
    for (int ii=0; ii<(columns); ii++) {
      for (int jj=0; jj<(rows); jj++) {
        DoubleTextField aij = new DoubleTextField(dtf);
        gp.add(aij, ii, jj);
      }
    }
    gp.setStyle("-fx-background-color: black;" +
                "-fx-padding: 2; -fx-hgap: 2; -fx-vgap: 2;");
    gp.setSnapToPixel(false);
    setGP(gp);
  }
  
  private void setGP(GridPane gp) {
    getChildren().setAll(gp);
  }
  
  @Override
  final public boolean isValid() {
    return true;
  }

  @Override
  public String  getErrorLabel() {
    return "OK";
  }
}
