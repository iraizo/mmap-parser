#[cfg(test)]
mod test {
    use wownav_parser::mmtile::{DtMeshHeader, MmapTileHeader};

    #[ctor::ctor]
    fn init() {
        pretty_env_logger::init();
    }
}
