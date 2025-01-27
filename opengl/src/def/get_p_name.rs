pub enum GetPName {
    //
    // Summary:
    //     Original was GL_CURRENT_COLOR = 0x0B00
    CurrentColor = 2816,
    //
    // Summary:
    //     Original was GL_CURRENT_INDEX = 0x0B01
    CurrentIndex = 2817,
    //
    // Summary:
    //     Original was GL_CURRENT_NORMAL = 0x0B02
    CurrentNormal = 2818,
    //
    // Summary:
    //     Original was GL_CURRENT_TEXTURE_COORDS = 0x0B03
    CurrentTextureCoords = 2819,
    //
    // Summary:
    //     Original was GL_CURRENT_RASTER_COLOR = 0x0B04
    CurrentRasterColor = 2820,
    //
    // Summary:
    //     Original was GL_CURRENT_RASTER_INDEX = 0x0B05
    CurrentRasterIndex = 2821,
    //
    // Summary:
    //     Original was GL_CURRENT_RASTER_TEXTURE_COORDS = 0x0B06
    CurrentRasterTextureCoords = 2822,
    //
    // Summary:
    //     Original was GL_CURRENT_RASTER_POSITION = 0x0B07
    CurrentRasterPosition = 2823,
    //
    // Summary:
    //     Original was GL_CURRENT_RASTER_POSITION_VALID = 0x0B08
    CurrentRasterPositionValid = 2824,
    //
    // Summary:
    //     Original was GL_CURRENT_RASTER_DISTANCE = 0x0B09
    CurrentRasterDistance = 2825,
    //
    // Summary:
    //     Original was GL_POINT_SMOOTH = 0x0B10
    PointSmooth = 2832,
    //
    // Summary:
    //     Original was GL_POINT_SIZE = 0x0B11
    PointSize = 2833,
    //
    // Summary:
    //     Original was GL_POINT_SIZE_RANGE = 0x0B12
    PointSizeRange = 2834,
    //
    // Summary:
    //     Original was GL_POINT_SIZE_GRANULARITY = 0x0B13
    PointSizeGranularity = 2835,
    //
    // Summary:
    //     Original was GL_LINE_SMOOTH = 0x0B20
    LineSmooth = 2848,
    //
    // Summary:
    //     Original was GL_LINE_WIDTH = 0x0B21
    LineWidth = 2849,
    //
    // Summary:
    //     Original was GL_LINE_WIDTH_RANGE = 0x0B22
    LineWidthRange = 2850,
    //
    // Summary:
    //     Original was GL_LINE_WIDTH_GRANULARITY = 0x0B23
    LineWidthGranularity = 2851,
    //
    // Summary:
    //     Original was GL_LINE_STIPPLE = 0x0B24
    LineStipple = 2852,
    //
    // Summary:
    //     Original was GL_LINE_STIPPLE_PATTERN = 0x0B25
    LineStipplePattern = 2853,
    //
    // Summary:
    //     Original was GL_LINE_STIPPLE_REPEAT = 0x0B26
    LineStippleRepeat = 2854,
    //
    // Summary:
    //     Original was GL_LIST_MODE = 0x0B30
    ListMode = 2864,
    //
    // Summary:
    //     Original was GL_MAX_LIST_NESTING = 0x0B31
    MaxListNesting = 2865,
    //
    // Summary:
    //     Original was GL_LIST_BASE = 0x0B32
    ListBase = 2866,
    //
    // Summary:
    //     Original was GL_LIST_INDEX = 0x0B33
    ListIndex = 2867,
    //
    // Summary:
    //     Original was GL_POLYGON_MODE = 0x0B40
    PolygonMode = 2880,
    //
    // Summary:
    //     Original was GL_POLYGON_SMOOTH = 0x0B41
    PolygonSmooth = 2881,
    //
    // Summary:
    //     Original was GL_POLYGON_STIPPLE = 0x0B42
    PolygonStipple = 2882,
    //
    // Summary:
    //     Original was GL_EDGE_FLAG = 0x0B43
    EdgeFlag = 2883,
    //
    // Summary:
    //     Original was GL_CULL_FACE = 0x0B44
    CullFace = 2884,
    //
    // Summary:
    //     Original was GL_CULL_FACE_MODE = 0x0B45
    CullFaceMode = 2885,
    //
    // Summary:
    //     Original was GL_FRONT_FACE = 0x0B46
    FrontFace = 2886,
    //
    // Summary:
    //     Original was GL_LIGHTING = 0x0B50
    Lighting = 2896,
    //
    // Summary:
    //     Original was GL_LIGHT_MODEL_LOCAL_VIEWER = 0x0B51
    LightModelLocalViewer = 2897,
    //
    // Summary:
    //     Original was GL_LIGHT_MODEL_TWO_SIDE = 0x0B52
    LightModelTwoSide = 2898,
    //
    // Summary:
    //     Original was GL_LIGHT_MODEL_AMBIENT = 0x0B53
    LightModelAmbient = 2899,
    //
    // Summary:
    //     Original was GL_SHADE_MODEL = 0x0B54
    ShadeModel = 2900,
    //
    // Summary:
    //     Original was GL_COLOR_MATERIAL_FACE = 0x0B55
    ColorMaterialFace = 2901,
    //
    // Summary:
    //     Original was GL_COLOR_MATERIAL_PARAMETER = 0x0B56
    ColorMaterialParameter = 2902,
    //
    // Summary:
    //     Original was GL_COLOR_MATERIAL = 0x0B57
    ColorMaterial = 2903,
    //
    // Summary:
    //     Original was GL_FOG = 0x0B60
    Fog = 2912,
    //
    // Summary:
    //     Original was GL_FOG_INDEX = 0x0B61
    FogIndex = 2913,
    //
    // Summary:
    //     Original was GL_FOG_DENSITY = 0x0B62
    FogDensity = 2914,
    //
    // Summary:
    //     Original was GL_FOG_START = 0x0B63
    FogStart = 2915,
    //
    // Summary:
    //     Original was GL_FOG_END = 0x0B64
    FogEnd = 2916,
    //
    // Summary:
    //     Original was GL_FOG_MODE = 0x0B65
    FogMode = 2917,
    //
    // Summary:
    //     Original was GL_FOG_COLOR = 0x0B66
    FogColor = 2918,
    //
    // Summary:
    //     [requires: NV_viewport_array, or OES_viewport_array] Original was GL_DEPTH_RANGE
    //     = 0x0B70
    DepthRange = 2928,
    //
    // Summary:
    //     Original was GL_DEPTH_TEST = 0x0B71
    DepthTest = 2929,
    //
    // Summary:
    //     Original was GL_DEPTH_WRITEMASK = 0x0B72
    DepthWritemask = 2930,
    //
    // Summary:
    //     Original was GL_DEPTH_CLEAR_VALUE = 0x0B73
    DepthClearValue = 2931,
    //
    // Summary:
    //     Original was GL_DEPTH_FUNC = 0x0B74
    DepthFunc = 2932,
    //
    // Summary:
    //     Original was GL_ACCUM_CLEAR_VALUE = 0x0B80
    AccumClearValue = 2944,
    //
    // Summary:
    //     Original was GL_STENCIL_TEST = 0x0B90
    StencilTest = 2960,
    //
    // Summary:
    //     Original was GL_STENCIL_CLEAR_VALUE = 0x0B91
    StencilClearValue = 2961,
    //
    // Summary:
    //     Original was GL_STENCIL_FUNC = 0x0B92
    StencilFunc = 2962,
    //
    // Summary:
    //     Original was GL_STENCIL_VALUE_MASK = 0x0B93
    StencilValueMask = 2963,
    //
    // Summary:
    //     Original was GL_STENCIL_FAIL = 0x0B94
    StencilFail = 2964,
    //
    // Summary:
    //     Original was GL_STENCIL_PASS_DEPTH_FAIL = 0x0B95
    StencilPassDepthFail = 2965,
    //
    // Summary:
    //     Original was GL_STENCIL_PASS_DEPTH_PASS = 0x0B96
    StencilPassDepthPass = 2966,
    //
    // Summary:
    //     Original was GL_STENCIL_REF = 0x0B97
    StencilRef = 2967,
    //
    // Summary:
    //     Original was GL_STENCIL_WRITEMASK = 0x0B98
    StencilWritemask = 2968,
    //
    // Summary:
    //     Original was GL_MATRIX_MODE = 0x0BA0
    MatrixMode = 2976,
    //
    // Summary:
    //     Original was GL_NORMALIZE = 0x0BA1
    Normalize = 2977,
    //
    // Summary:
    //     Original was GL_Viewport = 0X0ba2
    Viewport = 2978,
    //
    // Summary:
    //     Original was GL_MODELVIEW_STACK_DEPTH = 0x0BA3
    ModelviewStackDepth = 2979,
    //
    // Summary:
    //     Original was GL_PROJECTION_STACK_DEPTH = 0x0BA4
    ProjectionStackDepth = 2980,
    //
    // Summary:
    //     Original was GL_TEXTURE_STACK_DEPTH = 0x0BA5
    TextureStackDepth = 2981,
    //
    // Summary:
    //     Original was GL_MODELVIEW_MATRIX = 0x0BA6
    ModelviewMatrix = 2982,
    //
    // Summary:
    //     Original was GL_PROJECTION_MATRIX = 0x0BA7
    ProjectionMatrix = 2983,
    //
    // Summary:
    //     Original was GL_TEXTURE_MATRIX = 0x0BA8
    TextureMatrix = 2984,
    //
    // Summary:
    //     Original was GL_ATTRIB_STACK_DEPTH = 0x0BB0
    AttribStackDepth = 2992,
    //
    // Summary:
    //     Original was GL_CLIENT_ATTRIB_STACK_DEPTH = 0x0BB1
    ClientAttribStackDepth = 2993,
    //
    // Summary:
    //     Original was GL_ALPHA_TEST = 0x0BC0
    AlphaTest = 3008,
    //
    // Summary:
    //     Original was GL_ALPHA_TEST_FUNC = 0x0BC1
    AlphaTestFunc = 3009,
    //
    // Summary:
    //     Original was GL_ALPHA_TEST_REF = 0x0BC2
    AlphaTestRef = 3010,
    //
    // Summary:
    //     Original was GL_Dither = 0X0bd0
    Dither = 3024,
    //
    // Summary:
    //     Original was GL_BLEND_DST = 0x0BE0
    BlendDst = 3040,
    //
    // Summary:
    //     Original was GL_BLEND_SRC = 0x0BE1
    BlendSrc = 3041,
    //
    // Summary:
    //     Original was GL_Blend = 0X0be2
    Blend = 3042,
    //
    // Summary:
    //     Original was GL_LOGIC_OP_MODE = 0x0BF0
    LogicOpMode = 3056,
    //
    // Summary:
    //     Original was GL_LOGIC_OP = 0x0BF1
    LogicOp = 3057,
    //
    // Summary:
    //     Original was GL_COLOR_LOGIC_OP = 0x0BF2
    ColorLogicOp = 3058,
    //
    // Summary:
    //     Original was GL_AUX_BUFFERS = 0x0C00
    AuxBuffers = 3072,
    //
    // Summary:
    //     Original was GL_DRAW_BUFFER = 0x0C01
    DrawBuffer = 3073,
    //
    // Summary:
    //     Original was GL_READ_BUFFER = 0x0C02
    ReadBuffer = 3074,
    //
    // Summary:
    //     [requires: NV_viewport_array, or OES_viewport_array] Original was GL_SCISSOR_BOX
    //     = 0x0C10
    ScissorBox = 3088,
    //
    // Summary:
    //     [requires: NV_viewport_array, or OES_viewport_array] Original was GL_SCISSOR_TEST
    //     = 0x0C11
    ScissorTest = 3089,
    //
    // Summary:
    //     Original was GL_INDEX_CLEAR_VALUE = 0x0C20
    IndexClearValue = 3104,
    //
    // Summary:
    //     Original was GL_INDEX_WRITEMASK = 0x0C21
    IndexWritemask = 3105,
    //
    // Summary:
    //     Original was GL_COLOR_CLEAR_VALUE = 0x0C22
    ColorClearValue = 3106,
    //
    // Summary:
    //     [requires: EXT_draw_buffers_indexed, or OES_draw_buffers_indexed] Original was
    //     GL_COLOR_WRITEMASK = 0x0C23
    ColorWritemask = 3107,
    //
    // Summary:
    //     Original was GL_INDEX_MODE = 0x0C30
    IndexMode = 3120,
    //
    // Summary:
    //     Original was GL_RGBA_MODE = 0x0C31
    RgbaMode = 3121,
    //
    // Summary:
    //     Original was GL_DOUBLEBUFFER = 0x0C32
    Doublebuffer = 3122,
    //
    // Summary:
    //     Original was GL_STEREO = 0x0C33
    Stereo = 3123,
    //
    // Summary:
    //     Original was GL_RENDER_MODE = 0x0C40
    RenderMode = 3136,
    //
    // Summary:
    //     Original was GL_PERSPECTIVE_CORRECTION_HINT = 0x0C50
    PerspectiveCorrectionHint = 3152,
    //
    // Summary:
    //     Original was GL_POINT_SMOOTH_HINT = 0x0C51
    PointSmoothHint = 3153,
    //
    // Summary:
    //     Original was GL_LINE_SMOOTH_HINT = 0x0C52
    LineSmoothHint = 3154,
    //
    // Summary:
    //     Original was GL_POLYGON_SMOOTH_HINT = 0x0C53
    PolygonSmoothHint = 3155,
    //
    // Summary:
    //     Original was GL_FOG_HINT = 0x0C54
    FogHint = 3156,
    //
    // Summary:
    //     Original was GL_TEXTURE_GEN_S = 0x0C60
    TextureGenS = 3168,
    //
    // Summary:
    //     Original was GL_TEXTURE_GEN_T = 0x0C61
    TextureGenT = 3169,
    //
    // Summary:
    //     Original was GL_TEXTURE_GEN_R = 0x0C62
    TextureGenR = 3170,
    //
    // Summary:
    //     Original was GL_TEXTURE_GEN_Q = 0x0C63
    TextureGenQ = 3171,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_I_TO_I_SIZE = 0x0CB0
    PixelMapIToISize = 3248,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_S_TO_S_SIZE = 0x0CB1
    PixelMapSToSSize = 3249,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_I_TO_R_SIZE = 0x0CB2
    PixelMapIToRSize = 3250,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_I_TO_G_SIZE = 0x0CB3
    PixelMapIToGSize = 3251,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_I_TO_B_SIZE = 0x0CB4
    PixelMapIToBSize = 3252,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_I_TO_A_SIZE = 0x0CB5
    PixelMapIToASize = 3253,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_R_TO_R_SIZE = 0x0CB6
    PixelMapRToRSize = 3254,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_G_TO_G_SIZE = 0x0CB7
    PixelMapGToGSize = 3255,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_B_TO_B_SIZE = 0x0CB8
    PixelMapBToBSize = 3256,
    //
    // Summary:
    //     Original was GL_PIXEL_MAP_A_TO_A_SIZE = 0x0CB9
    PixelMapAToASize = 3257,
    //
    // Summary:
    //     Original was GL_UNPACK_SWAP_BYTES = 0x0CF0
    UnpackSwapBytes = 3312,
    //
    // Summary:
    //     Original was GL_UNPACK_LSB_FIRST = 0x0CF1
    UnpackLsbFirst = 3313,
    //
    // Summary:
    //     Original was GL_UNPACK_ROW_LENGTH = 0x0CF2
    UnpackRowLength = 3314,
    //
    // Summary:
    //     Original was GL_UNPACK_SKIP_ROWS = 0x0CF3
    UnpackSkipRows = 3315,
    //
    // Summary:
    //     Original was GL_UNPACK_SKIP_PIXELS = 0x0CF4
    UnpackSkipPixels = 3316,
    //
    // Summary:
    //     Original was GL_UNPACK_ALIGNMENT = 0x0CF5
    UnpackAlignment = 3317,
    //
    // Summary:
    //     Original was GL_PACK_SWAP_BYTES = 0x0D00
    PackSwapBytes = 3328,
    //
    // Summary:
    //     Original was GL_PACK_LSB_FIRST = 0x0D01
    PackLsbFirst = 3329,
    //
    // Summary:
    //     Original was GL_PACK_ROW_LENGTH = 0x0D02
    PackRowLength = 3330,
    //
    // Summary:
    //     Original was GL_PACK_SKIP_ROWS = 0x0D03
    PackSkipRows = 3331,
    //
    // Summary:
    //     Original was GL_PACK_SKIP_PIXELS = 0x0D04
    PackSkipPixels = 3332,
    //
    // Summary:
    //     Original was GL_PACK_ALIGNMENT = 0x0D05
    PackAlignment = 3333,
    //
    // Summary:
    //     Original was GL_MAP_COLOR = 0x0D10
    MapColor = 3344,
    //
    // Summary:
    //     Original was GL_MAP_STENCIL = 0x0D11
    MapStencil = 3345,
    //
    // Summary:
    //     Original was GL_INDEX_SHIFT = 0x0D12
    IndexShift = 3346,
    //
    // Summary:
    //     Original was GL_INDEX_OFFSET = 0x0D13
    IndexOffset = 3347,
    //
    // Summary:
    //     Original was GL_RED_SCALE = 0x0D14
    RedScale = 3348,
    //
    // Summary:
    //     Original was GL_RED_BIAS = 0x0D15
    RedBias = 3349,
    //
    // Summary:
    //     Original was GL_ZOOM_X = 0x0D16
    ZoomX = 3350,
    //
    // Summary:
    //     Original was GL_ZOOM_Y = 0x0D17
    ZoomY = 3351,
    //
    // Summary:
    //     Original was GL_GREEN_SCALE = 0x0D18
    GreenScale = 3352,
    //
    // Summary:
    //     Original was GL_GREEN_BIAS = 0x0D19
    GreenBias = 3353,
    //
    // Summary:
    //     Original was GL_BLUE_SCALE = 0x0D1A
    BlueScale = 3354,
    //
    // Summary:
    //     Original was GL_BLUE_BIAS = 0x0D1B
    BlueBias = 3355,
    //
    // Summary:
    //     Original was GL_ALPHA_SCALE = 0x0D1C
    AlphaScale = 3356,
    //
    // Summary:
    //     Original was GL_ALPHA_BIAS = 0x0D1D
    AlphaBias = 3357,
    //
    // Summary:
    //     Original was GL_DEPTH_SCALE = 0x0D1E
    DepthScale = 3358,
    //
    // Summary:
    //     Original was GL_DEPTH_BIAS = 0x0D1F
    DepthBias = 3359,
    //
    // Summary:
    //     Original was GL_MAX_EVAL_ORDER = 0x0D30
    MaxEvalOrder = 3376,
    //
    // Summary:
    //     Original was GL_MAX_LIGHTS = 0x0D31
    MaxLights = 3377,
    //
    // Summary:
    //     Original was GL_MAX_CLIP_DISTANCES = 0x0D32
    MaxClipDistances = 3378,
    //
    // Summary:
    //     Original was GL_MAX_TEXTURE_SIZE = 0x0D33
    MaxTextureSize = 3379,
    //
    // Summary:
    //     Original was GL_MAX_PIXEL_MAP_TABLE = 0x0D34
    MaxPixelMapTable = 3380,
    //
    // Summary:
    //     Original was GL_MAX_ATTRIB_STACK_DEPTH = 0x0D35
    MaxAttribStackDepth = 3381,
    //
    // Summary:
    //     Original was GL_MAX_MODELVIEW_STACK_DEPTH = 0x0D36
    MaxModelviewStackDepth = 3382,
    //
    // Summary:
    //     Original was GL_MAX_NAME_STACK_DEPTH = 0x0D37
    MaxNameStackDepth = 3383,
    //
    // Summary:
    //     Original was GL_MAX_PROJECTION_STACK_DEPTH = 0x0D38
    MaxProjectionStackDepth = 3384,
    //
    // Summary:
    //     Original was GL_MAX_TEXTURE_STACK_DEPTH = 0x0D39
    MaxTextureStackDepth = 3385,
    //
    // Summary:
    //     Original was GL_MAX_VIEWPORT_DIMS = 0x0D3A
    MaxViewportDims = 3386,
    //
    // Summary:
    //     Original was GL_MAX_CLIENT_ATTRIB_STACK_DEPTH = 0x0D3B
    MaxClientAttribStackDepth = 3387,
    //
    // Summary:
    //     Original was GL_SUBPIXEL_BITS = 0x0D50
    SubpixelBits = 3408,
    //
    // Summary:
    //     Original was GL_INDEX_BITS = 0x0D51
    IndexBits = 3409,
    //
    // Summary:
    //     Original was GL_RED_BITS = 0x0D52
    RedBits = 3410,
    //
    // Summary:
    //     Original was GL_GREEN_BITS = 0x0D53
    GreenBits = 3411,
    //
    // Summary:
    //     Original was GL_BLUE_BITS = 0x0D54
    BlueBits = 3412,
    //
    // Summary:
    //     Original was GL_ALPHA_BITS = 0x0D55
    AlphaBits = 3413,
    //
    // Summary:
    //     Original was GL_DEPTH_BITS = 0x0D56
    DepthBits = 3414,
    //
    // Summary:
    //     Original was GL_STENCIL_BITS = 0x0D57
    StencilBits = 3415,
    //
    // Summary:
    //     Original was GL_ACCUM_RED_BITS = 0x0D58
    AccumRedBits = 3416,
    //
    // Summary:
    //     Original was GL_ACCUM_GREEN_BITS = 0x0D59
    AccumGreenBits = 3417,
    //
    // Summary:
    //     Original was GL_ACCUM_BLUE_BITS = 0x0D5A
    AccumBlueBits = 3418,
    //
    // Summary:
    //     Original was GL_ACCUM_ALPHA_BITS = 0x0D5B
    AccumAlphaBits = 3419,
    //
    // Summary:
    //     Original was GL_NAME_STACK_DEPTH = 0x0D70
    NameStackDepth = 3440,
    //
    // Summary:
    //     Original was GL_AUTO_NORMAL = 0x0D80
    AutoNormal = 3456,
    //
    // Summary:
    //     Original was GL_MAP1_COLOR_4 = 0x0D90
    Map1Color4 = 3472,
    //
    // Summary:
    //     Original was GL_MAP1_INDEX = 0x0D91
    Map1Index = 3473,
    //
    // Summary:
    //     Original was GL_MAP1_NORMAL = 0x0D92
    Map1Normal = 3474,
    //
    // Summary:
    //     Original was GL_MAP1_TEXTURE_COORD_1 = 0x0D93
    Map1TextureCoord1 = 3475,
    //
    // Summary:
    //     Original was GL_MAP1_TEXTURE_COORD_2 = 0x0D94
    Map1TextureCoord2 = 3476,
    //
    // Summary:
    //     Original was GL_MAP1_TEXTURE_COORD_3 = 0x0D95
    Map1TextureCoord3 = 3477,
    //
    // Summary:
    //     Original was GL_MAP1_TEXTURE_COORD_4 = 0x0D96
    Map1TextureCoord4 = 3478,
    //
    // Summary:
    //     Original was GL_MAP1_VERTEX_3 = 0x0D97
    Map1Vertex3 = 3479,
    //
    // Summary:
    //     Original was GL_MAP1_VERTEX_4 = 0x0D98
    Map1Vertex4 = 3480,
    //
    // Summary:
    //     Original was GL_MAP2_COLOR_4 = 0x0DB0
    Map2Color4 = 3504,
    //
    // Summary:
    //     Original was GL_MAP2_INDEX = 0x0DB1
    Map2Index = 3505,
    //
    // Summary:
    //     Original was GL_MAP2_NORMAL = 0x0DB2
    Map2Normal = 3506,
    //
    // Summary:
    //     Original was GL_MAP2_TEXTURE_COORD_1 = 0x0DB3
    Map2TextureCoord1 = 3507,
    //
    // Summary:
    //     Original was GL_MAP2_TEXTURE_COORD_2 = 0x0DB4
    Map2TextureCoord2 = 3508,
    //
    // Summary:
    //     Original was GL_MAP2_TEXTURE_COORD_3 = 0x0DB5
    Map2TextureCoord3 = 3509,
    //
    // Summary:
    //     Original was GL_MAP2_TEXTURE_COORD_4 = 0x0DB6
    Map2TextureCoord4 = 3510,
    //
    // Summary:
    //     Original was GL_MAP2_VERTEX_3 = 0x0DB7
    Map2Vertex3 = 3511,
    //
    // Summary:
    //     Original was GL_MAP2_VERTEX_4 = 0x0DB8
    Map2Vertex4 = 3512,
    //
    // Summary:
    //     Original was GL_MAP1_GRID_DOMAIN = 0x0DD0
    Map1GridDomain = 3536,
    //
    // Summary:
    //     Original was GL_MAP1_GRID_SEGMENTS = 0x0DD1
    Map1GridSegments = 3537,
    //
    // Summary:
    //     Original was GL_MAP2_GRID_DOMAIN = 0x0DD2
    Map2GridDomain = 3538,
    //
    // Summary:
    //     Original was GL_MAP2_GRID_SEGMENTS = 0x0DD3
    Map2GridSegments = 3539,
    //
    // Summary:
    //     Original was GL_TEXTURE_1D = 0x0DE0
    Texture1D = 3552,
    //
    // Summary:
    //     [requires: EXT_sparse_texture] Original was GL_TEXTURE_2D = 0x0DE1
    Texture2D = 3553,
    //
    // Summary:
    //     Original was GL_FEEDBACK_BUFFER_SIZE = 0x0DF1
    FeedbackBufferSize = 3569,
    //
    // Summary:
    //     Original was GL_FEEDBACK_BUFFER_TYPE = 0x0DF2
    FeedbackBufferType = 3570,
    //
    // Summary:
    //     Original was GL_SELECTION_BUFFER_SIZE = 0x0DF4
    SelectionBufferSize = 3572,
    //
    // Summary:
    //     Original was GL_POLYGON_OFFSET_UNITS = 0x2A00
    PolygonOffsetUnits = 10752,
    //
    // Summary:
    //     Original was GL_POLYGON_OFFSET_POINT = 0x2A01
    PolygonOffsetPoint = 10753,
    //
    // Summary:
    //     Original was GL_POLYGON_OFFSET_LINE = 0x2A02
    PolygonOffsetLine = 10754,
    //
    // Summary:
    //     Original was GL_CLIP_PLANE0 = 0x3000
    ClipPlane0 = 12288,
    //
    // Summary:
    //     Original was GL_CLIP_PLANE1 = 0x3001
    ClipPlane1 = 12289,
    //
    // Summary:
    //     Original was GL_CLIP_PLANE2 = 0x3002
    ClipPlane2 = 12290,
    //
    // Summary:
    //     Original was GL_CLIP_PLANE3 = 0x3003
    ClipPlane3 = 12291,
    //
    // Summary:
    //     Original was GL_CLIP_PLANE4 = 0x3004
    ClipPlane4 = 12292,
    //
    // Summary:
    //     Original was GL_CLIP_PLANE5 = 0x3005
    ClipPlane5 = 12293,
    //
    // Summary:
    //     Original was GL_LIGHT0 = 0x4000
    Light0 = 16384,
    //
    // Summary:
    //     Original was GL_LIGHT1 = 0x4001
    Light1 = 16385,
    //
    // Summary:
    //     Original was GL_LIGHT2 = 0x4002
    Light2 = 16386,
    //
    // Summary:
    //     Original was GL_LIGHT3 = 0x4003
    Light3 = 16387,
    //
    // Summary:
    //     Original was GL_LIGHT4 = 0x4004
    Light4 = 16388,
    //
    // Summary:
    //     Original was GL_LIGHT5 = 0x4005
    Light5 = 16389,
    //
    // Summary:
    //     Original was GL_LIGHT6 = 0x4006
    Light6 = 16390,
    //
    // Summary:
    //     Original was GL_LIGHT7 = 0x4007
    Light7 = 16391,
    //
    // Summary:
    //     Original was GL_BlendColor = 0X8005
    BlendColor = 32773,
    //
    // Summary:
    //     Original was GL_BlendEquation = 0X8009
    BlendEquation = 32777,
    //
    // Summary:
    //     Original was GL_PACK_CMYK_HINT_EXT = 0x800E
    PackCmykHintExt = 32782,
    //
    // Summary:
    //     Original was GL_UNPACK_CMYK_HINT_EXT = 0x800F
    UnpackCmykHintExt = 32783,
    //
    // Summary:
    //     Original was GL_CONVOLUTION_1D_EXT = 0x8010
    Convolution1DExt = 32784,
    //
    // Summary:
    //     Original was GL_CONVOLUTION_2D_EXT = 0x8011
    Convolution2DExt = 32785,
    //
    // Summary:
    //     Original was GL_SEPARABLE_2D_EXT = 0x8012
    Separable2DExt = 32786,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_RED_SCALE_EXT = 0x801C
    PostConvolutionRedScaleExt = 32796,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_GREEN_SCALE_EXT = 0x801D
    PostConvolutionGreenScaleExt = 32797,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_BLUE_SCALE_EXT = 0x801E
    PostConvolutionBlueScaleExt = 32798,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_ALPHA_SCALE_EXT = 0x801F
    PostConvolutionAlphaScaleExt = 32799,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_RED_BIAS_EXT = 0x8020
    PostConvolutionRedBiasExt = 32800,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_GREEN_BIAS_EXT = 0x8021
    PostConvolutionGreenBiasExt = 32801,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_BLUE_BIAS_EXT = 0x8022
    PostConvolutionBlueBiasExt = 32802,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_ALPHA_BIAS_EXT = 0x8023
    PostConvolutionAlphaBiasExt = 32803,
    //
    // Summary:
    //     Original was GL_HISTOGRAM_EXT = 0x8024
    HistogramExt = 32804,
    //
    // Summary:
    //     Original was GL_MINMAX_EXT = 0x802E
    MinmaxExt = 32814,
    //
    // Summary:
    //     Original was GL_POLYGON_OFFSET_FILL = 0x8037
    PolygonOffsetFill = 32823,
    //
    // Summary:
    //     Original was GL_POLYGON_OFFSET_FACTOR = 0x8038
    PolygonOffsetFactor = 32824,
    //
    // Summary:
    //     Original was GL_POLYGON_OFFSET_BIAS_EXT = 0x8039
    PolygonOffsetBiasExt = 32825,
    //
    // Summary:
    //     Original was GL_RESCALE_NORMAL_EXT = 0x803A
    RescaleNormalExt = 32826,
    //
    // Summary:
    //     Original was GL_TEXTURE_BINDING_1D = 0x8068
    TextureBinding1D = 32872,
    //
    // Summary:
    //     Original was GL_TEXTURE_BINDING_2D = 0x8069
    TextureBinding2D = 32873,
    //
    // Summary:
    //     Original was GL_TEXTURE_BINDING_3D = 0x806A
    TextureBinding3D = 32874,
    //
    // Summary:
    //     Original was GL_PACK_SKIP_IMAGES_EXT = 0x806B
    PackSkipImagesExt = 32875,
    //
    // Summary:
    //     Original was GL_PACK_IMAGE_HEIGHT_EXT = 0x806C
    PackImageHeightExt = 32876,
    //
    // Summary:
    //     Original was GL_UNPACK_SKIP_IMAGES_EXT = 0x806D
    UnpackSkipImagesExt = 32877,
    //
    // Summary:
    //     Original was GL_UNPACK_IMAGE_HEIGHT_EXT = 0x806E
    UnpackImageHeightExt = 32878,
    //
    // Summary:
    //     Original was GL_TEXTURE_3D_EXT = 0x806F
    Texture3DExt = 32879,
    //
    // Summary:
    //     Original was GL_MAX_3D_TEXTURE_SIZE_EXT = 0x8073
    Max3DTextureSizeExt = 32883,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_VERTEX_ARRAY = 0x8074
    VertexArray = 32884,
    //
    // Summary:
    //     Original was GL_NORMAL_ARRAY = 0x8075
    NormalArray = 32885,
    //
    // Summary:
    //     Original was GL_COLOR_ARRAY = 0x8076
    ColorArray = 32886,
    //
    // Summary:
    //     Original was GL_INDEX_ARRAY = 0x8077
    IndexArray = 32887,
    //
    // Summary:
    //     Original was GL_TEXTURE_COORD_ARRAY = 0x8078
    TextureCoordArray = 32888,
    //
    // Summary:
    //     Original was GL_EDGE_FLAG_ARRAY = 0x8079
    EdgeFlagArray = 32889,
    //
    // Summary:
    //     Original was GL_VERTEX_ARRAY_SIZE = 0x807A
    VertexArraySize = 32890,
    //
    // Summary:
    //     Original was GL_VERTEX_ARRAY_TYPE = 0x807B
    VertexArrayType = 32891,
    //
    // Summary:
    //     Original was GL_VERTEX_ARRAY_STRIDE = 0x807C
    VertexArrayStride = 32892,
    //
    // Summary:
    //     Original was GL_VERTEX_ARRAY_COUNT_EXT = 0x807D
    VertexArrayCountExt = 32893,
    //
    // Summary:
    //     Original was GL_NORMAL_ARRAY_TYPE = 0x807E
    NormalArrayType = 32894,
    //
    // Summary:
    //     Original was GL_NORMAL_ARRAY_STRIDE = 0x807F
    NormalArrayStride = 32895,
    //
    // Summary:
    //     Original was GL_NORMAL_ARRAY_COUNT_EXT = 0x8080
    NormalArrayCountExt = 32896,
    //
    // Summary:
    //     Original was GL_COLOR_ARRAY_SIZE = 0x8081
    ColorArraySize = 32897,
    //
    // Summary:
    //     Original was GL_COLOR_ARRAY_TYPE = 0x8082
    ColorArrayType = 32898,
    //
    // Summary:
    //     Original was GL_COLOR_ARRAY_STRIDE = 0x8083
    ColorArrayStride = 32899,
    //
    // Summary:
    //     Original was GL_COLOR_ARRAY_COUNT_EXT = 0x8084
    ColorArrayCountExt = 32900,
    //
    // Summary:
    //     Original was GL_INDEX_ARRAY_TYPE = 0x8085
    IndexArrayType = 32901,
    //
    // Summary:
    //     Original was GL_INDEX_ARRAY_STRIDE = 0x8086
    IndexArrayStride = 32902,
    //
    // Summary:
    //     Original was GL_INDEX_ARRAY_COUNT_EXT = 0x8087
    IndexArrayCountExt = 32903,
    //
    // Summary:
    //     Original was GL_TEXTURE_COORD_ARRAY_SIZE = 0x8088
    TextureCoordArraySize = 32904,
    //
    // Summary:
    //     Original was GL_TEXTURE_COORD_ARRAY_TYPE = 0x8089
    TextureCoordArrayType = 32905,
    //
    // Summary:
    //     Original was GL_TEXTURE_COORD_ARRAY_STRIDE = 0x808A
    TextureCoordArrayStride = 32906,
    //
    // Summary:
    //     Original was GL_TEXTURE_COORD_ARRAY_COUNT_EXT = 0x808B
    TextureCoordArrayCountExt = 32907,
    //
    // Summary:
    //     Original was GL_EDGE_FLAG_ARRAY_STRIDE = 0x808C
    EdgeFlagArrayStride = 32908,
    //
    // Summary:
    //     Original was GL_EDGE_FLAG_ARRAY_COUNT_EXT = 0x808D
    EdgeFlagArrayCountExt = 32909,
    //
    // Summary:
    //     Original was GL_INTERLACE_SGIX = 0x8094
    InterlaceSgix = 32916,
    //
    // Summary:
    //     Original was GL_DETAIL_TEXTURE_2D_BINDING_SGIS = 0x8096
    DetailTexture2DBindingSgis = 32918,
    //
    // Summary:
    //     Original was GL_MULTISAMPLE_SGIS = 0x809D
    MultisampleSgis = 32925,
    //
    // Summary:
    //     Original was GL_SampleAlphaToCoverage = 0X809e
    SampleAlphaToCoverage = 32926,
    //
    // Summary:
    //     Original was GL_SAMPLE_ALPHA_TO_ONE_SGIS = 0x809F
    SampleAlphaToOneSgis = 32927,
    //
    // Summary:
    //     Original was GL_SampleCoverage = 0X80a0
    SampleCoverage = 32928,
    //
    // Summary:
    //     Original was GL_SampleBuffers = 0X80a8
    SampleBuffers = 32936,
    //
    // Summary:
    //     Original was GL_Samples = 0X80a9
    Samples = 32937,
    //
    // Summary:
    //     Original was GL_SampleCoverageValue = 0X80aa
    SampleCoverageValue = 32938,
    //
    // Summary:
    //     Original was GL_SampleCoverageInvert = 0X80ab
    SampleCoverageInvert = 32939,
    //
    // Summary:
    //     Original was GL_SAMPLE_PATTERN_SGIS = 0x80AC
    SamplePatternSgis = 32940,
    //
    // Summary:
    //     Original was GL_COLOR_MATRIX_SGI = 0x80B1
    ColorMatrixSgi = 32945,
    //
    // Summary:
    //     Original was GL_COLOR_MATRIX_STACK_DEPTH_SGI = 0x80B2
    ColorMatrixStackDepthSgi = 32946,
    //
    // Summary:
    //     Original was GL_MAX_COLOR_MATRIX_STACK_DEPTH_SGI = 0x80B3
    MaxColorMatrixStackDepthSgi = 32947,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_RED_SCALE_SGI = 0x80B4
    PostColorMatrixRedScaleSgi = 32948,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_GREEN_SCALE_SGI = 0x80B5
    PostColorMatrixGreenScaleSgi = 32949,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_BLUE_SCALE_SGI = 0x80B6
    PostColorMatrixBlueScaleSgi = 32950,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_ALPHA_SCALE_SGI = 0x80B7
    PostColorMatrixAlphaScaleSgi = 32951,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_RED_BIAS_SGI = 0x80B8
    PostColorMatrixRedBiasSgi = 32952,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_GREEN_BIAS_SGI = 0x80B9
    PostColorMatrixGreenBiasSgi = 32953,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_BLUE_BIAS_SGI = 0x80BA
    PostColorMatrixBlueBiasSgi = 32954,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_ALPHA_BIAS_SGI = 0x80BB
    PostColorMatrixAlphaBiasSgi = 32955,
    //
    // Summary:
    //     Original was GL_TEXTURE_COLOR_TABLE_SGI = 0x80BC
    TextureColorTableSgi = 32956,
    //
    // Summary:
    //     Original was GL_BlendDstRgb = 0X80c8
    BlendDstRgb = 32968,
    //
    // Summary:
    //     Original was GL_BlendSrcRgb = 0X80c9
    BlendSrcRgb = 32969,
    //
    // Summary:
    //     Original was GL_BlendDstAlpha = 0X80ca
    BlendDstAlpha = 32970,
    //
    // Summary:
    //     Original was GL_BlendSrcAlpha = 0X80cb
    BlendSrcAlpha = 32971,
    //
    // Summary:
    //     Original was GL_COLOR_TABLE_SGI = 0x80D0
    ColorTableSgi = 32976,
    //
    // Summary:
    //     Original was GL_POST_CONVOLUTION_COLOR_TABLE_SGI = 0x80D1
    PostConvolutionColorTableSgi = 32977,
    //
    // Summary:
    //     Original was GL_POST_COLOR_MATRIX_COLOR_TABLE_SGI = 0x80D2
    PostColorMatrixColorTableSgi = 32978,
    //
    // Summary:
    //     Original was GL_POINT_SIZE_MIN_SGIS = 0x8126
    PointSizeMinSgis = 33062,
    //
    // Summary:
    //     Original was GL_POINT_SIZE_MAX_SGIS = 0x8127
    PointSizeMaxSgis = 33063,
    //
    // Summary:
    //     Original was GL_POINT_FADE_THRESHOLD_SIZE_SGIS = 0x8128
    PointFadeThresholdSizeSgis = 33064,
    //
    // Summary:
    //     Original was GL_DISTANCE_ATTENUATION_SGIS = 0x8129
    DistanceAttenuationSgis = 33065,
    //
    // Summary:
    //     Original was GL_FOG_FUNC_POINTS_SGIS = 0x812B
    FogFuncPointsSgis = 33067,
    //
    // Summary:
    //     Original was GL_MAX_FOG_FUNC_POINTS_SGIS = 0x812C
    MaxFogFuncPointsSgis = 33068,
    //
    // Summary:
    //     Original was GL_PACK_SKIP_VOLUMES_SGIS = 0x8130
    PackSkipVolumesSgis = 33072,
    //
    // Summary:
    //     Original was GL_PACK_IMAGE_DEPTH_SGIS = 0x8131
    PackImageDepthSgis = 33073,
    //
    // Summary:
    //     Original was GL_UNPACK_SKIP_VOLUMES_SGIS = 0x8132
    UnpackSkipVolumesSgis = 33074,
    //
    // Summary:
    //     Original was GL_UNPACK_IMAGE_DEPTH_SGIS = 0x8133
    UnpackImageDepthSgis = 33075,
    //
    // Summary:
    //     Original was GL_TEXTURE_4D_SGIS = 0x8134
    Texture4DSgis = 33076,
    //
    // Summary:
    //     Original was GL_MAX_4D_TEXTURE_SIZE_SGIS = 0x8138
    Max4DTextureSizeSgis = 33080,
    //
    // Summary:
    //     Original was GL_PIXEL_TEX_GEN_SGIX = 0x8139
    PixelTexGenSgix = 33081,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_BEST_ALIGNMENT_SGIX = 0x813E
    PixelTileBestAlignmentSgix = 33086,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_CACHE_INCREMENT_SGIX = 0x813F
    PixelTileCacheIncrementSgix = 33087,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_WIDTH_SGIX = 0x8140
    PixelTileWidthSgix = 33088,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_HEIGHT_SGIX = 0x8141
    PixelTileHeightSgix = 33089,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_GRID_WIDTH_SGIX = 0x8142
    PixelTileGridWidthSgix = 33090,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_GRID_HEIGHT_SGIX = 0x8143
    PixelTileGridHeightSgix = 33091,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_GRID_DEPTH_SGIX = 0x8144
    PixelTileGridDepthSgix = 33092,
    //
    // Summary:
    //     Original was GL_PIXEL_TILE_CACHE_SIZE_SGIX = 0x8145
    PixelTileCacheSizeSgix = 33093,
    //
    // Summary:
    //     Original was GL_SPRITE_SGIX = 0x8148
    SpriteSgix = 33096,
    //
    // Summary:
    //     Original was GL_SPRITE_MODE_SGIX = 0x8149
    SpriteModeSgix = 33097,
    //
    // Summary:
    //     Original was GL_SPRITE_AXIS_SGIX = 0x814A
    SpriteAxisSgix = 33098,
    //
    // Summary:
    //     Original was GL_SPRITE_TRANSLATION_SGIX = 0x814B
    SpriteTranslationSgix = 33099,
    //
    // Summary:
    //     Original was GL_TEXTURE_4D_BINDING_SGIS = 0x814F
    Texture4DBindingSgis = 33103,
    //
    // Summary:
    //     Original was GL_MAX_CLIPMAP_DEPTH_SGIX = 0x8177
    MaxClipmapDepthSgix = 33143,
    //
    // Summary:
    //     Original was GL_MAX_CLIPMAP_VIRTUAL_DEPTH_SGIX = 0x8178
    MaxClipmapVirtualDepthSgix = 33144,
    //
    // Summary:
    //     Original was GL_POST_TEXTURE_FILTER_BIAS_RANGE_SGIX = 0x817B
    PostTextureFilterBiasRangeSgix = 33147,
    //
    // Summary:
    //     Original was GL_POST_TEXTURE_FILTER_SCALE_RANGE_SGIX = 0x817C
    PostTextureFilterScaleRangeSgix = 33148,
    //
    // Summary:
    //     Original was GL_REFERENCE_PLANE_SGIX = 0x817D
    ReferencePlaneSgix = 33149,
    //
    // Summary:
    //     Original was GL_REFERENCE_PLANE_EQUATION_SGIX = 0x817E
    ReferencePlaneEquationSgix = 33150,
    //
    // Summary:
    //     Original was GL_IR_INSTRUMENT1_SGIX = 0x817F
    IrInstrument1Sgix = 33151,
    //
    // Summary:
    //     Original was GL_INSTRUMENT_MEASUREMENTS_SGIX = 0x8181
    InstrumentMeasurementsSgix = 33153,
    //
    // Summary:
    //     Original was GL_CALLIGRAPHIC_FRAGMENT_SGIX = 0x8183
    CalligraphicFragmentSgix = 33155,
    //
    // Summary:
    //     Original was GL_FRAMEZOOM_SGIX = 0x818B
    FramezoomSgix = 33163,
    //
    // Summary:
    //     Original was GL_FRAMEZOOM_FACTOR_SGIX = 0x818C
    FramezoomFactorSgix = 33164,
    //
    // Summary:
    //     Original was GL_MAX_FRAMEZOOM_FACTOR_SGIX = 0x818D
    MaxFramezoomFactorSgix = 33165,
    //
    // Summary:
    //     Original was GL_GenerateMipmapHint = 0X8192
    GenerateMipmapHint = 33170,
    //
    // Summary:
    //     Original was GL_DEFORMATIONS_MASK_SGIX = 0x8196
    DeformationsMaskSgix = 33174,
    //
    // Summary:
    //     Original was GL_FOG_OFFSET_SGIX = 0x8198
    FogOffsetSgix = 33176,
    //
    // Summary:
    //     Original was GL_FOG_OFFSET_VALUE_SGIX = 0x8199
    FogOffsetValueSgix = 33177,
    //
    // Summary:
    //     Original was GL_LIGHT_MODEL_COLOR_CONTROL = 0x81F8
    LightModelColorControl = 33272,
    //
    // Summary:
    //     Original was GL_SHARED_TEXTURE_PALETTE_EXT = 0x81FB
    SharedTexturePaletteExt = 33275,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH = 0x8243
    DebugNextLoggedMessageLength = 33347,
    //
    // Summary:
    //     [requires: KHR_robustness] Original was GL_RESET_NOTIFICATION_STRATEGY = 0x8256
    ResetNotificationStrategy = 33366,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_MAX_DEBUG_GROUP_STACK_DEPTH = 0x826C
    MaxDebugGroupStackDepth = 33388,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_DEBUG_GROUP_STACK_DEPTH = 0x826D
    DebugGroupStackDepth = 33389,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_MAX_LABEL_LENGTH = 0x82E8
    MaxLabelLength = 33512,
    //
    // Summary:
    //     [requires: KHR_context_flush_control] Original was GL_CONTEXT_RELEASE_BEHAVIOR_KHR
    //     = 0x82FB
    ContextReleaseBehaviorKhr = 33531,
    //
    // Summary:
    //     Original was GL_CONVOLUTION_HINT_SGIX = 0x8316
    ConvolutionHintSgix = 33558,
    //
    // Summary:
    //     Original was GL_ASYNC_MARKER_SGIX = 0x8329
    AsyncMarkerSgix = 33577,
    //
    // Summary:
    //     Original was GL_PIXEL_TEX_GEN_MODE_SGIX = 0x832B
    PixelTexGenModeSgix = 33579,
    //
    // Summary:
    //     Original was GL_ASYNC_HISTOGRAM_SGIX = 0x832C
    AsyncHistogramSgix = 33580,
    //
    // Summary:
    //     Original was GL_MAX_ASYNC_HISTOGRAM_SGIX = 0x832D
    MaxAsyncHistogramSgix = 33581,
    //
    // Summary:
    //     Original was GL_PIXEL_TEXTURE_SGIS = 0x8353
    PixelTextureSgis = 33619,
    //
    // Summary:
    //     Original was GL_ASYNC_TEX_IMAGE_SGIX = 0x835C
    AsyncTexImageSgix = 33628,
    //
    // Summary:
    //     Original was GL_ASYNC_DRAW_PIXELS_SGIX = 0x835D
    AsyncDrawPixelsSgix = 33629,
    //
    // Summary:
    //     Original was GL_ASYNC_READ_PIXELS_SGIX = 0x835E
    AsyncReadPixelsSgix = 33630,
    //
    // Summary:
    //     Original was GL_MAX_ASYNC_TEX_IMAGE_SGIX = 0x835F
    MaxAsyncTexImageSgix = 33631,
    //
    // Summary:
    //     Original was GL_MAX_ASYNC_DRAW_PIXELS_SGIX = 0x8360
    MaxAsyncDrawPixelsSgix = 33632,
    //
    // Summary:
    //     Original was GL_MAX_ASYNC_READ_PIXELS_SGIX = 0x8361
    MaxAsyncReadPixelsSgix = 33633,
    //
    // Summary:
    //     Original was GL_VERTEX_PRECLIP_SGIX = 0x83EE
    VertexPreclipSgix = 33774,
    //
    // Summary:
    //     Original was GL_VERTEX_PRECLIP_HINT_SGIX = 0x83EF
    VertexPreclipHintSgix = 33775,
    //
    // Summary:
    //     Original was GL_FRAGMENT_LIGHTING_SGIX = 0x8400
    FragmentLightingSgix = 33792,
    //
    // Summary:
    //     Original was GL_FRAGMENT_COLOR_MATERIAL_SGIX = 0x8401
    FragmentColorMaterialSgix = 33793,
    //
    // Summary:
    //     Original was GL_FRAGMENT_COLOR_MATERIAL_FACE_SGIX = 0x8402
    FragmentColorMaterialFaceSgix = 33794,
    //
    // Summary:
    //     Original was GL_FRAGMENT_COLOR_MATERIAL_PARAMETER_SGIX = 0x8403
    FragmentColorMaterialParameterSgix = 33795,
    //
    // Summary:
    //     Original was GL_MAX_FRAGMENT_LIGHTS_SGIX = 0x8404
    MaxFragmentLightsSgix = 33796,
    //
    // Summary:
    //     Original was GL_MAX_ACTIVE_LIGHTS_SGIX = 0x8405
    MaxActiveLightsSgix = 33797,
    //
    // Summary:
    //     Original was GL_LIGHT_ENV_MODE_SGIX = 0x8407
    LightEnvModeSgix = 33799,
    //
    // Summary:
    //     Original was GL_FRAGMENT_LIGHT_MODEL_LOCAL_VIEWER_SGIX = 0x8408
    FragmentLightModelLocalViewerSgix = 33800,
    //
    // Summary:
    //     Original was GL_FRAGMENT_LIGHT_MODEL_TWO_SIDE_SGIX = 0x8409
    FragmentLightModelTwoSideSgix = 33801,
    //
    // Summary:
    //     Original was GL_FRAGMENT_LIGHT_MODEL_AMBIENT_SGIX = 0x840A
    FragmentLightModelAmbientSgix = 33802,
    //
    // Summary:
    //     Original was GL_FRAGMENT_LIGHT_MODEL_NORMAL_INTERPOLATION_SGIX = 0x840B
    FragmentLightModelNormalInterpolationSgix = 33803,
    //
    // Summary:
    //     Original was GL_FRAGMENT_LIGHT0_SGIX = 0x840C
    FragmentLight0Sgix = 33804,
    //
    // Summary:
    //     Original was GL_PACK_RESAMPLE_SGIX = 0x842E
    PackResampleSgix = 33838,
    //
    // Summary:
    //     Original was GL_UNPACK_RESAMPLE_SGIX = 0x842F
    UnpackResampleSgix = 33839,
    //
    // Summary:
    //     Original was GL_ALIASED_POINT_SIZE_RANGE = 0x846D
    AliasedPointSizeRange = 33901,
    //
    // Summary:
    //     Original was GL_ALIASED_LINE_WIDTH_RANGE = 0x846E
    AliasedLineWidthRange = 33902,
    //
    // Summary:
    //     Original was GL_ActiveTexture = 0X84e0
    ActiveTexture = 34016,
    //
    // Summary:
    //     Original was GL_MaxRenderbufferSize = 0X84e8
    MaxRenderbufferSize = 34024,
    //
    // Summary:
    //     Original was GL_TextureBindingCubeMap = 0X8514
    TextureBindingCubeMap = 34068,
    //
    // Summary:
    //     Original was GL_MaxCubeMapTextureSize = 0X851c
    MaxCubeMapTextureSize = 34076,
    //
    // Summary:
    //     Original was GL_PACK_SUBSAMPLE_RATE_SGIX = 0x85A0
    PackSubsampleRateSgix = 34208,
    //
    // Summary:
    //     Original was GL_UNPACK_SUBSAMPLE_RATE_SGIX = 0x85A1
    UnpackSubsampleRateSgix = 34209,
    //
    // Summary:
    //     Original was GL_NumCompressedTextureFormats = 0X86a2
    NumCompressedTextureFormats = 34466,
    //
    // Summary:
    //     Original was GL_CompressedTextureFormats = 0X86a3
    CompressedTextureFormats = 34467,
    //
    // Summary:
    //     Original was GL_StencilBackFunc = 0X8800
    StencilBackFunc = 34816,
    //
    // Summary:
    //     Original was GL_StencilBackFail = 0X8801
    StencilBackFail = 34817,
    //
    // Summary:
    //     Original was GL_StencilBackPassDepthFail = 0X8802
    StencilBackPassDepthFail = 34818,
    //
    // Summary:
    //     Original was GL_StencilBackPassDepthPass = 0X8803
    StencilBackPassDepthPass = 34819,
    //
    // Summary:
    //     Original was GL_BlendEquationAlpha = 0X883d
    BlendEquationAlpha = 34877,
    //
    // Summary:
    //     Original was GL_MaxVertexAttribs = 0X8869
    MaxVertexAttribs = 34921,
    //
    // Summary:
    //     Original was GL_MaxTextureImageUnits = 0X8872
    MaxTextureImageUnits = 34930,
    //
    // Summary:
    //     Original was GL_ArrayBufferBinding = 0X8894
    ArrayBufferBinding = 34964,
    //
    // Summary:
    //     Original was GL_ElementArrayBufferBinding = 0X8895
    ElementArrayBufferBinding = 34965,
    //
    // Summary:
    //     Original was GL_MaxVertexTextureImageUnits = 0X8b4c
    MaxVertexTextureImageUnits = 35660,
    //
    // Summary:
    //     Original was GL_MaxCombinedTextureImageUnits = 0X8b4d
    MaxCombinedTextureImageUnits = 35661,
    //
    // Summary:
    //     Original was GL_CurrentProgram = 0X8b8d
    CurrentProgram = 35725,
    //
    // Summary:
    //     Original was GL_ImplementationColorReadType = 0X8b9a
    ImplementationColorReadType = 35738,
    //
    // Summary:
    //     Original was GL_ImplementationColorReadFormat = 0X8b9b
    ImplementationColorReadFormat = 35739,
    //
    // Summary:
    //     Original was GL_StencilBackRef = 0X8ca3
    StencilBackRef = 36003,
    //
    // Summary:
    //     Original was GL_StencilBackValueMask = 0X8ca4
    StencilBackValueMask = 36004,
    //
    // Summary:
    //     Original was GL_StencilBackWritemask = 0X8ca5
    StencilBackWritemask = 36005,
    //
    // Summary:
    //     Original was GL_FramebufferBinding = 0X8ca6
    FramebufferBinding = 36006,
    //
    // Summary:
    //     Original was GL_RenderbufferBinding = 0X8ca7
    RenderbufferBinding = 36007,
    //
    // Summary:
    //     Original was GL_ShaderBinaryFormats = 0X8df8
    ShaderBinaryFormats = 36344,
    //
    // Summary:
    //     Original was GL_NumShaderBinaryFormats = 0X8df9
    NumShaderBinaryFormats = 36345,
    //
    // Summary:
    //     Original was GL_ShaderCompiler = 0X8dfa
    ShaderCompiler = 36346,
    //
    // Summary:
    //     Original was GL_MaxVertexUniformVectors = 0X8dfb
    MaxVertexUniformVectors = 36347,
    //
    // Summary:
    //     Original was GL_MaxVaryingVectors = 0X8dfc
    MaxVaryingVectors = 36348,
    //
    // Summary:
    //     Original was GL_MaxFragmentUniformVectors = 0X8dfd
    MaxFragmentUniformVectors = 36349,
    //
    // Summary:
    //     [requires: EXT_disjoint_timer_query] Original was GL_TIMESTAMP_EXT = 0x8E28
    TimestampExt = 36392,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_FRAGMENT_SHADING_RATE_NON_TRIVIAL_COMBINERS_SUPPORTED_EXT
    //     = 0x8F6F
    FragmentShadingRateNonTrivialCombinersSupportedExt = 36719,
    //
    // Summary:
    //     [requires: EXT_disjoint_timer_query] Original was GL_GPU_DISJOINT_EXT = 0x8FBB
    GpuDisjointExt = 36795,
    //
    // Summary:
    //     [requires: EXT_multiview_draw_buffers] Original was GL_MAX_MULTIVIEW_BUFFERS_EXT
    //     = 0x90F2
    MaxMultiviewBuffersExt = 37106,
    //
    // Summary:
    //     [requires: KHR_robustness] Original was GL_CONTEXT_ROBUST_ACCESS = 0x90F3
    ContextRobustAccess = 37107,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_MAX_DEBUG_MESSAGE_LENGTH = 0x9143
    MaxDebugMessageLength = 37187,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_MAX_DEBUG_LOGGED_MESSAGES = 0x9144
    MaxDebugLoggedMessages = 37188,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_DEBUG_LOGGED_MESSAGES = 0x9145
    DebugLoggedMessages = 37189,
    //
    // Summary:
    //     [requires: NV_representative_fragment_test] Original was GL_REPRESENTATIVE_FRAGMENT_TEST_NV
    //     = 0x937F
    RepresentativeFragmentTestNv = 37759,
    //
    // Summary:
    //     Original was GL_SUBGROUP_SIZE_KHR = 0x9532
    SubgroupSizeKhr = 38194,
    //
    // Summary:
    //     Original was GL_SUBGROUP_SUPPORTED_STAGES_KHR = 0x9533
    SubgroupSupportedStagesKhr = 38195,
    //
    // Summary:
    //     Original was GL_SUBGROUP_SUPPORTED_FEATURES_KHR = 0x9534
    SubgroupSupportedFeaturesKhr = 38196,
    //
    // Summary:
    //     Original was GL_SUBGROUP_QUAD_ALL_STAGES_KHR = 0x9535
    SubgroupQuadAllStagesKhr = 38197,
    //
    // Summary:
    //     [requires: NV_scissor_exclusive] Original was GL_SCISSOR_TEST_EXCLUSIVE_NV =
    //     0x9555
    ScissorTestExclusiveNv = 38229,
    //
    // Summary:
    //     [requires: NV_scissor_exclusive] Original was GL_SCISSOR_BOX_EXCLUSIVE_NV = 0x9556
    ScissorBoxExclusiveNv = 38230,
    //
    // Summary:
    //     [requires: EXT_memory_object, or EXT_semaphore] Original was GL_DEVICE_UUID_EXT
    //     = 0x9597
    DeviceUuidExt = 38295,
    //
    // Summary:
    //     [requires: EXT_memory_object, or EXT_semaphore] Original was GL_DRIVER_UUID_EXT
    //     = 0x9598
    DriverUuidExt = 38296,
    //
    // Summary:
    //     [requires: EXT_memory_object_win32, or EXT_semaphore_win32] Original was GL_DEVICE_LUID_EXT
    //     = 0x9599
    DeviceLuidExt = 38297,
    //
    // Summary:
    //     [requires: EXT_memory_object_win32, or EXT_semaphore_win32] Original was GL_DEVICE_NODE_MASK_EXT
    //     = 0x959A
    DeviceNodeMaskExt = 38298,
    //
    // Summary:
    //     [requires: NV_primitive_shading_rate] Original was GL_SHADING_RATE_IMAGE_PER_PRIMITIVE_NV
    //     = 0x95B1
    ShadingRateImagePerPrimitiveNv = 38321,
    //
    // Summary:
    //     [requires: NV_primitive_shading_rate] Original was GL_SHADING_RATE_IMAGE_PALETTE_COUNT_NV
    //     = 0x95B2
    ShadingRateImagePaletteCountNv = 38322,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_SHADING_RATE_EXT = 0x96D0
    ShadingRateExt = 38608,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_MIN_FRAGMENT_SHADING_RATE_ATTACHMENT_TEXEL_WIDTH_EXT
    //     = 0x96D7
    MinFragmentShadingRateAttachmentTexelWidthExt = 38615,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_MAX_FRAGMENT_SHADING_RATE_ATTACHMENT_TEXEL_WIDTH_EXT
    //     = 0x96D8
    MaxFragmentShadingRateAttachmentTexelWidthExt = 38616,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_MIN_FRAGMENT_SHADING_RATE_ATTACHMENT_TEXEL_HEIGHT_EXT
    //     = 0x96D9
    MinFragmentShadingRateAttachmentTexelHeightExt = 38617,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_MAX_FRAGMENT_SHADING_RATE_ATTACHMENT_TEXEL_HEIGHT_EXT
    //     = 0x96DA
    MaxFragmentShadingRateAttachmentTexelHeightExt = 38618,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_MAX_FRAGMENT_SHADING_RATE_ATTACHMENT_TEXEL_ASPECT_RATIO_EXT
    //     = 0x96DB
    MaxFragmentShadingRateAttachmentTexelAspectRatioExt = 38619,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_MAX_FRAGMENT_SHADING_RATE_ATTACHMENT_LAYERS_EXT
    //     = 0x96DC
    MaxFragmentShadingRateAttachmentLayersExt = 38620,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_FRAGMENT_SHADING_RATE_WITH_SHADER_DEPTH_STENCIL_WRITES_SUPPORTED_EXT
    //     = 0x96DD
    FragmentShadingRateWithShaderDepthStencilWritesSupportedExt = 38621,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_FRAGMENT_SHADING_RATE_WITH_SAMPLE_MASK_SUPPORTED_EXT
    //     = 0x96DE
    FragmentShadingRateWithSampleMaskSupportedExt = 38622,
    //
    // Summary:
    //     [requires: EXT_fragment_shading_rate] Original was GL_FRAGMENT_SHADING_RATE_ATTACHMENT_WITH_DEFAULT_FRAMEBUFFER_SUPPORTED_EXT
    //     = 0x96DF
    FragmentShadingRateAttachmentWithDefaultFramebufferSupportedExt = 38623,
    MaxSamples = 36183,
}

#[allow(non_upper_case_globals)]
impl GetPName {
    pub const SmoothPointSizeRange: GetPName = GetPName::PointSizeRange;
    pub const SmoothPointSizeGranularity: GetPName = GetPName::PointSizeGranularity;
    pub const SmoothLineWidthRange: GetPName = GetPName::LineWidthRange;
    pub const SmoothLineWidthGranularity: GetPName = GetPName::LineWidthGranularity;
    pub const Modelview0StackDepthExt: GetPName = GetPName::ModelviewStackDepth;
    pub const Modelview0MatrixExt: GetPName = GetPName::ModelviewMatrix;
    pub const AlphaTestQcom: GetPName = GetPName::AlphaTest;
    pub const AlphaTestFuncQcom: GetPName = GetPName::AlphaTestFunc;
    pub const AlphaTestRefQcom: GetPName = GetPName::AlphaTestRef;
    pub const IndexLogicOp: GetPName = GetPName::LogicOp;
    pub const DrawBufferExt: GetPName = GetPName::DrawBuffer;
    pub const ReadBufferExt: GetPName = GetPName::ReadBuffer;
    pub const ReadBufferNv: GetPName = GetPName::ReadBuffer;
    pub const MaxClipPlanes: GetPName = GetPName::MaxClipDistances;
    pub const BlendColorExt: GetPName = GetPName::BlendColor;
    pub const BlendEquationExt: GetPName = GetPName::BlendEquation;
    pub const BlendEquationRgb: GetPName = GetPName::BlendEquation;
    pub const Texture3DBindingExt: GetPName = GetPName::TextureBinding3D;
    pub const TextureBinding3DOes: GetPName = GetPName::TextureBinding3D;
    pub const Max3DTextureSizeOes: GetPName = GetPName::Max3DTextureSizeExt;
    pub const SampleAlphaToMaskSgis: GetPName = GetPName::SampleAlphaToCoverage;
    pub const SampleMaskSgis: GetPName = GetPName::SampleCoverage;
    pub const SampleBuffersSgis: GetPName = GetPName::SampleBuffers;
    pub const SamplesSgis: GetPName = GetPName::Samples;
    pub const SampleMaskValueSgis: GetPName = GetPName::SampleCoverageValue;
    pub const SampleMaskInvertSgis: GetPName = GetPName::SampleCoverageInvert;
    pub const GenerateMipmapHintSgis: GetPName = GetPName::GenerateMipmapHint;
}
