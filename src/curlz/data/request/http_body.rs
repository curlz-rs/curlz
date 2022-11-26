use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum HttpBody {
    InlineText(String),
    InlineBinary(Vec<u8>),
    Extern(PathBuf),
    None,
}

impl Default for HttpBody {
    fn default() -> Self {
        Self::None
    }
}

impl HttpBody {
    pub fn contents(&self) -> std::io::Result<Option<String>> {
        Ok(match self {
            HttpBody::InlineText(c) => Some(c.to_owned()),
            HttpBody::InlineBinary(_) => todo!("Binary data cannot be represented as string yet"),
            HttpBody::Extern(f) => Some(std::fs::read_to_string(f)?),
            HttpBody::None => None,
        })
    }
}
