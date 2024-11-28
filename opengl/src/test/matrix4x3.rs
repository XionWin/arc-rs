use core::Matrix2D;
use std::f32::consts::PI;

use crate::Matrix4x3;

#[test]
pub fn test() {
    let matrix2d_roate = Matrix2D::default();
    matrix2d_roate.rotate(PI / 6f32);
    util::print_info!("matrix2d_roate:");
    util::print_debug!("{}", matrix2d_roate);

    let matrix2d_trans = Matrix2D::default();
    matrix2d_trans.translate(100f32, 100f32);
    util::print_info!("matrix2d_trans:");
    util::print_debug!("{}", matrix2d_trans);

    let matrix2d = matrix2d_trans * matrix2d_roate;
    util::print_info!("matrix2d:");
    util::print_debug!("{}", matrix2d);

    let matrix4x3: Matrix4x3 = matrix2d.into();
    util::print_info!("matrix4x3:");
    util::print_debug!("{}", matrix4x3);
}
