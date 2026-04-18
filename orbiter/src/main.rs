/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use kiss3d::prelude::*;

//use orbiter::gx2inertial;
use orbiter::add_sparky;
use orbiter::add_axes;
use orbiter::add_earth;

#[kiss3d::main]
async fn main() {
    let axis_length = 10.0;
    let du = 1.0;
    let earth_drot =
        Quat::from_axis_angle(Vec3::Y, (std::f64::consts::PI/360.0) as f32);

    let mut window = Window::new("Orbiter").await;
    //let mut camera = OrbitCamera3d::default();
    let mut camera =
        OrbitCamera3d::new(Vec3::new(2.0*axis_length, 0.0, 2.0*axis_length),
                           Vec3::new(0.0, 0.0, 0.0));

    let mut scene = SceneNode3d::empty();
    scene
        .add_light(Light::point(500.0))
        .set_position(Vec3::new(0.0, axis_length, -axis_length));


    add_axes(&mut scene, axis_length);
    let mut earth = add_earth(&mut scene, du);

    scene.add_sphere(0.1*du)
        .set_color(RED)
        .set_position(Vec3::new(axis_length, 0.0, 0.0));
    scene.add_sphere(0.1*du)
        .set_color(GREEN)
        .set_position(Vec3::new(0.0, axis_length, 0.0));
    scene.add_sphere(0.1*du)
        .set_color(BLUE)
        .set_position(Vec3::new(0.0, 0.0, axis_length));

    let mut sparky = add_sparky(&mut scene);


    // Per-frame loop
    while window.render_3d(&mut scene, &mut camera).await {
        earth.rotate(earth_drot);
        //sparky.rotate(earth_drot);
    }
}
