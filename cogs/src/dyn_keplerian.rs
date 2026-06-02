/*
 * Copyright 2026, 2025 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! This struct represents

use nalgebra as na;

use crate::utl_const::DEG_PER_RAD;
use crate::utl_const::RAD_PER_DEG;
use crate::phy_const::GM;
use crate::phy_const::RE;

/// Minimum allowable eccentricty for this type of orbital element set
const ECC_EPS: f64 = 1.0e-5;
/// Minimum allowable inclination for this type of orbital element set
const INC_EPS: f64 = RAD_PER_DEG*0.05;

pub enum KeplerianElement {
    A,
    E,
    I,
    O,
    W,
    V,
}

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

/// Keplerian orbit
///
/// # Author
///
/// *  Kurt Motekew  2026/05/02  Initial, based on eom::Keplerian
///
//#[derive(Clone)]
pub struct Keplerian {
    oe: [f64; 6],
    cart: na::SMatrix<f64, 6, 1>,
}



//
// Constructors
//

impl Default for Keplerian {
    /// Creates default 
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
        let kep = [4.1632, 0.741, inc, node, argp, tanon];
        let rv = kep_to_cart(&kep).unwrap();
        Self {
            oe: kep,
            cart: rv,
        }
    }
}

/// Public immutable accessor methods
impl Keplerian {
    pub fn oe(&self, elem: KeplerianElement) -> f64 {
        self.oe[elem as usize]
    }

    /// # Return
    ///
    /// * Cartesian coordinates
    ///
    pub fn cartesian(&self) -> na::SMatrix<f64, 6, 1> {
        self.cart
    }
}

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
            self.oe[KeplerianElement::A as usize],
            self.oe[KeplerianElement::E as usize],
            DEG_PER_RAD*self.oe[KeplerianElement::I as usize],
            DEG_PER_RAD*self.oe[KeplerianElement::O as usize],
            DEG_PER_RAD*self.oe[KeplerianElement::W as usize],
            DEG_PER_RAD*self.oe[KeplerianElement::V as usize],
            self.cart)
    }
}



//
// Unit tests
//

#[cfg(test)]
mod tests {
    use super::*;

    // Unit test checking the Cartesian portion of the default
    // oblate spheroid was manually set to the correct value.
    #[test]
    fn os_default() {
        let oe0 = Keplerian::default();
        let oe1 = Keplerian::default();
        //let os1 = crate::oblate_spheroid::OblateSpheroid::try_from(
        //    &(0.0, 1.0, 0.0, 0.0)).expect("Bad Oblate Spheroid ");
        //    ecc  sma  lon  lat

        println!("OE: \n{}", oe0);
        assert!(oe0.cartesian() == oe1.cartesian());
    }
}




/*


namespace {
    // Gravitational parameter
  constexpr double gm {phy_const::gm};
    // Indexing
  constexpr int ia {0};           // Semimajor axis
  constexpr int ie {1};           // Eccentricity
  constexpr int ii {2};           // Inclination
  constexpr int io {3};           // RAAN
  constexpr int iw {4};           // Argument of perigee
  constexpr int iv {5};           // True anomaly
    // Convergence
  constexpr int niter {100};
  constexpr double eps {1.e-10};
    // oe_eps
}

namespace eom {

Keplerian::Keplerian()
{
    // Create default placeholder orbit
  std::array<double, 6> oe = {7.5, 0.75, 1.1, 0.5, 4.7, 0.0};
  this->set(oe);
}


Keplerian::Keplerian(const std::array<double, 6>& oe)
{
  this->set(oe);
}


/*
 * Based on Vallado's "Fundamentals of Astrodynamics and Applications",
 * 4th edition, Algorithm 9: RV2COE
 */
Keplerian::Keplerian(const Eigen::Matrix<double, 6, 1>& cart)
{
  m_cart = cart;

  Eigen::Matrix<double, 3, 1> rvec {m_cart.block<3,1>(0,0)};
  Eigen::Matrix<double, 3, 1> vvec {m_cart.block<3,1>(3,0)};

  Eigen::Matrix<double, 3, 1> hvec {rvec.cross(vvec)};
  Eigen::Matrix<double, 3, 1> khat {Eigen::Vector3d::UnitZ()};
  Eigen::Matrix<double, 3, 1> nvec {khat.cross(hvec)};
  double nmag {nvec.norm()};
    // By definition - potential numerical roundoff with cross product
  nvec(2) = 0.0;

  double rmag {rvec.norm()};
  double vmag {vvec.norm()};
  m_hmag = hvec.norm();
  double v2 {vmag*vmag};
  double rdotv {rvec.dot(vvec)};
  double muor {gm/rmag};

    // Eccentricity
  Eigen::Matrix<double, 3, 1> evec {((v2 - muor)*rvec - rdotv*vvec)/gm};
  double emag {evec.norm()};
    // Vis-viva eqn
  m_sme = v2/2.0 - muor;

    // Some initial error checking
  if (emag < oe_eps) {
    throw std::invalid_argument(
        "Keplerian::Keplerian(): Eccentricity too close to zero");
  }
  if (nmag < oe_eps) {
    throw std::invalid_argument(
        "Keplerian::Keplerian(): Inclination too close to zero");
  }
  if (m_sme >= 0.0) {
    throw std::invalid_argument(
        "Keplerian::Keplerian(): Orbit must be elliptical");
  }
    // Semimajor axis, perigee radius, final error check
  double sma {-0.5*gm/m_sme};
  m_rp = sma*(1.0 - emag);
  m_ra = sma*(1.0 + emag);
  if (m_rp < phy_const::re) {
    throw std::invalid_argument(
        "Keplerian::set(): Perigee distance less than 1 DU");
  }

    // Inclination
  double inc {};
  if (hvec(0) == 0.0  &&  hvec(1) == 0.0) {
    if (hvec(2) > 0.0) {
      inc = 0.0;
    } else {
      inc = utl_const::pi; 
    }
  } else {
    inc = std::acos(hvec(2)/m_hmag);
  }
    // RAAN
  double raan {};
  if (nvec(1) == 0.0) {
    raan = 0.0;
  } else {
    raan = std::acos(nvec(0)/nmag);
  }
  if (nvec(1) < 0.0) {
    raan = utl_const::tpi - raan;
  }
    // Argument of perigee
  Eigen::Matrix<double, 3, 1> ehat {evec/emag};
  Eigen::Matrix<double, 3, 1> nhat {nvec/nmag};
  double argp {mth_angle::unit_vec_angle<double>(nhat, ehat)};
  if (evec(2) < 0.0) {
    argp = utl_const::tpi - argp;
  }
    // True anomaly
  Eigen::Matrix<double, 3, 1> rhat {rvec/rmag};
  double ta {mth_angle::unit_vec_angle<double>(ehat, rhat)};
  if (rdotv < 0.0) {
    ta = utl_const::tpi - ta;
  }
  if (ta >= utl_const::tpi) {
    ta -= utl_const::tpi;
  }

  m_oe[ia] = sma;
  m_oe[ie] = emag;
  m_oe[ii] = inc;
  m_oe[io] = raan;
  m_oe[iw] = argp;
  m_oe[iv] = ta;

}


/*
 * Based on Vallado's "Fundamentals of Astrodynamics and Applications",
 * 4th edition, Algorithm 10: COE2RV
 */
void Keplerian::set(const std::array<double, 6>& oe)
{
}


double Keplerian::getEnergy() const
{
  return m_sme;
}


double Keplerian::getAngularMomentum() const
{
  return m_hmag;
}


double Keplerian::getSemimajorAxis() const
{
  return m_oe[ia];
}


double Keplerian::getEccentricity() const
{
  return m_oe[ie];
}


double Keplerian::getInclination() const
{
  return m_oe[ii];
}


double Keplerian::getRaan() const
{
  return m_oe[io];
}


double Keplerian::getArgumentOfPerigee() const
{
  return m_oe[iw];
}


double Keplerian::getTrueAnomaly() const
{
  return m_oe[iv];
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


double Keplerian::getMeanMotion() const
{
  return  std::sqrt(gm/(m_oe[ia]*m_oe[ia]*m_oe[ia]));
}


double Keplerian::getPeriod() const
{
  double a {m_oe[ia]};
  return utl_const::tpi*std::sqrt(a*a*a/gm);
}


double Keplerian::getPerigeeSpeed() const
{
  return std::sqrt(gm*(2.0/m_rp - 1.0/m_oe[ia]));
}


double Keplerian::getApogeeSpeed() const
{
  return std::sqrt(gm*(2.0/m_ra - 1.0/m_oe[ia]));
}


double Keplerian::getSemilatusRectum() const noexcept
{
  return m_oe[ia]*(1.0 - m_oe[ie]*m_oe[ie]);
}


double Keplerian::getSpeed(double r) const
{
  return std::sqrt(gm*(2.0/r - 1.0/m_oe[ia]));
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


Eigen::Matrix<double, 3, 3> Keplerian::getEciToPerifocal() const
{
  Eigen::Matrix<double, 3, 1> rvec {m_cart.block<3,1>(0,0)};
  Eigen::Matrix<double, 3, 1> vvec {m_cart.block<3,1>(3,0)};

    // Angular momentum
  Eigen::Matrix<double, 3, 1> hvec {rvec.cross(vvec)};
  hvec.normalize();
    // Eccentricity
  double v2 {vvec.dot(vvec)};
  double muor {gm/rvec.norm()};
  double rdotv {rvec.dot(vvec)};
  Eigen::Matrix<double, 3, 1> evec {((v2 - muor)*rvec - rdotv*vvec)/gm};
  evec.normalize();
    // y-axis
  Eigen::Matrix<double, 3, 1> fvec {hvec.cross(evec)};
  fvec.normalize();
    // eci to pef
  Eigen::Matrix<double, 3, 3> c_pi;
  c_pi.row(0) = evec;
  c_pi.row(1) = fvec;
  c_pi.row(2) = hvec;

  return c_pi;
}


std::ostream& operator<<(std::ostream& out, const Keplerian& kep)
{
  std::array<double, 6> oe = kep.getOrbitalElements();
  Eigen::Matrix<double, 6, 1> cart = kep.getCartesian();
  return out << std::fixed <<
                std::setprecision(2) <<
                "    (" <<
                phy_const::tu_per_day/kep.getPeriod() << " rev/day, " <<
                phy_const::min_per_tu*kep.getPeriod() << " minutes)" <<
                std::setprecision(3) <<
                "\n  a: " << phy_const::km_per_du*oe[0] << " km" <<
               std::setprecision(6) <<
               "  e: " << oe[1] <<
               "  i: " << utl_const::deg_per_rad*oe[2] << "\u00B0" <<
               "  o: " << utl_const::deg_per_rad*oe[3] << "\u00B0" <<
               "  w: " << utl_const::deg_per_rad*oe[4] << "\u00B0" <<
               "\n  v: " << utl_const::deg_per_rad*oe[5] << "\u00B0" <<
               "  M: " <<
               utl_const::deg_per_rad*kep.getMeanAnomaly() << "\u00B0" <<
               "  E: " <<
               utl_const::deg_per_rad*kep.getEccentricAnomaly() << "\u00B0" <<
               std::setprecision(3) <<
               "\n    {" << phy_const::km_per_du*cart(0) << "  " <<
                            phy_const::km_per_du*cart(1) << "  " <<
                            phy_const::km_per_du*cart(2) << "} km" <<
               std::setprecision(6) <<
               "\n    {" <<
               phy_const::km_per_du*cart(3)*phy_const::tu_per_sec << "  " <<
               phy_const::km_per_du*cart(4)*phy_const::tu_per_sec << "  " <<
               phy_const::km_per_du*cart(5)*phy_const::tu_per_sec <<
               "} km/sec";
}


}



*/
