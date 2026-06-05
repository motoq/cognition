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

impl Keplerian {
    pub fn try_from_oe(oe: &[(KeplerianElement, f64); 6])
        -> Result<Self, String> {

        let mut a = 0.0;
        let mut e = 0.0;
        let mut i = 0.0;
        let mut o = 0.0;
        let mut w = 0.0;
        let mut v = 0.0;
        for element in oe {
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
        let kep = [a, e, i, o, w, v];
        let rv: na::SMatrix<f64, 6, 1> = kep_to_cart(&kep)?;

        Ok(Self {
            oe: kep,
            cart: rv,
        })

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
    if rv[0] < 0.0 {
        return Err("X".to_string());
    }

    let rvec: na::SMatrix<f64, 3, 1> = rv.fixed_view::<3, 1>(0, 0).into();
    let vvec: na::SMatrix<f64, 3, 1> = rv.fixed_view::<3, 1>(3, 0).into();
    let rdotv = rvec.dot(&vvec);
    let rmag = rvec.norm();
    let vmag = vvec.norm();
    let v2 = vmag*vmag;
    let muor = GM/rmag;

    let hvec = rvec.cross(&vvec);
    let hmag = hvec.norm();

    let khat = na::Vector3::<f64>::z_axis();
    let mut nvec = khat.cross(&hvec);
    let nmag =  nvec.norm();
    // By definition - potential numerical roundoff with cross product
    nvec[2] = 0.0;









    Ok([rv[0], rv[1], rv[2], rv[3], rv[4], rv[5]])
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
    fn from_oe() {
        let eps = 1.0e-13;
        let oelmn: [(KeplerianElement, f64); 6] = [(KeplerianElement::A, 4.2),
                                                   (KeplerianElement::E, 0.7),
                                                   (KeplerianElement::I, 1.1),
                                                   (KeplerianElement::O, 0.5),
                                                   (KeplerianElement::W, 4.7),
                                                   (KeplerianElement::V, 0.25)];

        let kep = Keplerian::try_from_oe(&oelmn).expect("Bad Oblate Spheroid ");

        println!("cart: {}", &kep.cartesian());
        let r: na::SMatrix<f64, 3, 1> = kep.cartesian()
                                           .fixed_view::<3, 1>(0, 0).into();
        println!("cart: {}", &r);
        //println!("cart: {}", kep.cartesian().view((0,0), (3,1)));
        assert!((kep.cartesian()[0] -  5.3340990173540748e-01).abs() < eps);
        assert!((kep.cartesian()[1] - -3.4976218240773604e-01).abs() < eps);
        assert!((kep.cartesian()[2] - -1.1055221648157516e+00).abs() < eps);
        assert!((kep.cartesian()[3] -  9.6879318804014558e-01).abs() < eps);
        assert!((kep.cartesian()[4] -  6.0931883443767210e-01).abs() < eps);
        assert!((kep.cartesian()[5] -  1.3805066965568535e-01).abs() < eps);
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




Keplerian::Keplerian(const Eigen::Matrix<double, 6, 1>& cart)
{
  m_cart = cart;











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
    inc = std::acos(hvec(2)/hmag);
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
  return hmag;
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





*/
