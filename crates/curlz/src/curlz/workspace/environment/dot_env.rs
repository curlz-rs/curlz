use std::path::{Path, PathBuf};

use crate::workspace::Environment;

/// wraps a `.env` file, used to create an [`Environment`]
pub struct DotEnvFile(PathBuf);

/// turn a [`PathBuf`] into a [`DotEnvFile`]
impl From<&Path> for DotEnvFile {
    fn from(path: &Path) -> Self {
        Self(path.to_path_buf())
    }
}

impl TryFrom<DotEnvFile> for Environment {
    type Error = anyhow::Error;

    fn try_from(value: DotEnvFile) -> Result<Self, Self::Error> {
        let mut env = Environment::default();
        dotenvy::from_path_iter(value.0.as_path())
            .map_err(anyhow::Error::from)?
            .map(|i| i.unwrap())
            .for_each(|(key, value)| {
                env.as_mut().insert(key, value);
            });

        Ok(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::tests::create_file;

    use tempfile::TempDir;

    #[test]
    fn should_try_from_dot_env_file() {
        let tmp = TempDir::new().unwrap();
        create_file(
            &tmp,
            "../../../../../../.env",
            r#"protonmail_api_baseurl=https://api.protonmail.ch
email=some@user.com
"#,
        )
        .unwrap();
        let dot_file = DotEnvFile(tmp.path().join("../../../../../../.env"));

        let env = Environment::try_from(dot_file).unwrap();
        assert_eq!(
            env.get("protonmail_api_baseurl").unwrap().as_ref(),
            "https://api.protonmail.ch"
        );
        assert_eq!(env.get("email").unwrap().as_ref(), "some@user.com");
    }
}
