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
 * basis vectors).  Some functionality (e.g., get_surface_tangent()) relies
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
 *                        2025/02/26  Added get_surface_tangent()
 */
#[derive(Copy, Clone)]
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
     * @return  Point sphere with location at origin
     */
    fn default() -> Self {
        Self {
            ecc: 0.0,
            sma: 0.0,
            lon: 0.0,
            lat: 0.0,
            xyz: [0.0, 0.0, 0.0].into(),
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
        os.set_with_os(*eccentricity, *semimajor, 0.0, 0.0);
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
        os.set_with_os(*eccentricity, *semimajor, *longitude, *latitude);
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
    pub fn get_eccentricity(&self) -> f64 {
        self.ecc
    }

    /**
     * @return  Semimajor axis
     */
    pub fn get_semimajor(&self) -> f64 {
        self.sma
    }

    /**
     * @return  Semiminor axis
     */
    pub fn get_semiminor(&self) -> f64 {
        self.sma*(1.0 - self.ecc*self.ecc).sqrt()
    }

    /**
     * @return  Longitude, radians
     */
    pub fn get_longitude(&self) -> f64 {
        self.lon
    }

    /**
     * @return  Latitude
     */
    pub fn get_latitude(&self) -> f64 {
        self.lat
    }

    /**
     * @return  Cartesian coordinates
     */
    pub fn get_cartesian(&self) -> na::SMatrix<f64, 3, 1> {
        self.xyz
    }

    /**
     * @return  Covariant basis vectors at these coordinates
     */
    pub fn get_cov_basis(&self) -> (na::SMatrix<f64, 3, 1>,
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
     * @return  Contravariant basis vectors at these coordinates
     */
    pub fn get_cont_basis(&self) -> (na::SMatrix<f64, 3, 1>,
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
    pub fn get_surface_tangent(&self, pos: &na::SMatrix<f64, 3, 1>,
                                      pnt: &na::SMatrix<f64, 3, 1>) ->
                                            na::SMatrix<f64, 3, 1>
    {
        // Oblate spheroid to unit sphere affine transformation
        let mut aff: na::SMatrix<f64, 3, 3> = na::SMatrix::identity();
        aff[(0,0)] = 1.0/self.sma;
        aff[(1,1)] = 1.0/self.sma;
        aff[(2,2)] = 1.0/self.get_semiminor();

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
        aff[(2,2)] = self.get_semiminor();
        aff*to_3d*xyz
    }
}

/*
 * Private associated methods
 */

impl OblateSpheroid {
    /*
     * Update Cartesian coords with previously validated OS coords
     *
     * @param  eccen  Eccentricity defining parameter, 0 <= eccen < 1
     * @param  smaj   Semimajor axis defining parameter, smajor > 0
     * @param  lam    Longitude/Azimuth coordinate, -pi/2 < lambda < pi/2
     * @param  eta    Latitude/elevation coordinate, -1 <= eta <= 1 
     */
    fn set_with_os(&mut self, eccen: f64, smaj: f64, lam: f64, eta: f64) {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        let tp = os.get_surface_tangent(&pos, &pnt);
        os.set_with_cartesian(ecc, &tp);
        // The vector from the tangent point to the position should
        // be a linear combination of the tangent plane basis vectors
        let p2t = tp - pos;
        let (_, e2, e3) = os.get_cov_basis();
        let rank_2m = na::SMatrix::<f64, 3, 3>::from_columns(&[e2, e3, p2t]);
        // Check both determinant and rank just to illustrate use of both
        let det = rank_2m.determinant();
        let rank = rank_2m.rank(eps);
        assert!(det < eps  &&  rank == 2);
    }
}

