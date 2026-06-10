use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

/// Temporary directory removed on drop.
///
/// The directory is created under [`std::env::temp_dir`] with a process-local
/// unique suffix. On Unix platforms it is created with `0o700` permissions.
///
/// # Examples
///
/// ```
/// use bluetape_rs_test::TempDir;
///
/// let temp = TempDir::new("bluetape-rs")?;
/// assert!(temp.path().is_dir());
/// temp.close()?;
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug)]
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// Creates a temporary directory under the process temp directory.
    ///
    /// `prefix` must be a single relative path segment. Empty prefixes,
    /// absolute paths, and values containing path separators are rejected.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::TempDir;
    ///
    /// let temp = TempDir::new("cache-test")?;
    /// assert!(temp.path().exists());
    /// temp.close()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`std::io::ErrorKind::InvalidInput`] when `prefix` is not a
    /// single relative path segment. Also returns any filesystem error raised
    /// while creating the directory.
    pub fn new(prefix: impl AsRef<str>) -> std::io::Result<Self> {
        let prefix = prefix.as_ref();
        validate_temp_prefix(prefix)?;
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("{prefix}-{}-{nanos}-{id}", std::process::id(),));
        create_private_dir(&path)?;
        Ok(Self { path })
    }

    /// Returns the temporary directory path.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::TempDir;
    ///
    /// let temp = TempDir::new("path-test")?;
    /// assert!(temp.path().ends_with(temp.path().file_name().unwrap()));
    /// temp.close()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Removes the directory now and prevents duplicate cleanup in `Drop`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::TempDir;
    ///
    /// let temp = TempDir::new("close-test")?;
    /// let path = temp.path().to_path_buf();
    /// temp.close()?;
    /// assert!(!path.exists());
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns any filesystem error raised while recursively removing the
    /// temporary directory.
    pub fn close(mut self) -> std::io::Result<()> {
        close_path(&mut self.path, |path| std::fs::remove_dir_all(path))
    }
}

pub(crate) fn close_path(
    path: &mut PathBuf,
    remove_dir_all: impl FnOnce(&Path) -> std::io::Result<()>,
) -> std::io::Result<()> {
    if path.as_os_str().is_empty() {
        return Ok(());
    }
    remove_dir_all(path.as_path())?;
    path.clear();
    Ok(())
}

#[cfg(unix)]
fn create_private_dir(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    std::fs::DirBuilder::new().mode(0o700).create(path)
}

#[cfg(not(unix))]
fn create_private_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir(path)
}

fn validate_temp_prefix(prefix: &str) -> std::io::Result<()> {
    if prefix.is_empty()
        || Path::new(prefix).is_absolute()
        || prefix.contains('/')
        || prefix.contains('\\')
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "temporary directory prefix must be a single relative path segment",
        ));
    }
    Ok(())
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if !self.path.as_os_str().is_empty() {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}
