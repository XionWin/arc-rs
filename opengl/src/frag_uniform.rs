use vector::State;

#[repr(u32)]
pub enum FragUniformType {
    FillGradient,
    FillTexture,
    FillStencil,
    FillAlpha,
}

impl Into<u32> for FragUniformType {
    fn into(self) -> u32 {
        self as _
    }
}
impl Into<f32> for FragUniformType {
    fn into(self) -> f32 {
        Into::<u32>::into(self) as _
    }
}

#[repr(C)]
#[derive(Debug)]
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

impl From<&dyn vector::State> for FragUniform {
    fn from(state: &dyn vector::State) -> Self {
        match state.downcast_ref::<vector::StrokeState>() {
            Some(x) => from_stroke_state(x),
            None => match state.downcast_ref::<vector::FillState>() {
                Some(x) => from_fill_state(x),
                None => util::print_panic!("from state to frag_uniform"),
            },
        }
    }
}

fn from_stroke_state(state: &vector::StrokeState) -> FragUniform {
    FragUniform {
        _type: FragUniformType::FillGradient.into(),
        _font_type: 0f32, // DEFAULT
        _radius: state.get_paint().get_radius(),
        _feather: state.get_paint().get_feather(),
        _stroke_multiple: vector::get_stroke_multiple(state.get_stroke_width()),
        _stroke_threshold: 0f32, // DEFAULT
        _extent: match state.get_paint().get_image() {
            Some(x) => x.get_extent(),
            None => core::Extent::default(),
        },
        _scissor_matrix: match state.get_scissor() {
            Some(x) => x.get_transform().into(),
            None => crate::Matrix4x3::default(),
        },
        _scissor_extent: match state.get_scissor() {
            Some(x) => x.get_extent(),
            None => core::Extent::default(),
        },
        _scissor_scale: core::Scale::default(), // DEFAULT
        _paint_matrix: match state.get_paint().get_image() {
            Some(x) => x.get_transform().into(),
            None => crate::Matrix4x3::default(),
        },
        _inner_color: state.get_paint().get_color().get_inner_color(),
        _outer_color: state.get_paint().get_color().get_outer_color(),
    }
}

fn from_fill_state(state: &vector::FillState) -> FragUniform {
    FragUniform {
        _type: FragUniformType::FillGradient.into(),
        _font_type: 0f32, // DEFAULT
        _radius: state.get_paint().get_radius(),
        _feather: state.get_paint().get_feather(),
        _stroke_multiple: f32::MAX,
        _stroke_threshold: 0f32, // DEFAULT
        _extent: match state.get_paint().get_image() {
            Some(x) => x.get_extent(),
            None => core::Extent::default(),
        },
        _scissor_matrix: match state.get_scissor() {
            Some(x) => x.get_transform().into(),
            None => crate::Matrix4x3::default(),
        },
        _scissor_extent: match state.get_scissor() {
            Some(x) => x.get_extent(),
            None => core::Extent::default(),
        },
        _scissor_scale: core::Scale::default(), // DEFAULT
        _paint_matrix: match state.get_paint().get_image() {
            Some(x) => x.get_transform().into(),
            None => crate::Matrix4x3::default(),
        },
        _inner_color: state.get_paint().get_color().get_inner_color(),
        _outer_color: state.get_paint().get_color().get_outer_color(),
    }
}
