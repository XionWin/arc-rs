#[repr(C)]
pub struct Vector4 {
    _x: f32,
    _y: f32,
    _z: f32,
    _w: f32,
}

impl Vector4 {
    pub(crate) fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            _x: x,
            _y: y,
            _z: z,
            _w: w,
        }
    }
}

#[repr(C)]
pub struct Matrix4x3 {
    _row_0: Vector4,
    _row_1: Vector4,
    _row_2: Vector4,
}

impl Matrix4x3 {
    pub(crate) fn new(
        m11: f32,
        m12: f32,
        m13: f32,
        m14: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m24: f32,
        m31: f32,
        m32: f32,
        m33: f32,
        m34: f32,
    ) -> Self {
        Self {
            _row_0: Vector4::new(m11, m12, m13, m14),
            _row_1: Vector4::new(m21, m22, m23, m24),
            _row_2: Vector4::new(m31, m32, m33, m34),
        }
    }
}

impl From<&core::Matrix2D> for Matrix4x3 {
    fn from(value: &core::Matrix2D) -> Self {
        Self::new(
            value[0][0].get(),
            value[0][1].get(),
            value[0][2].get(),
            0f32,
            value[1][0].get(),
            value[1][1].get(),
            value[1][1].get(),
            0f32,
            0f32,
            0f32,
            1f32,
            0f32,
        )
    }
}
