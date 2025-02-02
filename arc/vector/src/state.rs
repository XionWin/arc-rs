use std::fmt::Debug;

pub trait State: core::AsAny + Debug {
    fn get_paint(&self) -> &core::Paint;
    fn get_transform(&self) -> &core::Matrix2D;
    fn get_scissor(&self) -> Option<&core::Scissor>;
}

impl dyn State {
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: State,
    {
        self.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug)]
pub struct StrokeState {
    _paint: core::Paint,
    _transform: core::Matrix2D,
    _scissor: Option<core::Scissor>,

    _stroke_width: f32,
    _stroke_multiple: f32,
    _line_join: core::LineJoin,
    _line_cap: core::LineCap,
}

impl StrokeState {
    pub fn new(stroke_width: f32) -> Self {
        Self {
            _paint: core::Paint::default(),
            _stroke_width: stroke_width,
            _stroke_multiple: (stroke_width / 2f32 + crate::parameter::FRINGE_WIDTH / 2f32)
                / crate::parameter::FRINGE_WIDTH,
            _line_join: core::LineJoin::default(),
            _line_cap: core::LineCap::default(),
            _transform: core::Matrix2D::default(),
            _scissor: None,
        }
    }

    pub fn get_stroke_width(&self) -> f32 {
        self._stroke_width
    }
}

impl core::AsAny for StrokeState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl State for StrokeState {
    fn get_paint(&self) -> &core::Paint {
        &self._paint
    }

    fn get_transform(&self) -> &core::Matrix2D {
        &self._transform
    }

    fn get_scissor(&self) -> Option<&core::Scissor> {
        match &self._scissor {
            Some(x) => Some(x),
            None => None,
        }
    }
}

impl From<&core::Style> for StrokeState {
    fn from(value: &core::Style) -> Self {
        let stroke_width = value.get_stroke_width().unwrap_or_default() as _;
        Self {
            _paint: Into::<core::Paint>::into(value.get_background()),
            _stroke_width: stroke_width,
            _stroke_multiple: (stroke_width / 2f32 + crate::parameter::FRINGE_WIDTH / 2f32)
                / crate::parameter::FRINGE_WIDTH,
            _line_join: core::LineJoin::default(),
            _line_cap: core::LineCap::default(),
            _transform: core::Matrix2D::default(),
            _scissor: None,
        }
    }
}

#[derive(Debug)]
pub struct FillState {
    _paint: core::Paint,
    _transform: core::Matrix2D,
    _scissor: Option<core::Scissor>,
}

impl Default for FillState {
    fn default() -> Self {
        Self {
            _paint: core::Paint::default(),
            _transform: core::Matrix2D::default(),
            _scissor: None,
        }
    }
}

impl core::AsAny for FillState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl State for FillState {
    fn get_paint(&self) -> &core::Paint {
        &self._paint
    }

    fn get_transform(&self) -> &core::Matrix2D {
        &self._transform
    }

    fn get_scissor(&self) -> Option<&core::Scissor> {
        match &self._scissor {
            Some(x) => Some(x),
            None => None,
        }
    }
}

impl FillState {
    pub fn new(paint: core::Paint, transform: core::Matrix2D) -> Self {
        Self {
            _paint: paint,
            _transform: transform,
            _scissor: None,
        }
    }
}

// impl From<&core::Style> for FillState {
//     fn from(value: &core::Style) -> Self {
//         Self {
//             _paint: Into::<core::Paint>::into(value.background.borrow()),
//             _transform: core::Matrix2D::default(),
//             _scissor: None,
//         }
//     }
// }
