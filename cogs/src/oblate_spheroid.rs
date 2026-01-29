/*
 * Copyright 2024, 2025 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use nalgebra as na;

use crate::utl_const::DEG_PER_RAD;
use crate::unit_circle;


/**
 * Oblate spheroid definition (eccentricity and semimajor axis length)
 * and coordinates (oblate spheroidal (os) and Cartesian) struct.  When
 * created, this struct defines a location in space using both Cartesian
 * and oblate spheroidal coordinates.  Properties related to this surface
 * are available at the location currently defined by the struct (e.g.,
 * basis vectors).  Some functionality (e.g., surface_tangent()) relies
 * on only the oblate spheroid definition vs. a specific location.  In
 * these cases, initialization with the default location at the origin of
 * the surface coordinates is sufficient.
 *
 * The oblate spheroid is a surface of revolution formed by rotating an
 * ellipse with a semiminor axis aligned with the Cartesian z-axis, rotated
 * about the z-axis.  For this implementation, the oblate spheroid itself
 * is defined by the eccentricity (ecc) and semimajor axis (sma).  This
 * is in contrast to traditional implementations that use the semiminor
 * axis as one of the os coordinates.  The longitude parameter (lon) is
 * identical to that defined for spherical or geodetic coordinates - it is
 * the angle measured in a right handed sense from the x-axis.  The latitude
 * (lat) is not an angle, but instead the +/- percentage along the z-axis,
 * with +1 being the north pole, and -1 the south pole.
 *
 * @author  Kurt Motekew  2024
 *                        2025/02/26  Added surface_tangent()
 *                        2026/01/25  Added metric tensors and volume
 *                                    element
 */
#[derive(Clone)]
pub struct OblateSpheroid {
    ecc: f64,                                    // 0   <= e   < 1
    sma: f64,                                    // 0   <  a   < inf
    lon: f64,                                    // -pi <  lon <= pi
    lat: f64,                                    // -1  <= lat <= 1
    xyz: na::SMatrix<f64, 3, 1>,
}


/*
 * Constructors
 */

impl Default for OblateSpheroid {
    /**
     * Default oblate spheroid definition and coordinates
     *
     * @return  Unit sphere with coordinate at (1, 0, 0)
     */
    fn default() -> Self {
        Self {
            ecc: 0.0,
            sma: 1.0,
            lon: 0.0,
            lat: 0.0,
            xyz: [1.0, 0.0, 0.0].into(),
        }
    }
}


impl TryFrom<&(f64, f64)> for OblateSpheroid {
    type Error = String;

    /**
     * Create OblateSpheroid and set coordinates to the origin on
     * the surface (zero longitude and latitude).
     *
     * @param  eccentricity  Eccentricity defining parameter, 0 <= eccen < 1
     * @param  semimajor     Semimajor axis defining parameter, smajor > 0
     *
     * @return  Ok:  OblateSpheroid
     *          Err: String
     */
    fn try_from(osp: &(f64, f64)) -> Result<Self, Self::Error> {
        let (eccentricity, semimajor) = osp;

        if *eccentricity < 0.0  ||  *eccentricity >= 1.0 {
            return Err("Invalid Eccentricity: ".to_string() +
                        &eccentricity.to_string());
        } else if *semimajor < 0.0 {
            return Err("Invalid Semimajor Axis: ".to_string() +
                       &semimajor.to_string());
        }

        let mut os = OblateSpheroid::default();
        os.set_with_ose(*eccentricity, *semimajor, 0.0, 0.0);
        Ok(os)
    }
}


impl TryFrom<&(f64, f64, f64, f64)> for OblateSpheroid {
    type Error = String;

    /**
     * Create OblateSpheroid and set coordinates with oblate spheroidal
     * coordinates.
     *
     * @param  eccentricity  Eccentricity defining parameter, 0 <= eccen < 1
     * @param  semimajor     Semimajor axis defining parameter, smajor > 0
     * @param  longitude     Longitude/Azimuth coordinate, -pi/2 < lambda < pi/2
     * @param  latitude      Latitude/elevation coordinate, -1 <= eta <= 1 
     *
     * @return  Ok:  OblateSpheroid
     *          Err: String
     */
    fn try_from(osp: &(f64, f64, f64, f64)) -> Result<Self, Self::Error> {
        let (eccentricity, semimajor, longitude, latitude) = osp;

        if *eccentricity < 0.0  ||  *eccentricity >= 1.0 {
            return Err("Invalid Eccentricity: ".to_string() +
                        &eccentricity.to_string());
        } else if *semimajor < 0.0 {
            return Err("Invalid Semimajor Axis: ".to_string() +
                       &semimajor.to_string());
        } else if *longitude <= -std::f64::consts::PI  ||
                  *longitude >   std::f64::consts::PI {
            return Err("Invalid Longitude: ".to_string() +
                       &(DEG_PER_RAD*longitude).to_string());
        } else if *latitude < -1.0  ||  *latitude >  1.0 {
            return Err("Invalid Latitude: ".to_string() +
                       &latitude.to_string());
        }

        let mut os = OblateSpheroid::default();
        os.set_with_ose(*eccentricity, *semimajor, *longitude, *latitude);
        Ok(os)
    }
}


impl TryFrom<&(f64, na::SMatrix<f64, 3, 1>)> for OblateSpheroid {
    type Error = String;

    /**
     * Create OblateSpheroid and set coordinates given Cartesian
     * coordinates
     *
     * @param  osp  Oblate spheroidal parameters
     *              .0  Eccentricity defining parameter, 0 <= eccen < 1
     *              .1  Cartesian coordinates
     *
     * @return  Ok:  OblateSpheroid
     *          Err: String
     */
    fn try_from(osp: &(f64,
                      na::SMatrix<f64, 3, 1>)) -> Result<Self, Self::Error> {

        let (eccentricity, cartesian) = osp;

        if *eccentricity < 0.0   ||  *eccentricity >= 1.0 {
            return Err("Invalid Eccentricity: ".to_string() +
                        &eccentricity.to_string());
        }
        let mut os = OblateSpheroid::default();
        os.set_with_cartesian(*eccentricity, cartesian);
        Ok(os)
    }
}


/*
 * Public immutable methods
 */

impl OblateSpheroid {
    /**
     * @return  Eccentricity
     */
    pub fn eccentricity(&self) -> f64 {
        self.ecc
    }

    /**
     * @return  Semimajor axis
     */
    pub fn semimajor(&self) -> f64 {
        self.sma
    }

    /**
     * @return  Semiminor axis
     */
    pub fn semiminor(&self) -> f64 {
        self.sma*(1.0 - self.ecc*self.ecc).sqrt()
    }

    /**
     * @return  Longitude, radians
     */
    pub fn longitude(&self) -> f64 {
        self.lon
    }

    /**
     * @return  Latitude
     */
    pub fn latitude(&self) -> f64 {
        self.lat
    }

    /**
     * @return  Cartesian coordinates
     */
    pub fn cartesian(&self) -> na::SMatrix<f64, 3, 1> {
        self.xyz
    }

    /**
     * Creates the paritals of the Cartesian coordinates w.r.t.
     * the oblate spheroidal coordinates;
     *
     * @return  Jacobian matrix
     */
    pub fn jacobian(&self) -> na::SMatrix<f64, 3, 3>
    {
        let a = self.sma;
        let eta = self.lat;
        let sqome2 = (1.0 - self.ecc*self.ecc).sqrt();
        let sqometa2 = (1.0 - eta*eta).sqrt();
        let cl = self.lon.cos();
        let sl = self.lon.sin();
        na::matrix![sqometa2*cl, -a*sqometa2*sl, -a*eta*cl/sqometa2 ;
                    sqometa2*sl,  a*sqometa2*cl, -a*eta*sl/sqometa2 ;
                    eta*sqome2,        0.0,       a*sqome2]
    }

    /**
     * @return  Covariant basis vectors at these coordinates
     */
    pub fn covariant_basis(&self) -> (na::SMatrix<f64, 3, 1>,
                                      na::SMatrix<f64, 3, 1>,
                                      na::SMatrix<f64, 3, 1>)
    {
        let a = self.sma;
        let eta = self.lat;
        let sqome2 = (1.0 - self.ecc*self.ecc).sqrt();
        let sqometa2 = (1.0 - eta*eta).sqrt();
        let cl = self.lon.cos();
        let sl = self.lon.sin();

        (na::matrix![sqometa2*cl ; sqometa2*sl ; eta*sqome2],
         na::matrix![-a*sqometa2*sl ; a*sqometa2*cl ; 0.0],
         na::matrix![-a*eta*cl/sqometa2 ; -a*eta*sl/sqometa2 ; a*sqome2])
    }

    /**
     * @return  Covariant metric tensor at these coordinates
     */
    pub fn covariant_metric(&self) -> na::SMatrix<f64, 3, 3>
    {
        let a2 = self.sma*self.sma;
        let e2 = self.ecc*self.ecc;
        let eta2 = self.lat*self.lat;
        let ometa2 = 1.0 - eta2;
        let naetae2 = -1.0*self.sma*self.lat*e2;

        na::matrix![1.0 - e2*eta2,       0.0,                     naetae2;
                              0.0, a2*ometa2,                         0.0;
                          naetae2,       0.0, a2*(1.0 - e2 + eta2/ometa2)  ]
    }

    /**
     * @return  Volume element at these coordinates
     */
    pub fn volume_element(&self) -> f64
    {
        self.sma*self.sma*(1.0 - self.ecc*self.ecc).sqrt()
    }

    /**
     * Creates the paritals of the oblate spheroidal coordinates w.r.t.
     * Cartesian coordinates;
     *
     * @return  Inverse Jacobian matrix
     */
    pub fn inverse_jacobian(&self) -> na::SMatrix<f64, 3, 3>
    {
        let a = self.sma;
        let ainv = 1.0/a;
        let ainv2 = ainv*ainv;
        let ainv3 = ainv*ainv2;
        let ome2 = 1.0 - self.ecc*self.ecc;
        let sqome2inv = 1.0/ome2.sqrt();
        let x = self.xyz[0];
        let x2 = x*x;
        let y = self.xyz[1];
        let y2 = y*y;
        let z = self.xyz[2];
        na::matrix![ainv*x, ainv*y, ainv*z/ome2 ;
                    -y/(x2 + y2),  1.0/(x*(1.0 + y2/x2)), 0.0 ;
                    -ainv3*x*z*sqome2inv, -ainv3*y*z*sqome2inv,
                     ainv*sqome2inv*(1.0 - z*z*ainv2/ome2)]
    }

    /**
     * @return  Contravariant basis vectors at these coordinates
     */
    pub fn contravariant_basis(&self) -> (na::SMatrix<f64, 3, 1>,
                                          na::SMatrix<f64, 3, 1>,
                                          na::SMatrix<f64, 3, 1>)
    {
        let a = self.sma;
        let eta = self.lat;
        let ometa2 = 1.0 - eta*eta;
        let sqome2 = (1.0 - self.ecc*self.ecc).sqrt();
        let sqometa2 = (ometa2).sqrt();
        let cl = self.lon.cos();
        let sl = self.lon.sin();

        (na::matrix![sqometa2*cl ; sqometa2*sl ; eta/sqome2],
         na::matrix![-sl/(a*sqometa2) ; cl/(a*sqometa2) ; 0.0],
         na::matrix![-eta*sqometa2*cl/a;-eta*sqometa2*sl/a;ometa2/(a*sqome2)])
    }

    /**
     * @return  Covariant metric tensor at these coordinates
     */
    pub fn contravariant_metric(&self) -> na::SMatrix<f64, 3, 3>
    {
        let e2 = self.ecc*self.ecc;
        let eta2 = self.lat*self.lat;
        let ome2 = 1.0 - e2;
        let ometa2 = 1.0 - eta2;

        let inv_a = 1.0/self.sma;
        let inv_a2 = inv_a*inv_a;
        let inv_ome2 = 1.0/ome2;
     
        let off_diag = self.lat*e2*ometa2*inv_ome2*inv_a;
        let z_11 = 1.0 + e2*eta2*inv_ome2;
        let z_33 = ometa2*(1.0 - e2*eta2)*inv_ome2*inv_a2;

        na::matrix![z_11,           0.0, off_diag;
                     0.0, inv_a2/ometa2,      0.0;
                off_diag,           0.0,     z_33 ]
    }

    /**
     * Given a cartesian location and pointing vector, return the
     * surface point tangent to the line from the position vector,
     * in the direction of the pointing vector and in the plane formed
     * by the position and pointing vectors.
     *
     * @param  pos  Cartesian position w.r.t. the origin of the oblate
     *              spheroid
     * @param  pnt  Cartesian pointing vector
     *
     * @return  Horizon point
     */
    pub fn surface_tangent(&self, pos: &na::SMatrix<f64, 3, 1>,
                                  pnt: &na::SMatrix<f64, 3, 1>) ->
                                        na::SMatrix<f64, 3, 1>
    {
        // Oblate spheroid to unit sphere affine transformation
        let mut aff: na::SMatrix<f64, 3, 3> = na::SMatrix::identity();
        aff[(0,0)] = 1.0/self.sma;
        aff[(1,1)] = 1.0/self.sma;
        aff[(2,2)] = 1.0/self.semiminor();

        // Oblate spheroid to spherical
        let pos_aff = aff*pos;
        let pnt_aff = aff*pnt;
        // 3D to 2D
        let yhat = pos_aff;
        let zhat = pos_aff.cross(&pnt_aff);
        let xhat = yhat.cross(&zhat);
        let xhat = na::Unit::new_normalize(xhat).into_inner();
        let yhat = na::Unit::new_normalize(yhat).into_inner();
        let zhat = na::Unit::new_normalize(zhat).into_inner();

        let to_3d = na::SMatrix::<f64, 3, 3>::from_columns(&[xhat, yhat, zhat]);
        let to_2d = to_3d.transpose();

        let pos_2d = to_2d*pos_aff;
        let pnt_2d = to_2d*pnt_aff;

        let r = pos_2d.fixed_view::<2,1>(0,0).into_owned();
        let p = pnt_2d.fixed_view::<2,1>(0,0).into_owned();

        let xy = unit_circle::tangent(&r, &p);

        let xyz = na::matrix![xy[0] ; xy[1] ; 0.0];

        aff[(0,0)] = self.sma;
        aff[(1,1)] = self.sma;
        aff[(2,2)] = self.semiminor();
        aff*to_3d*xyz
    }
}


/*
 * Private associated methods
 */

impl OblateSpheroid {
    /*
     * Update Cartesian coords with previously validated OS elements
     *
     * @param  eccen  Eccentricity defining parameter, 0 <= eccen < 1
     * @param  smaj   Semimajor axis defining parameter, smajor > 0
     * @param  lam    Longitude/Azimuth coordinate, -pi/2 < lambda < pi/2
     * @param  eta    Latitude/elevation coordinate, -1 <= eta <= 1 
     */
    fn set_with_ose(&mut self, eccen: f64, smaj: f64, lam: f64, eta: f64) {
        self.ecc = eccen;
        self.sma = smaj;
        self.lon = lam;
        self.lat = eta;

        let sqometa2 = (1.0 - eta*eta).sqrt();

        self.xyz[0] = smaj*sqometa2*lam.cos();
        self.xyz[1] = smaj*sqometa2*lam.sin();
        self.xyz[2] = smaj*eta*(1.0 - eccen*eccen).sqrt();
    }

    /*
     * Update OS coords with Cartesian
     *
     * @param  eccen  Eccentricity defining parameter, 0 <= eccen < 1
     * @param  cart   Cartesian coordinates
     */
    fn set_with_cartesian(&mut self, eccen: f64,
                          cart: &na::SMatrix<f64, 3, 1>) {
        self.ecc = eccen;
        self.xyz = *cart;

        let x2y2 = cart[0]*cart[0] + cart[1]*cart[1];
        let z2 = cart[2]*cart[2];
        let ome2 = 1.0 - eccen*eccen;

        self.sma = (x2y2 + z2/ome2).sqrt();
        self.lon = cart[1].atan2(cart[0]);
        self.lat = cart[2]/(self.sma*ome2.sqrt());
    }
}


/*
 * Utility
 */

impl std::fmt::Display for OblateSpheroid {
    /**
     * @return  Printable form of OblateSpheroid
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Eccentricity: {};  Semimajor: {}\
                   ;  Azimuth: {};  Elevation: {})",
            self.ecc, self.sma, DEG_PER_RAD*self.lon, self.lat)
    }
}


/*
 * Unit tests
 */

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Unit test checking the Cartesian portion of the default
     * oblate spheroid was manually set to the correct value.
     */
    #[test]
    fn default() {
        let os0 = OblateSpheroid::default();
        let os1 = crate::oblate_spheroid::OblateSpheroid::try_from(
            &(0.0, 1.0, 0.0, 0.0)).expect("Bad Oblate Spheroid ");
        //    ecc  sma  lon  lat

        assert!(os0.cartesian() == os1.cartesian());
    }

    /**
     * Unit test for the surface oblate_spheroid::surface_tangent function
     */
    #[test]
    fn surface_tangent() {
        let eps = 1.0e-13;
        // Define an oblate spheroid, a position, and pointing vector
        let ecc = 0.4;
        let smaj = 1.0;
        let mut os = crate::oblate_spheroid::OblateSpheroid::try_from(
            &(ecc, smaj)).expect("Bad Oblate Spheroid ");
        let pos = na::matrix![1.0 ; 1.0 ; 1.0];
        let pnt = na::matrix![-1.0 ; -1.0 ; 0.0];
        // Get tangent point.  Then, using the same eccentricity
        // update oblate spheroid point to pass through this point
        let tp = os.surface_tangent(&pos, &pnt);
        os.set_with_cartesian(ecc, &tp);
        // The vector from the tangent point to the position should
        // be a linear combination of the tangent plane basis vectors
        let p2t = tp - pos;
        let (_, e2, e3) = os.covariant_basis();
        let rank_2m = na::SMatrix::<f64, 3, 3>::from_columns(&[e2, e3, p2t]);
        // Check both determinant and rank just to illustrate use of both
        let det = rank_2m.determinant();
        let rank = rank_2m.rank(eps);
        assert!(det < eps  &&  rank == 2);
    }


    /**
     * Unit test checking the Jacobians are consistent (inverse of each other)
     */
    #[test]
    fn jacobian_inverse() {
        let eps = 1.0e-13;
        // Define an oblate spheroid
        let ecc = 0.4;
        let smaj = 1.0;
        let lon = 1.0;
        let lat = 0.5;
        let os = crate::oblate_spheroid::OblateSpheroid::try_from(
            &(ecc, smaj, lon, lat)).expect("Bad Oblate Spheroid ");
        let dcart_dos = os.jacobian();
        let dos_dcart = os.inverse_jacobian();
        let eye = dcart_dos*dos_dcart;
        let norm2 = (eye - na::SMatrix::<f64, 3, 3>::identity()).norm_squared();

        // Reminder that in matrix form, the covariant and contravariant
        // basis vectors are compatible as the transpose of each other
        //let (e1, e2, e3) = os.cov_basis();
        //let dcdo2 = na::Matrix3::from_columns(&[e1, e2, e3]);
        //let (e1, e2, e3) = os.cont_basis();
        //let dodc2 = na::Matrix3::from_columns(&[e1, e2, e3]);
        //println!("Z_ij Zij: {}", dcdo2*dodc2.transpose());

        assert!(norm2 < eps);
    }


    /**
     * Unit test checking the covariant and contravariant metric tensors are
     * are consistent (inverse of each other)
     */
    #[test]
    fn metric_inverse() {
        let eps = 1.0e-13;
        // Define an oblate spheroid
        let ecc = 0.4;
        let smaj = 1.0;
        let lon = 1.0;
        let lat = 0.5;
        let os = crate::oblate_spheroid::OblateSpheroid::try_from(
            &(ecc, smaj, lon, lat)).expect("Bad Oblate Spheroid ");
        let z_ij = os.covariant_metric();
        let zij = os.contravariant_metric();
        let eye = z_ij*zij;
        let norm2 = (eye - na::SMatrix::<f64, 3, 3>::identity()).norm_squared();

        assert!(norm2 < eps);
    }


    /**
     * Unit test checking volume element which also tests the covariant
     * metric tensor
     */
    #[test]
    fn volume_element() {
        let eps = 1.0e-13;
        // Define an oblate spheroid
        let ecc = 0.4;
        let smaj = 1.0;
        let lon = 1.0;
        let lat = 0.5;
        let os = crate::oblate_spheroid::OblateSpheroid::try_from(
            &(ecc, smaj, lon, lat)).expect("Bad Oblate Spheroid ");
        let z_ij = os.covariant_metric();
        let ve = z_ij.determinant().sqrt();
        let delta = (ve - os.volume_element()).abs();

        //println!("ve: {} vs. {}", ve, os.volume_element());
        //println!("z_ij: {}", z_ij);

        assert!(delta < eps);
    }


    /**
     * Unit test checking volume element which also tests the covariant
     * metric tensor
     */
    #[test]
    fn basis_vector() {
        let eps = 1.0e-13;
        // Define an oblate spheroid
        let ecc = 0.4;
        let smaj = 1.0;
        let lon = 1.0;
        let lat = 0.5;
        let os = crate::oblate_spheroid::OblateSpheroid::try_from(
            &(ecc, smaj, lon, lat)).expect("Bad Oblate Spheroid ");

        // Check z_1 passes through origin
        let (z_1, z_2, z_3) = os.covariant_basis();
        let r = os.cartesian();
        let zero_vec = r.cross(&z_1);
        assert!(zero_vec.norm() < eps);

        // Check z1 is orthogonal to z_2 and z_3 (orthogonal to
        // the tangent plane).  Since z_ij zij = delta^i_j,
        // this should pass by definition...
        let (z1, _, _) = os.contravariant_basis();
        assert!(z1.dot(&z_2).abs() < eps);
        assert!(z1.dot(&z_3).abs() < eps);
    }
}

