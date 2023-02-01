use crate::language::ast;
use crate::language::ast_visitor::{AstVisit, AstVisitAcceptor};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Default)]
struct Runtime<'source> {
    // todo: maybe an `Ident` should be the key for variables
    vars: HashMap<&'source str, ast::Value>,
    output: Vec<u8>,
}

impl<'source> Runtime<'source> {
    /// registers a variable with a given `id` that is the variable identifier
    pub fn with_variable(mut self, id: &'source str, var: impl Into<ast::Value>) -> Self {
        self.vars.insert(id, var.into());

        self
    }

    /// returns the rendered template as a string in form of a `Cow<'_, str>`
    pub fn rendered(&mut self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.output.as_slice())
    }

    #[cfg(test)]
    pub fn render(&mut self, source: &'source str) -> Cow<'_, str> {
        use crate::language::parser::Parser;

        let parsed = Parser::new(source).parse().unwrap();
        parsed.accept(self);

        self.rendered()
    }
}

impl<'source> AstVisit<'source> for Runtime<'source> {
    fn visit_stmt(&mut self, stmt: &ast::Stmt<'source>) {
        use ast::Stmt::*;

        match stmt {
            Template(spanned) => {
                for s in spanned.node.children.as_slice() {
                    s.accept(self);
                }
            }
            EmitRaw(spanned) => spanned.node.accept(self),
            EmitExpr(spanned) => spanned.node.expr.accept(self),
        }
    }

    fn visit_expr(&mut self, expr: &ast::Expr<'source>) {
        use ast::Expr::*;

        match expr {
            SysVar(var) => todo!(),
            Var(var) => {
                if let Some(var) = self.vars.get(var.node.id) {
                    self.output
                        .extend_from_slice(var.as_str().unwrap().as_bytes());
                }
            }
            Const(_) => {
                todo!()
            }
            Call(_) => {
                todo!()
            }
        }
    }

    fn visit_emit_raw(&mut self, raw: &ast::EmitRaw<'source>) {
        self.output.extend_from_slice(raw.raw.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::ast::IntoSpanned;

    #[test]
    fn test_expr_var() {
        let mut runtime = Runtime::default().with_variable("foo", "John");
        let expr = ast::Expr::Var(ast::Var { id: "foo" }.spanned());

        expr.accept(&mut runtime);
        assert_eq!(runtime.rendered(), "John");
    }

    #[test]
    fn test_expr_sys_var() {
        assert_eq!(
            Runtime::default().render("{{ $processEnv HOME }}"),
            env!("HOME")
        );
    }

    #[test]
    fn test_whole_template() {
        assert_eq!(
            Runtime::default()
                .with_variable("world", "John")
                .render("hello {{ world }}"),
            "hello John"
        );
    }

    #[test]
    fn test_whole_template_unhappy() {
        assert_eq!(Runtime::default().render("hello {{ world }}"), "hello ");
    }
}
