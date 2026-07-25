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

//
// Sec1: General with specialization
//
// Could:
//
// use nalgebra::{RealField, SMatrix};
//

pub trait M2m<T, const R: usize, const C: usize>
where
    T: na::RealField,
{
    fn m2m(&self, yy: &na::SMatrix<T, R, C>) -> na::SMatrix<T, R, C>;
}

pub struct SpecializedM2m {
    xx: na::SMatrix<f64, 6, 1>,
}

impl SpecializedM2m {
    pub fn new(xx: na::SMatrix<f64, 6, 1>) -> Self {
        Self { xx }
    }
}

impl M2m<f64, 6, 1> for SpecializedM2m {
    fn m2m(&self, yy: &na::SMatrix<f64, 6, 1>) -> na::SMatrix<f64, 6, 1> {
        yy + self.xx
    }
}

//
// Sec2: General with specialization
//

pub struct GeneralizedM2m<T, const R: usize, const C: usize>
where
    T: na::RealField,
{
    xx: na::SMatrix<T, R, C>,
}

impl<T, const R: usize, const C: usize> GeneralizedM2m<T, R, C>
where
    T: na::RealField,
{
    pub fn new(xx: na::SMatrix<T, R, C>) -> Self {
        Self { xx }
    }
}

impl<T, const R: usize, const C: usize> M2m<T, R, C> for GeneralizedM2m<T, R, C>
where
    T: na::RealField,
{
    fn m2m(&self, yy: &na::SMatrix<T, R, C>) -> na::SMatrix<T, R, C> {
        //yy + &self.xx              // or
        yy + self.xx.clone()
    }
}



//
// Sec3: Compile time vector size function call
//
// Compile time vector size to function call
//

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

    //
    // Sec1: General with specialization
    //

    let x: na::SMatrix<f64, 6, 1> = na::SMatrix::repeat(1.0);
    let y: na::SMatrix<f64, 6, 1> = na::SMatrix::repeat(2.0);
    let sm2m = SpecializedM2m::new(x);
    let z = sm2m.m2m(&y);
    println!("x = {x} y = {y} z = {z}");


    //
    // Sec2: General with generalized
    //

    let x: na::SMatrix<f64, 6, 2> = na::SMatrix::repeat(1.0);
    let y: na::SMatrix<f64, 6, 2> = na::SMatrix::repeat(2.0);
    let gm2m = GeneralizedM2m::<f64, 6, 2>::new(x);
    let z = gm2m.m2m(&y);
    println!("x = {x} y = {y} z = {z}");


    //
    // Sec3: Compile time vector size function call
    //
    let mv = na::matrix![1.0 ; 2.0 ; 3.0];
    vec_stuff(&mv);

}
