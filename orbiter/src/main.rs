use kiss3d::prelude::*;
use std::path::Path;

use orbiter::orbiter_gx2i;

#[kiss3d::main]
async fn main() {
    let axis_length = 10.0;
    let du = 1.0;

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

    let mut earth =scene.add_sphere(du)
        .set_color(WHITE)
        .set_texture_from_file(Path::new("./src/earth_lights_exp.jpg"),
                               "earth_texture");
    // Flip then align initial x-axis with Null Island
    let rot1 = Quat::from_axis_angle(Vec3::X, std::f64::consts::PI as f32);
    let rot2 = Quat::from_axis_angle(Vec3::Y, std::f64::consts::PI as f32);
    let rot = rot1*rot2;
    earth.rotate(rot);
    let earth_drot = Quat::from_axis_angle(Vec3::Y,
                                           (std::f64::consts::PI/360.0) as f32);

    scene.add_sphere(0.1*du)
        .set_color(RED)
        .set_position(Vec3::new(axis_length, 0.0, 0.0));
    scene.add_sphere(0.1*du)
        .set_color(GREEN)
        .set_position(Vec3::new(0.0, axis_length, 0.0));
    scene.add_sphere(0.1*du)
        .set_color(BLUE)
        .set_position(Vec3::new(0.0, 0.0, axis_length));

    let sparky_obj_path = Path::new("./src/sparkymatmesh.obj");
    let sparky_mtl_path = Path::new("./src");
    scene
        .add_obj(sparky_obj_path, sparky_mtl_path,
                 Vec3::new(0.005, 0.005, 0.005))
        .set_position(Vec3::new(1.0, 1.0, 1.0));
        


    // Per-frame loop
    while window.render_3d(&mut scene, &mut camera).await {
        earth.rotate(earth_drot);
    }
}

fn add_axis(scene: &mut SceneNode3d, length: f32, color: Color) -> SceneNode3d {
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

fn add_axes(scene: &mut SceneNode3d, length: f32) -> SceneNode3d {

    let mut grp = scene.add_group();

    _ = add_axis(&mut grp,
                 length, Color::new(0.0, 1.0, 0.0, 1.0));
    let mut axis = add_axis(&mut grp,
                            length, Color::new(1.0, 0.0, 0.0, 1.0));
    let rot = Quat::from_axis_angle(Vec3::Z, -0.5*std::f64::consts::PI as f32);
    axis.rotate(rot);
    let mut axis = add_axis(&mut grp,
                            length, Color::new(0.0, 0.0, 1.0, 1.0));
    let rot = Quat::from_axis_angle(Vec3::X, 0.5*std::f64::consts::PI as f32);
    axis.rotate(rot);

    grp.rotate(orbiter_gx2i());

    grp
}

