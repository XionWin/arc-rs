pub trait State {}

pub struct StrokeState {
    _stroke_paint: Option<core::Paint>,
    _stroke_width: f32,
    _stroke_multiple: f32,
    _line_join: core::LineJoin,
    _line_cap: core::LineCap,
    _transform: core::Matrix2D,
    _scissor: core::Scissor,
}

impl StrokeState {
    pub fn new(stroke_width: f32) -> Self {
        Self {
            _stroke_paint: Some(core::Paint::default()),
            _stroke_width: stroke_width,
            _stroke_multiple: (stroke_width / 2f32 + crate::FRINGE_WIDTH / 2f32)
                / crate::FRINGE_WIDTH,
            _line_join: core::LineJoin::default(),
            _line_cap: core::LineCap::default(),
            _transform: core::Matrix2D::default(),
            _scissor: core::Scissor::default(),
        }
    }
}

impl State for StrokeState {}

pub struct FillState {
    _fill_paint: Option<core::Paint>,
    _transform: core::Matrix2D,
    _scissor: core::Scissor,
}

impl FillState {
    pub fn new() -> Self {
        Self {
            _fill_paint: Some(core::Paint::default()),
            _transform: core::Matrix2D::default(),
            _scissor: core::Scissor::default(),
        }
    }
}

impl State for FillState {}
