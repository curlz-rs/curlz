use std::fmt::{Debug, Display, Formatter};

/// Represents a Token
#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    /// Raw template data
    TemplateData(&'a str),
    /// Variable block starts after a "{{"
    VariableStart,
    /// Variable block end with a "}}"
    VariableEnd,
    /// An identifier for a variable
    VarIdent(&'a str),
    /// An identifier for a system variable
    SysVarIdent(&'a str),
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::TemplateData(_) => write!(f, "template-data"),
            Token::VariableStart => write!(f, "start of variable block"),
            Token::VariableEnd => write!(f, "end of variable block"),
            Token::VarIdent(_) => write!(f, "variable identifier"),
            Token::SysVarIdent(_) => write!(f, "system variable identifier"),
        }
    }
}

/// Token span information
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Span {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " @ {}:{}-{}:{}",
            self.start_line, self.start_col, self.end_line, self.end_col
        )
    }
}
