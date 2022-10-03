use anyhow::anyhow;
use anyhow::Result;
use bytemuck::AnyBitPattern;
use bytemuck::PodCastError;
use std::{fs::File, intrinsics::size_of, io::Read};

#[derive(Debug, AnyBitPattern, Copy, Clone)]
#[repr(C)]
pub struct DtNavMeshParams {
    pub orig: [f32; 3],
    pub tile_height: f32,
    pub max_tiles: i32,
    pub max_polys: i32,
}

impl TryFrom<&[u8]> for DtNavMeshParams {
    type Error = PodCastError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        return match bytemuck::try_from_bytes::<DtNavMeshParams>(&buffer) {
            Ok(v) => Ok(*v),
            Err(err) => Err(err),
        };
    }
}

impl TryFrom<File> for DtNavMeshParams {
    type Error = anyhow::Error;

    fn try_from(mut f: File) -> Result<Self> {
        let mut buffer = vec![0u8; size_of::<DtNavMeshParams>()];
        f.read_exact(&mut buffer)?;

        let params =
            match bytemuck::try_from_bytes::<DtNavMeshParams>(&buffer[0..size_of::<Self>()]) {
                Ok(params) => params,
                Err(err) => return Err(anyhow!("Error parsing struct: {}", err)),
            };

        Ok(*params)
    }
}
