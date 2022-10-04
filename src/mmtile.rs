use anyhow::anyhow;
use bytemuck::AnyBitPattern;
use core::ffi::{c_float, c_int, c_uint};
use log::error;

/// Hi

const MMAP_MAGIC: u32 = 0x4d4d4150; // 'MMAP'
const MMAP_VERSION: u32 = 15;
#[derive(Debug, AnyBitPattern, Copy, Clone)]
#[repr(C)]
pub struct DtMeshHeader {
    magic: c_int,
    /// Tile magic number. (Used to identify the data format.)
    version: c_int,
    /// Tile data format version number.
    pub x: c_int,
    /// The x-position of the tile within the dtNavMesh tile grid. (x, y, layer)
    pub y: c_int,
    /// The y-position of the tile within the dtNavMesh tile grid. (x, y, layer)
    pub layer: c_int,
    /// The layer of the tile within the dtNavMesh tile grid. (x, y, layer)
    pub user_id: c_uint,
    /// The user defined id of the tile.
    pub poly_count: c_int,
    /// The number of polygons in the tile.
    pub vert_count: c_int,
    /// The number of vertices in the tile.
    pub max_link_count: c_int,
    /// The number of allocated links.
    pub detail_mesh_count: c_int,
    /// The number of sub-meshes in the detail mesh.

    /// The number of unique vertices in the detail mesh. (In addition to the polygon vertices.)
    pub detail_vert_count: c_int,

    pub detail_tri_count: c_int,
    /// The number of triangles in the detail mesh.
    pub bv_node_count: c_int,
    /// The number of bounding volume nodes. (Zero if bounding volumes are disabled.)
    pub off_mesh_con_count: c_int,
    /// The number of off-mesh connections.
    pub off_mesh_base: c_int,
    /// The index of the first polygon which is an off-mesh connection.
    pub walkable_height: c_float,
    /// The height of the agents using the tile.
    pub walkable_radius: c_float,
    /// The radius of the agents using the tile.
    pub walkable_climb: c_float,
    /// The maximum climb height of the agents using the tile.
    pub bmin: [c_float; 3],
    /// The minimum bounds of the tile's AABB. [(x, y, z)]
    pub bmax: [c_float; 3],
    /// The maximum bounds of the tile's AABB. [(x, y, z)]

    /// The bounding volume quantization factor.
    pub bv_quant_factor: c_float,
}

#[derive(Debug, AnyBitPattern, Copy, Clone)]
#[repr(C)]
pub struct MmapTileHeader {
    mmap_magic: u32,
    dt_version: u32,
    mmap_version: u32,
    size: u32,
    uses_liquids: u32,
}

impl TryFrom<&[u8]> for MmapTileHeader {
    type Error = anyhow::Error;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let header = match bytemuck::try_from_bytes::<MmapTileHeader>(&buffer) {
            Ok(header) => header,
            Err(err) => return Err(anyhow!("Error parsing to struct: {}", err)),
        };

        if header.mmap_magic != MMAP_MAGIC {
            return Err(anyhow!(
                "Invalid MMAP_MAGIC: {}, current: {}",
                header.mmap_magic,
                MMAP_MAGIC
            ));
        }

        if header.mmap_version != MMAP_VERSION {
            return Err(anyhow!(
                "Invalid MMAP_VERSION: {}, current: {}",
                header.mmap_version,
                MMAP_VERSION
            ));
        }

        /* TODO: check for header.size */

        Ok(*header)
    }
}

impl TryFrom<&[u8]> for DtMeshHeader {
    type Error = bytemuck::PodCastError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let header = bytemuck::try_from_bytes::<DtMeshHeader>(&buffer)?;
        Ok(*header)
    }
}
