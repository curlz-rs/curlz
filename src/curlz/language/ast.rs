use crate::language::tokens::Span;

// using toml::Value here only because of lazyness
pub use toml::Value;

#[derive(Debug)]
pub struct Spanned<T> {
    pub node: Box<T>,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self {
            node: Box::new(node),
            span,
        }
    }
}

#[derive(Debug)]
pub struct Template<'a> {
    pub children: Vec<Stmt<'a>>,
}

#[derive(Debug)]
pub struct Var<'a> {
    pub id: &'a str,
}

#[derive(Debug)]
pub struct SysVar<'a> {
    pub id: &'a str,
}

#[derive(Debug)]
pub struct EmitRaw<'a> {
    pub raw: &'a str,
}

#[derive(Debug)]
pub struct EmitExpr<'a> {
    pub expr: Expr<'a>,
}

#[derive(Debug)]
pub struct Const {
    pub value: Value,
}

#[derive(Debug)]
pub struct Call<'a> {
    pub expr: Expr<'a>,
    pub args: Vec<Expr<'a>>,
}

#[derive(Debug)]
pub enum Expr<'a> {
    SysVar(Spanned<SysVar<'a>>),
    Var(Spanned<Var<'a>>),
    Const(Spanned<Const>),
    Call(Spanned<Call<'a>>),
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Template(Spanned<Template<'a>>),
    EmitRaw(Spanned<EmitRaw<'a>>),
    EmitExpr(Spanned<EmitExpr<'a>>),
}

#[cfg(test)]
pub trait IntoSpanned {
    fn spanned(self) -> Spanned<Self>
    where
        Self: Sized,
    {
        Spanned::new(
            self,
            Span {
                start_line: 1,
                start_col: 0,
                end_line: 1,
                end_col: 1,
            },
        )
    }
}

#[cfg(test)]
impl<T> IntoSpanned for T {}
