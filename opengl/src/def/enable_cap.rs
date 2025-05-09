pub enum EnableCap {
    PointSmooth = 2832,
    LineSmooth = 2848,
    LineStipple = 2852,
    PolygonSmooth = 2881,
    PolygonStipple = 2882,
    CullFace = 2884,
    Lighting = 2896,
    ColorMaterial = 2903,
    Fog = 2912,
    DepthTest = 2929,
    StencilTest = 2960,
    Normalize = 2977,
    AlphaTest = 3008,
    Dither = 3024,
    Blend = 3042,
    IndexLogicOp = 3057,
    ColorLogicOp = 3058,
    ScissorTest = 3089,
    TextureGenS = 3168,
    TextureGenT = 3169,
    TextureGenR = 3170,
    TextureGenQ = 3171,
    AutoNormal = 3456,
    Map1Color4 = 3472,
    Map1Index = 3473,
    Map1Normal = 3474,
    Map1TextureCoord1 = 3475,
    Map1TextureCoord2 = 3476,
    Map1TextureCoord3 = 3477,
    Map1TextureCoord4 = 3478,
    Map1Vertex3 = 3479,
    Map1Vertex4 = 3480,
    Map2Color4 = 3504,
    Map2Index = 3505,
    Map2Normal = 3506,
    Map2TextureCoord1 = 3507,
    Map2TextureCoord2 = 3508,
    Map2TextureCoord3 = 3509,
    Map2TextureCoord4 = 3510,
    Map2Vertex3 = 3511,
    Map2Vertex4 = 3512,
    Texture1D = 3552,
    Texture2D = 3553,
    PolygonOffsetPoint = 10753,
    PolygonOffsetLine = 10754,
    ClipPlane0 = 12288,
    ClipPlane1 = 12289,
    ClipPlane2 = 12290,
    ClipPlane3 = 12291,
    ClipPlane4 = 12292,
    ClipPlane5 = 12293,
    Light0 = 16384,
    Light1 = 16385,
    Light2 = 16386,
    Light3 = 16387,
    Light4 = 16388,
    Light5 = 16389,
    Light6 = 16390,
    Light7 = 16391,
    Convolution1DExt = 32784,
    Convolution2DExt = 32785,
    Separable2DExt = 32786,
    HistogramExt = 32804,
    MinmaxExt = 32814,
    PolygonOffsetFill = 32823,
    RescaleNormalExt = 32826,
    Texture3DExt = 32879,
    VertexArray = 32884,
    NormalArray = 32885,
    ColorArray = 32886,
    IndexArray = 32887,
    TextureCoordArray = 32888,
    EdgeFlagArray = 32889,
    InterlaceSgix = 32916,
    MultisampleSgis = 32925,
    SampleAlphaToMaskSgis = 32926,
    SampleAlphaToOneSgis = 32927,
    SampleMaskSgis = 32928,
    TextureColorTableSgi = 32956,
    ColorTableSgi = 32976,
    PostConvolutionColorTableSgi = 32977,
    PostColorMatrixColorTableSgi = 32978,
    Texture4DSgis = 33076,
    PixelTexGenSgix = 33081,
    SpriteSgix = 33096,
    ReferencePlaneSgix = 33149,
    IrInstrument1Sgix = 33151,
    CalligraphicFragmentSgix = 33155,
    FramezoomSgix = 33163,
    FogOffsetSgix = 33176,
    SharedTexturePaletteExt = 33275,
    AsyncHistogramSgix = 33580,
    PixelTextureSgis = 33619,
    AsyncTexImageSgix = 33628,
    AsyncDrawPixelsSgix = 33629,
    AsyncReadPixelsSgix = 33630,
    FragmentLightingSgix = 33792,
    FragmentColorMaterialSgix = 33793,
    FragmentLight0Sgix = 33804,
    FragmentLight1Sgix = 33805,
    FragmentLight2Sgix = 33806,
    FragmentLight3Sgix = 33807,
    FragmentLight4Sgix = 33808,
    FragmentLight5Sgix = 33809,
    FragmentLight6Sgix = 33810,
    FragmentLight7Sgix = 33811,
    RepresentativeFragmentTestNv = 37759,
    SubgroupSizeKhr = 38194,
    SubgroupSupportedStagesKhr = 38195,
    SubgroupSupportedFeaturesKhr = 38196,
    SubgroupQuadAllStagesKhr = 38197,
    ScissorTestExclusiveNv = 38229,
    ScissorBoxExclusiveNv = 38230,
    ShadingRateImagePerPrimitiveNv = 38321,
    ShadingRateImagePaletteCountNv = 38322
}

#[allow(non_upper_case_globals)]
impl EnableCap {
    pub const SampleAlphaToCoverage: EnableCap = EnableCap::SampleAlphaToMaskSgis;
    pub const SampleCoverage: EnableCap = EnableCap::SampleMaskSgis;
}