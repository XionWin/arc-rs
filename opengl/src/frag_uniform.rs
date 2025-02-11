use core::State;

#[repr(u32)]
pub enum FragUniformType {
    FillGradient = 0,
    FillTexture = 1,
    FillStencil = 2,
    FillAlpha = 3,
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

pub const DEFAULT_MONOCHROME_FRAG_UNIFORM: FragUniform = FragUniform {
    _type: 0f32,
    _font_type: 0f32,
    _radius: 0f32,
    _feather: 0f32,
    _stroke_multiple: 0f32,
    _stroke_threshold: 0f32,
    _extent: core::DEFAULT_EXTENT,
    _scissor_matrix: crate::DEFAULT_MATRIX4X3,
    _scissor_extent: core::DEFAULT_EXTENT,
    _scissor_scale: core::DEFAULT_SCALE,
    _paint_matrix: crate::DEFAULT_MATRIX4X3,
    _inner_color: core::Color::White,
    _outer_color: core::Color::White,
};

impl From<&dyn State> for FragUniform {
    fn from(state: &dyn State) -> Self {
        match state.downcast_ref::<vector::StrokeState>() {
            Some(x) => from_stroke_state(x),
            None => match state.downcast_ref::<vector::FillState>() {
                Some(x) => from_fill_state(x),
                None => util::print_panic!("from state to frag_uniform"),
            },
        }
    }
}

impl Into<[f32; 44]> for &FragUniform {
    fn into(self) -> [f32; 44] {
        let extent = Into::<[f32; 2]>::into(&self._extent);
        let scissor_matrix = Into::<[f32; 12]>::into(&self._scissor_matrix);
        let scissor_extent = Into::<[f32; 2]>::into(&self._scissor_extent);
        let scissor_scale = Into::<[f32; 2]>::into(&self._scissor_scale);
        let paint_matrix = Into::<[f32; 12]>::into(&self._paint_matrix);
        let inner_color = Into::<[f32; 4]>::into(&self._inner_color);
        let outer_color = Into::<[f32; 4]>::into(&self._outer_color);
        [
            self._type,
            self._font_type,
            self._radius,
            self._feather,
            self._stroke_multiple,
            self._stroke_threshold,
            extent[0],
            extent[1],
            scissor_matrix[0],
            scissor_matrix[1],
            scissor_matrix[2],
            scissor_matrix[3],
            scissor_matrix[4],
            scissor_matrix[5],
            scissor_matrix[6],
            scissor_matrix[7],
            scissor_matrix[8],
            scissor_matrix[9],
            scissor_matrix[10],
            scissor_matrix[11],
            scissor_extent[0],
            scissor_extent[1],
            scissor_scale[0],
            scissor_scale[1],
            paint_matrix[0],
            paint_matrix[1],
            paint_matrix[2],
            paint_matrix[3],
            paint_matrix[4],
            paint_matrix[5],
            paint_matrix[6],
            paint_matrix[7],
            paint_matrix[8],
            paint_matrix[9],
            paint_matrix[10],
            paint_matrix[11],
            inner_color[0],
            inner_color[1],
            inner_color[2],
            inner_color[3],
            outer_color[0],
            outer_color[1],
            outer_color[2],
            outer_color[3],
        ]
    }
}

impl Into<[f32; 44]> for FragUniform {
    fn into(self) -> [f32; 44] {
        Into::<[f32; 44]>::into(&self)
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
        _extent: match state.get_paint().try_get_paint_texture() {
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
        _paint_matrix: match state.get_paint().try_get_paint_texture() {
            Some(x) => x.get_transform().into(),
            None => crate::Matrix4x3::default(),
        },
        _inner_color: state.get_paint().get_color().get_inner_color(),
        _outer_color: state.get_paint().get_color().get_outer_color(),
    }
}

fn from_fill_state(state: &vector::FillState) -> FragUniform {
    match state.get_paint().try_get_paint_texture() {
        Some(paint_texture) => FragUniform {
            _type: FragUniformType::FillTexture.into(),
            _font_type: 0f32, // DEFAULT
            _radius: state.get_paint().get_radius(),
            _feather: state.get_paint().get_feather(),
            _stroke_multiple: f32::MAX,
            _stroke_threshold: 0f32, // DEFAULT
            _extent: paint_texture.get_extent(),
            _scissor_matrix: match state.get_scissor() {
                Some(x) => x.get_transform().into(),
                None => crate::Matrix4x3::zero(),
            },
            _scissor_extent: match state.get_scissor() {
                Some(x) => x.get_extent(),
                None => core::Extent::default(),
            },
            _scissor_scale: core::Scale::default(), // DEFAULT
            _paint_matrix: paint_texture.get_transform().into(),
            _inner_color: state.get_paint().get_color().get_inner_color(),
            _outer_color: state.get_paint().get_color().get_outer_color(),
        },
        None => FragUniform {
            _type: FragUniformType::FillGradient.into(),
            _font_type: 0f32, // DEFAULT
            _radius: state.get_paint().get_radius(),
            _feather: state.get_paint().get_feather(),
            _stroke_multiple: f32::MAX,
            _stroke_threshold: 0f32, // DEFAULT
            _extent: core::Extent::default(),
            _scissor_matrix: match state.get_scissor() {
                Some(x) => x.get_transform().into(),
                None => crate::Matrix4x3::zero(),
            },
            _scissor_extent: match state.get_scissor() {
                Some(x) => x.get_extent(),
                None => core::Extent::default(),
            },
            _scissor_scale: core::Scale::default(), // DEFAULT
            _paint_matrix: crate::Matrix4x3::default(),
            _inner_color: state.get_paint().get_color().get_inner_color(),
            _outer_color: state.get_paint().get_color().get_outer_color(),
        },
    }
}
