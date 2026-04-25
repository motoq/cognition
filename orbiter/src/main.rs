/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use kiss3d::prelude::*;

use orbiter::gx2inertial_rot;
use orbiter::add_sparky;
use orbiter::add_axes;
use orbiter::add_earth;
use orbiter::update_earth;
use orbiter::update_sparky;

#[kiss3d::main]
async fn main() {
    // GX related - define as f32
    const AXIS_LENGTH: f32 = 10.0;
    const DANG: f32 = (15.0*std::f64::consts::PI/180.0) as f32;
    // Physics for this simulation - cast to f32 when needed for GX
    const DU: f64 = 1.0;            // Distance units
    const OMEGA_EARTH: f64 = 0.06;  // rad/TU
    const TU_PER_SEC: f64 = 1.0;    // Time units per real time

      // Graphics window
    let mut gx_window = Window::new("Orbiter").await;
    let mut camera =
        OrbitCamera3d::new(Vec3::new(2.0*AXIS_LENGTH, 0.0, 2.0*AXIS_LENGTH),
                           Vec3::new(0.0, 0.0, 0.0));
    let mut scene = SceneNode3d::empty();
    scene
        .add_light(Light::point(500.0))
        .set_position(Vec3::new(AXIS_LENGTH, AXIS_LENGTH, -AXIS_LENGTH));


    let mut axes = add_axes(&mut scene, AXIS_LENGTH);
    axes.rotate(gx2inertial_rot());
    let mut earth = add_earth(&mut scene, DU as f32);
    let q_i2f = Quat::from_axis_angle(Vec3::Z, 0.0);
    update_earth(&mut earth,  &q_i2f);

    scene.add_sphere(0.1*DU as f32)
        .set_color(RED)
        .set_position(Vec3::new(AXIS_LENGTH, 0.0, 0.0));
    scene.add_sphere(0.1*DU as f32)
        .set_color(GREEN)
        .set_position(Vec3::new(0.0, AXIS_LENGTH, 0.0));
    scene.add_sphere(0.1*DU as f32)
        .set_color(BLUE)
        .set_position(Vec3::new(0.0, 0.0, AXIS_LENGTH));

    let mut sparky = add_sparky(&mut scene);
    let q_i2b = Quat::from_axis_angle(Vec3::Z, 0.0);
    update_sparky(&mut sparky, &q_i2b);

    // Per-frame loop
    let epoch = std::time::Instant::now();
    let mut count = 0;
    let mut q_i2b_rot = Quat::from_axis_angle(Vec3::Z, 0.0);
    while gx_window.render_3d(&mut scene, &mut camera).await {
        count += 1;
        let now = std::time::Instant::now();
        let seconds = now.duration_since(epoch).as_secs_f64();
        if count % 100 == 0 {
          println!("Elapsed Time: {} seconds", seconds);
        }

        let sim_time = TU_PER_SEC*seconds;
        let earth_rot = sim_time*OMEGA_EARTH;
        let q_i2f = Quat::from_axis_angle(Vec3::Z,
                                          -1.0*earth_rot as f32);
        update_earth(&mut earth,  &q_i2f);

        for event in gx_window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    if button == Key::A {
                        q_i2b_rot = q_i2b_rot*
                                    Quat::from_axis_angle(Vec3::Z, DANG);
                        let q_i2b = q_i2b_rot.conjugate();
                        update_sparky(&mut sparky, &q_i2b);
                    } else if button == Key::G {
                        q_i2b_rot = q_i2b_rot*
                                    Quat::from_axis_angle(Vec3::Z, -DANG);
                        let q_i2b = q_i2b_rot.conjugate();
                        update_sparky(&mut sparky, &q_i2b);
                    } else if button == Key::E {
                        q_i2b_rot = q_i2b_rot*
                                    Quat::from_axis_angle(Vec3::Y, DANG);
                        let q_i2b = q_i2b_rot.conjugate();
                        update_sparky(&mut sparky, &q_i2b);
                    } else if button == Key::D {
                        q_i2b_rot = q_i2b_rot*
                                    Quat::from_axis_angle(Vec3::Y, -DANG);
                        let q_i2b = q_i2b_rot.conjugate();
                        update_sparky(&mut sparky, &q_i2b);
                    } else if button == Key::F {
                        q_i2b_rot = q_i2b_rot*
                                    Quat::from_axis_angle(Vec3::X, DANG);
                        let q_i2b = q_i2b_rot.conjugate();
                        update_sparky(&mut sparky, &q_i2b);
                    } else if button == Key::S {
                        q_i2b_rot = q_i2b_rot*
                                    Quat::from_axis_angle(Vec3::X, -DANG);
                        let q_i2b = q_i2b_rot.conjugate();
                        update_sparky(&mut sparky, &q_i2b);
                    }
                    //event.inhibited = true
                    // override default keyboard handler
                }
                _ => {}
            }
        }
    }
}
