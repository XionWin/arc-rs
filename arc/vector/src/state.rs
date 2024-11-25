pub struct State {
    fill_paint: core::Paint,
    stroke_paint: core::Paint,
    stroke_width: f32,
    stroke_multiple: f32,
    miter_limit: f32,
    line_join: core::LineJoin,
    line_cap: core::LineCap,
    transform: core::Matrix2D,
    scissor: core::Scissor,
    alpha: f32,
}
