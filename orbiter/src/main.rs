/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::env;
use std::fs;
use std::process;

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

use orbiter::orbiter_6dof::Orbiter6Dof;

use cogs::utl_const::RAD_PER_DEG;
use cogs::phy_const;
use cogs::phy_const::DU;
use cogs::dyn_keplerian::KeplerianElement;
use cogs::dyn_keplerian::Keplerian;

use cogs::dyn_two_body_gravity::TwoBodyGravity;
use cogs::dyn_orbit_deq::OrbitDeq;


#[kiss3d::main]
async fn main() {
    // Constants computed at runtime
    let sec_per_tu: f64 = phy_const::sec_per_tu();
    let tu_per_sec = 1.0/sec_per_tu;
    // Earth rotation - cast to f32 for graphics
    let omega_earth: f64 = phy_const::we_rad_tu();

    let args: Vec<String> = env::args().collect();
    println!("{} Arguments", args.len());
    if args.len() != 2 {
        println!("Correct use is: {} {}", &args[0], "<config_file>"); 
        process::exit(0);
    }

    let config = fs::read_to_string(&args[1])
        .expect(&("Error reading ".to_owned() + &args[1]));

    let config: OrbiterConfig = toml::from_str(&config).unwrap();
    println!("Flying {} in dynamic = {} mode.",
        config.name,
        config.dynamic,
    );

    // Integration step size, sec to TU
    let dt = config.dt*tu_per_sec;
    // Minimum step size considered when closing gaps
    let dt_eps = dt/100.0;
    // Speedup factor of runtime vs. simulation time
    let tfactor: f64 = config.tfactor;

    let oelmn: [(KeplerianElement, f64); 6] =
        [(KeplerianElement::A, config.orbit.semimajor_axis),
         (KeplerianElement::E, config.orbit.eccentricity),
         (KeplerianElement::I, RAD_PER_DEG*config.orbit.inclination),
         (KeplerianElement::O, RAD_PER_DEG*config.orbit.raan),
         (KeplerianElement::W, RAD_PER_DEG*config.orbit.arg_perigee),
         (KeplerianElement::V, RAD_PER_DEG*config.orbit.true_anomaly)];
    let kep_oe = if config.dynamic {
        Keplerian::try_from_oe(&oelmn).expect("Bad orbital elements")
    } else {
        Keplerian::default()
    };

    if config.dynamic {
        println!("One Time Unit is {} seconds", sec_per_tu);
        println!("Earth angular velocity is {} rad/TU", phy_const::we_rad_tu());
        println!("Integration step size is {} sec and time factor is {}",
            config.dt, tfactor);
        println!("Orbit Definition\n{}", &kep_oe);
    }

    let twobdy = Box::new(TwoBodyGravity::new(1.0));
    let eom = OrbitDeq::new(twobdy);
    let mut orbit = Orbiter6Dof::new(eom, dt, 0.0, kep_oe.cartesian());
    /*
    let argp: f64 = if evec[2] < 0.0 {
        std::f64::consts::TAU - tmp
    } else {
        tmp
    };
    */

//    let ihat = na::Vector3::<f64>::x_axis();
//    let jhat = na::Vector3::<f64>::y_axis();
    let khat = na::Vector3::<f64>::z_axis();

    // GX related - define as f32
    const AXIS_LENGTH: f32 = 10.0;
    const NO_DNY_SF:   f32 = 0.025;

    let camera_offset = if config.dynamic {
        2.0*AXIS_LENGTH
    } else {
        NO_DNY_SF*2.0*AXIS_LENGTH
    };

    // Graphics window, etc.
    let mut gx_window = Some(Window::new("Orbiter").await);
    let mut gx_camera =
        OrbitCamera3d::new(Vec3::new(camera_offset, 0.0, camera_offset),
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

    // A dynamic simulation will show the earth with axes
    // The static simulation will create a small non-textured
    // sphere that will be hidden by the orbiter.  The dummy
    // earth is created in this case simply to avoid complicating
    // logic with the need to check if the sim is dynamic or not.
    let mut axes = if config.dynamic {
        add_axes(&mut gx_scene, AXIS_LENGTH)
    } else {
        add_axes(&mut gx_scene, NO_DNY_SF*AXIS_LENGTH)
    };
    axes.rotate(gx2inertial_rot());
    let mut earth = add_earth(&mut gx_scene, &config, DU as f32);
    let q_i2f = Quat::from_axis_angle(Vec3::Z, 0.0);
    update_earth(&mut earth,  &q_i2f);

    // The RBG spheres are references for the graphics environment
    // basis vectors (vs. the dynamics environment plotted with "arrows"
    gx_scene.add_sphere(0.1*DU as f32)
        .set_color(RED)
        .set_position(Vec3::new(AXIS_LENGTH, 0.0, 0.0));
    gx_scene.add_sphere(0.1*DU as f32)
        .set_color(GREEN)
        .set_position(Vec3::new(0.0, AXIS_LENGTH, 0.0));
    gx_scene.add_sphere(0.1*DU as f32)
        .set_color(BLUE)
        .set_position(Vec3::new(0.0, 0.0, AXIS_LENGTH));

    // Create spacecraft and set initial position based on mode
    let mut sparky = add_sparky(&mut gx_scene, &config);
    let r_s_o_i = if config.dynamic {
        kep_oe.position()
    } else {
        na::matrix![0.0 ; 0.0 ; 0.0]
    };
    let mut q_i2b = na::UnitQuaternion::<f64>::from_axis_angle(&khat, 0.0);
    update_sparky(&mut sparky, &r_s_o_i, &q_i2b);

    //
    // Simulation and render loop
    //

    // Per-frame loop
    let epoch = std::time::Instant::now();
    // Track simulation time although physics works in TU
    let mut runtime_seconds: f64 = 0.0;
    // Simulation time for equations of motion
    let mut sim_time = tfactor*tu_per_sec*runtime_seconds;
    // Time of last integration step
    let mut last_sim_step_time: f64 = sim_time;
    // Position and velocity state vector updated with each loop
    let mut pv: na::SMatrix<f64, 6, 1> = na::SMatrix::repeat(0.0);
    // Continue simulation while graphics window is open
    while gx_window.is_some() {
        if let Some(window) = &mut gx_window {
            if !window.render_3d(&mut gx_scene, &mut gx_camera).await {
                gx_window = None;
                continue;
            }

            // Update attitude for static model - don't update time
            // and skip all dynamics
            if !config.dynamic {
                q_i2b = dynamics_off_event_handler(
                    &mut window.events(), &mut sparky, &q_i2b);
                if let Some(window) = &mut txt_window {
                    if !window.render_3d(
                        &mut txt_scene, &mut txt_camera).await {
                        txt_window = None;
                        continue;
                    }
                    let txt = format!("Inertial to Body:  {}",
                        attitude_string(&q_i2b));
                    window.draw_text(
                        &txt, Vec2::new(0.0, 20.0), 20.0, &font, WHITE);
                }
                continue;
            }

            // Get time since epoch and convert to simulation time
            let now = std::time::Instant::now();
            runtime_seconds = now.duration_since(epoch).as_secs_f64();
            sim_time = tfactor*tu_per_sec*runtime_seconds;

            // Sim time does not require the EOM to catch up
            if dt > sim_time - last_sim_step_time {
                println!("Did not have to Integrate at {} seconds",
                    runtime_seconds);
                continue
            }

            // Update state through integration of EOM up to sim_time
            // Propagate by configured dt steps until time less than dt
            // remains.  Then close the gap
            let end_time = sim_time - dt;
            while last_sim_step_time < end_time {
                last_sim_step_time = orbit.propagate(dt, &mut pv);
            }
            let final_dt = sim_time - last_sim_step_time;
            if final_dt > dt_eps {
                last_sim_step_time = orbit.propagate(final_dt, &mut pv);
            }

            update_sparky(
                &mut sparky,
                &pv.fixed_view::<3, 1>(0, 0).into(),
                &q_i2b);

            let earth_rot = sim_time*omega_earth;
            let q_i2f = Quat::from_axis_angle(Vec3::Z, -1.0*earth_rot as f32);
            update_earth(&mut earth,  &q_i2f);

    
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
            let txt = format!("Elapsed SimulationTime (TU): {:>8.2}",
                sim_time);
            window.draw_text(&txt, Vec2::ZERO, 20.0, &font, WHITE);
            let txt = format!("Inertial to Body:  {}",
                              attitude_string(&q_i2b));
            window.draw_text(&txt, Vec2::new(0.0, 20.0), 20.0, &font, WHITE);
        }
    }
}

            //count += 1;
            //if count % 100 != 0 {
            //    continue;
            //}
