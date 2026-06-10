use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

/// Temporary directory removed on drop.
#[derive(Debug)]
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// Creates a temporary directory under the process temp directory.
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
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Removes the directory now and prevents duplicate cleanup in `Drop`.
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
