package cognition.math.test;

import org.apache.commons.math3.linear.RealMatrix;
import org.apache.commons.math3.linear.Array2DRowRealMatrix;

import cognition.math.Q;
import cognition.math.Basis3D;
import cognition.math.VectorEnum;
import cognition.math.tensor.TVector;
import cognition.math.MatrixEnum;
import cognition.math.tensor.TMatrix;

public class TestMatrix {

  public static void main(String[] args) {

    /*
     * Test Matrices
     */

      // First test matrix
    double[][] a1 = { {  1.,  2., -1.,  4. },
                      { -2., -5.,  3.,  7. },
                      { -1., -1.,  1., -3. } };
    RealMatrix rm1 = new Array2DRowRealMatrix(a1);
    MatrixEnum<Basis3D, Q> m1 = new MatrixEnum<>(Basis3D.I, Q.Q0);
    m1.set(a1);
      // Second test matrix
    double[][] a2 = { { 3.,  1., 2. },
                      { 5., -3., 4. },
                      { 1., -2., 1. } };
    RealMatrix rm2 = new Array2DRowRealMatrix(a2);
    MatrixEnum<Basis3D, Basis3D> m2 = MatrixEnum.factory(Basis3D.class,
                                                         Basis3D.class);
    m2.set(a2);
      // Third test matrix
    double[][] a3 = { { 3., 2., 1. },
                      { 2., 4., 3. },
                      { 1., 3., 5. } };
    RealMatrix rm3 = new Array2DRowRealMatrix(a3);
    MatrixEnum<Basis3D, Basis3D> m3 = new MatrixEnum<>(Basis3D.I, Basis3D.I);
    m3.set(a3);
      // Test vector
    double[] av1 = { 9., 8., 7. };
    RealMatrix rmv1 = new Array2DRowRealMatrix(av1);
    TVector v1 = new TVector(av1);
    

    /*
     * Matrix & Matrix/Vector multiply
     */

    RealMatrix rmmult = rm2.multiply(rm1);
    MatrixEnum<Basis3D, Q> mmult1 = new MatrixEnum<>(Basis3D.I, Q.Q0);
    mmult1.mult(m2, m1);
    TMatrix mmult2 = m2.mult(m1);

    RealMatrix rmvmult = rm3.multiply(rmv1);
    VectorEnum<Basis3D> mvult1 = VectorEnum.factory(Basis3D.class);
    mvult1.mult(m3, v1);
    //TVector vmult2 = m3.mult(v1);
    VectorEnum<Basis3D> mvult2 = new VectorEnum<>(Basis3D.I);
    mvult2.mult(m3, v1);

    System.out.println("Norm: " + diffMatrix(rmmult, mmult1));
    System.out.println("Norm: " + diffMatrix(rmmult, mmult2));
    System.out.println("Norm: " + diffMatrix(rmvmult, mvult1));
    System.out.println("Norm: " + diffMatrix(rmvmult, mvult2));


  }

  public static double diffMatrix(RealMatrix rm, TMatrix tm) {
    double[][] a = rm.getData();
    TMatrix tma = new TMatrix(a);
    TMatrix tm2 = tm.minus(tma);
    //TMatrix tm2 = new TMatrix(tm);
    //tm2.minusEquals(tma);
    return tm2.norm();
  }
}
