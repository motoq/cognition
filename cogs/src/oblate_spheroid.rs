/*
 * Copyright 2024 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use nalgebra as na;

use crate::utl_const::DEG_PER_RAD;

/**
 * Oblate spheroid definition (eccentricity and semimajor axis length)
 * and coordinates (oblate spheroidal and Cartesian) struct.
 */
#[derive(Copy, Clone)]
pub struct OblateSpheroid {
    ecc: f64,
    sma: f64,
    lon: f64,
    lat: f64,
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

        if *eccentricity < 0.0   ||  *eccentricity >= 1.0 {
            return Err("Invalid Eccentricity: ".to_string() +
                        &eccentricity.to_string());
        } else if *semimajor < 0.0 {
            return Err("Invalid Semimajor Axis: ".to_string() +
                       &semimajor.to_string());
        } else if *longitude < -std::f64::consts::PI  ||
                  *longitude >  std::f64::consts::PI {
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

