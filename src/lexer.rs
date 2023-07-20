use crate::ast::token::Token;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut keywords = HashMap::new();
        keywords.insert("scratch", Token::Set);
        keywords.insert("amew", Token::Anew);
        keywords.insert("pawction", Token::Func);
        keywords.insert("purrhaps", Token::If);
        keywords.insert("meowtually", Token::Else);
        keywords.insert("tail", Token::Return);
        keywords.insert("pawckage", Token::Include);
        keywords.insert("purrfect", Token::Boolean(true));
        keywords.insert("clawful", Token::Boolean(false));
        keywords.insert("furreal", Token::Typeof);
        keywords.insert("furrever", Token::Loop);
        keywords.insert("hiss", Token::Break);
        keywords.insert("continue", Token::Continue);

        // Kaomojis
        /* 
        keywords.insert("＾◡＾っ", Token::Return);
        keywords.insert("╯°□°╯", Token::Break);
        keywords.insert("≧^◡^≦", Token::Boolean(true));
        keywords.insert("｡•́︿•̀｡", Token::Boolean(false));
        keywords.insert("=^･_･^=〇", Token::Continue);
        keywords.insert("◔.◔", Token::If);
        keywords.insert("´υ｀", Token::Else);
        keywords.insert("๑╹o╹", Token::Set);
        keywords.insert("o_o", Token::Func);
        keywords.insert(">人<", Token::Else);
        */
        keywords
    };
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn find_indentifier(ident: &str) -> Option<&Token> {
    KEYWORDS.get(ident)
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            ch: input.chars().next().unwrap(),
            input,
            position: 0,
            read_position: 1,
        }
    }

    pub fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let pos: usize = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        self.input[pos..self.position].to_string()
    }

    fn read_number(&mut self) -> f64 {
        let pos: usize = self.position;
        while self.ch.is_numeric() || self.ch == '.' {
            self.read_char();
        }
        self.input[pos..self.position].parse::<f64>().unwrap()
    }

    fn read_string(&mut self) -> String {
        let pos: usize = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == (0 as char) {
                break;
            }
        }
        self.input[pos..self.position].to_string()
    }

    pub fn read_comment(&mut self) -> String {
        let pos: usize = self.position;
        loop {
            self.read_char();
            if self.ch == '\n' || self.ch == (0 as char) {
                break;
            }
        }
        self.input[pos..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return 0 as char;
        }
        self.input.chars().nth(self.read_position).unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Equals
                } else {
                    Token::Assign
                }
            }
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '&' => Token::AND,
            '|' => Token::OR,
            '^' => Token::XOR,
            '%' => Token::Percent,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEquals
                } else {
                    Token::Bang
                }
            }
            '~' => Token::In,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '*' => Token::Asterisk,
            '/' => {
                if self.peek_char() == '/' {
                    self.read_comment();
                    Token::Comment
                } else {
                    Token::Slash
                }
            }
            '<' => match self.peek_char() {
                    '<' => Token::LeftShift,
                    '=' => Token::LessEqual,
                    _ => Token::Less
                },
            '>' => match self.peek_char() {
                    '>' => Token::RightShift,
                    '=' => Token::GreaterEqual,
                    _ => Token::Greater
                },
            '"' => Token::String(self.read_string()),
            '\u{0}' => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let i: String = self.read_identifier();
                    return match find_indentifier(i.as_str()) {
                        Some(a) => a.to_owned(),
                        _ => Token::Ident(i),
                    };
                } else if self.ch.is_numeric() {
                    let i: f64 = self.read_number();
                    return Token::Number(i);
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }

}