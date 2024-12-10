#[repr(u32)]
pub enum PrimitiveType {
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
    Quads = 7,
    QuadStrip = 8,
    Polygon = 9,
    LinesAdjacency = 10,
    LineStripAdjacency = 11,
    TrianglesAdjacency = 12,
    TriangleStripAdjacency = 13,
    Patches = 14,
}

#[allow(non_upper_case_globals)]
impl PrimitiveType {
    pub const QuadsExt: PrimitiveType = PrimitiveType::Quads;
    pub const LinesAdjacencyArb: PrimitiveType = PrimitiveType::LinesAdjacency;
    pub const LinesAdjacencyExt: PrimitiveType = PrimitiveType::LinesAdjacency;
    pub const LineStripAdjacencyArb: PrimitiveType = PrimitiveType::LineStripAdjacency;
    pub const LineStripAdjacencyExt: PrimitiveType = PrimitiveType::LineStripAdjacency;
    pub const TrianglesAdjacencyArb: PrimitiveType = PrimitiveType::TrianglesAdjacency;
    pub const TrianglesAdjacencyExt: PrimitiveType = PrimitiveType::TrianglesAdjacency;
    pub const TriangleStripAdjacencyArb: PrimitiveType = PrimitiveType::TriangleStripAdjacency;
    pub const TriangleStripAdjacencyExt: PrimitiveType = PrimitiveType::TriangleStripAdjacency;
    pub const PatchesExt: PrimitiveType = PrimitiveType::Patches;
}
