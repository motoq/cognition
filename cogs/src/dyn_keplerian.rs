/*
 * Copyright 2026 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! This struct contains an osculating Keplerian element set and the equivalent
//! Cartesian state vector.  It can be set with either, providing access to
//! individual orbital elements or position and velocity vectors.  Radians
//! and canonical distance and time units (see phy_const.rs) are used.  Minimal
//! eccentricity and inclination constraints are enforced as this structure is
//! limited to classical orbital elements.  Maximum eccentricity is also
//! enforced as parabolic and hyperbolic orbits are not supported.
//!
//! # Author
//!
//! *  Kurt Motekew  2026/05/02  Initial, based on eom::Keplerian

use nalgebra as na;

use crate::utl_const::DEG_PER_RAD;
use crate::utl_const::RAD_PER_DEG;
use crate::phy_const::GM;
use crate::phy_const::RE;

use crate::mth_angle;

/// Minimum allowable eccentricty for this type of orbital element set
const ECC_EPS: f64 = 1.0e-6;
/// Minimum allowable inclination for this type of orbital element set
const INC_EPS: f64 = RAD_PER_DEG*0.05;
/// Generic minimum orbital element value
const OE_EPS: f64 = 1.0e-5;

/// Keplerian Elements
pub enum KeplerianElement {
    /// Semimajor axis
    A,
    /// Eccentricity
    E,
    /// Inclination
    I,
    /// Right ascension of the ascending node
    O,
    /// Argument of perigee
    W,
    /// True anomaly
    V,
}

//#[derive(Clone)]
pub struct Keplerian {
    /// Holds orbital elements, DU and rad
    oe: [f64; 6],
    /// Cartesian position and velocity, DU and DU/TU
    cart: na::SMatrix<f64, 6, 1>,
}

//
// Constructors
//

impl Default for Keplerian {
    /// Creates a valid Keplerian orbital element set
    ///
    /// # Return
    ///
    /// * Keplerian orbit
    ///
    fn default() -> Self {
        let inc = ((4.0/5.0 as f64).sqrt()).asin();
        let node = std::f64::consts::PI/6.0;
        let argp = 1.5*std::f64::consts::PI;
        let tanon = std::f64::consts::PI/12.0;
        let oelmn = [4.1632, 0.741, inc, node, argp, tanon];
        let rv = kep_to_cart(&oelmn).unwrap();
        Self {
            oe: oelmn,
            cart: rv,
        }
    }
}

impl Keplerian {
    pub fn try_from_oe(oelmn: &[(KeplerianElement, f64); 6]) -> Result<Self,
                                                                    String> {
        let mut a = 0.0;
        let mut e = 0.0;
        let mut i = 0.0;
        let mut o = 0.0;
        let mut w = 0.0;
        let mut v = 0.0;
        for element in oelmn {
            let (oe_type, oe_value) = element;
            match oe_type {
                KeplerianElement::A => a = *oe_value,
                KeplerianElement::E => e = *oe_value,
                KeplerianElement::I => i = *oe_value,
                KeplerianElement::O => o = *oe_value,
                KeplerianElement::W => w = *oe_value,
                KeplerianElement::V => v = *oe_value,
            }
        }
        let oelmn = [a, e, i, o, w, v];
        let rv: na::SMatrix<f64, 6, 1> = kep_to_cart(&oelmn)?;

        Ok(Self {
            oe: oelmn,
            cart: rv,
        })
    }

    pub fn try_from_cart(rv: &na::SMatrix<f64, 6, 1>) -> Result<Self, String> {
        let oelmn: [f64; 6] = cart_to_kep(&rv)?;

        Ok(Self {
            oe: oelmn,
            cart: *rv,
        })
    }
}

/// Public immutable accessor methods
impl Keplerian {
    pub fn orbital_element(&self, elem: KeplerianElement) -> f64 {
        self.oe[elem as usize]
    }

    /// # Return
    ///
    /// * Cartesian coordinates
    ///
    pub fn cartesian(&self) -> na::SMatrix<f64, 6, 1> {
        self.cart
    }

    pub fn position(&self) -> na::SMatrix<f64, 3, 1> {
        self.cart.fixed_view::<3, 1>(0, 0).into()
    }

    pub fn velocity(&self) -> na::SMatrix<f64, 3, 1> {
        self.cart.fixed_view::<3, 1>(3, 0).into()
    }
}


/*
 * Based on Vallado's "Fundamentals of Astrodynamics and Applications",
 * 4th edition, Algorithm 10: COE2RV
 */
fn kep_to_cart(kepv: &[f64; 6]) -> Result<na::SMatrix<f64, 6, 1>, String> {
    let a = kepv[0];
    let e = kepv[1];
    let i = kepv[2];
    let o = kepv[3];
    let w = kepv[4];
    let v = kepv[5];
    
    let rp = a*(1.0 - e);

    // Error checking.  Gravity models not valid below scaling radius.
    // Etc...
    if rp < RE {
        return Err("Perigee distance less than 1 DU:  ".to_string() +
                   &rp.to_string());
    }
    if e < ECC_EPS {
        return Err("Eccentricity too small or negative:  ".to_string() +
                   &e.to_string());
    }
    if i < INC_EPS {
        return Err("Inclination too small for this type of OE:  ".to_string() +
                   &i.to_string());
    }

  let semip = a*(1.0 - e*e);
  let cv = v.cos();
  let sv = v.sin();
  let ecv = e*cv;
  let suop = (GM/semip).sqrt();

  let r_pqw = na::matrix![semip*cv/(1.0 + ecv) ;
                          semip*sv/(1.0 + ecv) ;
                          0.0];
  let v_pqw = na::matrix![-suop*sv ;
                           suop*(e + cv) ;
                           0.0];
  let qw = na::UnitQuaternion::<f64>::from_axis_angle(
      &na::Vector3::<f64>::z_axis(), w);
  let qi = na::UnitQuaternion::<f64>::from_axis_angle(
      &na::Vector3::<f64>::x_axis(), i);
  let qo = na::UnitQuaternion::<f64>::from_axis_angle(
      &na::Vector3::<f64>::z_axis(), o);
  let q_pqw2eci = qo*qi*qw;

  let r_cart = q_pqw2eci*r_pqw;
  let v_cart = q_pqw2eci*v_pqw;

  Ok(na::matrix![r_cart[0] ;
                 r_cart[1] ;
                 r_cart[2] ;
                 v_cart[0] ;
                 v_cart[1] ;
                 v_cart[2]])
}

// Based on Vallado's "Fundamentals of Astrodynamics and Applications",
// 4th edition, Algorithm 9: RV2COE
//
fn cart_to_kep(rv: &na::SMatrix<f64, 6, 1>) -> Result<[f64; 6], String> {
    let rvec: na::SMatrix<f64, 3, 1> = rv.fixed_view::<3, 1>(0, 0).into();
    let vvec: na::SMatrix<f64, 3, 1> = rv.fixed_view::<3, 1>(3, 0).into();
    let rdotv = rvec.dot(&vvec);
    let rmag = rvec.norm();
    let vmag = vvec.norm();
    let v2 = vmag*vmag;
    let muor = GM/rmag;

    // Vis-viva eqn - first check this is an elliptical orbit
    let sme = v2/2.0 - muor;
    if sme >= 0.0 {
        return Err("Orbit must be elliptical Energy >= 0:  ".to_string() +
                   &sme.to_string());
    }
    
    let hvec = rvec.cross(&vvec);
    let hmag = hvec.norm();

    let khat = na::Vector3::<f64>::z_axis();
    let mut nvec = khat.cross(&hvec);
    let nmag =  nvec.norm();
    // By definition - potential numerical roundoff with cross product
    nvec[2] = 0.0;
      // Potential divide by zero error later on - check now
    if nmag < OE_EPS {
        return Err("hxk too small:  ".to_string() + &nmag.to_string());
    }

    // Eccentricity
    let evec = ((v2 - muor)*rvec - rdotv*vvec)/GM;
    let emag = evec.norm();
    if emag < ECC_EPS {
        return Err("Eccentricity too small or negative:  ".to_string() +
                   &emag.to_string());
    }

    // Semimajor axis, perigee radius, final error check
    let sma = -0.5*GM/sme;
    let rp = sma*(1.0 - emag);
    if rp < RE {
        return Err("Perigee distance less than 1 DU:  ".to_string() +
                   &rp.to_string());
    }

    // Inclination
    let inc: f64 = if hvec[0] == 0.0  &&  hvec[1] == 0.0 {
        if hvec[2] > 0.0 {
            0.0
        } else {
            std::f64::consts::PI
        }
    } else {
        (hvec[2]/hmag).acos()
    };

    // RAAN
    let raan: f64 = if nvec[1] == 0.0 {
        0.0
    } else {
        let tmp = (nvec[0]/nmag).acos();
        if nvec[1] < 0.0 {
            std::f64::consts::TAU - tmp
        } else {
            tmp
        }
    };

    // Argument of perigee
    let ehat = evec/emag;
    let nhat = nvec/nmag;
    let tmp  = mth_angle::unit_vec_angle(&nhat, &ehat);
    let argp: f64 = if evec[2] < 0.0 {
        std::f64::consts::TAU - tmp
    } else {
        tmp
    };

    // True anomaly
    let rhat = rvec/rmag;
    let tmp = mth_angle::unit_vec_angle(&ehat, &rhat);
    let tmp: f64 = if rdotv < 0.0 {
        std::f64::consts::TAU - tmp
    } else {
        tmp
    };
    let ta: f64 = if tmp >= std::f64::consts::TAU {
        tmp - std::f64::consts::TAU
    } else {
        tmp
    };

    Ok([sma, emag, inc, raan, argp, ta])
}


//
// I/O
//

impl std::fmt::Display for Keplerian {
    /// Write Keplerian and Cartesian coordinates
    ///
    /// # Return
    ///
    /// * Printable form of OblateSpheroid
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Semimajor:    {} (DU)\n \
                    Eccentricity: {}\n \
                    Inclination:  {} (deg)\n \
                    RAAN:         {} (deg)\n \
                    ArgPerigee    {} (deg)\n \
                    TrueAnomaly   {} (deg)\n \
                    Cartesian     {})",
            self.orbital_element(KeplerianElement::A),
            self.orbital_element(KeplerianElement::E),
            DEG_PER_RAD*self.orbital_element(KeplerianElement::I),
            DEG_PER_RAD*self.orbital_element(KeplerianElement::O),
            DEG_PER_RAD*self.orbital_element(KeplerianElement::W),
            DEG_PER_RAD*self.orbital_element(KeplerianElement::V),
            self.cart)
    }
}


//
// Unit tests
//

#[cfg(test)]
mod tests {
    use super::*;

    // Check that a specific Keplerian orbital element converts to the
    // expected Cartesian state vector and check consistency of converting
    // back to Keplerian
    #[test]
    fn oe_cart() {
        // Allowable error in conversions
        let eps = 1.0e-13;
        // Hard coded reference orbital elements with hard coded expected
        // Cartesian below
        let oelmn: [(KeplerianElement, f64); 6] = [(KeplerianElement::A, 4.2),
                                                   (KeplerianElement::E, 0.7),
                                                   (KeplerianElement::I, 1.1),
                                                   (KeplerianElement::O, 0.5),
                                                   (KeplerianElement::W, 4.7),
                                                   (KeplerianElement::V, 0.25)];

        // first convert to Cartesian, then from Cartesian back to Keplerian
        let kep1 = Keplerian::try_from_oe(&oelmn).expect("Bad Cartesian Orbit");
        let kep2 = Keplerian::try_from_cart(&kep1.cartesian())
                                                 .expect("Bad Keplerain OE");

        // Test print format
        println!("kep1: {}", &kep1);
        println!("kep2: {}", &kep2);
        assert!((kep1.cartesian()[0] -  5.3340990173540748e-01).abs() < eps);
        // Check hard coded Cartesian vs. Keplerian
        assert!((kep1.cartesian()[1] - -3.4976218240773604e-01).abs() < eps);
        assert!((kep1.cartesian()[2] - -1.1055221648157516e+00).abs() < eps);
        assert!((kep1.cartesian()[3] -  9.6879318804014558e-01).abs() < eps);
        assert!((kep1.cartesian()[4] -  6.0931883443767210e-01).abs() < eps);
        assert!((kep1.cartesian()[5] -  1.3805066965568535e-01).abs() < eps);
        // Consistency check back to Keplerian
        assert!((kep1.orbital_element(KeplerianElement::A) -
                 kep2.orbital_element(KeplerianElement::A).abs()) < eps);
        assert!((kep1.orbital_element(KeplerianElement::E) -
                 kep2.orbital_element(KeplerianElement::E).abs()) < eps);
        assert!((kep1.orbital_element(KeplerianElement::I) -
                 kep2.orbital_element(KeplerianElement::I).abs()) < eps);
        assert!((kep1.orbital_element(KeplerianElement::O) -
                 kep2.orbital_element(KeplerianElement::O).abs()) < eps);
        assert!((kep1.orbital_element(KeplerianElement::W) -
                 kep2.orbital_element(KeplerianElement::W).abs()) < eps);
        assert!((kep1.orbital_element(KeplerianElement::V) -
                 kep2.orbital_element(KeplerianElement::V).abs()) < eps);
    }
}




/*

namespace {
    // Gravitational parameter
    // Convergence
  constexpr int niter {100};
  constexpr double eps {1.e-10};
    // oe_eps
}

/*
 * Based on Vallado's "Fundamentals of Astrodynamics and Applications",
 * 4th edition, Algorithm 5: RV2COE
 */
double Keplerian::getEccentricAnomaly() const
{
  double e {m_oe[ie]};
  double v {m_oe[iv]};
  double denom {1.0/(1.0 + e*std::cos(v))};
  double se {std::sin(v)*std::sqrt(1.0 - e*e)*denom};
  double ce {(e + std::cos(v))*denom};

  return std::atan2(se, ce);
}

double Keplerian::getMeanAnomaly() const
{
  double ea {this->getEccentricAnomaly()};
  return  ea - m_oe[ie]*std::sin(ea);
}

/*
 * Based on Vallado's "Fundamentals of Astrodynamics and Applications",
 * 4th edition, Algorithms 2 KepEqtnE and 6 Anomaly to
 */
void Keplerian::setWithMeanAnomaly(double ma)
{
  std::array<double, 6> oe = m_oe;

  double e {oe[ie]};
    // Initial guess to E set to M modified by e
  double ea {ma + e};
  if (ma > utl_const::pi  ||  (ma > -utl_const::pi  &&  ma < 0.0)) {
    ea = ma - e;
  }

  int itr {0};
  double ea0 {ea};
  while (itr < niter) {
    ea = ea0 + (ma - ea0 + e*std::sin(ea0))/(1.0 - e*std::cos(ea0));
    if (std::fabs(ea - ea0) < eps) {
      break;
    }
    ea0 = ea;
    itr++;
  }
  if (itr == niter) {
    throw NonconvergenceException("Keplerian::setWithMeanAnomaly()");
  }

  double cea {std::cos(ea)};
  double denom {1.0/(1.0 - e*cea)};
  double sv {std::sin(ea)*std::sqrt(1.0 - e*e)*denom};
  double cv {(cea - e)*denom};

  oe[iv] = std::atan2(sv, cv);
  this->set(oe);
}

*/
