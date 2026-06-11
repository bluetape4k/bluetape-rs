use crate::{CompressionConfig, CompressionError};
use std::io::{Read, Write};
#[cfg(not(any(
    feature = "gzip",
    feature = "zlib",
    feature = "deflate",
    feature = "zstd",
    feature = "lz4",
    feature = "snappy"
)))]
use std::marker::PhantomData;

/// Streaming compression writer for enabled algorithms.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "gzip")]
/// # {
/// use bluetape_rs_compression::{CompressionConfig, Compressor, Gzip};
/// use std::io::Write;
///
/// let mut writer = Gzip
///     .compression_writer(Vec::new(), CompressionConfig::new())
///     .unwrap();
/// writer.write_all(b"payload").unwrap();
/// let compressed = writer.finish().unwrap();
///
/// let restored = Gzip.decompress(&compressed).unwrap();
/// assert_eq!(restored, b"payload");
/// # }
/// ```
///
/// Call [`CompressionWriter::finish`] after the final write to flush codec
/// trailers and recover the wrapped writer.
pub enum CompressionWriter<W: Write> {
    /// gzip stream writer.
    #[cfg(feature = "gzip")]
    Gzip(flate2::write::GzEncoder<W>),
    /// zlib stream writer.
    #[cfg(feature = "zlib")]
    Zlib(flate2::write::ZlibEncoder<W>),
    /// raw deflate stream writer.
    #[cfg(feature = "deflate")]
    Deflate(flate2::write::DeflateEncoder<W>),
    /// zstd stream writer.
    #[cfg(feature = "zstd")]
    Zstd(zstd::stream::write::Encoder<'static, W>),
    /// lz4 framed stream writer.
    #[cfg(feature = "lz4")]
    Lz4(lz4_flex::frame::FrameEncoder<W>),
    /// snappy framed stream writer.
    #[cfg(feature = "snappy")]
    Snappy(Box<snap::write::FrameEncoder<W>>),
    #[cfg(not(any(
        feature = "gzip",
        feature = "zlib",
        feature = "deflate",
        feature = "zstd",
        feature = "lz4",
        feature = "snappy"
    )))]
    #[doc(hidden)]
    NoAlgorithms(PhantomData<W>),
}

impl<W: Write> CompressionWriter<W> {
    /// Finishes the compression stream and returns the wrapped writer.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying encoder fails to write
    /// its final bytes.
    pub fn finish(self) -> Result<W, CompressionError> {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "gzip",
                        source,
                    })
            }
            #[cfg(feature = "zlib")]
            Self::Zlib(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "zlib",
                        source,
                    })
            }
            #[cfg(feature = "deflate")]
            Self::Deflate(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "deflate",
                        source,
                    })
            }
            #[cfg(feature = "zstd")]
            Self::Zstd(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "zstd",
                        source,
                    })
            }
            #[cfg(feature = "lz4")]
            Self::Lz4(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "lz4",
                        source: source.into(),
                    })
            }
            #[cfg(feature = "snappy")]
            Self::Snappy(writer) => {
                (*writer)
                    .into_inner()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "snappy",
                        source: source.into_error(),
                    })
            }
            #[cfg(not(any(
                feature = "gzip",
                feature = "zlib",
                feature = "deflate",
                feature = "zstd",
                feature = "lz4",
                feature = "snappy"
            )))]
            Self::NoAlgorithms(_) => Err(CompressionError::CompressFinish {
                algorithm: "none",
                source: std::io::Error::other("no compression algorithms enabled"),
            }),
        }
    }
}

impl<W: Write> Write for CompressionWriter<W> {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip(writer) => writer.write(_buf),
            #[cfg(feature = "zlib")]
            Self::Zlib(writer) => writer.write(_buf),
            #[cfg(feature = "deflate")]
            Self::Deflate(writer) => writer.write(_buf),
            #[cfg(feature = "zstd")]
            Self::Zstd(writer) => writer.write(_buf),
            #[cfg(feature = "lz4")]
            Self::Lz4(writer) => writer.write(_buf),
            #[cfg(feature = "snappy")]
            Self::Snappy(writer) => writer.write(_buf),
            #[cfg(not(any(
                feature = "gzip",
                feature = "zlib",
                feature = "deflate",
                feature = "zstd",
                feature = "lz4",
                feature = "snappy"
            )))]
            Self::NoAlgorithms(_) => {
                Err(std::io::Error::other("no compression algorithms enabled"))
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip(writer) => writer.flush(),
            #[cfg(feature = "zlib")]
            Self::Zlib(writer) => writer.flush(),
            #[cfg(feature = "deflate")]
            Self::Deflate(writer) => writer.flush(),
            #[cfg(feature = "zstd")]
            Self::Zstd(writer) => writer.flush(),
            #[cfg(feature = "lz4")]
            Self::Lz4(writer) => writer.flush(),
            #[cfg(feature = "snappy")]
            Self::Snappy(writer) => writer.flush(),
            #[cfg(not(any(
                feature = "gzip",
                feature = "zlib",
                feature = "deflate",
                feature = "zstd",
                feature = "lz4",
                feature = "snappy"
            )))]
            Self::NoAlgorithms(_) => {
                Err(std::io::Error::other("no compression algorithms enabled"))
            }
        }
    }
}

/// Streaming decompression reader for enabled algorithms.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "gzip")]
/// # {
/// use bluetape_rs_compression::{CompressionConfig, Compressor, Gzip};
/// use std::io::Read;
///
/// let compressed = Gzip.compress(b"payload").unwrap();
/// let mut reader = Gzip
///     .decompression_reader(&compressed[..], CompressionConfig::new())
///     .unwrap();
///
/// let mut restored = Vec::new();
/// reader.read_to_end(&mut restored).unwrap();
/// assert_eq!(restored, b"payload");
/// # }
/// ```
pub enum DecompressionReader<R: Read> {
    /// gzip stream reader.
    #[cfg(feature = "gzip")]
    Gzip(LimitedReader<flate2::read::GzDecoder<R>>),
    /// zlib stream reader.
    #[cfg(feature = "zlib")]
    Zlib(LimitedReader<flate2::read::ZlibDecoder<R>>),
    /// raw deflate stream reader.
    #[cfg(feature = "deflate")]
    Deflate(LimitedReader<flate2::read::DeflateDecoder<R>>),
    /// zstd stream reader.
    #[cfg(feature = "zstd")]
    Zstd(LimitedReader<zstd::stream::read::Decoder<'static, std::io::BufReader<R>>>),
    /// lz4 framed stream reader.
    #[cfg(feature = "lz4")]
    Lz4(LimitedReader<lz4_flex::frame::FrameDecoder<R>>),
    /// snappy framed stream reader.
    #[cfg(feature = "snappy")]
    Snappy(LimitedReader<snap::read::FrameDecoder<R>>),
    #[cfg(not(any(
        feature = "gzip",
        feature = "zlib",
        feature = "deflate",
        feature = "zstd",
        feature = "lz4",
        feature = "snappy"
    )))]
    #[doc(hidden)]
    NoAlgorithms(PhantomData<R>),
}

impl<R: Read> Read for DecompressionReader<R> {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            #[cfg(feature = "gzip")]
            Self::Gzip(reader) => reader.read(_buf),
            #[cfg(feature = "zlib")]
            Self::Zlib(reader) => reader.read(_buf),
            #[cfg(feature = "deflate")]
            Self::Deflate(reader) => reader.read(_buf),
            #[cfg(feature = "zstd")]
            Self::Zstd(reader) => reader.read(_buf),
            #[cfg(feature = "lz4")]
            Self::Lz4(reader) => reader.read(_buf),
            #[cfg(feature = "snappy")]
            Self::Snappy(reader) => reader.read(_buf),
            #[cfg(not(any(
                feature = "gzip",
                feature = "zlib",
                feature = "deflate",
                feature = "zstd",
                feature = "lz4",
                feature = "snappy"
            )))]
            Self::NoAlgorithms(_) => {
                Err(std::io::Error::other("no compression algorithms enabled"))
            }
        }
    }
}

/// Reader wrapper that enforces decompressed-size limits.
#[cfg_attr(
    not(any(
        feature = "gzip",
        feature = "zlib",
        feature = "deflate",
        feature = "zstd",
        feature = "lz4",
        feature = "snappy"
    )),
    allow(dead_code)
)]
pub struct LimitedReader<R> {
    algorithm: &'static str,
    inner: R,
    limit: Option<usize>,
    read: usize,
}

impl<R> LimitedReader<R> {
    #[cfg_attr(
        not(any(
            feature = "gzip",
            feature = "zlib",
            feature = "deflate",
            feature = "zstd",
            feature = "lz4",
            feature = "snappy"
        )),
        allow(dead_code)
    )]
    pub(crate) fn new(algorithm: &'static str, inner: R, config: CompressionConfig) -> Self {
        Self {
            algorithm,
            inner,
            limit: config.max_decompressed_size,
            read: 0,
        }
    }
}

impl<R: Read> Read for LimitedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let allowed = match self.limit {
            Some(limit) if self.read >= limit => {
                let mut extra = [0_u8; 1];
                return match self.inner.read(&mut extra) {
                    Ok(0) => Ok(0),
                    Ok(read) => Err(std::io::Error::other(
                        CompressionError::DecompressedSizeLimitExceeded {
                            algorithm: self.algorithm,
                            limit,
                            actual: self.read.saturating_add(read),
                        },
                    )),
                    Err(source) => Err(source),
                };
            }
            Some(limit) => buf.len().min(limit - self.read),
            None => buf.len(),
        };
        let read = self.inner.read(&mut buf[..allowed])?;
        self.read = self.read.saturating_add(read);
        Ok(read)
    }
}
