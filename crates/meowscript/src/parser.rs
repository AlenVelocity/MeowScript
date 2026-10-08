//! Recursive descent with Pratt precedence climbing. Stops at the first error.

use crate::ast::*;
use crate::error::{Error, Result};
use crate::lexer::tokenize;
use crate::span::Span;
use crate::token::{Token, TokenKind};
use std::rc::Rc;

pub fn parse(source: &str) -> Result<Program> {
    let tokens = tokenize(source)?;
    Parser { tokens, pos: 0 }.program()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn peek_at(&self, n: usize) -> &TokenKind {
        &self.tokens[(self.pos + n).min(self.tokens.len() - 1)].kind
    }

    fn at_eof(&self) -> bool {
        matches!(self.peek_kind(), TokenKind::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if !self.at_eof() {
            self.pos += 1;
        }
        tok
    }

    fn at(&self, kind: &TokenKind) -> bool {
        self.peek_kind() == kind
    }

    fn eat(&mut self, kind: &TokenKind) -> Option<Token> {
        if self.at(kind) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn expect(&mut self, kind: &TokenKind, context: &str) -> Result<Token> {
        if self.at(kind) {
            Ok(self.advance())
        } else {
            Err(self.error_here(format!(
                "expected `{}` {context}, found {}",
                kind.symbol(),
                self.peek_kind()
            )))
        }
    }

    fn expect_ident(&mut self, context: &str) -> Result<Ident> {
        match self.peek_kind() {
            TokenKind::Ident(_) => {
                let tok = self.advance();
                let TokenKind::Ident(name) = tok.kind else {
                    unreachable!()
                };
                Ok(Ident {
                    name: name.into(),
                    span: tok.span,
                })
            }
            found => {
                let msg = match found {
                    k if TokenKind::keyword(k.symbol()).is_some() => format!(
                        "expected a name {context}, found the keyword `{}` (keywords can't be used as names)",
                        k.symbol()
                    ),
                    k => format!("expected a name {context}, found {k}"),
                };
                Err(self.error_here(msg))
            }
        }
    }

    fn error_here(&self, message: impl Into<String>) -> Error {
        let tok = self.peek();
        let err = Error::syntax(message, tok.span);
        if self.at_eof() { err.incomplete() } else { err }
    }

    fn program(&mut self) -> Result<Program> {
        let mut stmts = Vec::new();
        while !self.at_eof() {
            stmts.push(self.statement()?);
        }
        Ok(Program { stmts })
    }

    fn block(&mut self, context: &str) -> Result<Block> {
        let open = self.expect(&TokenKind::LBrace, context)?;
        let mut stmts = Vec::new();
        while !self.at(&TokenKind::RBrace) {
            if self.at_eof() {
                return Err(
                    Error::syntax("this block never closes; it needs a `}`", open.span)
                        .incomplete(),
                );
            }
            stmts.push(self.statement()?);
        }
        let close = self.advance();
        Ok(Block {
            stmts,
            span: open.span.to(close.span),
        })
    }

    fn statement(&mut self) -> Result<Stmt> {
        let start = self.peek().span;
        let kind = match self.peek_kind() {
            TokenKind::Scratch => self.let_stmt()?,
            TokenKind::Amew => self.assign_stmt()?,
            TokenKind::Tail => self.return_stmt()?,
            TokenKind::Pawckage => self.import_stmt()?,
            TokenKind::Hiss => {
                self.advance();
                self.end_statement("`hiss`")?;
                StmtKind::Break
            }
            TokenKind::Continue => {
                self.advance();
                self.end_statement("`continue`")?;
                StmtKind::Continue
            }
            TokenKind::Pawction if matches!(self.peek_at(1), TokenKind::Ident(_)) => {
                self.fn_decl()?
            }
            TokenKind::Furrever => self.loop_stmt()?,
            TokenKind::Fur => self.for_stmt()?,
            TokenKind::Semicolon => {
                return Err(self.error_here("stray `;`; there is no statement before it"));
            }
            _ => {
                let expr = self.expression(0)?;
                if expr.ends_with_block() {
                    self.eat(&TokenKind::Semicolon);
                } else {
                    self.end_statement("this expression")?;
                }
                StmtKind::Expr(expr)
            }
        };
        let end = self.tokens[self.pos.saturating_sub(1)].span;
        Ok(Stmt {
            kind,
            span: start.to(end),
        })
    }

    // The `;` may be left out before a closing `}` and at the end of the input.
    fn end_statement(&mut self, what: &str) -> Result<()> {
        if self.eat(&TokenKind::Semicolon).is_some() || self.at(&TokenKind::RBrace) || self.at_eof()
        {
            return Ok(());
        }
        let prev_end = self.tokens[self.pos.saturating_sub(1)].span.end;
        Err(Error::syntax(
            format!("expected `;` after {what}, found {}", self.peek_kind()),
            Span::new(prev_end, prev_end),
        ))
    }

    fn let_stmt(&mut self) -> Result<StmtKind> {
        self.advance();
        let name = self.expect_ident("after `scratch`")?;
        self.expect(
            &TokenKind::Assign,
            &format!("after `scratch {}`", name.name),
        )?;
        let value = self.expression(0)?;
        self.end_statement(&format!("`scratch {} = ...`", name.name))?;
        Ok(StmtKind::Let { name, value })
    }

    fn assign_stmt(&mut self) -> Result<StmtKind> {
        self.advance();
        let target = self.postfix_chain()?;
        if !matches!(
            target.kind,
            ExprKind::Ident(_) | ExprKind::Index { .. } | ExprKind::Member { .. }
        ) {
            return Err(Error::syntax(
                "`amew` can only assign to a variable, `thing's key`, or `thing[index]`",
                target.span,
            ));
        }
        self.expect(&TokenKind::Assign, "after the `amew` target")?;
        let value = self.expression(0)?;
        self.end_statement("the `amew` assignment")?;
        Ok(StmtKind::Assign { target, value })
    }

    fn return_stmt(&mut self) -> Result<StmtKind> {
        self.advance();
        let value =
            if self.at(&TokenKind::Semicolon) || self.at(&TokenKind::RBrace) || self.at_eof() {
                None
            } else {
                Some(self.expression(0)?)
            };
        self.end_statement("`tail`")?;
        Ok(StmtKind::Return { value })
    }

    fn import_stmt(&mut self) -> Result<StmtKind> {
        self.advance();
        let tok = self.advance();
        let TokenKind::Str(path) = tok.kind else {
            return Err(Error::syntax(
                format!(
                    "`pawckage` needs a quoted name like \"nya:furrball\" or \"./cat.meow\", found {}",
                    tok.kind
                ),
                tok.span,
            ));
        };
        self.end_statement("`pawckage`")?;
        Ok(StmtKind::Import {
            path,
            span: tok.span,
        })
    }

    fn fn_decl(&mut self) -> Result<StmtKind> {
        self.advance();
        let name = self.expect_ident("after `pawction`")?;
        let params = self.params()?;
        let body = self.block("to start the pawction body")?;
        self.eat(&TokenKind::Semicolon);
        let func = Rc::new(FuncDef {
            name: Some(name.name.clone()),
            params,
            body,
        });
        Ok(StmtKind::FnDecl { name, func })
    }

    fn loop_stmt(&mut self) -> Result<StmtKind> {
        self.advance();
        let cond = if self.at(&TokenKind::LBrace) {
            None
        } else {
            Some(self.expression(0)?)
        };
        let body = self.block("to start the `furrever` body")?;
        Ok(StmtKind::Loop { cond, body })
    }

    fn for_stmt(&mut self) -> Result<StmtKind> {
        self.advance();
        let parens = self.eat(&TokenKind::LParen).is_some();
        let var = self.expect_ident("after `fur`")?;
        if self.eat(&TokenKind::Tilde).is_none() {
            return Err(self.error_here(format!(
                "expected `~` after `fur {}` (as in `fur toy ~ toys`), found {}",
                var.name,
                self.peek_kind()
            )));
        }
        let iter = self.expression(0)?;
        if parens {
            self.expect(&TokenKind::RParen, "to close the `fur (...)`")?;
        }
        let body = self.block("to start the `fur` body")?;
        Ok(StmtKind::ForIn { var, iter, body })
    }

    fn expression(&mut self, min_bp: u8) -> Result<Expr> {
        let mut left = self.unary()?;
        while let Some(op) = binary_op(self.peek_kind()) {
            let (lbp, rbp) = op.binding_power();
            if lbp < min_bp {
                break;
            }
            self.advance();
            let right = self.expression(rbp)?;
            let span = left.span.to(right.span);
            left = Expr {
                kind: ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr> {
        let op = match self.peek_kind() {
            TokenKind::Minus => UnaryOp::Neg,
            TokenKind::Bang => UnaryOp::Not,
            TokenKind::Furreal => UnaryOp::Typeof,
            _ => return self.postfix_chain(),
        };
        let start = self.advance().span;
        let operand = self.unary()?;
        let span = start.to(operand.span);
        Ok(Expr {
            kind: ExprKind::Unary {
                op,
                operand: Box::new(operand),
            },
            span,
        })
    }

    fn postfix_chain(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;
        loop {
            expr = match self.peek_kind() {
                TokenKind::LParen => {
                    self.advance();
                    let args = self.comma_list(
                        &TokenKind::RParen,
                        "to close the call",
                        Self::expression0,
                    )?;
                    let end = self.tokens[self.pos - 1].span;
                    let span = expr.span.to(end);
                    Expr {
                        kind: ExprKind::Call {
                            callee: Box::new(expr),
                            args,
                        },
                        span,
                    }
                }
                TokenKind::LBracket => {
                    self.advance();
                    let index = self.expression(0)?;
                    let close = self.expect(&TokenKind::RBracket, "to close the index")?;
                    let span = expr.span.to(close.span);
                    Expr {
                        kind: ExprKind::Index {
                            object: Box::new(expr),
                            index: Box::new(index),
                        },
                        span,
                    }
                }
                TokenKind::Possessive => {
                    self.advance();
                    let name = self.expect_ident("after `'s`")?;
                    let span = expr.span.to(name.span);
                    Expr {
                        kind: ExprKind::Member {
                            object: Box::new(expr),
                            name,
                        },
                        span,
                    }
                }
                _ => return Ok(expr),
            };
        }
    }

    fn expression0(&mut self) -> Result<Expr> {
        self.expression(0)
    }

    fn primary(&mut self) -> Result<Expr> {
        let tok = self.peek().clone();
        let span = tok.span;
        let kind = match tok.kind {
            TokenKind::Number(n) => {
                self.advance();
                ExprKind::Number(n)
            }
            TokenKind::Str(s) => {
                self.advance();
                ExprKind::Str(s.into())
            }
            TokenKind::Ident(name) => {
                self.advance();
                ExprKind::Ident(name.into())
            }
            TokenKind::Purrfect => {
                self.advance();
                ExprKind::Bool(true)
            }
            TokenKind::Clawful => {
                self.advance();
                ExprKind::Bool(false)
            }
            TokenKind::Mew => {
                self.advance();
                ExprKind::Null
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.expression(0)?;
                let close = self.expect(&TokenKind::RParen, "to close the parenthesis")?;
                return Ok(Expr {
                    kind: inner.kind,
                    span: span.to(close.span),
                });
            }
            TokenKind::LBracket => {
                self.advance();
                let items = self.comma_list(
                    &TokenKind::RBracket,
                    "to close the furrball",
                    Self::expression0,
                )?;
                let end = self.tokens[self.pos - 1].span;
                return Ok(Expr {
                    kind: ExprKind::Array(items),
                    span: span.to(end),
                });
            }
            TokenKind::LBrace => return self.object_literal(),
            TokenKind::Pawction => return self.function_expr(),
            TokenKind::Purrhaps => return self.if_expr(),
            TokenKind::Meowtually => {
                return Err(self.error_here("`meowtually` needs a `purrhaps` before it"));
            }
            TokenKind::Eof => {
                return Err(self.error_here("expected an expression, found end of input"));
            }
            other => {
                let hint = match other {
                    TokenKind::Amew => " (did you mean `scratch` to declare a new variable?)",
                    TokenKind::Assign => " (use `==` to compare, `amew` to assign)",
                    _ => "",
                };
                return Err(self.error_here(format!("expected an expression, found {other}{hint}")));
            }
        };
        Ok(Expr { kind, span })
    }

    fn if_expr(&mut self) -> Result<Expr> {
        let start = self.advance().span;
        let cond = self.expression(0)?;
        let then = self.block("to start the `purrhaps` body")?;
        let mut end = then.span;
        let otherwise = if self.eat(&TokenKind::Meowtually).is_some() {
            if self.at(&TokenKind::Purrhaps) {
                let nested = self.if_expr()?;
                end = nested.span;
                Some(Box::new(Else::If(nested)))
            } else {
                let block = self.block("after `meowtually`")?;
                end = block.span;
                Some(Box::new(Else::Block(block)))
            }
        } else {
            None
        };
        Ok(Expr {
            kind: ExprKind::If {
                cond: Box::new(cond),
                then,
                otherwise,
            },
            span: start.to(end),
        })
    }

    fn function_expr(&mut self) -> Result<Expr> {
        let start = self.advance().span;
        let name = match self.peek_kind() {
            TokenKind::Ident(_) => Some(self.expect_ident("")?.name),
            _ => None,
        };
        let params = self.params()?;
        let body = self.block("to start the pawction body")?;
        let span = start.to(body.span);
        Ok(Expr {
            kind: ExprKind::Function(Rc::new(FuncDef { name, params, body })),
            span,
        })
    }

    fn params(&mut self) -> Result<Vec<Ident>> {
        self.expect(&TokenKind::LParen, "to start the parameter list")?;
        let params = self.comma_list(&TokenKind::RParen, "to close the parameter list", |p| {
            p.expect_ident("in the parameter list")
        })?;
        let mut seen: Vec<&str> = Vec::new();
        for p in &params {
            if seen.contains(&&*p.name) {
                return Err(Error::syntax(
                    format!("parameter `{}` is listed twice", p.name),
                    p.span,
                ));
            }
            seen.push(&p.name);
        }
        Ok(params)
    }

    fn object_literal(&mut self) -> Result<Expr> {
        let start = self.advance().span;
        let mut entries = Vec::new();
        loop {
            if self.at(&TokenKind::RBrace) {
                break;
            }
            let key_tok = self.advance();
            let key = match key_tok.kind {
                TokenKind::Ident(name) => {
                    // `{ name }` is shorthand for `{ name: name }`.
                    if self.at(&TokenKind::Comma) || self.at(&TokenKind::RBrace) {
                        let key = Expr {
                            kind: ExprKind::Str(name.clone().into()),
                            span: key_tok.span,
                        };
                        let value = Expr {
                            kind: ExprKind::Ident(name.into()),
                            span: key_tok.span,
                        };
                        entries.push((key, value));
                        if self.eat(&TokenKind::Comma).is_none() {
                            break;
                        }
                        continue;
                    }
                    Expr {
                        kind: ExprKind::Str(name.into()),
                        span: key_tok.span,
                    }
                }
                TokenKind::Str(s) => Expr {
                    kind: ExprKind::Str(s.into()),
                    span: key_tok.span,
                },
                TokenKind::Number(n) => Expr {
                    kind: ExprKind::Str(crate::value::format_number(n).into()),
                    span: key_tok.span,
                },
                TokenKind::LBracket => {
                    let inner = self.expression(0)?;
                    let close = self.expect(&TokenKind::RBracket, "to close the computed key")?;
                    Expr {
                        kind: inner.kind,
                        span: key_tok.span.to(close.span),
                    }
                }
                TokenKind::Eof => {
                    return Err(
                        Error::syntax("this object never closes; it needs a `}`", start)
                            .incomplete(),
                    );
                }
                other => {
                    return Err(Error::syntax(
                        format!("object keys are names or strings, found {other}"),
                        key_tok.span,
                    ));
                }
            };
            self.expect(&TokenKind::Colon, "after the object key")?;
            let value = self.expression(0)?;
            entries.push((key, value));
            if self.eat(&TokenKind::Comma).is_none() {
                break;
            }
        }
        let close = self.expect(&TokenKind::RBrace, "to close the object")?;
        Ok(Expr {
            kind: ExprKind::Object(entries),
            span: start.to(close.span),
        })
    }

    // Allows a trailing comma and an empty list.
    fn comma_list<T>(
        &mut self,
        close: &TokenKind,
        context: &str,
        mut item: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        let mut items = Vec::new();
        loop {
            if self.eat(close).is_some() {
                return Ok(items);
            }
            if self.at_eof() {
                return Err(self.error_here(format!("expected `{}` {context}", close.symbol())));
            }
            items.push(item(self)?);
            if self.eat(&TokenKind::Comma).is_none() {
                self.expect(close, context)?;
                return Ok(items);
            }
        }
    }
}

fn binary_op(kind: &TokenKind) -> Option<BinaryOp> {
    Some(match kind {
        TokenKind::Plus => BinaryOp::Add,
        TokenKind::Minus => BinaryOp::Sub,
        TokenKind::Star => BinaryOp::Mul,
        TokenKind::Slash => BinaryOp::Div,
        TokenKind::Percent => BinaryOp::Rem,
        TokenKind::Eq => BinaryOp::Eq,
        TokenKind::NotEq => BinaryOp::NotEq,
        TokenKind::Lt => BinaryOp::Lt,
        TokenKind::Gt => BinaryOp::Gt,
        TokenKind::LtEq => BinaryOp::LtEq,
        TokenKind::GtEq => BinaryOp::GtEq,
        TokenKind::Amp => BinaryOp::BitAnd,
        TokenKind::Pipe => BinaryOp::BitOr,
        TokenKind::Caret => BinaryOp::BitXor,
        TokenKind::Shl => BinaryOp::Shl,
        TokenKind::Shr => BinaryOp::Shr,
        TokenKind::AndAnd => BinaryOp::And,
        TokenKind::OrOr => BinaryOp::Or,
        TokenKind::Tilde => BinaryOp::In,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expr(src: &str) -> Expr {
        let program = parse(src).unwrap();
        match program.stmts.into_iter().next().unwrap().kind {
            StmtKind::Expr(e) => e,
            other => panic!("expected expression statement, got {other:?}"),
        }
    }

    fn show(e: &Expr) -> String {
        match &e.kind {
            ExprKind::Number(n) => crate::value::format_number(*n),
            ExprKind::Ident(i) => i.to_string(),
            ExprKind::Str(s) => format!("{s:?}"),
            ExprKind::Bool(b) => b.to_string(),
            ExprKind::Null => "mew".into(),
            ExprKind::Unary { op, operand } => format!("({op} {})", show(operand)),
            ExprKind::Binary { op, left, right } => {
                format!("({} {op} {})", show(left), show(right))
            }
            ExprKind::Call { callee, args } => {
                let args: Vec<_> = args.iter().map(show).collect();
                format!("{}({})", show(callee), args.join(", "))
            }
            ExprKind::Index { object, index } => format!("{}[{}]", show(object), show(index)),
            ExprKind::Member { object, name } => format!("{}'s {}", show(object), name.name),
            ExprKind::Array(items) => {
                let items: Vec<_> = items.iter().map(show).collect();
                format!("[{}]", items.join(", "))
            }
            other => format!("{other:?}"),
        }
    }

    #[test]
    fn precedence_matches_python_not_c() {
        assert_eq!(show(&parse_expr("1 + 2 * 3;")), "(1 + (2 * 3))");
        assert_eq!(show(&parse_expr("1 & 2 == 3;")), "((1 & 2) == 3)");
        assert_eq!(show(&parse_expr("1 + 2 << 3;")), "((1 + 2) << 3)");
        assert_eq!(show(&parse_expr("a || b && c;")), "(a || (b && c))");
        assert_eq!(show(&parse_expr("a < b == c > d;")), "((a < b) == (c > d))");
        assert_eq!(show(&parse_expr("1 - 2 - 3;")), "((1 - 2) - 3)");
        assert_eq!(show(&parse_expr("x ~ xs && y;")), "((x ~ xs) && y)");
    }

    #[test]
    fn unary_binds_tighter_than_binary_but_looser_than_postfix() {
        assert_eq!(show(&parse_expr("-a[0] + 1;")), "((- a[0]) + 1)");
        assert_eq!(show(&parse_expr("!f(x)'s ok;")), "(! f(x)'s ok)");
        assert_eq!(show(&parse_expr("furreal x + 1;")), "((furreal x) + 1)");
    }

    #[test]
    fn postfix_chains() {
        assert_eq!(
            show(&parse_expr("cats[0]'s toys[1](2);")),
            "cats[0]'s toys[1](2)"
        );
    }

    #[test]
    fn statements_need_semicolons_except_before_brace_or_eof() {
        assert!(parse("scratch x = 1").is_ok());
        assert!(parse("scratch x = 1; scratch y = 2").is_ok());
        let err = parse("scratch x = 1 scratch y = 2;").unwrap_err();
        assert!(err.message.contains("expected `;`"), "{}", err.message);
        assert!(parse("pawction f() { tail 1 }").is_ok());
        assert!(parse("purrhaps (x) { 1 } meowtually { 2 } meow(1);").is_ok());
    }

    #[test]
    fn if_else_chains() {
        let e = parse_expr("purrhaps a { 1 } meowtually purrhaps b { 2 } meowtually { 3 }");
        let ExprKind::If {
            otherwise: Some(else1),
            ..
        } = e.kind
        else {
            panic!()
        };
        let Else::If(nested) = *else1 else { panic!() };
        let ExprKind::If {
            otherwise: Some(else2),
            ..
        } = nested.kind
        else {
            panic!()
        };
        assert!(matches!(*else2, Else::Block(_)));
    }

    #[test]
    fn function_declaration_and_expression() {
        let p = parse("pawction add(a, b) { tail a + b; } scratch f = pawction(x) { x };").unwrap();
        assert!(matches!(p.stmts[0].kind, StmtKind::FnDecl { .. }));
        assert!(matches!(p.stmts[1].kind, StmtKind::Let { .. }));
        assert!(
            parse("pawction f(a, a) {}")
                .unwrap_err()
                .message
                .contains("twice")
        );
    }

    #[test]
    fn assignment_targets() {
        assert!(parse("amew x = 1;").is_ok());
        assert!(parse("amew cat's age = 4;").is_ok());
        assert!(parse("amew arr[0] = 4;").is_ok());
        assert!(parse("amew f() = 4;").is_err());
        assert!(parse("amew 1 = 4;").is_err());
    }

    #[test]
    fn loops() {
        assert!(parse("furrever { hiss; }").is_ok());
        assert!(parse("furrever i < 3 { amew i = i + 1; }").is_ok());
        assert!(parse("furrever (i < 3) { continue; }").is_ok());
        assert!(parse("fur x ~ xs { meow(x); }").is_ok());
        assert!(parse("fur (x ~ xs) { meow(x); }").is_ok());
        assert!(parse("fur x in xs {}").unwrap_err().message.contains("`~`"));
    }

    #[test]
    fn object_literals() {
        assert!(parse(r#"scratch c = { name: "Tom", "age": 3, 1: 2, [k]: v, short, };"#).is_ok());
        assert!(parse("scratch c = {};").is_ok());
        assert!(parse("scratch c = { 1 + 1: 2 };").is_err());
    }

    #[test]
    fn incomplete_input_is_flagged() {
        assert!(parse("pawction f() {").unwrap_err().incomplete);
        assert!(parse("scratch x = [1, 2").unwrap_err().incomplete);
        assert!(parse("scratch x = ").unwrap_err().incomplete);
        assert!(!parse("scratch x = )").unwrap_err().incomplete);
    }

    #[test]
    fn keyword_as_name_gets_a_hint() {
        let err = parse("scratch tail = 1;").unwrap_err();
        assert!(err.message.contains("keyword"), "{}", err.message);
    }
}
