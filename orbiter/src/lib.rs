use kiss3d::prelude::{Vec3, Quat}; 
//use kiss3d::nalgebra::{UnitQuaternion, Vector3};

pub fn orbiter_gx2i() -> Quat {
  Quat::from_axis_angle(Vec3::X, -0.5*std::f64::consts::PI as f32)
}

