use crate::template::variables::Placeholder;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

use super::dot_env::DotEnvFile;
use super::yaml_env::YamlEnvFile;

#[derive(Default, Debug)]
pub struct Environment(pub(crate) HashMap<String, String>);

impl Environment {
    /// returns the value for a given key
    pub fn get(&'_ self, key: impl AsRef<str>) -> Option<impl AsRef<str> + '_> {
        self.0.get(key.as_ref())
    }

    /// inserts a key with it's value, copies the data
    pub fn insert(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        self.0
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
    }
}

impl AsMut<HashMap<String, String>> for Environment {
    fn as_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl TryFrom<&Path> for Environment {
    type Error = anyhow::Error;

    fn try_from(env_file: &Path) -> Result<Self, Self::Error> {
        if env_file.exists() && env_file.is_file() {
            match env_file.extension().and_then(OsStr::to_str) {
                None => DotEnvFile::from(env_file).try_into(),
                Some("yml" | "yaml") => YamlEnvFile::from(env_file).try_into(),
                Some(ext) => todo!("Environment loading for file extension {}", ext),
            }
        } else if env_file.is_dir() {
            todo!("Support directory environments");
        } else {
            Ok(Environment::default())
        }
    }
}

impl From<&Environment> for minijinja::value::Value {
    fn from(env: &Environment) -> Self {
        minijinja::value::Value::from_serializable(&env.0)
    }
}

/// creates an [`Environment`] from a `.env` | `.yaml` | `.yml` file
/// If the file does not exist, an empty [`Environment`] is returned.
///
/// ## Fallible
/// If `env_file` is not a `.env` | `.yaml` | `.yml` file, an error is returned.
/// If `env_file` is a directory, an error is returned.
pub fn create_environment(
    env_file: impl AsRef<Path>,
    placeholders: &[Placeholder],
) -> crate::Result<Environment> {
    Environment::try_from(env_file.as_ref()).map(|mut env| {
        placeholders
            .iter()
            .map(|placeholder| {
                let Placeholder {
                    name,
                    value,
                    default,
                    ..
                } = placeholder;
                (name, value.as_ref().or(default.as_ref()).unwrap())
            })
            .for_each(|(k, v)| env.insert(k, v));
        env
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_file;

    #[test]
    fn should_load_a_dot_env_file() {
        let tmp = create_file(".env", "").unwrap();
        Environment::try_from(tmp.path().join(".env").as_path()).unwrap();
    }

    #[test]
    fn should_load_a_yaml_env_file() {
        let tmp = create_file("staging.yml", "").unwrap();
        Environment::try_from(tmp.path().join(".staging.yml").as_path()).unwrap();
    }

    #[test]
    fn should_gracefully_ignore_not_existing_files() {
        Environment::try_from(Path::new("foo.bar.yml")).unwrap();
    }

    #[test]
    #[should_panic(expected = "not yet implemented: Environment loading for file extension json")]
    fn should_not_load_a_json_env_file() {
        let tmp = create_file("staging.json", "").unwrap();
        Environment::try_from(tmp.path().join("staging.json").as_path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "not yet implemented: Support directory environments")]
    fn should_not_load_a_env_directory() {
        let tmp = create_file("env/staging.yml", "").unwrap();
        Environment::try_from(tmp.path().join("env").as_path()).unwrap();
    }
}
