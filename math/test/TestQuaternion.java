package cognition.math.test;

import cognition.math.Q;
import cognition.math.Basis3D;
import org.apache.commons.math3.geometry.euclidean.threed.Rotation;
import org.apache.commons.math3.geometry.euclidean.threed.RotationOrder;
import org.apache.commons.math3.geometry.euclidean.threed.RotationConvention;

import cognition.math.Vector3D;
import cognition.math.Matrix3X3;
import cognition.math.Quaternion;


public class TestQuaternion {

  public static void main(String[] args) {

    Matrix3X3 rot3 = new Matrix3X3();
    Matrix3X3 rot2 = new Matrix3X3();
    Matrix3X3 rot1 = new Matrix3X3();

    rot3.rotX(Math.toRadians( 30.));
    rot2.rotY(Math.toRadians(-95.));
    rot1.rotZ(Math.toRadians(195.));

    Matrix3X3 rot21 = new Matrix3X3();
    rot21.mult(rot2, rot1);
    Matrix3X3 rot321 = new Matrix3X3();
    rot321.mult(rot3, rot21);
    
    Quaternion q = new Quaternion(rot321);

    Vector3D v0 = new Vector3D(3., 2., 1.);
    Vector3D vq = new Vector3D();
    vq.transform(q, v0);
    Vector3D vm = new Vector3D();
    vm.mult(rot321, v0);

    Vector3D dv = new Vector3D();
    dv.set(vq);
    dv.minus(vm);
    System.out.println("Matrix3X3 vs. Quaternion:  " + dv.norm());

    Vector3D v02 = new Vector3D();
    v02.rotate(q, vq);
    dv.set(v0);
    dv.minus(v02);
    System.out.println("Quaternion transform/rotate:  " + dv.norm());
    
    Rotation apRot = new Rotation(RotationOrder.ZYX,
                                  RotationConvention.FRAME_TRANSFORM,
                                  Math.toRadians(195.),
                                  Math.toRadians(-95.),
                                  Math.toRadians( 30.));
    org.apache.commons.math3.geometry.euclidean.threed.Vector3D av0 =
        new org.apache.commons.math3.geometry.euclidean.threed.Vector3D(3., 2., 1.);
    org.apache.commons.math3.geometry.euclidean.threed.Vector3D av = apRot.applyTo(av0);
    System.out.println("Commons Rotation:  " + diffVec(av, vq));

    Quaternion q1 = new Quaternion(rot1);
    Quaternion q2 = new Quaternion(rot2);
    Quaternion q3 = new Quaternion(rot3);
    Quaternion q12 = new Quaternion();
    q12.mult(q1, q2);
    Quaternion q123 = new Quaternion();
    q123.mult(q12, q3);
    Vector3D vqmult = new Vector3D();
    vqmult.transform(q123, v0);
    dv.set(vq);
    dv.minus(vqmult);
    System.out.println("Quaternion multiply:  " + dv.norm());

  }
  
  public static double diffVec(
    org.apache.commons.math3.geometry.euclidean.threed.Vector3D av, Vector3D v) {
    double n2 = (av.getX()-v.get(Basis3D.I))*(av.getX()-v.get(Basis3D.I) +
                 av.getY()-v.get(Basis3D.J))*(av.getY()-v.get(Basis3D.J) +
                 av.getZ()-v.get(Basis3D.K))*(av.getZ()-v.get(Basis3D.K));
    return Math.sqrt(n2);
  }
  
}
