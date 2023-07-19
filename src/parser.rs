use crate::{ast::{*, token::Token}, lexer::Lexer};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>
}

impl Parser {

    pub fn new(lexer: Lexer) -> Self {
        let mut p: Parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = vec![];
        while self.current_token != Token::Eof {
            let stmt: Option<Statement> = self.parse_statement();
            if stmt != None {
                statements.push(stmt.unwrap());
            };
            self.next_token();
        }
        Program { statements }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Set => self.parse_set_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Include => self.parse_include_statement(),
            Token::Anew => self.parse_anew_expr(),
            Token::Break => self.parse_break_statement(),
            Token::Continue => self.parse_continue_statement(),
            _ => self.parse_expr_statement(),
        }
    }


    pub fn parse_expr_statement(&mut self) -> Option<Statement> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expression) => {
                if self.peek_token(&Token::Semicolon) {
                    self.next_token();
                }
                Some(Statement::Expression(expression))
            }
            None => None,
        }
    }

    pub fn parse_set_statement(&mut self) -> Option<Statement> {
        match &self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => {
                self.peek_error(Token::Ident(String::new()));
                return None;
            }
        }

        let name: Ident = match self.parse_ident() {
            Some(Expr::Ident(ref mut s)) => s.clone(),
            _ => return None,
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();

        let lit: Expr = match self.parse_expr(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };

        while !self.current_token(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Set(name, lit))
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let exp = match self.parse_expr(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };

        while !self.current_token(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return(exp))
    }

    pub fn parse_include_statement(&mut self) -> Option<Statement> {
        self.next_token();
        // the string next to the include keyword is the lib
        let lib = match &self.current_token {
            Token::String(ref s) => s.clone(),
            _ => {
                self.peek_error(Token::String(String::new()));
                return None;
            } 
        };
        while !self.current_token(Token::Semicolon) {
            self.next_token();
        }
        Some(Statement::Include(lib))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        self.next_token();

        let mut statements = vec![];
        while !self.current_token(Token::RightBrace) && !self.current_token(Token::Eof) {
            if let Some(s) = self.parse_statement() {
                statements.push(s);
            }
            self.next_token();
        }

        statements
    }

    pub fn parse_anew_expr(&mut self) -> Option<Statement> {
        match &self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => {
                self.peek_error(Token::Ident(String::new()));
                return None;
            }
        }

        let name: Ident = match self.parse_ident() {
            Some(Expr::Ident(ref mut s)) => s.clone(),
            _ => return None,
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();

        let lit: Expr = match self.parse_expr(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };

        while !self.current_token(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Anew(name, lit))
    }

    pub fn parse_break_statement(&mut self) -> Option<Statement> {
        self.next_token();
        while !self.current_token(Token::Semicolon) {
            self.next_token();
        }
        Some(Statement::Break)
    }

    pub fn parse_continue_statement(&mut self) -> Option<Statement> {
        self.next_token();
        while !self.current_token(Token::Semicolon) {
            self.next_token();
        }
        Some(Statement::Continue)
    }

    fn parse_typof_expr(&mut self) -> Option<Expr> {
        self.next_token();
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };
        Some(Expr::Typeof { expr: Box::new(expr) })
    }

    fn parse_loop_expr(&mut self) -> Option<Expr> {
        self.next_token();
        let body = self.parse_block_statement();
        Some(Expr::Loop { body })
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left: Option<Expr> = match self.current_token {
            Token::Ident(_) => self.parse_ident(),
            Token::Bang | Token::Minus | Token::Plus => self.parse_prefix_expr(),
            Token::LeftParen => self.parse_grouped_expr(),
            Token::If => self.parse_if_expr(),
            Token::Func => self.parse_fn_expr(),
            Token::Number(_) => self.parse_int_literal(),
            Token::Boolean(_) => self.parse_boolean_literal(),
            Token::String(_) => self.parse_string_literal(),
            Token::LeftBracket => self.parse_array_literal(),
            Token::LeftBrace => self.parse_object_literal(),
            Token::Typeof => self.parse_typof_expr(),
            Token::Loop => self.parse_loop_expr(),
            _ => {
                None
            }
        };

        while !self.peek_token(&Token::Semicolon) && precedence < self.next_token_precedence() {
            match self.peek_token {
                Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Equals
                | Token::Slash
                | Token::Percent
                | Token::NotEquals
                | Token::Less
                | Token::LessEqual
                | Token::Greater
                | Token::GreaterEqual
                | Token::AND
                | Token::OR
                | Token::XOR
                | Token::In => {
                    self.next_token();
                    left = self.parse_infix_expr(left.unwrap());
                }

                Token::LeftShift => {
                    self.lexer.next_token();
                    self.next_token();
                    left = self.parse_infix_expr(left.unwrap());
                }

                Token::RightShift => {
                    self.lexer.next_token();
                    self.next_token();
                    left = self.parse_infix_expr(left.unwrap());
                }

                Token::LeftParen => {
                    self.next_token();
                    left = self.parse_call_expr(left.unwrap());
                }
                Token::LeftBracket => {
                    self.next_token();
                    left = self.parse_index_expr(left.unwrap());
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_object_literal(&mut self) -> Option<Expr> {
        let mut obj = vec![];
        while !self.peek_token(&Token::RightBrace) {
            self.next_token();
            let key = match self.parse_expr(Precedence::Lowest) {
                Some(o) => o,
                None => return None,
            };
            if !self.expect_peek(Token::Colon) {
                return None;
            }

            self.next_token();
            let val = match self.parse_expr(Precedence::Lowest) {
                Some(o) => o,
                None => return None,
            };
            obj.push((key, val));
            if !self.peek_token(&Token::RightBrace) && !self.expect_peek(Token::Comma) {
                return None;
            }
        }
        if !self.expect_peek(Token::RightBrace) {
            return None;
        }

        Some(Expr::Literal(Literal::Object(obj)))
    }

    fn parse_int_literal(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Number(ref mut int) => Some(Expr::Literal(Literal::Number(*int))),
            _ => None,
        }
    }

    fn parse_boolean_literal(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Boolean(boolean) => Some(Expr::Literal(Literal::Boolean(boolean))),
            _ => None,
        }
    }

    fn parse_string_literal(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::String(ref mut str) => Some(Expr::Literal(Literal::String(str.clone()))),
            _ => None,
        }
    }

    fn parse_array_literal(&mut self) -> Option<Expr> {
        self.parse_expr_list(Token::RightBracket)
            .map(|list| Expr::Literal(Literal::Array(list)))
    }

    

    fn parse_expr_list(&mut self, end: Token) -> Option<Vec<Expr>> {
        let mut list = vec![];
        if self.peek_token(&end) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        match self.parse_expr(Precedence::Lowest) {
            Some(a) => list.push(a),
            None => return None,
        };
        while self.peek_token(&Token::Comma) {
            self.next_token();
            if self.peek_token(&Token::RightBracket) {
                break;
            }
            self.next_token();

            match self.parse_expr(Precedence::Lowest) {
                Some(expr) => list.push(expr),
                None => return None,
            }
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(list)
    }

    fn parse_ident(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Ident(ref mut ident) => Some(Expr::Ident(Ident(ident.clone()))),
            _ => None,
        }
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.current_token {
            Token::Bang => Prefix::Exclamation,
            Token::Minus => Prefix::Minus,
            Token::Plus => Prefix::Plus,
            _ => return None,
        };

        self.next_token();

        self.parse_expr(Precedence::Prefix)
            .map(|expr| Expr::Prefix(prefix, Box::new(expr)))
    }

    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.current_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Divide,
            Token::Asterisk => Infix::Times,
            Token::Percent => Infix::Modulo,
            Token::Equals => Infix::Equals,
            Token::NotEquals => Infix::NotEquals,
            Token::Less => Infix::LessThan,
            Token::Greater => Infix::GreaterThan,
            Token::LessEqual => Infix::LessThanEqual,
            Token::GreaterEqual => Infix::GreaterThanEqual,
            Token::LeftShift => Infix::LeftShift,
            Token::RightShift => Infix::RightShift,
            Token::AND => Infix::AND,
            Token::OR => Infix::OR,
            Token::XOR => Infix::XOR,
            Token::In => Infix::In,
            _ => return None,
        };

        let precedence = self.current_token_precedence();
        self.next_token();

        self.parse_expr(precedence)
            .map(|e| Expr::Infix(infix, Box::new(left), Box::new(e)))
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        let exp = self.parse_expr(Precedence::Lowest);
        if !self.expect_peek(Token::RightParen) {
            return None;
        }
        exp
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        if !self.expect_peek(Token::LeftParen) {
            return None;
        }

        self.next_token();

        let expr: Expr = match self.parse_expr(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };

        if !self.expect_peek(Token::RightParen) || !self.expect_peek(Token::LeftBrace) {
            return None;
        }

        let cons: Vec<Statement> = self.parse_block_statement();
        let mut alternative: Option<Vec<Statement>> = None;
        if self.peek_token(&Token::Else) {
            self.next_token();

            if self.peek_token(&Token::If) {
                self.next_token();
                let else_if = self.parse_if_expr();
                alternative = Some(vec![Statement::Expression(else_if.unwrap())]);
            } else if !self.expect_peek(Token::LeftBrace) {
                return None;
            } else {
                alternative = Some(self.parse_block_statement())
            };
        }

        Some(Expr::If {
            cond: Box::new(expr),
            then: Box::new(cons),
            else_: alternative,
        })
    }

    fn parse_fn_expr(&mut self) -> Option<Expr> {
        if !self.expect_peek(Token::LeftParen) {
            return None;
        }
        let params = match self.parse_params() {
            Some(s) => s,
            None => return None,
        };
        self.next_token();
        let body = self.parse_block_statement();
        
        Some(Expr::Fun { params, body })
    }

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Equals | Token::NotEquals => Precedence::Equals,
            Token::Less | Token::LessEqual => Precedence::LessGreater,
            Token::Greater | Token::GreaterEqual => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk | Token::Percent => Precedence::Product,
            Token::LeftBracket => Precedence::Index,
            Token::LeftParen => Precedence::Call,
            Token::In => Precedence::In,
            Token::LeftShift => Precedence::LeftShift,
            Token::RightShift => Precedence::RightShift,
            Token::AND => Precedence::AND,
            Token::OR => Precedence::OR,
            Token::XOR => Precedence::XOR,
            _ => Precedence::Lowest,
        }
    }

    fn parse_params(&mut self) -> Option<Vec<Ident>> {
        let mut idents: Vec<Ident> = vec![];
        if self.peek_token(&Token::RightParen) {
            self.next_token();
            return Some(idents);
        }

        self.next_token();
        match self.current_token {
            Token::Ident(ref mut ident) => idents.push(Ident(ident.clone())),
            _ => {
                self.errors.push(
                   format!("Expected identifier as parameter name. Got: {}", self.current_token)
                );
                return None;
            }
        };

        while self.peek_token(&Token::Comma) {
            self.next_token();
            self.next_token();
            match self.current_token {
                Token::Ident(ref mut ident) => idents.push(Ident(ident.clone())),
                _ => return None,
            };
        }

        if !self.expect_peek(Token::RightParen) {
            return None;
        }

        Some(idents)
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expr>> {
        let mut args: Vec<Expr> = vec![];

        if self.peek_token(&Token::RightParen) {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        match self.parse_expr(Precedence::Lowest) {
            Some(e) => args.push(e),
            None => return None,
        };

        while self.peek_token(&Token::Comma) {
            self.next_token();
            self.next_token();

            match self.parse_expr(Precedence::Lowest) {
                Some(e) => args.push(e),
                None => return None,
            };
        }

        if !self.expect_peek(Token::RightParen) {
            return None;
        }

        Some(args)
    }

    fn parse_index_expr(&mut self, left: Expr) -> Option<Expr> {
        self.next_token();
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };
        if !self.expect_peek(Token::RightBracket) {
            return None;
        }

        Some(Expr::Index {
            array: Box::new(left),
            index: Box::new(expr),
        })
    }

    fn parse_call_expr(&mut self, left: Expr) -> Option<Expr> {
        let args = match self.parse_call_arguments() {
            Some(e) => e,
            None => return None,
        };

        Some(Expr::Call {
            function: Box::new(left),
            args,
        })
    }

    fn peek_token(&self, t: &Token) -> bool {
        self.peek_token == *t
    }

    fn current_token(&self, t: Token) -> bool {
        self.current_token == t
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if let Token::Ident(..) = t {
            self.next_token();
            return true;
        }

        if self.peek_token(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!(
            "Expected next token to be {}, got {} instead",
            t,
            self.peek_token
        );
        self.errors.push(msg);
    }

    fn current_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.current_token)
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.peek_token)
    }
}