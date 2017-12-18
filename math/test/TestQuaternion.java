package cognition.math.test;

/*
import org.apache.commons.math3.linear.RealMatrix;
import org.apache.commons.math3.linear.Array2DRowRealMatrix;

import cognition.math.Q;
import cognition.math.Basis3D;
import cognition.math.VectorEnum;
import cognition.math.tensor.TVector;
import cognition.math.MatrixEnum;
*/
import cognition.math.Basis3D;
import org.apache.commons.math3.geometry.euclidean.threed.Rotation;
import org.apache.commons.math3.geometry.euclidean.threed.RotationOrder;
import org.apache.commons.math3.geometry.euclidean.threed.RotationConvention;

import cognition.math.tensor.TMatrix;
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
    rot1.rotZ(Math.toRadians(225.));

    Matrix3X3 rot21 = rot2.mult(rot1);
    Matrix3X3 rot321 = rot3.mult(rot21);
    
    Quaternion q = new Quaternion(rot321);

    Vector3D v0 = new Vector3D(3., 2., 1.);
    Vector3D vq = q.transform(v0);
    Vector3D vm = rot321.mult(v0);

    TMatrix dv = vq.minus(vm);
    System.out.println("Matrix3X3 vs. Quaternion:  " + dv.norm());

    Vector3D v02 = q.rotate(vq);
    dv.set(v0);
    dv.minusEquals(v02);
    System.out.println("Quaternion transform/rotate:  " + dv.norm());
    
    Rotation apRot = new Rotation(RotationOrder.ZYX,
                                  RotationConvention.FRAME_TRANSFORM,
                                  Math.toRadians(225.),
                                  Math.toRadians(-95.),
                                  Math.toRadians( 30.));
    org.apache.commons.math3.geometry.euclidean.threed.Vector3D av0 =
        new org.apache.commons.math3.geometry.euclidean.threed.Vector3D(3., 2., 1.);
    org.apache.commons.math3.geometry.euclidean.threed.Vector3D av = apRot.applyTo(av0);
    System.out.println("Commons Rotation:  " + diffVec(av, vq));
  

  }
  
  public static double diffVec(
    org.apache.commons.math3.geometry.euclidean.threed.Vector3D av, Vector3D v) {
    double n2 = (av.getX()-v.get(Basis3D.I))*(av.getX()-v.get(Basis3D.I) +
                 av.getY()-v.get(Basis3D.J))*(av.getY()-v.get(Basis3D.J) +
                 av.getZ()-v.get(Basis3D.K))*(av.getZ()-v.get(Basis3D.K));
    return Math.sqrt(n2);
  }
  
}
