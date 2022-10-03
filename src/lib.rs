//! This crate is used to parse World of Warcraft or rather Trinitycore/vMangos `.mmap` & `.mmtile` files.  
//! This is very Work in progress and I am working on adding proper documentation for all types since there isnt much to find anywhere else.  
//! The different modules can be used for each file extension, the structs included in those are only supposed to work for their extension not the other way around.
//! ### Examples
//! You can find some examples on how to parse raw bytes into the given structures inside the `tests` folder.  
//! More examples with `File`/`Reader` usage will be written/documented.
#![feature(core_intrinsics)]
pub mod mmap;
pub mod mmtile;
