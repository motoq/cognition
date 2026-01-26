/*
 * Copyright 2024 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use nalgebra as na;

use cogs::utl_const::RAD_PER_DEG;
use cogs::oblate_spheroid;

/**
 * This program performs a consistency check on the oblate spheroidal
 * struct.  It iterates over a wide range of eccentricities, semimajor axes
 * longitudes, and latitude.  Each set is used to initialize an OblateSpheroid
 * object.  The Cartesian coordinates are then pulled to initialize another
 * OblateSpheroid struct.  The difference between all the elements are RSS'ed
 * and added to a total error that is printed upon completion.
 *
 * In addition, the covariant (Z^i) and contravariant (Z_i) basis vectors
 * for each point are generated and tested as orthonormal:
 * Z^i dot Z_j = delta^i_j
 */
fn main() {
    let decc: f64 = 0.05;
    let dsma: f64 = 0.1;
    let dlon: f64 = RAD_PER_DEG*3.0;
    let dlat: f64 = 0.05;

    let mut count = 0;
    let mut rss_error: f64  = 0.0;
    let mut basis_error: f64 = 0.0;
    let mut jacobian_error: f64 = 0.0;
    let mut ecc: f64 = 0.0;
    // Collect deviations
    while ecc < 0.9 {
        let mut sma: f64 = 0.1;
        while sma < 7.5 {
            let mut lat: f64 = -0.9;
            while lat < 0.9 {
                let mut lon: f64 = RAD_PER_DEG*-180.0 + dlon;
                while lon < RAD_PER_DEG*180.0 {
                    // Define as OS coords
                    let os1 = oblate_spheroid::
                              OblateSpheroid::try_from(&(ecc, sma, lon, lat))
                                  .expect("OblateSpheroid Construction: ");
                    // Convert to Cartesian
                    let xyz = os1.cartesian();
                    // ...and back
                    let os2 = oblate_spheroid::
                              OblateSpheroid::try_from(&(ecc, xyz))
                                  .expect("OblateSpheroid Construction: ");
                    // Accumulate coordinate errors
                    let de = os2.eccentricity() - os1.eccentricity();
                    let da = os2.semimajor() - os1.semimajor();
                    let dn = os2.longitude() - os1.longitude();
                    let dt = os2.latitude() - os1.latitude();
                    rss_error += (de*de + da*da + dn*dn + dt*dt).sqrt();
                    // Check basis vectors
                    let cov = os1.covariant_basis();
                    let cont = os1.contravariant_basis();
                    // Orthoginal basis vectors
                    let d01 = cov.0.dot(&cont.1);
                    let d02 = cov.0.dot(&cont.2);
                    let d12 = cov.1.dot(&cont.2);
                    // Unit dot product
                    let d00 = 1.0 - cov.0.dot(&cont.0);
                    let d11 = 1.0 - cov.1.dot(&cont.1);
                    let d22 = 1.0 - cov.2.dot(&cont.2);
                    basis_error += (d01*d01 + d02*d02 + d12*d12 +
                                    d00*d00 + d11*d11 + d22*d22).sqrt();
                    // Check Jacobians
                    let dcart_dos = os1.jacobian();
                    let dos_dcart = os1.inverse_jacobian();
                    let eye = dcart_dos*dos_dcart;
                    let norm2 = (eye -
                        na::SMatrix::<f64, 3, 3>::identity()).norm_squared();
                    jacobian_error += norm2.sqrt();
                    count += 1;
                    lon += dlon;
                }
                lat += dlat;
            }
            sma += dsma;
        }
        ecc += decc;
    }
    println!("RSS Error over {} tests: {}", count, rss_error);
    println!("Basis Error: {}", basis_error);
    println!("Jacobian Error: {}", jacobian_error);
}
