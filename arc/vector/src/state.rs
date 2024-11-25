const FRINGE_WIDTH: f32 = 1f32;
pub struct StrokeState {
    _stroke_paint: Option<core::Paint>,
    _stroke_width: f32,
    _stroke_multiple: f32,
    _line_join: core::LineJoin,
    _line_cap: core::LineCap,
}

impl StrokeState {
    pub fn new(stroke_width: f32) -> Self {
        Self {
            _stroke_paint: Some(core::Paint::default()),
            _stroke_width: stroke_width,
            _stroke_multiple: (stroke_width * 0.5f32 + FRINGE_WIDTH * 0.5f32) / FRINGE_WIDTH,
            _line_join: core::LineJoin::default(),
            _line_cap: core::LineCap::default(),
        }
    }
}

pub struct FillState {
    _fill_paint: Option<core::Paint>,
}

pub struct State {
    _fill: Option<FillState>,
    _stroke: Option<StrokeState>,
    _transform: core::Matrix2D,
    _scissor: core::Scissor,
    _alpha: f32,
}
