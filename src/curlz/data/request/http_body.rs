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
    // todo: not in sync with `.as_bytes()`
    pub fn contents(&self) -> std::io::Result<Option<String>> {
        Ok(match self {
            HttpBody::InlineText(c) => Some(c.to_owned()),
            HttpBody::InlineBinary(_) => todo!("Binary data cannot be represented as string yet"),
            HttpBody::Extern(f) => Some(std::fs::read_to_string(f)?),
            HttpBody::None => None,
        })
    }

    // todo: not in sync with `.contents()`
    pub fn as_bytes(&self) -> std::io::Result<&[u8]> {
        Ok(match self {
            HttpBody::InlineText(t) => t.as_bytes(),
            HttpBody::InlineBinary(b) => b.as_slice(),
            HttpBody::Extern(_) => todo!("not yet there.."),
            // HttpBody::Extern(e) => {
            //     let mut f = fs::File::open(e).unwrap();
            //     let mut data = Vec::new();
            //     f.read_to_end(&mut data).unwrap();
            //     data.as_slice()
            // }
            HttpBody::None => b"",
        })
    }
}
