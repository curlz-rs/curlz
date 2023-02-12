use crate::language::ast;
use crate::language::ast::Spanned;
use crate::language::lexer::tokenize;
use crate::language::tokens::{Span, Token};
use anyhow::Error;
use toml::Value;

pub struct Parser<'a> {
    tokens: Box<dyn Iterator<Item = Result<(Token<'a>, Span), Error>> + 'a>,
    current_token: Option<Result<(Token<'a>, Span), Error>>,
    last_span: Span,
}

impl<'a> Parser<'a> {
    fn expand_span(&self, mut span: Span) -> Span {
        span.end_line = self.last_span.end_line;
        span.end_col = self.last_span.end_col;
        span
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut tokens = Box::new(tokenize(source));
        let current_token = tokens.next();
        let last_span = Default::default();

        Self {
            tokens,
            current_token,
            last_span,
        }
    }

    pub fn parse(&mut self) -> Result<ast::Stmt<'a>, Error> {
        self.parse_template()
    }

    fn parse_template(&mut self) -> Result<ast::Stmt<'a>, Error> {
        let span = self.last_span;
        Ok(ast::Stmt::Template(Spanned::new(
            ast::Template {
                children: {
                    let mut rv = Vec::new();
                    while let Some(Ok((token, span))) = self.current_token.take() {
                        match token {
                            Token::TemplateData(raw) => rv
                                .push(ast::Stmt::EmitRaw(Spanned::new(ast::EmitRaw { raw }, span))),
                            Token::VariableStart => {
                                let expr = self.parse_expr()?;
                                rv.push(ast::Stmt::EmitExpr(Spanned::new(
                                    ast::EmitExpr { expr },
                                    self.expand_span(span),
                                )));
                                self.ensure_next_token(Token::VariableEnd);
                            }
                            _ => unreachable!("the lexer messed hard up"),
                        }
                        self.current_token = self.tokens.next();
                    }
                    rv
                },
            },
            self.expand_span(span),
        )))
    }

    // todo: this could also be a macro:
    //  expect_token!(self, Token::VariableEnd, "end of variable block");
    fn ensure_next_token(&mut self, expected_token: Token) {
        if let Some(Ok((token, span))) = self.tokens.next() {
            if token != expected_token {
                panic!("{expected_token} was not found at {span:?}");
            }
        } else {
            panic!("{expected_token} was not found at {:?}", self.last_span);
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expr<'a>, Error> {
        // todo: this would not remain here, ident is only the most simplest expression
        self.parse_ident()
    }

    fn parse_ident(&mut self) -> Result<ast::Expr<'a>, Error> {
        if let Some(Ok((token, span))) = self.tokens.next() {
            match token {
                Token::VarIdent("true" | "True") => Ok(ast::Expr::Const(Spanned::new(
                    ast::Const {
                        value: Value::Boolean(true),
                    },
                    span,
                ))),
                Token::VarIdent(name) => {
                    Ok(ast::Expr::Var(Spanned::new(ast::Var { id: name }, span)))
                }
                Token::SysVarIdent(name) => {
                    let expr = ast::Expr::SysVar(Spanned::new(ast::SysVar { id: name }, span));
                    match name {
                        // has one argument
                        "processEnv" => {
                            let arg = self.parse_ident()?;
                            Ok(ast::Expr::Call(Spanned::new(
                                ast::Call {
                                    expr,
                                    args: vec![arg],
                                },
                                span,
                            )))
                        }
                        &_ => Ok(expr),
                    }
                }
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod expr {
        use super::*;

        #[test]
        fn test_parse_boolean() {
            let mut p = Parser::new("hello {{ true }}");
            insta::assert_debug_snapshot!(p.parse());
        }

        #[test]
        fn test_parse_var() {
            let mut p = Parser::new("hello {{ world }}");
            insta::assert_debug_snapshot!(p.parse());
        }

        #[test]
        fn test_parse_sys_var() {
            let mut p = Parser::new("hello {{ $HOME }}");
            insta::assert_debug_snapshot!(p.parse());
        }

        #[test]
        fn test_parse_process_env() {
            let mut p = Parser::new("hello {{ $processEnv HOME }}");
            insta::assert_debug_snapshot!(p.parse());
        }
    }
}
