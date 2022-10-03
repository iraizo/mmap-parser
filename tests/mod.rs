#[cfg(test)]
mod test {
    use std::mem::size_of;

    use wownav_parser::{mmap::DtNavMeshParams, mmtile::MmapTileHeader};

    #[ctor::ctor]
    fn init() {
        pretty_env_logger::init();
    }

    #[test]
    fn parse_tile_header() {
        let buffer = [
            80, 65, 77, 77, 7, 0, 0, 0, 15, 0, 0, 0, 116, 163, 1, 0, 1, 0, 0, 0,
        ];
        insta::assert_debug_snapshot!(MmapTileHeader::from_bytes(&buffer));
    }

    #[test]
    fn parse_mesh_header() {
        let buffer = [
            86, 65, 78, 68, 7, 0, 0, 0, 7, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 5, 0,
            0, 166, 7, 0, 0, 130, 20, 0, 0, 27, 5, 0, 0, 242, 5, 0, 0, 196, 16, 0, 0, 54, 10, 0, 0,
            0, 0, 0, 0, 27, 5, 0, 0, 201, 204, 204, 63, 134, 136, 8, 63, 201, 204, 204, 63, 255,
            255, 71, 197, 0, 0, 250, 195, 85, 85, 133, 69, 170, 170, 38, 197, 195, 53, 25, 67, 0,
            0, 150, 69, 4, 0, 112, 64,
        ];

        insta::assert_debug_snapshot!(&buffer);
    }
}
