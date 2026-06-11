#[cfg(feature = "deflate")]
mod deflate;
#[cfg(any(feature = "gzip", feature = "zlib", feature = "deflate"))]
mod flate;
#[cfg(feature = "gzip")]
mod gzip;
#[cfg(feature = "lz4")]
mod lz4;
#[cfg(feature = "snappy")]
mod snappy;
#[cfg(feature = "zlib")]
mod zlib;
#[cfg(feature = "zstd")]
mod zstd;

#[cfg(feature = "deflate")]
pub use deflate::Deflate;
#[cfg(feature = "gzip")]
pub use gzip::Gzip;
#[cfg(feature = "lz4")]
pub use lz4::Lz4;
#[cfg(feature = "snappy")]
pub use snappy::Snappy;
#[cfg(feature = "zlib")]
pub use zlib::Zlib;
#[cfg(feature = "zstd")]
pub use zstd::Zstd;
