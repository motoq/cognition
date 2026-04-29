/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use kiss3d::prelude::*;

use nalgebra as na;

use orbiter::OrbiterConfig;
use orbiter::gx2inertial_rot;
use orbiter::add_sparky;
use orbiter::add_axes;
use orbiter::add_earth;
use orbiter::update_earth;
use orbiter::update_sparky;
use orbiter::attitude_string;
use orbiter::dynamics_off_event_handler;


#[kiss3d::main]
async fn main() {

    let config = r#"
        name = "Sparky"
        dynamic = true 
        [others]
        x = 1.0
        y = 2.0
        z = 3.0
    "#;

    let config: OrbiterConfig = toml::from_str(config).unwrap();
    println!("name: {}\ndynamic: {}\nx: {}\ny: {}\nz: {}",
        config.name,
        config.dynamic,
        config.others.x,
        config.others.y,
        config.others.z,
    );


//    let ihat = na::Vector3::<f64>::x_axis();
//    let jhat = na::Vector3::<f64>::y_axis();
    let khat = na::Vector3::<f64>::z_axis();

    // GX related - define as f32
    const AXIS_LENGTH: f32 = 10.0;
    // Physics for this simulation - cast to f32 when needed for GX
    const DU: f64 = 1.0;            // Distance units
    const OMEGA_EARTH: f64 = 0.06;  // rad/TU
    const TU_PER_SEC: f64 = 1.0;    // Time units per real time

    // Graphics window, etc.
    let mut gx_window = Some(Window::new("Orbiter").await);
    let mut gx_camera =
        OrbitCamera3d::new(Vec3::new(2.0*AXIS_LENGTH, 0.0, 2.0*AXIS_LENGTH),
                           Vec3::new(0.0, 0.0, 0.0));
    let mut gx_scene = SceneNode3d::empty();
    gx_scene
        .add_light(Light::point(500.0))
        .set_position(Vec3::new(AXIS_LENGTH, AXIS_LENGTH, -AXIS_LENGTH));

    // Text window, etc.
    let mut txt_window = Some(Window::new("Orbiter Telemetry").await);
    let mut txt_camera = OrbitCamera3d::default();
    let mut txt_scene = SceneNode3d::empty();
    let font = Font::default();
    //let font = Font::new(std::path::Path::new("...")).unwrap();
    //let font =  std::sync::Arc::new(font);

    let mut axes = add_axes(&mut gx_scene, AXIS_LENGTH);
    axes.rotate(gx2inertial_rot());
    let mut earth = add_earth(&mut gx_scene, DU as f32);
    let q_i2f = Quat::from_axis_angle(Vec3::Z, 0.0);
    update_earth(&mut earth,  &q_i2f);

    gx_scene.add_sphere(0.1*DU as f32)
        .set_color(RED)
        .set_position(Vec3::new(AXIS_LENGTH, 0.0, 0.0));
    gx_scene.add_sphere(0.1*DU as f32)
        .set_color(GREEN)
        .set_position(Vec3::new(0.0, AXIS_LENGTH, 0.0));
    gx_scene.add_sphere(0.1*DU as f32)
        .set_color(BLUE)
        .set_position(Vec3::new(0.0, 0.0, AXIS_LENGTH));

    let mut sparky = add_sparky(&mut gx_scene);
    //let q_i2b = Quat::from_axis_angle(Vec3::Z, 0.0);
    let q_i2b = na::UnitQuaternion::<f64>::from_axis_angle(&khat, 0.0);
    update_sparky(&mut sparky, &q_i2b);


    //
    // Simulation and render loop
    //

    // Per-frame loop
    let epoch = std::time::Instant::now();
    let mut seconds: f64 = 0.0;
    let mut q_i2b_rot = na::UnitQuaternion::<f64>::from_axis_angle(&khat, 0.0);
    // Continue simulation while graphics window is open
    while gx_window.is_some() {
        if let Some(window) = &mut gx_window {
            if !window.render_3d(&mut gx_scene, &mut gx_camera).await {
                gx_window = None;
                continue;
            }
            let now = std::time::Instant::now();
            seconds = now.duration_since(epoch).as_secs_f64();
    
            let sim_time = TU_PER_SEC*seconds;
            let earth_rot = sim_time*OMEGA_EARTH;
            let q_i2f = Quat::from_axis_angle(Vec3::Z,
                                              -1.0*earth_rot as f32);
            update_earth(&mut earth,  &q_i2f);

            q_i2b_rot = dynamics_off_event_handler(&mut window.events(),
                                                   &mut sparky,
                                                   &q_i2b_rot);
    
            /*
            for event in window.events().iter() {
                match event.value {
                    WindowEvent::Key(button, Action::Press, _) => {
                        if button == Key::A {
                            q_i2b_rot = q_i2b_rot*
                                na::UnitQuaternion::<f64>::
                                    from_axis_angle(&khat, DANG);
                            let q_i2b = q_i2b_rot.conjugate();
                            update_sparky(&mut sparky, &q_i2b);
                        } else if button == Key::G {
                            q_i2b_rot = q_i2b_rot*
                                na::UnitQuaternion::<f64>::
                                    from_axis_angle(&khat, -DANG);
                            let q_i2b = q_i2b_rot.conjugate();
                            update_sparky(&mut sparky, &q_i2b);
                        } else if button == Key::E {
                            q_i2b_rot = q_i2b_rot*
                                na::UnitQuaternion::<f64>::
                                    from_axis_angle(&jhat, DANG);
                            let q_i2b = q_i2b_rot.conjugate();
                            update_sparky(&mut sparky, &q_i2b);
                        } else if button == Key::D {
                            q_i2b_rot = q_i2b_rot*
                                na::UnitQuaternion::<f64>::
                                    from_axis_angle(&jhat, -DANG);
                            let q_i2b = q_i2b_rot.conjugate();
                            update_sparky(&mut sparky, &q_i2b);
                        } else if button == Key::F {
                            q_i2b_rot = q_i2b_rot*
                                na::UnitQuaternion::<f64>::
                                    from_axis_angle(&ihat, DANG);
                            let q_i2b = q_i2b_rot.conjugate();
                            update_sparky(&mut sparky, &q_i2b);
                        } else if button == Key::S {
                            q_i2b_rot = q_i2b_rot*
                                na::UnitQuaternion::<f64>::
                                    from_axis_angle(&ihat, -DANG);
                            let q_i2b = q_i2b_rot.conjugate();
                            update_sparky(&mut sparky, &q_i2b);
                        }
                        //event.inhibited = true
                        // override default keyboard handler
                    }
                    _ => {}
                }
            }
            */
        }
        // If still active, update text window
        if let Some(window) = &mut txt_window {
            if !window.render_3d(&mut txt_scene, &mut txt_camera).await {
                txt_window = None;
                continue;
            }
            let txt = format!("Elapsed Time (TU): {:>8.2}", seconds*TU_PER_SEC);
            window.draw_text(&txt, Vec2::ZERO, 20.0, &font, WHITE);
            let txt = format!("Inertial to Body:  {}",
                              attitude_string(&q_i2b_rot));
            window.draw_text(&txt, Vec2::new(0.0, 20.0), 20.0, &font, WHITE);
        }
    }
}

            //count += 1;
            //if count % 100 != 0 {
            //    continue;
            //}
