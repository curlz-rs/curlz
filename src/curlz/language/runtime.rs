use crate::language::ast;
use crate::language::ast::Spanned;
use std::collections::HashMap;

#[derive(Default)]
struct Runtime<'source> {
    // todo: maybe an `Ident` should be the key for variables
    vars: HashMap<&'source str, ast::Value>,
    interpreted: Vec<u8>,
}

trait Visitable<'source> {
    fn accept<V: Visitor<'source>>(&self, visitor: &mut V);
}

trait Visitor<'source> {
    fn visit_stmt(&mut self, stmt: &ast::Stmt<'source>);
    fn visit_expr(&mut self, expr: &ast::Expr<'source>);
    fn visit_emit_raw(&mut self, raw: &ast::EmitRaw<'source>);
}

impl<'source> Visitable<'source> for ast::Stmt<'source> {
    fn accept<V: Visitor<'source>>(&self, visitor: &mut V) {
        visitor.visit_stmt(self);
    }
}

impl<'source> Visitable<'source> for ast::Expr<'source> {
    fn accept<V: Visitor<'source>>(&self, visitor: &mut V) {
        visitor.visit_expr(self);
    }
}

impl<'source> Visitable<'source> for ast::EmitRaw<'source> {
    fn accept<V: Visitor<'source>>(&self, visitor: &mut V) {
        visitor.visit_emit_raw(self);
    }
}

impl<'source> Visitor<'source> for Runtime<'source> {
    fn visit_stmt(&mut self, stmt: &ast::Stmt<'source>) {
        match stmt {
            ast::Stmt::Template(spanned) => {
                for s in spanned.node.children.as_slice() {
                    s.accept(self);
                }
            }
            ast::Stmt::EmitRaw(spanned) => spanned.node.accept(self),
            ast::Stmt::EmitExpr(spanned) => spanned.node.expr.accept(self),
        }
    }

    fn visit_expr(&mut self, expr: &ast::Expr<'source>) {
        use ast::Expr;

        match expr {
            Expr::SysVar(var) => todo!(),
            Expr::Var(var) => {
                if let Some(var) = self.vars.get(var.node.id) {
                    self.interpreted
                        .extend_from_slice(var.as_str().unwrap().as_bytes());
                }
            }
            Expr::Const(_) => {
                todo!()
            }
            Expr::Call(_) => {
                todo!()
            }
        }
    }

    fn visit_emit_raw(&mut self, raw: &ast::EmitRaw<'source>) {
        self.interpreted.extend_from_slice(raw.raw.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::parser::Parser;

    #[test]
    fn test_x() {
        let mut p = Parser::new("hello {{ world }}");
        let stmt = p.parse().unwrap();
        let mut runtime = Runtime::default();
        runtime
            .vars
            .insert("world", ast::Value::String("John".to_string()));

        stmt.accept(&mut runtime);

        let result = String::from_utf8_lossy(runtime.interpreted.as_slice());
        assert_eq!(result, "hello John");
    }
}
