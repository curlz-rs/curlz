mod bookmark_collection;
mod environment;

pub use bookmark_collection::*;
pub use environment::*;

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::Path;
    use tempfile::TempDir;

    pub fn create_file(
        base_path: &TempDir,
        filename: impl AsRef<Path>,
        contents: &str,
    ) -> anyhow::Result<()> {
        let path = base_path.path().join(filename);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::File::create(&path)?
            .write_all(contents.as_ref())
            .map_err(|e| e.into())
    }
}
