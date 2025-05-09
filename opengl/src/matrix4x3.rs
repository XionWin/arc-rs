use std::fmt::Display;

#[repr(C)]
#[derive(Debug)]
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

impl Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [self._x, self._y, self._z, self._w]
                .iter()
                .map(|x| {
                    let mut number = format!("{:#}", x);
                    {
                        let str: String = match number.len() {
                            v if v < 8 => {
                                vec![' '; (8 - number.len()).max(0)].into_iter().collect()
                            }
                            _ => String::new(),
                        };
                        number.push_str(&str);
                        number
                    }
                })
                .collect::<Vec<String>>()
                .join("\t")
        )
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Matrix4x3 {
    _row_0: Vector4,
    _row_1: Vector4,
    _row_2: Vector4,
}

pub const DEFAULT_MATRIX4X3: Matrix4x3 = Matrix4x3 {
    _row_0: Vector4 {
        _x: 1f32,
        _y: 0f32,
        _z: 0f32,
        _w: 0f32,
    },
    _row_1: Vector4 {
        _x: 0f32,
        _y: 1f32,
        _z: 0f32,
        _w: 0f32,
    },
    _row_2: Vector4 {
        _x: 0f32,
        _y: 0f32,
        _z: 0f32,
        _w: 0f32,
    },
};

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

    pub fn zero() -> Self {
        Self::new(
            0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,
        )
    }
}

impl Default for Matrix4x3 {
    fn default() -> Self {
        crate::DEFAULT_MATRIX4X3
    }
}

impl From<&core::Matrix2D> for Matrix4x3 {
    fn from(value: &core::Matrix2D) -> Self {
        Self::new(
            value[0][0].get(),
            value[0][1].get(),
            0f32,
            0f32,
            value[1][0].get(),
            value[1][1].get(),
            0f32,
            0f32,
            value[2][0].get(),
            value[2][1].get(),
            0f32,
            0f32,
        )
    }
}

impl From<core::Matrix2D> for Matrix4x3 {
    fn from(value: core::Matrix2D) -> Self {
        (&value).into()
    }
}

impl Into<[f32; 12]> for Matrix4x3 {
    fn into(self) -> [f32; 12] {
        [
            self._row_0._x,
            self._row_0._y,
            self._row_0._z,
            self._row_0._w,
            self._row_1._x,
            self._row_1._y,
            self._row_1._z,
            self._row_1._w,
            self._row_2._x,
            self._row_2._y,
            self._row_2._z,
            self._row_2._w,
        ]
    }
}
impl Into<[f32; 12]> for &Matrix4x3 {
    fn into(self) -> [f32; 12] {
        [
            self._row_0._x,
            self._row_0._y,
            self._row_0._z,
            self._row_0._w,
            self._row_1._x,
            self._row_1._y,
            self._row_1._z,
            self._row_1._w,
            self._row_2._x,
            self._row_2._y,
            self._row_2._z,
            self._row_2._w,
        ]
    }
}

impl Display for Matrix4x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}\n", self._row_0, self._row_1, self._row_2)
    }
}
