use core::Matrix2D;
use std::f32::consts::PI;

use crate::Matrix4x3;

#[test]
pub fn test() {
    let matrix2d_roate = Matrix2D::default();
    matrix2d_roate.rotate(PI / 6f32);
    println!("matrix2d_roate:\n{}", matrix2d_roate);
    let matrix2d_trans = Matrix2D::default();
    matrix2d_trans.translate(100f32, 100f32);
    // let matrix2d = matrix2d_roate * matrix2d_trans;
    // println!("matrix2d:\n{}", matrix2d);

    let matrix2d = matrix2d_trans * matrix2d_roate;
    println!("matrix2d:\n{}", matrix2d);

    let matrix4x3: Matrix4x3 = matrix2d.into();
    println!("matrix4x3:\n{}", matrix4x3);
}
