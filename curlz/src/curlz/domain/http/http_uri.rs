//! # HTTP URI
//! Describes a HTTP URI with template variables or functions contained
//!
//! ## not yet implemented:
//! - [ ] TODO: impl validation for URIs with placeholders

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HttpUri(String);

impl AsRef<str> for HttpUri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&str> for HttpUri {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl TryFrom<String> for HttpUri {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // todo: validation might go here
        Ok(Self(value))
    }
}
