package cognition.math.test;

import cognition.math.Basis3D;
import org.apache.commons.math3.geometry.euclidean.threed.Rotation;
import org.apache.commons.math3.geometry.euclidean.threed.RotationOrder;
import org.apache.commons.math3.geometry.euclidean.threed.RotationConvention;

import cognition.math.Vector3D;
import cognition.math.Matrix3X3;
import cognition.math.Quaternion;


public class TestQuaternion {

  public static void main(String[] args) {
    double xrot = Math.toRadians( 30.);
    double yrot = Math.toRadians(-95.);
    double zrot = Math.toRadians(195.);
    
    Matrix3X3 rot3 = new Matrix3X3();
    Matrix3X3 rot2 = new Matrix3X3();
    Matrix3X3 rot1 = new Matrix3X3();

    for (int ii=0; ii<4; ii++) {
      switch (ii) {
        case 1:
          System.out.println("2nd Run");
          xrot *= -1.;
          yrot *= -1.;
          zrot *= -1.;
          break;
        case 2:
          System.out.println("3rd Run");
          xrot *= -1.5;
          yrot *= -1.5;
          zrot *= -1.5;
          break;
        case 3:
          System.out.println("4th Run");
          xrot -= 3;
          yrot -= 3;
          zrot -= 3;
          break;
        default:
          System.out.println("1st Run");
    }    

      rot3.rotX(xrot);
      rot2.rotY(yrot);
      rot1.rotZ(zrot);

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
                                    zrot, yrot, xrot);
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
      System.out.println("Quaternion multiply:  " + diffVec(av, vqmult));
    }
  }
  
  public static double diffVec(
    org.apache.commons.math3.geometry.euclidean.threed.Vector3D av, Vector3D v) {
    double n2 = Math.pow(av.getX()-v.get(Basis3D.I), 2) +
                Math.pow(av.getY()-v.get(Basis3D.J), 2) +
                Math.pow(av.getZ()-v.get(Basis3D.K), 2);
    return Math.sqrt(n2);
  }
  
}
