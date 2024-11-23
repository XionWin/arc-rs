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

    let values = matrix.get_value();
    assert_eq!(
        values.len(),
        matrix.get_row_count() * matrix.get_col_count()
    );

    matrix[(1, 0)].set(3.0f32);
    util::print_debug!("matrix[(1, 0)]: {:?}", matrix[(1, 0)]);
    util::print_debug!("matrix after col changed:\n{}", matrix);

    // assert_eq!(values[0].get(), 1f32);
    // assert_eq!(values[5].get(), 1f32);
    // assert_eq!(values[10].get(), 1f32);

    // assert_eq!(values[4].get(), 2.0f32);
    // assert_eq!(values[9].get(), 2.0f32);
}
