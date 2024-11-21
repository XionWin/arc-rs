use crate::Matrix;

#[test]
pub fn matrix_test() {
    let matrix = Matrix::<f32>::new(3, 4);

    util::print_debug!("matrix:\n{}", matrix);

    let row_1 = matrix.get_row(1);
    util::print_debug!("{:?}", row_1);

    row_1[0].set(2.0f32);
    util::print_debug!("matrix after row changed:\n{}", matrix);

    let col_1 = matrix.get_col(1);
    util::print_debug!("{:?}", col_1);
    col_1[2].set(2.0f32);
    util::print_debug!("matrix after col changed:\n{}", matrix);

    let values = matrix.get_value();
    assert_eq!(values.len(), 3 * 4);

    assert_eq!(values[0], 1f32);
    assert_eq!(values[5], 1f32);
    assert_eq!(values[10], 1f32);

    assert_eq!(values[4], 2.0f32);
    assert_eq!(values[9], 2.0f32);
}

