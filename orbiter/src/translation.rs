/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Handles bookeeping of translational motion performance characteristics
//! for a platform with a single simple thruster.  Force modeling is based
//! on simple minimum and maximum limits.  Mass is not depleted with
//! thrusting.

/// Mass and thrust parameters needed to generate acceleration.  Acceleration
/// based on the current thrust level or a desired thrust level (limited to
/// max_thrust return values) can be requested.
///
/// # Author
///
/// Kurt Motekew  2026/10/14
///
pub struct Translation {
    /// kilograms
    mass: f64,
    /// Newtons
    max_thrust: f64,
    /// Amount by which to increment or decrement current thrust level, Newtons
    delta_thrust: f64,
    /// Current thrust tracked by this struct, Newtons
    thrust: f64,
}

impl Default for Translation {
    /// The default is a platform of 1 km in mass with no thrusting capability.
    fn default() -> Self {
        Self {
            mass: 1.0,
            max_thrust: 0.0,
            delta_thrust: 0.0,
            thrust: 0.0,
        }
    }
}

impl Translation {
    /// Initialize with mass and thrust properties
    ///
    /// # Arguments
    ///
    /// * mass          Mass, kg
    /// * max_thrust    Maximum allowable thrust level, N
    /// * delta_thrust  Thrust increment/decrement when stepping internal
    ///                 thrust setting
    /// * thrust        Initial thrust level
    ///
    pub fn new(
        mass: f64,
        max_thrust: f64,
        delta_thrust: f64,
        thrust: f64,
    ) -> Self {
        Self { mass, max_thrust, delta_thrust, thrust }
    }
}

/// Public mutable accessor methods
impl Translation {
    /// Increase thrust by one delta, up to maximum allowable
    ///
    pub fn increment_thrust(&mut self) {
        self.thrust =
            (self.thrust + self.delta_thrust).clamp(0.0, self.max_thrust);
    }

    ///  Decrease thrust by one delta, no lower than 0
    ///
    pub fn decrement_thrust(&mut self) {
        self.thrust =
            (self.thrust - self.delta_thrust).clamp(0.0, self.max_thrust);
    }

    /// Set thrust to zero
    pub fn zero_thrust(&mut self) {
        self.thrust = 0.0;
    }
}

/// Public immutable accessor methods
impl Translation {
    /// # Return
    ///
    /// * Acceleration based on mass and current thrust level, N
    ///
    pub fn stepper_accel(&mut self) -> f64 {
        self.thrust/self.mass
    }

    /// # Return
    ///
    /// * Acceleration based on mass and requested thrust level, N,
    ///   with thrust bound by the maximum thrust level.
    ///
    pub fn clamped_accel(&mut self, thrust: f64) -> f64 {
        thrust.clamp(0.0, self.max_thrust)/self.mass
    }
}
