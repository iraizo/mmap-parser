#[cfg(test)]
mod test {
    use std::{fs::File, mem::size_of};
    use wownav_parser::mmap::DtNavMeshParams;

    #[test]
    fn parse_mesh_params() {
        let buffer = [
            171, 170, 216, 197, 0, 0, 128, 0, 255, 255, 121, 198, 85, 85, 5, 68, 85, 85, 5, 68,
            175, 2, 0, 0, 0, 0, 0, 128,
        ];
        let params = &buffer[0..size_of::<DtNavMeshParams>()];
        insta::assert_debug_snapshot!(DtNavMeshParams::try_from(params));
    }

    #[test]
    fn parse_mesh_params_file() {
        let f = File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "\\resources\\tests\\000.mmap")
            .unwrap();

        insta::assert_debug_snapshot!(DtNavMeshParams::try_from(f).unwrap());
    }
}
