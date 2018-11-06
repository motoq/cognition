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
import javafx.scene.input.KeyEvent;
import javafx.scene.input.KeyCode;
import javafx.geometry.Pos;
import javafx.event.EventHandler;

import cognition.util.IErrorReportable;

public class DoubleTextField extends TextField implements IErrorReportable {
  private double doubleValue;
  private double minValue = -Double.MAX_VALUE;
  private double maxValue =  Double.MAX_VALUE;
  private String errorText = "";
  private DecimalFormat df = new DecimalFormat("0.00000000E00");
  private boolean valid = true;
  private boolean newTextValue = false;

  public DoubleTextField() {
    this(0.0, -Double.MAX_VALUE, Double.MAX_VALUE);
  }
  
   public DoubleTextField(double doubleValue) {
     this(doubleValue, -Double.MAX_VALUE, Double.MAX_VALUE);
   }
   
   public DoubleTextField(DoubleTextField dtf) {
     this(dtf.doubleValue, dtf.minValue, dtf.maxValue);
     this.df = new DecimalFormat(dtf.df.toPattern(), dtf.df.getDecimalFormatSymbols());
     set();
   }

  public DoubleTextField(double doubleValue, double minValue, double maxValue) {
    this.doubleValue = doubleValue;
    this.minValue = minValue;
    this.maxValue = maxValue;
    setAlignment(Pos.CENTER_RIGHT);
    set();
    textProperty().addListener((o, old, nw) -> {
      newTextValue = true;
      System.out.println("text property");
    });
    
    setOnKeyPressed(new EventHandler<KeyEvent>() {
      @Override
      public void handle(KeyEvent keyEvent) {
        if (keyEvent.getCode() == KeyCode.ENTER)  {
          get();
          System.out.println("Enter");
        }
      }
    });
  }
  
  /** Tries to parse and then set the extracted double.  Does nothing
      if no changes have been made to the TextField */
  private void get() {
    if (newTextValue) {
      try {
        doubleValue = Double.parseDouble(getText());
        newTextValue = false;
        set();
      } catch(NumberFormatException e) {
        errorText = "Unable to convert " + getText() + " to a double: " + e;
        setStyle("-fx-background-color: red;");
        valid = false;
      }
    }
  }
  
  private void set() {
    if (doubleValue <= maxValue  &&  doubleValue >= minValue) {
      errorText = "";
      setText(df.format(doubleValue));
      setStyle("-fx-background-color: white;");
      valid = true;
    } else {
      errorText = "Entered value out of range";
      setStyle("-fx-background-color: red;");
      valid = false;
    } 
  }

  @Override
  final public boolean isValid() {
    get();
    return valid;
  }

  @Override
  public String  getErrorLabel() {
    get();
    return errorText;
  }

    /** @return  Value entered into text field */
  public double getDoubleValue() {
    get();
    return doubleValue;
  }
  
  public double getMinValue() { return minValue; }
  
  public double getMaxValue() { return maxValue; }
  
  public void setFormat(DecimalFormat df) {
    this.df = df;
    set();
  }
  
}