use crate::{Matrix, Matrix2D};

#[test]
pub fn matrix_test() {
    let matrix = Matrix2D::<f32>::new();
    util::print_debug!("matrix:\n{}", matrix);

    let row_1 = matrix.get_row(1);
    util::print_debug!("{:?}", row_1);

    row_1[0].set(2.0f32);
    util::print_debug!("matrix after row changed:\n{}", matrix);

    let col_2 = matrix.get_col(2);
    util::print_debug!("{:?}", col_2);
    col_2[0].set(2.0f32);
    col_2[1].set(2.0f32);
    util::print_debug!("matrix after col changed:\n{}", matrix);

    matrix[[1, 0]].set(3.0f32);
    util::print_debug!("matrix[(1, 0)]: {:?}", matrix[[1, 0]]);
    util::print_debug!("matrix after col changed:\n{}", matrix);

    util::print_debug!("{:?}", matrix.get_value());
}

#[test]
pub fn matrix_calc_test() {
    let matrix = Matrix2D::<f32>::new();
    util::print_debug!("matrix:\n{}", matrix);
    matrix[[1, 0]].set(2.0f32);
    matrix[[0, 2]].set(2.0f32);
    matrix[[1, 2]].set(2.0f32);
    matrix[[1, 0]].set(3.0f32);
    util::print_debug!("matrix after col changed:\n{}", matrix);

    let matrix2 = Matrix2D::<f32>::new();
    util::print_debug!("matrix2:\n{}", matrix2);

    let result = matrix * matrix2;

    util::print_debug!("result:\n{}", result);
}
