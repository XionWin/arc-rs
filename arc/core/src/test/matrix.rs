use std::f32::consts::PI;

use crate::Matrix2D;

#[test]
pub fn matrix_test() {
    let matrix = Matrix2D::default();
    util::print_info!("matrix:");
    util::print_debug!("{}", matrix);

    let row_1 = matrix.get_row(1);
    util::print_info!("row_1:");
    util::print_debug!("{}", row_1);

    row_1[0].set(2.0f32);
    util::print_info!("matrix after row changed:");
    util::print_debug!("{}", matrix);

    matrix[1][0].set(3.0f32);
    util::print_info!("matrix after col changed:");
    util::print_debug!("{}", matrix);

    util::print_info!("matrix.get_value():");
    util::print_debug!("{}", matrix.get_value());
}

#[test]
pub fn matrix_calc_test() {
    let matrix = Matrix2D::default();
    util::print_debug!("matrix:\n{}", matrix);
    matrix[1][0].set(2.0f32);
    matrix[0][1].set(2.0f32);
    matrix[1][1].set(2.0f32);
    matrix[1][0].set(3.0f32);
    util::print_debug!("matrix after col changed:\n{}", matrix);

    let m1 = Matrix2D::default();
    m1.translate(20f32, 50f32);
    util::print_debug!("m1:\n{}", m1);
    let m2 = Matrix2D::default();
    m2.rotate(PI / 6f32);
    util::print_debug!("m2:\n{}", m2);

    let result = &m1 * &m2;
    util::print_debug!("result:\n{}", result);

    let m3 = Matrix2D::default();
    m3.rotate(-PI / 6f32);
    util::print_debug!("m3:\n{}", m3);
    let m4 = Matrix2D::default();
    m4.translate(-20f32, -50f32);
    util::print_debug!("m4:\n{}", m4);
    let result = &(&(&m1 * &m2) * &m3) * &m4;
    util::print_debug!("result:\n{}", result);

    assert_eq!(result, Matrix2D::default());
}
