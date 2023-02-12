mod bookmark_collection;
mod environment;

pub use bookmark_collection::*;
pub use environment::*;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::ops::Not;
    use std::path::Path;
    use tempfile::TempDir;

    pub fn create_file(
        filename: impl AsRef<str>,
        contents: impl AsRef<str>,
    ) -> anyhow::Result<TempDir> {
        let base_path = tempfile::tempdir()?;
        let path = base_path.path().join(filename.as_ref());

        if let Some(parent) = path.parent() {
            if parent != base_path.path() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let mut file = File::create(&path)?;
        file.write_all(contents.as_ref().as_ref())?;

        Ok(base_path)
    }

    #[test]
    fn test_create_file_cleanup() {
        let path = {
            let tmp = create_file("foobar", "Hello").unwrap();
            tmp.path()
                .join("foobar")
                .as_path()
                .to_str()
                .unwrap()
                .to_string()
        };
        assert!(Path::exists(Path::new(&path)).not());
    }

    #[test]
    fn test_create_file() {
        let tmp = create_file("foobar", "Hello").unwrap();

        let mut hello = String::new();
        let mut f = File::open(tmp.path().join("foobar").as_path()).unwrap();
        f.read_to_string(&mut hello).unwrap();
        assert_eq!(hello.as_str(), "Hello");
    }

    #[test]
    fn test_create_file_with_dir() {
        let tmp = create_file("foo/foobar", "Hello").unwrap();

        let mut hello = String::new();
        let mut f = File::open(tmp.path().join("foo").join("foobar").as_path()).unwrap();
        f.read_to_string(&mut hello).unwrap();
        assert_eq!(hello.as_str(), "Hello");
    }
}
