use fs4::FileExt;
use std::fs::File;
use std::path::Path;
use anyhow::Result;

pub struct FileLock {
    file: File,
}

impl FileLock {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        file.lock_exclusive()?;
        Ok(Self { file })
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}
