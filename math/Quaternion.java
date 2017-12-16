/*
 c  Quaternion.java
 c
 c  Copyright (C) 2000, 2017 Kurt Motekew
 c
 c  This library is free software; you can redistribute it and/or
 c  modify it under the terms of the GNU Lesser General Public
 c  License as published by the Free Software Foundation; either
 c  version 2.1 of the License, or (at your option) any later version.
 c
 c  This library is distributed in the hope that it will be useful,
 c  but WITHOUT ANY WARRANTY; without even the implied warranty of
 c  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 c  Lesser General Public License for more details.
 c
 c  You should have received a copy of the GNU Lesser General Public
 c  License along with this library; if not, write to the Free Software
 c  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 c  02110-1301 USA
 */

package cognition.math;

/**
 * This class represents a unit Quaternion meant to be used for
 * reference frame transformations and vector rotations.
 * <P>
 * See "Quaternions and Rotation Sequences" by Jack B. Kuipers for
 * more information.
 *
 * @author Kurt Motekew
 * @since  20080830
 * @since  20171211  Brought over from VSE project
 */
public class Quaternion {
  private static final Vector3D IHAT = new Vector3D(1.0, 0.0, 0.0);
  private static final Vector3D JHAT = new Vector3D(0.0, 1.0, 0.0);
  private static final Vector3D KHAT = new Vector3D(0.0, 0.0, 1.0);  

    /** 
     * DCM to quaternion alg selection factor.  0.25 requires
     * the first solved for quaternion element to be at least
     * 1/4 in magnitude or greater.  1.0 would result in the
     * first needing to be 1/2 or larger (see notes in code).
     */
  public static final double KAPPA = 0.25;

  private static final double TOL = 1.e-26;

    // used for Tuple3D indexing
  private static final Basis3D I = Basis3D.I;
  private static final Basis3D J = Basis3D.J;
  private static final Basis3D K = Basis3D.K;

    // and for external Quaternion indexing
  private static final Q Q0 = Q.Q0;
  private static final Q QI = Q.QI;
  private static final Q QJ = Q.QJ;
  private static final Q QK = Q.QK;

    // quaternion components - set to unit quaternion with
    // zero rotation angle
  private double q0 = 1;
  private double qi = 0;
  private double qj = 0;
  private double qk = 0;

  /**
   * Default constructor
   */
  public Quaternion() {
  }

  /**
   * @param  q  Instantiate with an existing Quaternion by copying
   *            components.  The input quaternion is assumed to be
   *            of unit magnitude.
   */
  public Quaternion(Quaternion q) {
    set(q);
  }

  /**
   * Gets the component values of this quaternion.
   *
   * @param  ndx  A <code>Q</code> indicating which component to
   *              retrieve.
   *
   * @return  The value of the requested component
   */
  public final double get(Q ndx) {
    switch(ndx) {
      case Q0:
        return q0;
      case QI:
        return qi;
      case QJ:
        return qj;
      case QK:
        return qk;
      default:
        return 0.0;
    }
  }

  /**
   * @param  q  Copy the components of q into this quaternion.  The
   *            input quaternion is assumed to be of unit length.
   */
  public final void set(Quaternion q) {
    q0 = q.get(Q0);
    qi = q.get(QI);
    qj = q.get(QJ);
    qk = q.get(QK);
  }

  /**
   * Sets the quaternion given a UNIT pointing vector and angle
   * of rotation.
   * 
   * @param  alpha  Angle, in radians
   * @param  axis   Unit vector aligned with axis of rotation.
   */
  public final void set(double alpha, Vector3D axis) {
    alpha /= 2.0;
    q0 = Math.cos(alpha);
    alpha = Math.sin(alpha);
    qi = axis.get(I)*alpha;
    qj = axis.get(J)*alpha;
    qk = axis.get(K)*alpha;
    normalize();
  }

  /**
   * Sets the quaternion given a Cartesian basis vector about which to rotate
   * and an angle. 
   * 
   * @param  alpha  Angle, in radians
   * @param  axis   Basis vector about which to rotate
   */
  public final void set(double alpha, Basis3D axis) {
    switch (axis) {
      case I:
        set(alpha, IHAT);
        break;
      case J:
        set(alpha, JHAT);
        break;
      case K:
        set(alpha, KHAT);
        break;
    }
  }

  /**
   * Sets this quaternion based on the input direction cosine matrix.
   * <P>
   * Method based discussion in "Quaternion Computation from a Geometric Point
   * of View" by Malcolm Shuster and Gregory Natanson.
   * <P>
   * Many methods check for division by a number "near" zero.  Others begin
   * by determining the quaternion element with the largest magnitude.  This
   * one resembles the 2nd method, except instead of searching for the
   * largest quaternion element (extra computations) it finds the first
   * element that is greater than or equal to 1/2 in magnitude and bases
   * the rest of the quaternion components on that element.
   * <P>
   * Updated to use KAPPA instead of '1' in algorithm selection logic, so the
   * required magnitude of the first solved for quaternion will be dependent
   * on this setting:  KAPPA = 1 -> q_first > 1/2; KAPPA = 0.25 -> q_first > 1/4
   * ("DCM to Quaternion and Back Again", by Kurt Motekew).
   *
   * @param  dcm  Reference frame transformation matrix
   */
  public final void set(Matrix3X3 dcm) {
    double tmp, d4;

      // epsilon < KAPPA < 1, where epsilon is large enough to not
      // cause numerical issues.  If KAPPA is set to 1, change the
      // '>' to '>='.
    if ((tmp = 1 + dcm.get(0,0) + dcm.get(1,1) + dcm.get(2,2)) > KAPPA) {
      tmp = Math.sqrt(tmp);
      q0 = 0.5*tmp;
      d4 = 0.5/tmp;
      qi = (dcm.get(1,2) - dcm.get(2,1))*d4;
      qj = (dcm.get(2,0) - dcm.get(0,2))*d4;
      qk = (dcm.get(0,1) - dcm.get(1,0))*d4;
    } else if ((tmp = 1 + dcm.get(0,0) - dcm.get(1,1) - dcm.get(2,2)) > KAPPA) {
      tmp = Math.sqrt(tmp);
      qi = 0.5*tmp;
      d4 = 0.5/tmp;
      q0 = (dcm.get(1,2) - dcm.get(2,1))*d4;
      qj = (dcm.get(0,1) + dcm.get(1,0))*d4;
      qk = (dcm.get(0,2) + dcm.get(2,0))*d4;
    } else if ((tmp = 1 - dcm.get(0,0) + dcm.get(1,1) - dcm.get(2,2)) > KAPPA) {
      tmp = Math.sqrt(tmp);
      qj = 0.5*tmp;
      d4 = 0.5/tmp;
      q0 = (dcm.get(2,0) - dcm.get(0,2))*d4;
      qi = (dcm.get(0,1) + dcm.get(1,0))*d4;
      qk = (dcm.get(1,2) + dcm.get(2,1))*d4;
    } else if ((tmp = 1 - dcm.get(0,0) - dcm.get(1,1) + dcm.get(2,2)) > KAPPA) {
      tmp = Math.sqrt(tmp);
      qk = 0.5*tmp;
      d4 = 0.5/tmp;
      q0 = (dcm.get(0,1) - dcm.get(1,0))*d4;
      qi = (dcm.get(0,2) + dcm.get(2,0))*d4;
      qj = (dcm.get(1,2) + dcm.get(2,1))*d4;
    } else {
      throw new ArithmeticException("Quaternion.set(dcm):  Can't extract" +
                                             " Quaternion from input DCM");
    }
    normalize();
  }

  /**
   * Sets the value of this quaternion to the product
   * of the two input quaternions:  this = p*q
   *
   * @param  p  first quaternion
   * @param  q  second quaternion
   */
  public void mult(Quaternion p, Quaternion q) {
    q0 =   p.get(Q0)*q.get(Q0) - p.get(QI)*q.get(QI)
         - p.get(QJ)*q.get(QJ) - p.get(QK)*q.get(QK);
    qi =   p.get(Q0)*q.get(QI) + p.get(QI)*q.get(Q0)
         + p.get(QJ)*q.get(QK) - p.get(QK)*q.get(QJ);
    qj =   p.get(Q0)*q.get(QJ) - p.get(QI)*q.get(QK)
         + p.get(QJ)*q.get(Q0) + p.get(QK)*q.get(QI);
    qk =   p.get(Q0)*q.get(QK) + p.get(QI)*q.get(QJ)
         - p.get(QJ)*q.get(QI) + p.get(QK)*q.get(Q0);
    normalize();
  }

  /**
   * Returns the product of this and the input quaternion.
   *
   * @param  q  Input quaternion
   *
   * @return  this*q
   */
  public Quaternion mult(Quaternion q) {
    Quaternion p = new Quaternion(this);
    p.mult(q);
    return p;
  }

  /**
   * Sets the value of this quaternion to the complex
   * conjugate of itself.
   */
  public final void conj() {
    qi = -qi;
    qj = -qj;
    qk = -qk;
  }

  /**
   * Sets the value of this quaternion to the complex
   * conjugate of the input quaternion.
   *
   * @param  q  quaternion from which to form complex conj
   */
  public void conj(Quaternion q) {
    set(q);
    conj();
  }

  /**
   * Sets to a zero rotation, equivalent to an identity DCM.
   */
  public final void identity() {
    q0 = 1.0;
    qi = 0.0;
    qj = 0.0;
    qk = 0.0;
  }

  /**
   * Multiplies this quaternion by -1 if the scalar element is less
   * than zero.  Calling this method will ensure the scalar element
   * is always positive.
   */
  public void standardize() {
    if (q0 < 0.0) {
      q0 *= -1.0;
      qi *= -1.0;
      qj *= -1.0;
      qk *= -1.0;
    }
  }

  /**
   * Normalizes this quaternion.  This process divides each
   * component of this quaternion by its magnitude.
   */
  public final void normalize() {
    final double n2 = q0*q0 + qi*qi + qj*qj + qk*qk;
    final double delta = Math.abs(1.0 - n2);

    if (delta < TOL) {
      // No need to normalize
    } else if (delta < 2.107342e-08) {
      // Normalization criteria and method as described by
      // David Hammen on stackoverflow, Oct 17 '12
      // Pade approximant 
      final double norm_inv = 2.0/(1.0 + n2);
      q0 *= norm_inv;
      qi *= norm_inv;
      qj *= norm_inv;
      qk *= norm_inv;
    } else {
      final double norm_inv = 1.0/Math.sqrt(n2);
      q0 *= norm_inv;
      qi *= norm_inv;
      qj *= norm_inv;
      qk *= norm_inv;
    }
  }

  /**
   * @return  Rotation angle about axis, radians.
   */
  public double angle() {
    return 2.0*Math.acos(q0);
  }

  /**
   * q*vq  (quaternion multiplication performed left to right)
   * <P>
   * Return the result of a reference frame transformation on the
   * input <code>Vector3D</code>.
   *
   * @param  v  Vector to be subjected to a reference frame transformation.
   *
   * @return  The result of applying this quaternion as a reference frame
   *          transformation to the input vector.
   */
  public Vector3D transform(Vector3D v) {
    Vector3D v2 = new Vector3D();
    v2.set(v);
    v2.transform(this, v2);
    return v2;
  }

  /**
   * qvq*  (quaternion multiplication performed left to right)
   * <P>
   * Return the result of a rotating the input <code>Vector3D</code>.
   *
   * @param  v  Vector to be rotated.
   *
   * @return  The result of applying this quaternion as a rotatoin to the
   *          input vector.
   */
  public Vector3D rotate(Vector3D v) {
    Vector3D v2 = new Vector3D();
    v2.set(v);
    v2.rotate(this, v2);
    return v2;
  }

  /**
   * @return  String representation of the components of this
   *          <code>Quaternion</code>
   */
  @Override
  public String toString() {
    return("q0:  " + q0 + "  q:  " + qi + " " + qj + " " + qk); 
  }
}
