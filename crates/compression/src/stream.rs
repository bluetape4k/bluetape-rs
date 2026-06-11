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

/// Opaque streaming compression writer for enabled algorithms.
///
/// The concrete backend writer type is intentionally hidden so codec backend
/// upgrades remain an internal implementation detail.
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
pub struct CompressionWriter<W: Write> {
    inner: CompressionWriterInner<W>,
}

enum CompressionWriterInner<W: Write> {
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
    #[allow(dead_code)]
    NoAlgorithms(PhantomData<W>),
}

impl<W: Write> CompressionWriter<W> {
    #[cfg(feature = "gzip")]
    pub(crate) fn gzip(writer: flate2::write::GzEncoder<W>) -> Self {
        Self {
            inner: CompressionWriterInner::Gzip(writer),
        }
    }

    #[cfg(feature = "zlib")]
    pub(crate) fn zlib(writer: flate2::write::ZlibEncoder<W>) -> Self {
        Self {
            inner: CompressionWriterInner::Zlib(writer),
        }
    }

    #[cfg(feature = "deflate")]
    pub(crate) fn deflate(writer: flate2::write::DeflateEncoder<W>) -> Self {
        Self {
            inner: CompressionWriterInner::Deflate(writer),
        }
    }

    #[cfg(feature = "zstd")]
    pub(crate) fn zstd(writer: zstd::stream::write::Encoder<'static, W>) -> Self {
        Self {
            inner: CompressionWriterInner::Zstd(writer),
        }
    }

    #[cfg(feature = "lz4")]
    pub(crate) fn lz4(writer: lz4_flex::frame::FrameEncoder<W>) -> Self {
        Self {
            inner: CompressionWriterInner::Lz4(writer),
        }
    }

    #[cfg(feature = "snappy")]
    pub(crate) fn snappy(writer: snap::write::FrameEncoder<W>) -> Self {
        Self {
            inner: CompressionWriterInner::Snappy(Box::new(writer)),
        }
    }
}

impl<W: Write> CompressionWriter<W> {
    /// Finishes the compression stream and returns the wrapped writer.
    ///
    /// # Errors
    ///
    /// Returns [`CompressionError`] when the underlying encoder fails to write
    /// its final bytes.
    pub fn finish(self) -> Result<W, CompressionError> {
        match self.inner {
            #[cfg(feature = "gzip")]
            CompressionWriterInner::Gzip(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "gzip",
                        source,
                    })
            }
            #[cfg(feature = "zlib")]
            CompressionWriterInner::Zlib(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "zlib",
                        source,
                    })
            }
            #[cfg(feature = "deflate")]
            CompressionWriterInner::Deflate(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "deflate",
                        source,
                    })
            }
            #[cfg(feature = "zstd")]
            CompressionWriterInner::Zstd(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "zstd",
                        source,
                    })
            }
            #[cfg(feature = "lz4")]
            CompressionWriterInner::Lz4(writer) => {
                writer
                    .finish()
                    .map_err(|source| CompressionError::CompressFinish {
                        algorithm: "lz4",
                        source: source.into(),
                    })
            }
            #[cfg(feature = "snappy")]
            CompressionWriterInner::Snappy(writer) => {
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
            CompressionWriterInner::NoAlgorithms(_) => Err(CompressionError::CompressFinish {
                algorithm: "none",
                source: std::io::Error::other("no compression algorithms enabled"),
            }),
        }
    }
}

impl<W: Write> Write for CompressionWriter<W> {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            #[cfg(feature = "gzip")]
            CompressionWriterInner::Gzip(writer) => writer.write(_buf),
            #[cfg(feature = "zlib")]
            CompressionWriterInner::Zlib(writer) => writer.write(_buf),
            #[cfg(feature = "deflate")]
            CompressionWriterInner::Deflate(writer) => writer.write(_buf),
            #[cfg(feature = "zstd")]
            CompressionWriterInner::Zstd(writer) => writer.write(_buf),
            #[cfg(feature = "lz4")]
            CompressionWriterInner::Lz4(writer) => writer.write(_buf),
            #[cfg(feature = "snappy")]
            CompressionWriterInner::Snappy(writer) => writer.write(_buf),
            #[cfg(not(any(
                feature = "gzip",
                feature = "zlib",
                feature = "deflate",
                feature = "zstd",
                feature = "lz4",
                feature = "snappy"
            )))]
            CompressionWriterInner::NoAlgorithms(_) => {
                Err(std::io::Error::other("no compression algorithms enabled"))
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.inner {
            #[cfg(feature = "gzip")]
            CompressionWriterInner::Gzip(writer) => writer.flush(),
            #[cfg(feature = "zlib")]
            CompressionWriterInner::Zlib(writer) => writer.flush(),
            #[cfg(feature = "deflate")]
            CompressionWriterInner::Deflate(writer) => writer.flush(),
            #[cfg(feature = "zstd")]
            CompressionWriterInner::Zstd(writer) => writer.flush(),
            #[cfg(feature = "lz4")]
            CompressionWriterInner::Lz4(writer) => writer.flush(),
            #[cfg(feature = "snappy")]
            CompressionWriterInner::Snappy(writer) => writer.flush(),
            #[cfg(not(any(
                feature = "gzip",
                feature = "zlib",
                feature = "deflate",
                feature = "zstd",
                feature = "lz4",
                feature = "snappy"
            )))]
            CompressionWriterInner::NoAlgorithms(_) => {
                Err(std::io::Error::other("no compression algorithms enabled"))
            }
        }
    }
}

/// Opaque streaming decompression reader for enabled algorithms.
///
/// Read failures that come from decompressed-size limit enforcement preserve the
/// typed [`CompressionError`] as the source of the returned [`std::io::Error`].
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
pub struct DecompressionReader<R: Read> {
    inner: DecompressionReaderInner<R>,
}

enum DecompressionReaderInner<R: Read> {
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
    #[allow(dead_code)]
    NoAlgorithms(PhantomData<R>),
}

impl<R: Read> DecompressionReader<R> {
    #[cfg(feature = "gzip")]
    pub(crate) fn gzip(reader: LimitedReader<flate2::read::GzDecoder<R>>) -> Self {
        Self {
            inner: DecompressionReaderInner::Gzip(reader),
        }
    }

    #[cfg(feature = "zlib")]
    pub(crate) fn zlib(reader: LimitedReader<flate2::read::ZlibDecoder<R>>) -> Self {
        Self {
            inner: DecompressionReaderInner::Zlib(reader),
        }
    }

    #[cfg(feature = "deflate")]
    pub(crate) fn deflate(reader: LimitedReader<flate2::read::DeflateDecoder<R>>) -> Self {
        Self {
            inner: DecompressionReaderInner::Deflate(reader),
        }
    }

    #[cfg(feature = "zstd")]
    pub(crate) fn zstd(
        reader: LimitedReader<zstd::stream::read::Decoder<'static, std::io::BufReader<R>>>,
    ) -> Self {
        Self {
            inner: DecompressionReaderInner::Zstd(reader),
        }
    }

    #[cfg(feature = "lz4")]
    pub(crate) fn lz4(reader: LimitedReader<lz4_flex::frame::FrameDecoder<R>>) -> Self {
        Self {
            inner: DecompressionReaderInner::Lz4(reader),
        }
    }

    #[cfg(feature = "snappy")]
    pub(crate) fn snappy(reader: LimitedReader<snap::read::FrameDecoder<R>>) -> Self {
        Self {
            inner: DecompressionReaderInner::Snappy(reader),
        }
    }
}

impl<R: Read> Read for DecompressionReader<R> {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            #[cfg(feature = "gzip")]
            DecompressionReaderInner::Gzip(reader) => reader.read(_buf),
            #[cfg(feature = "zlib")]
            DecompressionReaderInner::Zlib(reader) => reader.read(_buf),
            #[cfg(feature = "deflate")]
            DecompressionReaderInner::Deflate(reader) => reader.read(_buf),
            #[cfg(feature = "zstd")]
            DecompressionReaderInner::Zstd(reader) => reader.read(_buf),
            #[cfg(feature = "lz4")]
            DecompressionReaderInner::Lz4(reader) => reader.read(_buf),
            #[cfg(feature = "snappy")]
            DecompressionReaderInner::Snappy(reader) => reader.read(_buf),
            #[cfg(not(any(
                feature = "gzip",
                feature = "zlib",
                feature = "deflate",
                feature = "zstd",
                feature = "lz4",
                feature = "snappy"
            )))]
            DecompressionReaderInner::NoAlgorithms(_) => {
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
