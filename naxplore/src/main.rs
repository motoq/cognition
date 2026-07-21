//! Nalgebra/Rust experiments before integration into cogs somewhere

use nalgebra as na;

pub trait Ode {
    fn xdot<const R: usize>(&self,
        t: f64, x: &na::SMatrix<f64, R, 1>) -> na::SMatrix<f64, R, 1>;
}

//
// Static trait member
//

pub struct Deq<T: Ode> {
    force: T, 
}

impl<T: Ode> Deq<T> {
    pub fn new(force: T) -> Self {
        Self {
            force,
        }
    }
}

impl<T: Ode> Ode for Deq<T> {
    fn xdot<const R: usize>(&self,
        t: f64, x: &na::SMatrix<f64, R, 1>) -> na::SMatrix<f64, R, 1> {

        self.force.xdot(t, &x)
    }
}



// Compile time vector size to function call
//fn vec_stuff<const R: usize, const C: usize>(matrix: &na::SMatrix<f64, R, C>)
fn vec_stuff<const R: usize>(fv: &na::SMatrix<f64, R, 1>) {
    println!("vec: {}", fv);
}

fn main() {
    println!("Hello, world!");

    // array, matrix, and looping syntax
    let arr: [i32; 6] = [11, 22, 33, 12, 13, 23];
    let mut mat: na::SMatrix<i32, 3, 3> = na::SMatrix::<i32, 3, 3>::zeros();
    for ii in 0..3 {
        mat[(ii, ii)] = arr[ii];
        for jj in (ii+1)..3 {
            mat[(ii, jj)] = arr[ii+jj+2];
            mat[(jj, ii)] = mat[(ii, jj)];
        }
    }
    println!("\nM {}", mat);

    // Compile time vector size to function call
    let mv = na::matrix![1.0 ; 2.0 ; 3.0];
    vec_stuff(&mv);

}
