use anyhow::anyhow;
use bytemuck::AnyBitPattern;
use core::ffi::{c_float, c_int, c_uint};
use log::error;
use std::{fs::File, intrinsics::size_of, io::Read};

/// Hi

const MMAP_MAGIC: u32 = 0x4d4d4150; // 'MMAP'
const MMAP_VERSION: u32 = 15;

/// A magic number used to detect compatibility of navigation tile data.
const DT_NAVMESH_MAGIC: usize =
    ('D' as usize) << 24 | ('N' as usize) << 16 | ('A' as usize) << 8 | 'V' as usize;

/// A version number used to detect compatibility of navigation tile data.
const DT_NAVMESH_VERSION: i8 = 7;

/// A magic number used to detect the compatibility of navigation tile states.
const DT_NAVMESH_STATE_MAGIC: usize =
    ('D' as usize) << 24 | ('N' as usize) << 16 | ('M' as usize) << 8 | ('S' as usize);

/// A version number used to detect compatibility of navigation tile states.
const DT_NAVMESH_STATE_VERSION: i8 = 1;

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

        MmapTileHeader::sanity(*header)?;

        Ok(*header)
    }
}

impl TryFrom<File> for MmapTileHeader {
    type Error = anyhow::Error;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {
        let mut buffer = vec![0u8; size_of::<MmapTileHeader>()];
        f.read_exact(&mut buffer)?;

        let header = match bytemuck::try_from_bytes::<MmapTileHeader>(&buffer) {
            Ok(header) => header,
            Err(err) => return Err(anyhow!("Error parsing to struct: {}", err)),
        };

        MmapTileHeader::sanity(*header)?;

        Ok(*header)
    }
}

impl MmapTileHeader {
    fn sanity(self) -> Result<(), anyhow::Error> {
        /* TODO: check for header.size */
        if self.mmap_magic != MMAP_MAGIC {
            return Err(anyhow!(
                "Invalid MMAP_MAGIC: {}, current: {}",
                self.mmap_magic,
                MMAP_MAGIC
            ));
        }

        if self.mmap_version != MMAP_VERSION {
            return Err(anyhow!(
                "Invalid MMAP_VERSION: {}, current: {}",
                self.mmap_version,
                MMAP_VERSION
            ));
        }

        /* TODO: check for header.size */

        Ok(())
    }
}

impl TryFrom<&[u8]> for DtMeshHeader {
    type Error = anyhow::Error;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let header = match bytemuck::try_from_bytes::<DtMeshHeader>(&buffer) {
            Ok(header) => header,
            Err(err) => return Err(anyhow!("Error parsing to struct: {}", err)),
        };

        DtMeshHeader::sanity(*header)?;
        Ok(*header)
    }
}

impl DtMeshHeader {
    fn sanity(self) -> Result<(), anyhow::Error> {
        if self.magic != DT_NAVMESH_MAGIC as i32 {
            return Err(anyhow!("Invalid navmesh magic"));
        }

        if self.version != DT_NAVMESH_VERSION as i32 {
            return Err(anyhow!("Invalid navmesh version"));
        }

        Ok(())
    }
}
