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

import java.text.DecimalFormat;

import javafx.scene.control.TextField;
import javafx.geometry.Pos;

import cognition.util.IErrorReportable;

public class DoubleTextField extends TextField implements IErrorReportable {
  private double doubleValue;
  private double minValue = -Double.MAX_VALUE;
  private double maxValue =  Double.MAX_VALUE;
  private String errorText = "";
  private DecimalFormat df = new DecimalFormat("0.00000000E00");

  public DoubleTextField() {
    this(0.0);
  }

  public DoubleTextField(double dval) {
    setAlignment(Pos.CENTER_RIGHT);
    setText(df.format(dval));
    setStyle("-fx-background-color: white;");
    isValid();
  }

  @Override
  final public boolean isValid() {
    try {
      doubleValue = Double.parseDouble(getText());
      if (doubleValue <= maxValue  &&  doubleValue >= minValue) {
        errorText = "";
        setText(df.format(doubleValue));
        setStyle("-fx-background-color: white;");
        return true;
      } else {
        errorText = "Entered value out of range";
        setStyle("-fx-background-color: red;");
        return false;
      }
    } catch(NumberFormatException e) {
      doubleValue = Double.NaN;
      errorText = "Unable to convert " + getText() + " to a double: " + e;
      setStyle("-fx-background-color: red;");
      return false;
    }
  }
  
  @Override
  public String  getErrorLabel() {
    isValid();
    return errorText;
  }
  
  public void set(double dval) {
    setText(df.format(dval));
    isValid();
  }

  /** @return  Value entered into text field */
  public double get() {
    isValid();
    return doubleValue;
  }

  public void setMinMaxValues(double min, double max) {
    minValue = min;
    maxValue = max;
  }

  public double getMinValue() { return minValue; }
  
  public double getMaxValue() { return maxValue; }
  
  public void setFormat(DecimalFormat df) {
    this.df = df;
    set(doubleValue);
  }
}