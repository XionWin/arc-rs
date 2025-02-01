use crate::Number;

#[derive(Default, Debug, Clone, Copy)]
pub struct Size<T>
where
    T: Number,
{
    width: T,
    height: T,
}

impl<T> Size<T>
where
    T: Number,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn get_width(&self) -> T {
        self.width
    }

    pub fn get_height(&self) -> T {
        self.height
    }

    pub fn scale(&self, scale_factor: f32) -> Self {
        Self {
            width: Number::from_f32(self.width.into_f32() * scale_factor),
            height: Number::from_f32(self.height.into_f32() * scale_factor),
        }
    }

    pub fn scale_x_y(&self, x_scale_factor: f32, y_scale_factor: f32) -> Self {
        Self {
            width: Number::from_f32(self.width.into_f32() * x_scale_factor),
            height: Number::from_f32(self.height.into_f32() * y_scale_factor),
        }
    }

    pub fn mul(&self, factor: T) -> Self {
        self * &Size::new(factor, factor)
    }
}

impl<T> std::ops::Mul for &Size<T>
where
    T: Number,
{
    type Output = Size<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Size {
            width: self.width * rhs.width,
            height: self.height * rhs.height,
        }
    }
}

impl<T> std::ops::Mul for Size<T>
where
    T: Number,
{
    type Output = Size<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<T> Into<(T, T)> for Size<T>
where
    T: Number,
{
    fn into(self) -> (T, T) {
        (self.width, self.height)
    }
}
