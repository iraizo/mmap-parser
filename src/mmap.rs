use bytemuck::AnyBitPattern;

#[derive(Debug, AnyBitPattern, Copy, Clone)]
#[repr(C)]
pub struct DtNavMeshParams {
    pub orig: [f32; 3],
    pub tile_height: f32,
    pub max_tiles: i32,
    pub max_polys: i32,
}

impl DtNavMeshParams {
    pub fn from_bytes(buffer: &[u8]) -> Self {
        return *bytemuck::from_bytes::<DtNavMeshParams>(&buffer);
    }
}
