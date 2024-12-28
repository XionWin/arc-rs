use crate::Number;

#[derive(Debug, Clone, Copy)]
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
