use crate::workspace::Environment;

use std::fs::File;
use std::path::{Path, PathBuf};

/// wraps a `.yml` file, used to create an [`Environment`]
pub struct YamlEnvFile(PathBuf);

/// turn a [`PathBuf`] into a [`YamlEnvFile`]
impl From<&Path> for YamlEnvFile {
    fn from(path: &Path) -> Self {
        Self(path.to_path_buf())
    }
}

impl TryFrom<YamlEnvFile> for Environment {
    type Error = anyhow::Error;

    fn try_from(value: YamlEnvFile) -> Result<Self, Self::Error> {
        let file = File::open(value.0.as_path())?;
        serde_yaml::from_reader(file)
            .map(Self)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::tests::create_file;
    use tempfile::TempDir;

    #[test]
    fn should_try_from_yaml_env_file() {
        let tmp = TempDir::new().unwrap();
        create_file(
            &tmp,
            ".staging.yml",
            r#"protonmail_api_baseurl: https://api.protonmail.ch
email: some@user.com
"#,
        )
        .unwrap();
        let dot_file = YamlEnvFile(tmp.path().join(".staging.yml"));

        let env = Environment::try_from(dot_file).unwrap();
        assert_eq!(
            env.get("protonmail_api_baseurl").unwrap().as_ref(),
            "https://api.protonmail.ch"
        );
        assert_eq!(env.get("email").unwrap().as_ref(), "some@user.com");
    }
}
