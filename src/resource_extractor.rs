use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceExtractor {
    root: PathBuf,
}

impl ResourceExtractor {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn extract_matching(
        &self,
        file_name: &str,
        destination: impl AsRef<Path>,
    ) -> io::Result<bool> {
        if file_name.is_empty() || !self.root.is_dir() {
            return Ok(false);
        }

        let destination = destination.as_ref();
        if destination.is_dir() {
            return Ok(false);
        }

        let Some(source) = self.find_matching_file(file_name)? else {
            return Ok(false);
        };

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(source, destination)?;
        Ok(true)
    }

    fn find_matching_file(&self, file_name: &str) -> io::Result<Option<PathBuf>> {
        let mut pending = VecDeque::from([self.root.clone()]);

        while let Some(path) = pending.pop_front() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                let metadata = entry.metadata()?;

                if metadata.is_dir() {
                    pending.push_back(path);
                    continue;
                }

                let relative = path.strip_prefix(&self.root).unwrap_or(path.as_path());
                if relative.to_string_lossy().contains(file_name) {
                    return Ok(Some(path));
                }
            }
        }

        Ok(None)
    }
}
