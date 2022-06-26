use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct HttpHeaders(Vec<(String, String)>);

impl HttpHeaders {
    /// reverses the order of headers
    pub fn reverse(&mut self) {
        self.0.reverse();
    }
}

impl HttpHeaders {
    /// adds a new header in the form of key, value
    pub fn push(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        self.0
            .push((key.as_ref().to_string(), value.as_ref().to_string()));
    }
}

impl AsRef<[(String, String)]> for HttpHeaders {
    fn as_ref(&self) -> &[(String, String)] {
        self.0.as_slice()
    }
}
