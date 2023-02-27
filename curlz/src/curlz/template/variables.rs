use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Placeholder {
    pub name: String,
    pub value: Option<String>,
    pub default: Option<String>,
    pub prompt: Option<String>,
}

impl Placeholder {
    pub fn new(key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        Self {
            name: key.as_ref().to_owned(),
            value: value.as_ref().to_owned().into(),
            default: None,
            prompt: None,
        }
    }
}
