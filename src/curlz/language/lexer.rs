use anyhow::{anyhow, Error};
use std::ops::Not;

use crate::language::tokens::{Span, Token};

enum LexerState {
    Template,
    InVariable,
}

struct TokenizerState<'s> {
    stack: Vec<LexerState>,
    rest: &'s str,
    failed: bool,
    current_line: usize,
    current_col: usize,
}

impl<'s> TokenizerState<'s> {}

impl<'s> TokenizerState<'s> {
    /// advance by `n_bytes` and keeps track of the position in the stream
    fn advance(&mut self, n_bytes: usize) -> &'s str {
        let (skipped, new_rest) = self.rest.split_at(n_bytes);
        self.rest = new_rest;

        skipped.chars().for_each(|c| match c {
            '\n' => {
                self.current_line += 1;
                self.current_col = 0;
            }
            _ => self.current_col += 1,
        });

        skipped
    }

    /// advance forward for as long as whitespaces appear
    fn skip_whitespaces(&mut self) {
        let skip = self
            .rest
            .chars()
            .map_while(|c| c.is_whitespace().then(|| c.len_utf8()))
            .sum::<usize>();
        if skip > 0 {
            self.advance(skip);
        }
    }

    #[inline(always)]
    fn loc(&self) -> (usize, usize) {
        (self.current_line, self.current_col)
    }

    fn span(&self, (start_line, start_col): (usize, usize)) -> Span {
        Span {
            start_line,
            start_col,
            end_line: self.current_line,
            end_col: self.current_col,
        }
    }

    fn eat_identifier(&mut self) -> Result<(Token<'s>, Span), Error> {
        let ident_len = lex_identifier(self.rest);
        if ident_len > 0 {
            let old_loc = self.loc();
            let ident = self.advance(ident_len);
            let token = if let Some(b'$') = ident.as_bytes().get(0) {
                Token::SysVarIdent(&ident[1..])
            } else {
                Token::VarIdent(ident)
            };

            Ok((token, self.span(old_loc)))
        } else {
            Err(self.syntax_error("unexpected character"))
        }
    }

    fn syntax_error(&mut self, msg: &'static str) -> Error {
        self.failed = true;
        anyhow!(msg)
        // Error::new(ErrorKind::SyntaxError, msg)
    }
}

fn lex_identifier(s: &str) -> usize {
    s.chars()
        .enumerate()
        .map_while(|(idx, c)| {
            let cont = if c == '_' || c == '$' || c == '-' {
                true
            } else if idx == 0 {
                unicode_ident::is_xid_start(c)
            } else {
                unicode_ident::is_xid_continue(c)
            };
            cont.then(|| c.len_utf8())
        })
        .sum::<usize>()
}

fn memchr(haystack: &[u8], needle: u8) -> Option<usize> {
    haystack.iter().position(|&x| x == needle)
}

#[inline(always)]
fn find_marker(a: &str) -> Option<usize> {
    let bytes = a.as_bytes();
    let mut offset = 0;
    loop {
        if let Some(idx) = memchr(&bytes[offset..], b'{') {
            if let Some(b'{') = bytes.get(offset + idx + 1).copied() {
                // this prevents the `${{` situation
                if let Some(b'$') = bytes.get(offset + idx - 1) {
                    break None;
                } else {
                    break Some(offset + idx);
                }
            }
            offset += idx + 1;
        } else {
            break None;
        }
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Result<(Token<'_>, Span), Error>> {
    let mut state = TokenizerState {
        rest: input,
        stack: vec![LexerState::Template],
        failed: false,
        current_line: 1,
        current_col: 0,
    };

    std::iter::from_fn(move || loop {
        if state.rest.is_empty() || state.failed {
            return None;
        }

        let prev_loc = state.loc();
        match state.stack.last() {
            Some(LexerState::Template) => {
                if let Some("{{") = state.rest.get(..2) {
                    // entering the `InVariable` state
                    state.advance(2);
                    state.stack.push(LexerState::InVariable);
                    return Some(Ok((Token::VariableStart, state.span(prev_loc))));
                }

                let (lead, span) = match find_marker(state.rest) {
                    Some(start) => (state.advance(start), state.span(prev_loc)),
                    None => (state.advance(state.rest.len()), state.span(prev_loc)),
                };

                if lead.is_empty().not() {
                    return Some(Ok((Token::TemplateData(lead), span)));
                }
            }
            Some(LexerState::InVariable) => {
                state.skip_whitespaces();

                if let Some("}}") = state.rest.get(..2) {
                    state.stack.pop();
                    state.advance(2);
                    return Some(Ok((Token::VariableEnd, state.span(prev_loc))));
                }

                return Some(state.eat_identifier());
            }
            None => todo!("lexer state is empty!?"),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_data() {
        match tokenize("hello {{ world }}").next() {
            Some(Ok((Token::TemplateData(data), _))) if data == "hello " => {}
            s => panic!("did not get a matching token result: {:?}", s),
        }
    }

    #[test]
    fn test_template_data_with_dollar_2braces() {
        match tokenize("hello ${{ world }}").next() {
            Some(Ok((Token::TemplateData(data), _))) if data == "hello ${{ world }}" => {}
            s => panic!("did not get a matching token result: {:?}", s),
        }
    }

    #[test]
    fn test_tokenize_var_ident() {
        let mut tokens = tokenize("hello {{ world }}").skip(1);

        assert_eq!(tokens.next().unwrap().unwrap().0, Token::VariableStart);

        match tokens.next() {
            Some(Ok((Token::VarIdent(id), _))) if id == "world" => {}
            s => panic!("did not get a matching token result: {:?}", s),
        }

        assert_eq!(tokens.next().unwrap().unwrap().0, Token::VariableEnd);
        assert!(tokens.next().is_none())
    }

    #[test]
    fn test_tokenize_var_ident_containing_dash() {
        let mut tokens = tokenize("hello {{ new-world }}").skip(2);

        match tokens.next() {
            Some(Ok((Token::VarIdent(id), _))) if id == "new-world" => {}
            s => panic!("did not get a matching token result: {:?}", s),
        }

        assert_eq!(tokens.next().unwrap().unwrap().0, Token::VariableEnd);
        assert!(tokens.next().is_none())
    }

    #[test]
    fn test_tokenize_sys_var_ident() {
        let mut tokens = tokenize("hello {{ $world }}").skip(2);

        match tokens.next() {
            Some(Ok((Token::SysVarIdent(id), _))) if id == "world" => {}
            s => panic!("did not get a matching token result: {:?}", s),
        }

        assert_eq!(tokens.next().unwrap().unwrap().0, Token::VariableEnd);
    }

    #[test]
    fn test_tokenize_sys_var_ident_with_a_argument() {
        let mut tokens = tokenize("hello {{ $processEnv envVarName }}").skip(2);

        match tokens.next() {
            Some(Ok((Token::SysVarIdent(id), _))) if id == "processEnv" => {}
            s => panic!("did not get a matching token result: {:?}", s),
        }

        match tokens.next() {
            Some(Ok((Token::VarIdent(id), _))) if id == "envVarName" => {}
            s => panic!("did not get a matching token result: {:?}", s),
        }

        assert_eq!(tokens.next().unwrap().unwrap().0, Token::VariableEnd);
    }
}
