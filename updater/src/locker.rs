#![allow(dead_code)]

use fs2::FileExt;
use std::fs::File;
use std::path::PathBuf;

/// OS-wide locking via file lock
#[derive(Debug)]
pub struct Locker {
    path: PathBuf,
    file: Option<File>,
}

impl Locker {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            file: None,
        }
    }

    pub fn lock(&mut self) -> bool {
        if let Ok(file) = File::create(&self.path) {
            if file.try_lock_exclusive().is_ok() {
                self.file = Some(file);
                return true;
            }
        }

        false
    }

    pub fn unlock(&mut self) -> bool {
        if self.file.is_some() && self.file.as_ref().unwrap().unlock().is_ok() {
            self.file = None;
            return true;
        }

        false
    }

    pub fn is_locked(&self) -> bool {
        self.file.is_some()
    }
}

impl Default for Locker {
    fn default() -> Self {
        let mut path = std::env::current_exe().expect("Failed to get running executable path");
        path.set_extension("lock");
        Self::new(path)
    }
}

impl Drop for Locker {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_unlock() {
        let mut locker = Locker::new(lockfile());

        assert!(!locker.is_locked());
        assert!(locker.lock());
        assert!(locker.is_locked());
        assert!(!locker.lock());
        assert!(locker.is_locked());
        assert!(locker.unlock());
        assert!(!locker.is_locked());
        assert!(!locker.unlock());
        assert!(!locker.is_locked());
    }

    fn lockfile() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push("test_lock_unlock");
        path
    }
}
