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
//! of some graphics related content.  Objects created are aligned with
//! the graphics reference frame.

use kiss3d::prelude::*;
use std::path::Path;
//use kiss3d::prelude::{Vec3, Quat}; 
//use kiss3d::nalgebra::{UnitQuaternion, Vector3};

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

/// Rotation from texture to earth fixed (body)
///
/// # Return
///
/// * Texture coordinates to earth fixed
///
pub fn earthtexture2fixed_rot() -> Quat {
    let rot1 = Quat::from_axis_angle(Vec3::Z, -std::f64::consts::PI as f32);
    let rot2 = Quat::from_axis_angle(Vec3::X, 0.5*std::f64::consts::PI as f32);
    rot2*rot1
}

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
/// * scene  Scene graph to update
/// * er     Earth radius to use in graphics environment
///
/// # Return
///
/// * Sphere representing the earth object rotated to align with the
///   graphics coordinate sysytem such that the GX z-axis is through
///   the north pole and the GX x-axis is through (0, 0) latitude and
///   longitude.
///
pub fn add_earth(scene: &mut SceneNode3d, er: f32) -> SceneNode3d {
    let earth = scene
        .add_sphere(er)
        .set_texture_from_file(Path::new("./media/earth_lights_exp.jpg"),
                               "earth_texture");
    earth
}

/// q_i2f  Inertial to Fixed reference frame transformation
pub fn update_earth(earth_node: &mut SceneNode3d, q_i2f: &Quat) {
    earth_node.set_rotation(gx2inertial_rot()*
                            q_i2f.conjugate()*
                            earthtexture2fixed_rot());
}

/// Creates the object representing Sparky the spacecraft
///
/// # Argument
///
/// * scene  Scene graph to update
///
/// # Return
///
/// * Sparky object with body x-axis out the nose and z-axis up.
///   Aligned with grapchics reference frame
///
pub fn add_sparky(scene: &mut SceneNode3d) -> SceneNode3d {
    let sparky_obj_path = Path::new("./media/sparkymatmesh.obj");
    let sparky_mtl_path = Path::new("./media");
    let sparky = scene
        .add_obj(sparky_obj_path, sparky_mtl_path,
                 Vec3::new(0.005, 0.005, 0.005))
        .set_position(Vec3::new(1.0, 1.0, 1.0));

    sparky
}

pub fn update_sparky(sparky_node: &mut SceneNode3d, q_i2b: &Quat) {
    sparky_node.set_rotation(gx2inertial_rot()*
                             q_i2b.conjugate()*
                             sparkymodel2body_rot());
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
pub fn add_axis(scene: &mut SceneNode3d, length: f32,
                                         color: Color) -> SceneNode3d {
    let mut grp = scene.add_group();
    let cone_length = 0.05*length;
    let cone_width = 0.25*cone_length;
    let width = 0.25*cone_width;
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
    _ = add_axis(&mut grp,
                 length, Color::new(0.0, 1.0, 0.0, 1.0));
    let mut axis = add_axis(&mut grp,
                            length, Color::new(1.0, 0.0, 0.0, 1.0));
    let rot = Quat::from_axis_angle(Vec3::Z,
                                    -0.5*std::f64::consts::PI as f32);
    axis.rotate(rot);
    let mut axis = add_axis(&mut grp,
                            length, Color::new(0.0, 0.0, 1.0, 1.0));
    let rot = Quat::from_axis_angle(Vec3::X, 0.5*std::f64::consts::PI as f32);
    axis.rotate(rot);

    grp
}


