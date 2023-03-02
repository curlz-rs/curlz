use minijinja::functions::Function;
use minijinja::value::{FunctionArgs, FunctionResult, Value};
use minijinja::Environment;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Not;
use std::path::Path;
use tempfile::TempDir;

pub mod sample_requests {
    use crate::domain::http::*;
    use indoc::indoc;

    pub fn post_request() -> HttpRequest {
        HttpRequest {
            url: "https://httpbin.org/anything".into(),
            method: HttpMethod::Post,
            version: HttpVersion::Http11,
            headers: HttpHeaders::from(
                [
                    "Accept: application/json".to_owned(),
                    "Content-Type: application/json".to_owned(),
                ]
                .as_slice(),
            ),
            body: HttpBody::InlineText(
                indoc! {r#"
                    {
                        "foo": "Bar",
                        "bool": true
                    }
                "#}
                .to_owned(),
            ),
            curl_params: Default::default(),
            placeholders: Default::default(),
        }
    }
}

/// [`RenderBuilder`] simplifies test case creation
pub struct RenderBuilder<'source> {
    env: Environment<'source>,
}

impl<'source> RenderBuilder<'source> {
    pub fn with_env_var<N>(mut self, name: N, value: impl Into<Value>) -> Self
    where
        N: Into<Cow<'source, str>>,
    {
        self.env.add_global(name, value.into());

        self
    }
}

impl<'source> Default for RenderBuilder<'source> {
    fn default() -> Self {
        Self {
            env: Environment::empty(),
        }
    }
}

impl<'source> RenderBuilder<'source> {
    /// creates a new fresh builder
    pub fn new() -> Self {
        Self::default()
    }

    /// registers a template filter function
    pub fn with_function<F, Rv, Args>(mut self, name: &'source str, f: F) -> Self
    where
        // the crazy bounds here exist to enable borrowing in closures
        F: Function<Rv, Args> + for<'a> Function<Rv, <Args as FunctionArgs<'a>>::Output>,
        Rv: FunctionResult,
        Args: for<'a> FunctionArgs<'a>,
    {
        self.env.add_function(name, f);

        self
    }

    /// registers an object as e.g. global object
    pub fn with_object<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<Cow<'source, str>>,
        V: Into<Value>,
    {
        self.env.add_global(name.into(), value.into());

        self
    }

    /// it renders a given template
    pub fn render(mut self, template: &'source str) -> String {
        let name = "render-builder-template";
        self.env.add_template(name, template).unwrap();
        let template = self.env.get_template(name).unwrap();

        let ctx = Value::default();
        template.render(&ctx).unwrap()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
