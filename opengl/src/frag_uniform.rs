#[repr(C)]
pub struct FragUniform {
    _type: f32,
    _font_type: f32,
    _radius: f32,
    _feather: f32,
    _stroke_multiple: f32,
    _stroke_threshold: f32,
    _extent: core::Extent<i32>,
    _scissor_matrix: crate::Matrix4x3,
    _scissor_extent: core::Extent<i32>,
    _scissor_scale: core::Scale<f32>,
    _paint_matrix: crate::Matrix4x3,
    _inner_color: core::Color,
    _outer_color: core::Color,
}

// impl From<&dyn vector::State> for FragUniform {
//     fn from(state: &dyn vector::State) -> Self {
//         match state.get_stroke_state() {
//             Some(stroke_state) => todo!(),
//             None => match state.get_fill_state() {
//                 Some(fill_state) => todo!(),
//                 None => util::print_panic!("from state to frag_uniform"),
//             },
//         }
//     }
// }
