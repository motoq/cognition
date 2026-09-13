/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Primarily graphics and graphical environment related utilities
//! including rotations from the computational frame to the graphics
//! environment, loadking of orbiter app specific objects, and creation
//! of some graphics related content.  External useers of objects should
//! use the add_*() functions to create objects, and the update_*()
//! functions to set their states.

//use kiss3d::prelude::{Vec3, Quat}; 
use kiss3d::prelude::*;
use nalgebra as na;

use serde::Deserialize;
use std::path::Path;

use cogs::phy_const;
use cogs::dyn_gravity::Gravity;
use cogs::dyn_two_body_gravity::TwoBodyGravity;
use cogs::dyn_j2_gravity::J2Gravity;
use cogs::mth_quaternion::Quaternion;


pub mod orbiter_3dof;
pub mod orbiter_6dof;

/// Configuration structs, in progress
#[derive(Deserialize)]
pub struct OrbiterConfig {
    pub name: String,
    pub dynamic: bool,
    /// Integration step size, seconds
    pub dt: f64,
    /// Simulation time vs. runtime mutliplication factor
    pub tfactor: f64,
    /// Text window update rate, seconds
    pub text_refresh: f64,
    /// Text description of gravity model to implement
    pub gravity_model: String,
    /// Orbital parameters
    pub orbit: OrbitDef,
    /// Reference point location
    pub ref_point_lat_lon_alt: [f64; 3],
}

#[derive(Deserialize)]
pub struct OrbitDef {
    /// DU
    pub semimajor_axis: f64,
    /// Nondimensional
    pub eccentricity: f64,
    /// deg
    pub inclination: f64,
    /// deg
    pub raan: f64,
    /// deg
    pub arg_perigee: f64,
    /// deg
    pub true_anomaly: f64,
}

/// Available gravity models
pub enum GravityModelType {
    TwoBody,
    J2,
}

/// Returns the appropriate gravity model type given an string
/// or an Err if an invalid string is supplied.
///
pub fn gravity_model_type(model: &str) -> Result<GravityModelType, String> {
    match model {
        "two_body" => Ok(GravityModelType::TwoBody),
        "j2" => Ok(GravityModelType::J2),
        &_ => Err(("Bad Gravity Model".to_owned() + model).to_string()),
    }
}

/// Returns the gravity model based on an enum type
///
/// # Argument
///
/// * model_type  Available gravity model type
///
/// # Return
///
/// * Corresponding Gravity model implementation
///
pub fn gravity_model(model_type: GravityModelType) -> Box::<dyn Gravity> {
    match model_type {
        GravityModelType::TwoBody => Box::new(TwoBodyGravity::new(1.0)),
        GravityModelType::J2 => Box::new(J2Gravity::new(1.0, phy_const::J2)),
    }
}

/// Inertial to (earth) fixed reference frame transformation
///
/// # Argument
///
/// * sim_time  Orbiter simulation time.
///
/// # Return
///
/// * ECI to ECF passive transformation (basis vector rotation)
///
pub fn i2f(sim_time: f64) -> Quaternion {
    Quaternion::from_angle_axis(
        sim_time*phy_const::we_rad_tu(),
        &na::Vector3::<f64>::z_axis(),
    )
}

/// Earth fixed to inertial reference frame transformation
///
/// # Argument
///
/// * sim_time  Orbiter simulation time.
///
/// # Return
///
/// * ECF to ECI passive transformation (basis vector rotation)
///
pub fn f2i(sim_time: f64) -> Quaternion {
    Quaternion::from_angle_axis(
        -sim_time*phy_const::we_rad_tu(),
        &na::Vector3::<f64>::z_axis(),
    )
}

// Axis and related scale factors
const LS1: f32 = 0.05;
const LS2: f32 = 0.25;
const LS3: f32 = 0.25;

/// Convert an nalgebra SMatrix<f64, 6, 1> to a Glam Vec3
///
/// # Arguments
///
/// * nv3  nalgebra 3x1 vector
///
/// # Return
///
/// * Glam 3x1 vector
///
pub fn v_na2glamt(nv3: &na::SMatrix<f64, 3, 1>) -> Vec3 {
    Vec3::new(nv3[0] as f32, nv3[1] as f32, nv3[2] as f32)
}

/// Convert a Glam Vec3 to an nalgebra SMatrix<f64, 6, 1>
///
/// # Arguments
///
/// * Glam 3x1 vector
///
/// # Return
///
/// * nv3  nalgebra 3x1 vector
///
pub fn v_glam2nat(gv3: &Vec3) -> na::SMatrix<f64, 3, 1> {
    na::matrix![gv3[0] as f64 ; gv3[1] as f64 ; gv3[2] as f64]
}

/// Convert an nalgebra UnitQuaternion to a Glam Quat
///
/// # Arguments
///
/// * nq  nalgebra quaternion
///
/// # Return
///
/// * Glam quaternion
///
pub fn q_na2glamt(nq: &na::UnitQuaternion<f64>) -> Quat {
    Quat::from_xyzw(
        nq.vector()[0] as f32,
        nq.vector()[1] as f32,
        nq.vector()[2] as f32,
        nq.scalar() as f32
    )
}

/// Convert a Cog Quaternion to a Glam Quat
///
/// # Arguments
///
/// * cq  nalgebra quaternion
///
/// # Return
///
/// * Glam quaternion
///
pub fn q_cog2glamt(nc: &Quaternion) -> Quat {
    // Take conjugate to account for different convention
    let qi = -nc.imaginary();
    Quat::from_xyzw(
        qi[0] as f32,
        qi[1] as f32,
        qi[2] as f32,
        nc.real() as f32
    )
}

/// Rotation from the graphics environment (y-axis up, z-axis out of the
/// paper) to the computational (inertial) reference frame (z-axis up).
///
/// # Return
///
/// * Graphics to computational rotation quaternion
///
pub fn gx2inertial_rot() -> Quat {
    Quat::from_axis_angle(Vec3::X, -0.5*std::f64::consts::PI as f32)
}

/// Rotation from texture to earth fixed (body), where the z-axis
/// is out the north pole and the x-axis Null Island.
///
/// # Return
///
/// * Earth image coordinates to earth fixed
///
pub fn earthtexture2fixed_rot() -> Quat {
    let rot1 = Quat::from_axis_angle(Vec3::Z, -std::f64::consts::PI as f32);
    let rot2 = Quat::from_axis_angle(Vec3::X, 0.5*std::f64::consts::PI as f32);
    rot2*rot1
}


/// Rotation from orbiter model to body
///
/// # Return
///
/// * Orbiter model coordinates to body
///
pub fn sparkymodel2body_rot() -> Quat {
    let rot1 = Quat::from_axis_angle(Vec3::Y, 0.5*std::f64::consts::PI as f32);
    let rot2 = Quat::from_axis_angle(Vec3::X, 0.5*std::f64::consts::PI as f32);
    rot2*rot1
}

/// Creates a sphere with an earth image texture and adds it to the scene,
/// returning the earth for further manipulation
///
/// # Arguments
///
/// * scene   Scene graph to update
/// * config  If dynamic, a sphere with a 1 er radius will be created
///           using an earth image texture.  Otherwise, just a small
///           sphere object will be created.
/// * er      Earth radius to use in graphics environment
///
/// # Return
///
/// * Sphere representing the earth object
///
pub fn add_earth(
    scene: &mut SceneNode3d,
    config: &OrbiterConfig,
    er: f32
) -> SceneNode3d {
    let earth = if config.dynamic {
        scene.add_sphere(er).set_texture_from_file(
            Path::new("./resources/earth_lights_exp.jpg"), "earth_texture")
    } else {
        scene.add_sphere(LS1*LS2*LS3*er)
    };
    earth
}

/// Updates the orientation of the earth as displayed in the graphics
/// environment given an inertial to earth fixed (body) reference frame
/// transformation.
///
/// # Return
///
/// * q_i2f  Inertial to Fixed reference frame transformation
///
pub fn update_earth(
    earth_node: &mut SceneNode3d,
    q_i2f: &Quaternion
) {
    // Convert q_i2f to a vector rotation
    earth_node.set_rotation(
        gx2inertial_rot()
        *q_cog2glamt(q_i2f).conjugate()
        *earthtexture2fixed_rot()
    );
}

/// Creates the orbiter object, sets its position, and defaults to
/// an identity inertial to body transformation.
///
/// # Argument
///
/// * scene   Scene graph to update
/// * config  Scenario configuration settings
/// * pos     Initial inertial position, DU
///
/// # Return
///
/// * Sparky orbiter
///
pub fn add_sparky(
    scene: &mut SceneNode3d,
    config: &OrbiterConfig,
    pos: &na::SMatrix<f64, 3, 1>,
) -> SceneNode3d {
    let sparky_obj_path = Path::new("./resources/sparkymatmesh.obj");
    let sparky_mtl_path = Path::new("./resources");
    let sparky = if config.dynamic {
        scene.add_obj(
            sparky_obj_path,
            sparky_mtl_path,
            Vec3::new(0.005, 0.005, 0.005)
        ).set_position(gx2inertial_rot()*v_na2glamt(&pos))
    } else {
        scene.add_obj(
            sparky_obj_path,
            sparky_mtl_path,
            Vec3::new(0.005, 0.005, 0.005)
        )
    };
    sparky
}

/// Updates the orientation of the orbiter object as displayed in the
/// graphics environment given an inertial to body reference frame 
/// transformation.
///
/// # Argument
///
/// * sparky_node  Node for which to update display state
/// * pos          Inertial position, DU
/// * q_i2b        Inertial to body reference frame transformation
///
pub fn update_sparky(
    sparky_node: &mut SceneNode3d,
    pos: &na::SMatrix<f64, 3, 1>,
    q_i2b: &na::UnitQuaternion<f64>,
) {
    sparky_node.set_position(gx2inertial_rot()*v_na2glamt(&pos));
    sparky_node.set_rotation(
        gx2inertial_rot()*q_na2glamt(q_i2b).conjugate()*sparkymodel2body_rot()
    );
}

/// Creates axes for a Cartesian coordinate system with RGB representing
/// XYX
///
/// # Arguments
///
/// * scene   Scene graph to update
/// * length  Length of the axis, not including ending arrows
/// * color   Axis color
///
/// # Return
///
/// * Axis aligned starting from the origin and extending along the
///   y-axis of the graphics reference frame.
///
pub fn add_axis(
    scene: &mut SceneNode3d,
    length: f32,
    color: Color,
) -> SceneNode3d {
    let mut grp = scene.add_group();
    let cone_length = LS1*length;
    let cone_width = LS2*cone_length;
    let width = LS3*cone_width;
    grp.add_cylinder(width, length)
        .set_color(color)
        .set_position(Vec3::new(0.0, length/2.0, 0.0));
    grp.add_cone(cone_width, cone_length)
        .set_color(color)
        .set_position(Vec3::new(0.0, length, 0.0));
    grp
}

/// Creates axes for a Cartesian coordinate system with RGB representing
/// XYX
///
/// # Arguments
///
/// * scene   Scene graph to update
/// * length  Length of each axis, not including ending arrows
///
/// # Return
///
/// * XYZ/RGB axes aligned with the graphics reference frame
///
pub fn add_axes(scene: &mut SceneNode3d, length: f32) -> SceneNode3d {
    let mut grp = scene.add_group();
    _ = add_axis(&mut grp, length, Color::new(0.0, 1.0, 0.0, 1.0));
    let mut axis = add_axis(&mut grp, length, Color::new(1.0, 0.0, 0.0, 1.0));
    let rot = Quat::from_axis_angle(Vec3::Z, -0.5*std::f64::consts::PI as f32);
    axis.rotate(rot);
    let mut axis = add_axis(&mut grp, length, Color::new(0.0, 0.0, 1.0, 1.0));
    let rot = Quat::from_axis_angle(Vec3::X, 0.5*std::f64::consts::PI as f32);
    axis.rotate(rot);
    grp
}

/// Creates a reference point fixed to the central body frame
///
/// # Arguments
///
/// * scene   Scene graph to update
/// * er      Earth radius to use in graphics environment
/// * pos     Initial inertial position, DU
///
/// # Return
///
/// * Reference point object
///
pub fn add_ref_point(
    scene: &mut SceneNode3d,
    er: f32,
    pos: &na::SMatrix<f64, 3, 1>,
) -> SceneNode3d {
    let rp = scene.add_sphere(LS1*er).set_texture_from_file(
        Path::new("./resources/foil_gold_256.jpg"), "rp_texture"
    ).set_position(gx2inertial_rot()*v_na2glamt(&pos));
    rp
}

/// Update reference point location
///
/// # Arguments
///
/// * ref_point_node  Node for which to update display state
/// * pos             Inertial position, DU
///
pub fn update_ref_point(
    ref_point_node: &mut SceneNode3d,
    pos: &na::SMatrix<f64, 3, 1>,
) {
    ref_point_node.set_position(gx2inertial_rot()*v_na2glamt(&pos));
}

/// Formats a quaternion for text output
///
/// # Arguments
///
/// * q_i2b  Quaternion to convert to a String
///
/// # Return
///
/// * String representation of quaternion in scalar + vector format
///
pub fn attitude_string(q_i2b: &na::UnitQuaternion<f64>) -> String {
    format!(
        "{:1.6} + [{:1.6} {:1.6} {:1.6}]",
        q_i2b.scalar(),
        q_i2b.imag().x,
        q_i2b.imag().y,
        q_i2b.imag().z
    )
}

/// Handles keyboard events for the case where are dynamics are turned off
/// and keyboard inputs manually orient the spacecraft w.r.t. inertial space.
/// Each keyboard input increments the attitude of the corresponding axis
/// by DANG defined below.
///
/// # Keyboard Inputs
///
/// * A/G  Yaw
/// * D/E  Pitch
/// * S/F  Roll
///
/// # Arguments
///
/// * events  EventManager from which events will be matched and consumed
/// * sparky  Node for which to update inertial to body rotation
/// * q_i2b   Current inertial to body reference frame transformation
///
/// # Return
///
/// * Updated inertial to body rotation
///
pub fn dynamics_off_event_handler(
    events: &mut EventManager,
    mut sparky: &mut SceneNode3d,
    q_i2b: &na::UnitQuaternion<f64>
) -> na::UnitQuaternion<f64> {

    let ihat = na::Vector3::<f64>::x_axis();
    let jhat = na::Vector3::<f64>::y_axis();
    let khat = na::Vector3::<f64>::z_axis();
    const DANG: f64 = 5.0*std::f64::consts::PI/180.0;

    let pos = v_glam2nat(&sparky.position());
    let mut q_i2b_rot = q_i2b.conjugate();

    for event in events.iter() {
        match event.value {
            WindowEvent::Key(button, Action::Press, _) => {
                if button == Key::A {
                    q_i2b_rot = q_i2b_rot
                        *na::UnitQuaternion::<f64>::from_axis_angle(
                            &khat, DANG
                        );
                    let q_i2b = q_i2b_rot.conjugate();
                    update_sparky(&mut sparky, &pos, &q_i2b);
                } else if button == Key::G {
                    q_i2b_rot = q_i2b_rot
                        *na::UnitQuaternion::<f64>::from_axis_angle(
                            &khat, -DANG
                        );
                    let q_i2b = q_i2b_rot.conjugate();
                    update_sparky(&mut sparky, &pos, &q_i2b);
                } else if button == Key::E {
                    q_i2b_rot = q_i2b_rot
                        *na::UnitQuaternion::<f64>::from_axis_angle(
                            &jhat, DANG
                        );
                    let q_i2b = q_i2b_rot.conjugate();
                    update_sparky(&mut sparky, &pos, &q_i2b);
                } else if button == Key::D {
                    q_i2b_rot = q_i2b_rot
                        *na::UnitQuaternion::<f64>::from_axis_angle(&jhat,
                            -DANG
                        );
                    let q_i2b = q_i2b_rot.conjugate();
                    update_sparky(&mut sparky, &pos, &q_i2b);
                } else if button == Key::F {
                    q_i2b_rot = q_i2b_rot
                        *na::UnitQuaternion::<f64>::from_axis_angle(
                            &ihat, DANG
                        );
                    let q_i2b = q_i2b_rot.conjugate();
                    update_sparky(&mut sparky, &pos, &q_i2b);
                } else if button == Key::S {
                    q_i2b_rot = q_i2b_rot
                        *na::UnitQuaternion::<f64>::from_axis_angle(
                            &ihat, -DANG
                        );
                    let q_i2b = q_i2b_rot.conjugate();
                    update_sparky(&mut sparky, &pos, &q_i2b);
                }
                //event.inhibited = true
                // override default keyboard handler
            }
            _ => {}
        }
    }
    // q_i2b_rot updated in event match - convert from vector to
    // basis rotation
    q_i2b_rot.conjugate()
}
