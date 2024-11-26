#[repr(C)]
pub struct Vector4 {
    _x: f32,
    _y: f32,
    _z: f32,
    _w: f32,
}

#[repr(C)]
pub struct Matrix4x3 {
    _row_0: Vector4,
    _row_1: Vector4,
    _row_2: Vector4,
}
