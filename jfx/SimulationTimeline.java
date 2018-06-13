/*
 c  SimulationTimeline.java
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

package cognition.jfx;

import javafx.util.Duration;
import javafx.animation.Timeline;
import javafx.animation.KeyFrame;

/**
 * Creates an INDEFINITE JavaFX TimeLine that (finish connecting sim to time
 * update)
 *
 * Only the forward progression of time is supported along with the ability to
 * pause the simulation.
 *
 * @author Kurt Motekew
 * @since  20180612
 */
public class SimulationTimeline {
  private final Timeline tl;
  private final double dtmills = 100.0;
  private long cycles = 0L;
  private boolean working = false;

  public SimulationTimeline() {
    tl = new Timeline(
      new KeyFrame(new Duration(dtmills), t-> {
        cycles++;
        if (working) {
          System.out.println("Still Working");
        } else {
          working = true;
          //sparkyPos.mult(1.01);
          //sparkyTransform.set(sparkyAtt, sparkyPos);
          System.out.println("Time:  " + cycles*dtmills/1000.0);
          working = false;
        }
      })
    );
    tl.setCycleCount(Timeline.INDEFINITE);
  }

  /**
   * Begin or resume the simulation.
   */
  public void play() {
    tl.play();
  }

  /**
   * Pause the progression of time.
   */
  public void pause() {
    tl.pause();
  }
}
