use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateSlots {
    pub(crate) var_name: String,
    pub(crate) var_info: VarInfo,
    pub(crate) prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VarInfo {
    Bool { default: Option<bool> },
    String { entry: Box<StringEntry> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringEntry {
    pub(crate) default: Option<String>,
    pub(crate) choices: Option<Vec<String>>,
    #[serde(with = "serde_regex")]
    pub(crate) regex: Option<Regex>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum VarDataType {
    Bool,
    String,
}
