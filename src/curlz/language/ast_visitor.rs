/*!
this module contains AST related tooling such as the visitor trait and
the double dispatch (impl of [`AstVisitAcceptor`]) for all AST nodes.
*/
use crate::language::ast;

pub trait AstVisitAcceptor<'ast> {
    fn accept<V: AstVisit<'ast>>(&self, visitor: &mut V);
}

pub trait AstVisit<'ast> {
    fn visit_stmt(&mut self, _stmt: &ast::Stmt<'ast>) {}
    fn visit_expr(&mut self, _expr: &ast::Expr<'ast>) {}
    fn visit_emit_raw(&mut self, _raw: &ast::EmitRaw<'ast>) {}
}

impl<'ast> AstVisitAcceptor<'ast> for ast::Stmt<'ast> {
    fn accept<V: AstVisit<'ast>>(&self, visitor: &mut V) {
        visitor.visit_stmt(self);
    }
}

impl<'ast> AstVisitAcceptor<'ast> for ast::Expr<'ast> {
    fn accept<V: AstVisit<'ast>>(&self, visitor: &mut V) {
        visitor.visit_expr(self);
    }
}

impl<'ast> AstVisitAcceptor<'ast> for ast::EmitRaw<'ast> {
    fn accept<V: AstVisit<'ast>>(&self, visitor: &mut V) {
        visitor.visit_emit_raw(self);
    }
}
