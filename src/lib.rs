#![warn(fat_ptr_transmutes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

//! A Rust implementation of the varint format as used in Google's Protocol
//! Buffers. Adds `read_*`/`write_*` methods for various sizes of
//! varints on top of the standard IO traits.

mod read;
mod write;

pub use read::ReadVarInt;
pub use write::WriteVarInt;
